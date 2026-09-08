#!/usr/bin/env python3
"""Audit paired manifests, frozen inputs, complete outcomes and lifecycle accounting."""
import argparse
import json
from pathlib import Path
import statistics

from closed_subtree_pilot import OUT, CELLS, VERSIONS, manifest, write_json
from regional_pilot import digest

COUNTS = dict(zip(['one-zero', 'one-work', 'two-work', 'owner-product', 'asym-first',
                   'asym-last', 'duplicate-eight', 'mixed-add-infer'],
                  [(2, 2), (2, 2), (4, 4), (256, 256), (16, 16), (16, 16), (256, 1), (1, 1)]))


def validate_result(row):
    d = row['result']
    assert d['schema_version'] == 1
    assert [d[k] for k in ['case', 'mode', 'quantum', 'limit']] == row['cell']
    work, meter = row['kind'] == 'work', row['kind'] == 'allocation'
    for flag in ['metrics', 'persistent_metrics', 'kernel_metrics', 'observer_metrics', 'compiled_metrics']:
        assert d[flag] is work, flag
    assert d['metered'] is meter
    assert (d['work'] is not None) == work
    for field in ['construct_ns', 'search_ns', 'shutdown_ns', 'drop_ns', 'contiguous_ns',
                  'validation_ns', 'output_drop_ns', 'total_ns', 'service_calls', 'budget', 'answers']:
        assert type(d[field]) is int and d[field] >= 0, field
    assert d['total_ns'] == d['contiguous_ns'] + d['output_drop_ns']
    assert sum(d[k] for k in ['construct_ns', 'search_ns', 'shutdown_ns', 'drop_ns']) <= d['contiguous_ns']
    if d['answers']:
        assert d['construct_ns'] <= d['first_answer_ns'] <= d['contiguous_ns']
    else:
        assert d['first_answer_ns'] is None
    expected_budget = 20_000_000 if d['mode'] == 'Specialized' else (100_000 if d['case'] == 'mixed-add-infer' else 5_000_000)
    assert d['budget'] == expected_budget
    assert d['service_calls'] <= d['budget']
    if d['status'] == 'cutoff':
        assert d['exhausted'] is False and d['service_calls'] == d['budget']
    else:
        assert d['status'] == 'complete' and d['exhausted'] is True
        assert (d['raw'], d['answers']) == COUNTS[d['case']]
    life, drop = d['memory']['lifecycle'], d['memory']['output_drop']
    if meter:
        for reading in [life, drop]:
            for k in ['allocation_calls', 'requested_bytes', 'baseline_live', 'peak_live', 'final_live']:
                assert type(reading[k]) is int and reading[k] >= 0
            assert reading['peak_live'] >= max(reading['baseline_live'], reading['final_live'])
        assert life['final_live'] == drop['baseline_live']
        retained = drop['final_live'] - life['baseline_live']
        assert d['retained_bytes'] == retained
        assert retained in ([0, 48] if d['mode'].startswith('Threads') else [0]), ('unexpected retention', retained)
    else:
        assert life is None and drop is None and d['retained_bytes'] is None
    if work:
        w = d['work']; t = w['transport']
        assert w['actual_applications'] >= w['applications']
        if d['mode'] != 'Specialized':
            assert t['issued'] == t['received']
            assert t['issued'] - t['accepted'] == t['unaccepted_at_shutdown']
            assert t['outstanding'] == t['buffered'] == 0
            assert t['max_outstanding'] <= d['limit'] and t['owner_buffered_peak'] <= d['limit']
            assert w['actual_source_steps'] == t['actual_source_steps'] >= t['accepted_source_steps'] == w['accepted_source_steps']
        else:
            assert all(value == 0 for value in t.values())


def audit(out, live_verify=True):
    meta = json.loads((out / 'metadata.json').read_text())
    assert meta['schema_version'] == 1 and meta['seed'] == 44045
    assert meta['order'] == manifest() and len(meta['order']) == 288
    assert all(meta['commits'][v].startswith(ref) for v, ref in VERSIONS.items())
    assert (meta['timeout_seconds'], meta['address_space_bytes'], meta['batch_seconds']) == (30, 1073741824, 1200)
    hardware = meta['hardware']
    assert hardware['child_affinity'] == [0, 2, 4]
    assert {0, 2, 4} <= set(hardware['parent_allowed'])
    assert set(hardware['topology']) == {'0', '2', '4'}
    assert len({(r['physical_package_id'], r['core_id']) for r in hardware['topology'].values()}) == 3
    assert meta['ancestor_limits']
    for limits in meta['ancestor_limits'].values():
        if limits['cpu.max'] and not limits['cpu.max'].startswith('max '):
            quota, period = map(int, limits['cpu.max'].split())
            assert quota / period >= 3
    check = json.loads((out / 'source-check.json').read_text())
    assert check['frozen'] == meta['frozen'] and not check['changed']
    if live_verify:
        assert all(Path(p).is_file() and digest(Path(p)) == sha for p, sha in meta['frozen'].items()), 'freeze mismatch or unavailable; --archived checks stored evidence only'
    assert set(meta['sources']) == set(VERSIONS)
    for version, sources in meta['sources'].items():
        for path, sha in sources.items():
            assert meta['frozen'][str(Path(meta['roots'][version]) / path)] == sha
    for version in VERSIONS:
        assert {'Cargo.toml', 'Cargo.lock'} <= set(meta['sources'][version])
        for kind in ['primary', 'allocation', 'work']:
            command = meta['build_commands'][version][kind]
            assert '--locked' in command and '--release' in command
            assert ('--no-default-features' in command) == (kind != 'work')
            assert command[command.index('--features') + 1] == 'lifecycle'
            assert meta['binaries'][version][kind] in meta['frozen']
    for path, sha in meta['sources']['baseline'].items():
        if path.startswith('research/chr-factors/examples/'):
            assert sha == meta['sources']['current'][path]
    rows = [json.loads(line) for line in (out / 'runs.jsonl').read_text().splitlines()]
    assert len(rows) <= 288
    failures, successful = [], []
    for index, row in enumerate(rows):
        assert all(row[k] == meta['order'][index][k] for k in ['kind', 'rep', 'version', 'cell'])
        if not row.get('attempted') or row.get('exit') != 0 or 'result' not in row or row.get('validation_error'):
            failures.append(row); continue
        validate_result(row)
        if row['result']['status'] != 'complete': failures.append(row)
        else: successful.append(row)
    for cell in CELLS:
        paired = [next((r['result'] for r in successful if r['version'] == v and
                        r['kind'] == 'work' and r['cell'] == list(cell)), None) for v in VERSIONS]
        if all(paired):
            assert paired[0]['work']['applications'] == paired[1]['work']['applications'], ('source applications', cell)
            assert (paired[0]['raw'], paired[0]['answers']) == (paired[1]['raw'], paired[1]['answers'])
    summary = []
    for version in VERSIONS:
        for cell in CELLS:
            group = [r for r in successful if r['version'] == version and r['cell'] == list(cell)]
            primary = sorted([r for r in group if r['kind'] == 'primary'], key=lambda r: r['rep'])
            entry = dict(version=version, cell=list(cell), primary_complete=len(primary),
                         samples_ns=[r['result']['total_ns'] for r in primary],
                         complete_comparison=len(primary) == 5)
            if len(primary) == 5:
                for field in ['total_ns', 'contiguous_ns', 'construct_ns', 'first_answer_ns', 'search_ns',
                              'shutdown_ns', 'drop_ns', 'validation_ns', 'output_drop_ns']:
                    values = [r['result'][field] for r in primary]
                    entry[field] = dict(samples=values, min=min(values), median=statistics.median(values), max=max(values))
            for kind in ['allocation', 'work']:
                sample = next((r['result'] for r in group if r['kind'] == kind), None)
                entry[kind] = sample
            summary.append(entry)
    result = dict(freeze_verification='live' if live_verify else 'archived-source-check-only', planned=288, recorded=len(rows), missing=288-len(rows),
                  attempted=sum(bool(r.get('attempted')) for r in rows), failures=failures,
                  complete_primary_cells=sum(s['complete_comparison'] for s in summary))
    write_json(out / 'audit.json', result)
    write_json(out / 'summary.json', summary)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, default=OUT)
    parser.add_argument('--archived', action='store_true', help='Verify stored freeze receipt only; do not claim live source/binary verification')
    args = parser.parse_args()
    result = audit(args.out, live_verify=not args.archived)
    print(json.dumps(result))
    if result['missing'] or result['failures']: raise SystemExit(1)


if __name__ == '__main__':
    main()
