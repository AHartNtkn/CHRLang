#!/usr/bin/env python3
"""Registered pointwise map contrasts and old/new ordered-control calibration."""
import hashlib
import json
import math
import random
import statistics
import deduction_sizing as sizing
from shared_template_analysis import phases, primary, PHASES
OUT = sizing.ROOT / 'docs/experiments/results/s02-equality-map-attribution'
CONTRASTS = [('persistent-shared', 'shared'), ('persistent', 'contextual'), ('shared', 'contextual'), ('persistent-shared', 'persistent')]


def read(root, kind, index):
    raw = json.loads((root / f'{kind}-{index:04}.json').read_text())
    assert raw['returncode'] == 0 and not raw.get('timeout')
    value = json.loads(raw['stdout'].splitlines()[-1])
    assert not value['counters'] and value['meter'] == (kind in ['allocation', 'cancel-meter'])
    assert all(s['complete'] == (not kind.startswith('cancel') or i % 2 == 1) for i, s in enumerate(value['samples']))
    return value


def contrast(a, b, seed):
    logs = [math.log(primary(x) / primary(y)) for x, y in zip(a, b)]
    rng = random.Random(seed)
    draws = sorted(math.exp(sum(rng.choices(logs, k=7)) / 7) for _ in range(10000))
    lo, hi = draws[249], draws[9749]
    return {'ratio': math.exp(statistics.mean(logs)), 'interval95': [lo, hi],
            'classification': 'gain' if hi < .9 else 'loss' if lo > 1.1 else 'unresolved'}


def describe(values, meter, replay):
    assert sizing.runner.allocation_records(meter) == sizing.runner.allocation_records(replay)
    measured = list(phases(meter).values())
    all_phases = measured + [meter['source_build']] + [s['input_build'] for s in meter['samples']]
    for s in meter['samples']:
        assert s['input_build']['memory']['live_start'] == s['answer_drop']['memory']['live_end']
    return {'primary_median_ms': statistics.median(primary(v) for v in values) / 1e6,
            'inclusive_median_ms': statistics.median(primary(v) + v['source_build']['ns'] + sum(s['input_build']['ns'] for s in v['samples']) for v in values) / 1e6,
            'requested_bytes': sum(p['memory']['requested_bytes'] for p in measured),
            'peak_requested_growth_bytes': max(p['memory']['peak_live'] for p in all_phases) - meter['source_build']['memory']['live_start'],
            'first_answer_median_ms': statistics.median(s['first_answer_ns'] for v in values for s in v['samples']) / 1e6,
            'phase_median_ms': {k: statistics.median(sum(s[k]['ns'] for s in v['samples']) for v in values) / 1e6 for k in PHASES}}


def main():
    freeze = json.loads((OUT / 'freeze.json').read_text())
    for p, h in freeze['sources'].items():
        assert hashlib.sha256((OUT / 'source' / p).read_bytes()).hexdigest() == h
    results = []
    configs, modes = freeze['configs'], freeze['modes']
    for i, config in enumerate(configs):
        values = {m: [read(OUT, 'ordinary', (r * len(configs) + i) * len(modes) + j) for r in range(7)] for j, m in enumerate(modes)}
        row = {'config': config, 'modes': {}, 'contrasts': {}}
        for j, mode in enumerate(modes):
            k = i * len(modes) + j
            row['modes'][mode] = describe(values[mode], read(OUT, 'allocation', k), read(OUT, 'allocation', len(configs) * len(modes) + k))
        for j, (a, b) in enumerate(CONTRASTS):
            row['contrasts'][f'{a}/{b}'] = contrast(values[a], values[b], 7222 + 4 * i + j)
        results.append(row)
    calibration = []
    for i, config in enumerate(freeze['calibration']):
        n = len(freeze['calibration'])
        values = {v: [read(OUT / v, 'ordinary', r * n + i) for r in range(7)] for v in ['before', 'after']}
        row = {'config': config, 'contrast': contrast(values['after'], values['before'], 7322 + i)}
        for v in values:
            row[v] = describe(values[v], read(OUT / v, 'allocation', i), read(OUT / v, 'allocation', n + i))
        calibration.append(row)
    for kind in ['cancel', 'cancel-meter']:
        for i in range(8):
            read(OUT, kind, i)
    counts = {f'{a}/{b}': {c: sum(r['contrasts'][f'{a}/{b}']['classification'] == c for r in results) for c in ['gain', 'loss', 'unresolved']} for a, b in CONTRASTS}
    (OUT / 'summary.json').write_text(json.dumps({'counts': counts, 'results': results, 'calibration': calibration}, indent=2) + '\n')
    print(json.dumps(counts, indent=2))
    for row in calibration:
        print('calibration', row['config'][:2], row['contrast'], row['before']['requested_bytes'], row['after']['requested_bytes'])
    for row in results:
        if row['config'][1:] == [32, 4, True, False]:
            print(row['config'][0], row['modes'])


if __name__ == '__main__':
    main()
