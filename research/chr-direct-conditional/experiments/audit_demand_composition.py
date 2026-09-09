#!/usr/bin/env python3
"""Read-only replay audit and complete cell summary for the S10 pilot."""
import csv
import hashlib
import itertools
import json
from pathlib import Path
import statistics
import sys
p = Path(sys.argv[1])
cells = list(itertools.product(
    [f'{lower}-{mode}' for lower in ['original', 'fused'] for mode in
     ['contextual', 'demand']],
    ['plain', 'choices', 'duplicates', 'shared', 'spare', 'history'], [0, 1, 3], [1, 4]))
for name, digest in json.loads((p/'freeze.json').read_text()).items():
    assert hashlib.sha256(Path(name).read_bytes()).hexdigest() == digest, name
rows = []
for mode, family, n, reuse in cells:
    expected_names = ['inference']
    for i in range(reuse):
        expected_names += ['input', 'certify'] + (['prepare'] if i == 0 else [])
        expected_names += ['setup', 'first-delivery-or-exhaustion', 'remaining-delivery', 'search-drop', 'answers-drop', 'input-drop']
    expected_names += ['cancel-input', 'cancel-certify', 'cancel-setup', 'cancel-unit', 'cancel-search-drop', 'cancel-input-drop', 'prepared-drop', 'inference-drop']
    readings = []
    times = []
    for kind, reps in [('meter', 2), ('time', 5)]:
        for r in range(reps):
            d = json.loads((p/kind/f'{r}-{mode}-{family}-{n}-{reuse}.json').read_text())
            assert (d['mode'], d['family'], d['firings'], d['reuse'], d['meter']) == (mode, family, n, reuse, kind == 'meter')
            phases = d['phases']
            assert [x['phase'] for x in phases] == expected_names
            if kind == 'time':
                assert all(x['memory'] is None for x in phases)
                times.append(sum(x['ns'] for x in phases))
                continue
            ms = [x['memory'] for x in phases]
            assert all(a['live_end'] == b['live_start'] for a,b in zip(ms,ms[1:]))
            assert ms[0]['live_start'] == ms[-1]['live_end']
            prep = next(x['memory'] for x in phases if x['phase'] == 'prepare')
            retained = prep['live_end'] - prep['live_start']
            for x in phases:
                if x['phase'] in ['input-drop', 'cancel-input-drop']:
                    assert x['memory']['live_end'] == ms[0]['live_end'] + retained
            assert ms[-2]['live_end'] == ms[0]['live_end']
            readings.append(ms)
    assert readings[0] == readings[1]
    m = readings[0]
    rows.append(dict(mode=mode,family=family,n=n,reuse=reuse,
        requested_bytes=sum(x['requested_bytes'] for x in m),
        peak_above_start=max(x['peak_live'] for x in m)-m[0]['live_start'],
        wall_median_ns=statistics.median(times), wall_min_ns=min(times), wall_max_ns=max(times)))
for kind,reps in [('meter',2),('time',5)]:
    assert len(list((p/kind).glob('*.json'))) == reps*len(cells)
    order = json.loads((p/f'{kind}-order.json').read_text())
    assert set(map(tuple,order)) == {(r,*c) for r in range(reps) for c in cells}
    assert len(order) == reps*len(cells)
with (p/'summary.csv').open('w') as f:
    w=csv.DictWriter(f,fieldnames=rows[0].keys());w.writeheader();w.writerows(rows)
print('1008 processes, 144 exact allocation pairs; phase, query, cancellation, prepared and inference ownership pass; frozen inputs unchanged')
