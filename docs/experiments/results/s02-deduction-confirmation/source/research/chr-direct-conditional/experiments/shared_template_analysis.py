#!/usr/bin/env python3
"""Registered pointwise paired analysis; no aggregate architecture ranking."""
import hashlib
import json
import math
import random
import statistics
from pathlib import Path
import derivation_sizing as runner

OUT = runner.ROOT / 'docs/experiments/results/s03-shared-template-bounded'
PHASES = ['setup', 'execute_observe', 'engine_drop', 'answer_drop']


def read(version, kind, index):
    path = OUT / version / f'{kind}-{index:04}.json'
    raw = json.loads(path.read_text())
    assert raw['returncode'] == 0 and not raw.get('timeout'), path
    value = json.loads(raw['stdout'].splitlines()[-1])
    assert not value['counters']
    assert value['meter'] == (kind in ['allocation', 'cancel-meter'])
    assert all(s['complete'] == (not kind.startswith('cancel') or i % 2 == 1) for i, s in enumerate(value['samples']))
    return value


def phases(value):
    result = {'preparation': value['preparation'], 'prepared_drop': value['prepared_drop']}
    for i, sample in enumerate(value['samples']):
        result.update({f'{i}:{k}': sample[k] for k in PHASES})
    return result


def primary(value):
    return sum(p['ns'] for p in phases(value).values())


def main():
    freeze = json.loads((OUT / 'freeze.json').read_text())
    configs = freeze['configs']
    for path, expected in freeze['sources'].items():
        assert hashlib.sha256((OUT / 'source' / path).read_bytes()).hexdigest() == expected
    for path, expected in freeze['binaries'].items():
        assert hashlib.sha256(Path(path).read_bytes()).hexdigest() == expected
    results = []
    for i, config in enumerate(configs):
        values = {v: [read(v, 'ordinary', r * len(configs) + i) for r in range(7)] for v in ['before', 'after']}
        logs = [math.log(primary(a) / primary(b)) for a, b in zip(values['after'], values['before'])]
        rng = random.Random(7122 + i)
        draws = sorted(math.exp(sum(rng.choices(logs, k=7)) / 7) for _ in range(10000))
        lo, hi = draws[249], draws[9749]
        item = {'config': config, 'ratio': math.exp(statistics.mean(logs)), 'interval95': [lo, hi],
                'classification': 'gain' if hi < .9 else 'loss' if lo > 1.1 else 'unresolved'}
        for version in values:
            samples = values[version]
            meter = read(version, 'allocation', i)
            replay = read(version, 'allocation', len(configs) + i)
            assert runner.allocation_records(meter) == runner.allocation_records(replay)
            for sample in meter['samples']:
                assert sample['input_build']['memory']['live_start'] == sample['answer_drop']['memory']['live_end']
            primary_phases = list(phases(meter).values())
            all_phases = primary_phases + [meter['source_build']] + [s['input_build'] for s in meter['samples']]
            item[version] = {
                'primary_median_ms': statistics.median(primary(v) for v in samples) / 1e6,
                'primary_requested_bytes': sum(p['memory']['requested_bytes'] for p in primary_phases),
                'peak_requested_growth_bytes': max(p['memory']['peak_live'] for p in all_phases) - meter['source_build']['memory']['live_start'],
                'source_input_inclusive_median_ms': statistics.median(primary(v) + v['source_build']['ns'] + sum(s['input_build']['ns'] for s in v['samples']) for v in samples) / 1e6,
                'first_answer_median_ms': statistics.median(s['first_answer_ns'] for v in samples for s in v['samples']) / 1e6,
                'phase_median_ms': {k: statistics.median(sum(s[k]['ns'] for s in v['samples']) for v in samples) / 1e6 for k in PHASES},
            }
        results.append(item)
    for version in ['before', 'after']:
        for kind in ['cancel', 'cancel-meter']:
            for i in range(2):
                read(version, kind, i)
    counts = {mode: {c: sum(x['config'][0] == mode and x['classification'] == c for x in results)
                     for c in ['gain', 'loss', 'unresolved']} for mode in ['templates', 'dependencies']}
    summary = {'interpretation': 'pointwise repair attribution, not an architecture ranking', 'counts': counts, 'results': results}
    (OUT / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
    print(json.dumps(counts))
    for row in results:
        c = row['config']
        if c[0] == 'templates' and c[2] > 0 and c[3:] == [8, True, False]:
            print(c[1], json.dumps(row))


if __name__ == '__main__':
    main()
