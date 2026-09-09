#!/usr/bin/env python3
"""Registered pointwise full-control comparison of equality representations."""
import hashlib
import json
import equality_map_analysis as base
OUT = base.sizing.ROOT / 'docs/experiments/results/s02-deduction-confirmation'
CONTRASTS = [('shared', 'contextual'), ('persistent-shared', 'persistent'), ('persistent-shared', 'shared'),
             ('persistent', 'contextual'), ('persistent-shared', 'contextual'), ('shared', 'relational'),
             ('persistent-shared', 'relational'), ('contextual', 'scan'), ('shared', 'scan'),
             ('persistent-shared', 'scan'), ('persistent-shared', 'sealed'), ('lowered', 'persistent-shared'), ('lowered', 'scan')]


def read(kind, index, config=None):
    value = base.read(OUT, kind, index)
    if config:
        raw = json.loads((OUT / f'{kind}-{index:04}.json').read_text())
        expected = [str(int(x)) if isinstance(x, bool) else str(x) for x in config]
        assert raw['command'][1:] == expected
        assert all(s['answers'] == (1 if config[1] == 'single' else 4) for s in value['samples'])
    return value


def main():
    freeze = json.loads((OUT / 'freeze.json').read_text())
    configs, modes = freeze['configs'], freeze['modes']
    for p, digest in freeze['sources'].items():
        assert hashlib.sha256((OUT / 'source' / p).read_bytes()).hexdigest() == digest
    results = []
    for i, config in enumerate(configs):
        values = {m: [read('ordinary', (r * len(configs) + i) * len(modes) + j, (m, *config)) for r in range(7)] for j, m in enumerate(modes)}
        row = {'config': config, 'modes': {}, 'contrasts': {}}
        for j, mode in enumerate(modes):
            k = i * len(modes) + j
            row['modes'][mode] = base.describe(values[mode], read('allocation', k, (mode, *config)), read('allocation', len(configs) * len(modes) + k, (mode, *config)))
        for j, (a, b) in enumerate(CONTRASTS):
            row['contrasts'][f'{a}/{b}'] = base.contrast(values[a], values[b], 7232 + 13 * i + j)
        results.append(row)
    for kind in ['cancel', 'cancel-meter']:
        for i in range(18):
            read(kind, i)
    counts = {f'{a}/{b}': {c: sum(r['contrasts'][f'{a}/{b}']['classification'] == c for r in results) for c in ['gain', 'loss', 'unresolved']} for a, b in CONTRASTS}
    (OUT / 'summary.json').write_text(json.dumps({'counts': counts, 'results': results}, indent=2) + '\n')
    print(json.dumps(counts, indent=2))
    for row in results:
        if row['config'][1:] == [32, 4, True, False]:
            print(row['config'][0], {m: round(v['primary_median_ms'], 3) for m, v in row['modes'].items()})


if __name__ == '__main__':
    main()
