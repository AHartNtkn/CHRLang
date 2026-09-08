#!/usr/bin/env python3
"""Audit the registered manifest, lifecycle ownership and every recorded outcome."""
import csv
import json
from pathlib import Path
import statistics
from regional_pilot import CELLS, OUT, ROOT, digest


def main():
    meta = json.loads((OUT / 'metadata.json').read_text())
    planned = {(r['kind'], r['rep'], tuple(r['cell'])) for r in meta['order']}
    expected = {(kind, rep, tuple(cell)) for cell in CELLS for kind, rep in
                [('warmup', 0), *[('primary', i) for i in range(5)], ('allocation', 0), ('work', 0)]}
    assert planned == expected and len(meta['order']) == 400
    rows = [json.loads(line) for line in (OUT / 'runs.jsonl').read_text().splitlines()]
    seen = set()
    failures = []
    for index, row in enumerate(rows):
        assert all(row[k] == meta['order'][index][k] for k in ['kind', 'rep', 'cell'])
        key = (row['kind'], row['rep'], tuple(row['cell']))
        assert key in expected and key not in seen, key
        seen.add(key)
        if row.get('exit') != 0 or 'result' not in row:
            failures.append({k: row.get(k) for k in ['kind', 'rep', 'cell', 'exit', 'timeout', 'stderr']})
            continue
        d = row['result']
        assert [d[k] for k in ['case', 'mode', 'quantum', 'limit']] == row['cell']
        for flag in ['metrics', 'persistent_metrics', 'kernel_metrics', 'observer_metrics', 'compiled_metrics']:
            assert d[flag] == (row['kind'] == 'work'), (key, flag)
        assert d['metered'] == (row['kind'] == 'allocation')
        assert (d['work'] is not None) == (row['kind'] == 'work')
        assert d['total_ns'] == d['contiguous_ns'] + d['output_drop_ns']
        assert d['contiguous_ns'] >= sum(d[k] for k in ['construct_ns', 'search_ns', 'shutdown_ns', 'drop_ns'])
        if d['answers']:
            assert d['first_answer_ns'] is not None and d['first_answer_ns'] <= d['contiguous_ns']
        else:
            assert d['first_answer_ns'] is None
        assert d['service_calls'] <= d['budget']
        if d['mode'] == 'Specialized':
            assert d['budget'] == 20_000_000
        if d['status'] == 'cutoff':
            assert not d['exhausted'] and d['service_calls'] == d['budget']
            failures.append({'kind': row['kind'], 'rep': row['rep'], 'cell': row['cell'], 'status': 'cutoff'})
        elif d['case'] == 'stream-prefix':
            assert d['status'] == 'prefix' and not d['exhausted'] and d['raw'] is None and d['answers'] == 8
        else:
            assert d['status'] == 'complete' and d['exhausted'] and d['raw'] is not None
            expected_counts = {'one-zero': (2, 2), 'one-work': (2, 2), 'two-work': (4, 4),
                               'owner-product': (256, 256), 'asym-first': (16, 16),
                               'asym-last': (16, 16), 'duplicate-eight': (256, 1),
                               'mixed-add-infer': (1, 1), 'refute-loop': (0, 0)}
            assert (d['raw'], d['answers']) == expected_counts[d['case']], (key, d['raw'], d['answers'])
        if row['kind'] == 'allocation':
            life, drop = d['memory']['lifecycle'], d['memory']['output_drop']
            assert life['final_live'] == drop['baseline_live']
            retained = drop['final_live'] - life['baseline_live']
            assert d['retained_bytes'] == retained
            allowed = [0, 48] if d['mode'].startswith('Threads') else [0]
            if retained not in allowed:
                failures.append({'kind': 'allocation', 'cell': row['cell'], 'unexpected_retained_bytes': retained})
        else:
            assert d['memory']['lifecycle'] is None and d['memory']['output_drop'] is None
        if row['kind'] == 'work' and d['mode'] != 'Specialized':
            w = d['work']; t = w['transport']
            assert t['issued'] == t['received']
            assert t['issued'] - t['accepted'] == t['unaccepted_at_shutdown']
            assert t['outstanding'] == t['buffered'] == 0
            assert t['max_outstanding'] <= d['limit']
            assert w['actual_source_steps'] == t['actual_source_steps'] >= t['accepted_source_steps'] == w['accepted_source_steps']
    assert not json.loads((OUT / 'source-check.json').read_text())['changed']
    assert all(digest(ROOT / path) == sha for path, sha in meta['source_and_binary_hashes'].items())
    accepted = {}
    for row in rows:
        d = row.get('result', {})
        if row['kind'] == 'work' and d.get('status') in ['complete', 'prefix'] and d['mode'] != 'Specialized':
            key = (d['case'], d['quantum'], d['limit'])
            w = d['work']
            signature = (w['applications'], w['accepted_source_steps'], w['products'], w['source_turns'], d['raw'], d['answers'])
            if key in accepted:
                assert accepted[key] == signature, (key, accepted[key], signature)
            accepted[key] = signature
    summary = []
    for cell in sorted(CELLS):
        group = [r for r in rows if tuple(r['cell']) == cell]
        primary = [r['result'] for r in group if r['kind'] == 'primary' and r.get('result', {}).get('status') in ['complete', 'prefix']]
        out = dict(zip(['case', 'mode', 'quantum', 'limit'], cell))
        out.update(recorded=len(group), primary_complete=len(primary))
        if primary:
            vals = [d['total_ns'] / 1e6 for d in primary]
            out.update(median_ms=statistics.median(vals), min_ms=min(vals), max_ms=max(vals), samples_ms=vals)
            for k in ['construct_ns', 'first_answer_ns', 'search_ns', 'shutdown_ns', 'drop_ns', 'output_drop_ns']:
                values = [d[k] / 1e6 for d in primary if d[k] is not None]
                out[k.replace('_ns', '_ms')] = statistics.median(values) if values else None
        allocation = next((r['result'] for r in group if r['kind'] == 'allocation' and r.get('exit') == 0), None)
        if allocation:
            out['allocation_status'] = allocation['status']
            out['retained_bytes'] = allocation['retained_bytes']
        if allocation and allocation['status'] in ['complete', 'prefix']:
            life, drop = allocation['memory']['lifecycle'], allocation['memory']['output_drop']
            out['requested_bytes'] = life['requested_bytes'] + drop['requested_bytes']
            out['peak_bytes'] = max(life['peak_live'], drop['peak_live']) - life['baseline_live']
        summary.append(out)
    audit = {'planned': len(expected), 'recorded': len(seen), 'missing': len(expected - seen),
             'failures': failures, 'complete_primary_cells': sum(r['primary_complete'] == 5 for r in summary)}
    (OUT / 'audit.json').write_text(json.dumps(audit, indent=2) + '\n')
    (OUT / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
    columns = list(dict.fromkeys(k for row in summary for k in row if k != 'samples_ms'))
    with (OUT / 'summary.tsv').open('w') as output:
        writer = csv.DictWriter(output, fieldnames=columns, delimiter='\t', extrasaction='ignore')
        writer.writeheader(); writer.writerows(summary)
    print(json.dumps(audit))

if __name__ == '__main__':
    main()
