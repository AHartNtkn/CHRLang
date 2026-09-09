#!/usr/bin/env python3
"""Prospective pointwise analysis of the repaired derivation confirmation."""
import hashlib
import json
import math
import random
import statistics
from pathlib import Path
import derivation_sizing as runner
from shared_template_analysis import phases, primary, PHASES

OUT = runner.ROOT / 'docs/experiments/results/s08-observation-control-confirmation'
CONTRASTS = [('templates', 'dependencies'), ('templates', 'scan'), ('templates', 'indexed'),
             ('templates', 'sealed'), ('lowered', 'templates'), ('lowered', 'sealed')]


def read(kind, index, config=None):
    p = OUT / f'{kind}-{index:04}.json'
    raw = json.loads(p.read_text())
    assert raw['returncode'] == 0 and not raw.get('timeout'), p
    value = json.loads(raw['stdout'].splitlines()[-1])
    assert not value['counters'] and value['meter'] == (kind in ['allocation', 'cancel-meter'])
    assert all(s['complete'] == (not kind.startswith('cancel') or i % 2 == 1) for i, s in enumerate(value['samples']))
    if config:
        expected = [str(int(x)) if isinstance(x, bool) else str(x) for x in config]
        assert raw['command'][1:] == expected
        assert all(s['answers'] == (16 if config[1] == 'choice' else 1) for s in value['samples'])
    return value


def main():
    freeze = json.loads((OUT / 'freeze.json').read_text())
    configs, modes = freeze['configs'], freeze['modes']
    for p, digest in freeze['sources'].items():
        assert hashlib.sha256((OUT / 'source' / p).read_bytes()).hexdigest() == digest
    results = []
    for i, config in enumerate(configs):
        values = {m: [read('ordinary', (r * len(configs) + i) * len(modes) + j, (m, *config)) for r in range(7)] for j, m in enumerate(modes)}
        item = {'config': config, 'modes': {}, 'contrasts': {}}
        for j, mode in enumerate(modes):
            index = i * len(modes) + j
            meter = read('allocation', index, (mode, *config))
            replay = read('allocation', len(configs) * len(modes) + index, (mode, *config))
            assert runner.allocation_records(meter) == runner.allocation_records(replay)
            for s in meter['samples']:
                assert s['input_build']['memory']['live_start'] == s['answer_drop']['memory']['live_end']
            measured = list(phases(meter).values())
            all_phases = measured + [meter['source_build']] + [s['input_build'] for s in meter['samples']]
            item['modes'][mode] = {
                'median_primary_ms': statistics.median(primary(v) for v in values[mode]) / 1e6,
                'median_inclusive_ms': statistics.median(primary(v) + v['source_build']['ns'] + sum(s['input_build']['ns'] for s in v['samples']) for v in values[mode]) / 1e6,
                'requested_bytes': sum(p['memory']['requested_bytes'] for p in measured),
                'peak_requested_growth_bytes': max(p['memory']['peak_live'] for p in all_phases) - meter['source_build']['memory']['live_start'],
                'median_first_answer_ms': statistics.median(s['first_answer_ns'] for v in values[mode] for s in v['samples']) / 1e6,
                'median_phase_ms': {k: statistics.median(sum(s[k]['ns'] for s in v['samples']) for v in values[mode]) / 1e6 for k in PHASES},
            }
        for j, (a, b) in enumerate(CONTRASTS):
            logs = [math.log(primary(x) / primary(y)) for x, y in zip(values[a], values[b])]
            rng = random.Random(7762 + 6 * i + j)
            draws = sorted(math.exp(sum(rng.choices(logs, k=7)) / 7) for _ in range(10000))
            lo, hi = draws[249], draws[9749]
            item['contrasts'][f'{a}/{b}'] = {'ratio': math.exp(statistics.mean(logs)), 'interval95': [lo, hi],
                'classification': 'gain' if hi < .9 else 'loss' if lo > 1.1 else 'unresolved'}
        results.append(item)
    for kind in ['cancel', 'cancel-meter']:
        for i in range(12):
            read(kind, i)
    counts = {f'{a}/{b}': {c: sum(r['contrasts'][f'{a}/{b}']['classification'] == c for r in results)
                          for c in ['gain', 'loss', 'unresolved']} for a, b in CONTRASTS}
    (OUT / 'summary.json').write_text(json.dumps({'counts': counts, 'results': results}, indent=2) + '\n')
    print(json.dumps(counts, indent=2))
    for r in results:
        if r['config'][1] > 0 and r['config'][2:] == [8, True, False]:
            print(r['config'][0], {m: round(v['median_primary_ms'], 3) for m, v in r['modes'].items()})


if __name__ == '__main__':
    main()
