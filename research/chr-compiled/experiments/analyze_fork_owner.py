#!/usr/bin/env python3
"""Audit the registered allocation-only fork-owner experiment (T051)."""
import argparse
import hashlib
import json
import pathlib
import random

from analyze_state_preservation import check_row as check_lifecycle, phases

ROOT = pathlib.Path(__file__).resolve().parents[3]
OWNERS = ('rules regions arena_nodes arena_closed arena_intern arena_predicates '
          'arena_predicate_lookup bindings store pools dispatch history pending outputs '
          'queue queued dependencies watchers index occurrence_keys trace audit search').split()
TRAFFIC = ('allocation_calls', 'requested_bytes', 'deallocation_calls')


def without_times(value):
    if isinstance(value, dict):
        return {k: without_times(v) for k, v in value.items()
                if k != 'ns' and not k.endswith('_ns')}
    if isinstance(value, list):
        return [without_times(v) for v in value]
    return value


def check_row(row):
    failures, answers = check_lifecycle(row)
    if failures:
        return failures, answers
    for sample in row['result']['samples']:
        fork = sample['fork_diagnostics']
        assert list(fork['owners']) == OWNERS
        split = sample['diagnostic']['split']
        for owner in [*fork['owners'].values(), fork['remainder']]:
            assert owner['count'] == sample['splits']
            assert all(isinstance(v, int) and v >= 0 for v in owner.values())
        for key in TRAFFIC:
            inclusive = split['memory'][key] if split['memory'] else 0
            assert sum(v[key] for v in fork['owners'].values()) + fork['remainder'][key] == inclusive
        assert sum(v['ns'] for v in fork['owners'].values()) + fork['remainder']['ns'] == split['ns']
        for name in ('rules', 'regions', 'dispatch', 'bindings'):
            assert all(fork['owners'][name][k] == 0 for k in TRAFFIC)
        segments = fork['segments']
        assert set(segments) == {'split', 'failed', 'complete'}
        expected = {'split': max(sample['splits'] - 1, 0),
                    'failed': sample['failed'] if sample['splits'] else 0,
                    'complete': sample['answers'] if sample['splits'] else 0}
        assert sum(v['segments'] for v in segments.values()) == 2 * sample['splits']
        for endpoint, v in segments.items():
            assert v['segments'] == expected[endpoint]
            assert all(isinstance(x, int) and x >= 0 for x in v.values())
            assert v['mutation_free_segments'] + v['first_miss_segments'] <= v['segments']
            assert v['mutation_free_inherited_nodes'] + v['first_miss_inherited_nodes'] <= v['inherited_nodes']
            assert v['first_miss_nodes'] == v['first_miss_inherited_nodes']
            if not v['first_miss_segments']:
                assert v['first_miss_nodes'] == v['requests_before_first_miss'] == 0
            if not v['predicate_insertions']:
                assert v['mutation_free_segments'] + v['first_miss_segments'] == v['segments']
                assert v['mutation_free_inherited_nodes'] + v['first_miss_inherited_nodes'] == v['inherited_nodes']
        assert sum(v['first_miss_segments'] for v in segments.values()) <= fork['interning']['misses']
        if sample['splits'] == 0:
            assert not any(fork['interning'].values())
            assert not any(x for v in segments.values() for x in v.values())
    return failures, answers


def audit(out):
    meta = json.loads((out / 'metadata.json').read_text())
    assert meta['seed'] == 51051
    assert meta['bounds'] == {'process_seconds': 30, 'process_address_bytes': 1024**3,
                              'execution_seconds': 600, 'build_seconds': 180}
    cells = [(n, a, o, q) for n in (0, 64, 512) for a in (1, 8, 64)
             for o in ('mostly-fail', 'all-success') for q in (1, 4)]
    planned = [('allocation', rep, cell) for rep in range(2) for cell in cells]
    random.Random(51051).shuffle(planned)
    assert [(j['mode'], j['rep'], tuple(j['cell'])) for j in meta['jobs']] == planned
    rows = [json.loads(line) for line in (out / 'runs.jsonl').read_text().splitlines()] if (out / 'runs.jsonl').exists() else []
    assert [(r['mode'], r['rep'], tuple(r['cell'])) for r in rows] == planned[:len(rows)]
    build_command = meta.get('builds', {}).get('allocation', {}).get('command', [])
    executable = (str(pathlib.Path(build_command[build_command.index('--target-dir') + 1]) / 'release/chr-state-preservation-cost')
                  if '--target-dir' in build_command else None)
    failures, answers, valid = [], 0, set()
    for row in rows:
        try:
            assert row['command'] == [executable, *map(str, row['cell']), 'read']
            found, count = check_row(row)
            failures.extend(found)
            answers += count
            if not found:
                valid.add(id(row))
        except (AssertionError, KeyError, TypeError) as error:
            failures.append({'cell': row['cell'], 'rep': row['rep'], 'invalid': repr(error)})
    summary = []
    for cell in cells:
        group = [r for r in rows if tuple(r['cell']) == cell]
        item = {'cell': cell, 'recorded': len(group)}
        if len(group) == 2 and all(id(r) in valid for r in group):
            repeated = without_times(group[0]['result']) == without_times(group[1]['result'])
            item['exact_non_time_repeat'] = repeated
            if not repeated:
                failures.append({'cell': cell, 'repeat_mismatch': True})
            d = group[0]['result']
            item['samples'] = []
            for s in d['samples']:
                fork = s['fork_diagnostics']
                split_bytes = s['diagnostic']['split']['memory']['requested_bytes'] if s['splits'] else 0
                arena_bytes = sum(v['requested_bytes'] for k, v in fork['owners'].items() if k.startswith('arena_'))
                item['samples'].append({'n': s['n'], 'splits': s['splits'], 'answers': s['answers'],
                    'failed': s['failed'], 'split_requested_bytes': split_bytes,
                    'arena_requested_bytes': arena_bytes,
                    'arena_split_fraction': arena_bytes / split_bytes if split_bytes else None,
                    'owners': without_times(fork['owners']), 'remainder': without_times(fork['remainder']),
                    'interning': fork['interning'], 'segments': fork['segments']})
            item['lifecycle_requested_bytes'] = sum(p['memory']['requested_bytes'] for p in phases(d))
        summary.append(item)
    freeze_errors = []
    for name, expected in meta['sources'].items():
        if hashlib.sha256((ROOT / name).read_bytes()).hexdigest() != expected:
            freeze_errors.append(name)
    build = meta.get('builds', {}).get('allocation', {})
    if build.get('status') != 'complete':
        freeze_errors.append('allocation-build')
    else:
        command = build['command']
        assert command[command.index('--features') + 1] == 'alloc-meter,fork-diagnostics'
        assert '--no-default-features' in command
        binary = pathlib.Path(command[command.index('--target-dir') + 1]) / 'release/chr-state-preservation-cost'
        if hashlib.sha256(binary.read_bytes()).hexdigest() != build['binary_sha256']:
            freeze_errors.append('allocation-binary')
    result = {'planned': len(planned), 'recorded': len(rows), 'missing': len(planned)-len(rows),
              'failures': failures, 'validated_answers': answers,
              'exact_repeat_cells': sum(s.get('exact_non_time_repeat', False) for s in summary),
              'freeze_errors': freeze_errors}
    (out / 'audit.json').write_text(json.dumps(result, indent=2) + '\n')
    (out / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
    print(json.dumps(result))
    return result


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', type=pathlib.Path, required=True)
    result = audit(parser.parse_args().output.resolve())
    raise SystemExit(bool(result['failures'] or result['missing'] or result['freeze_errors']))
