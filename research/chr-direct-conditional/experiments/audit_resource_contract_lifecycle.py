#!/usr/bin/env python3
import csv
import hashlib
import itertools
import json
from pathlib import Path
import statistics
import sys
p=Path(sys.argv[1])
cells=list(itertools.product(['inferred','declared'],['common','independent'],[0,3],[0,16],[1,4],['eligible','missing-permit','unknown']))
for name,h in json.loads((p/'freeze.json').read_text()).items():assert hashlib.sha256(Path(name).read_bytes()).hexdigest()==h,name
rows=[]
for mode,family,k,depth,reuse,schedule in cells:
    names=['prepare']+['input','admit-lower-setup','first-delivery','remaining-delivery','search-or-error-drop','answers-drop','input-drop']*reuse+['cancel-input','cancel-admit-lower-setup','cancel-unit','cancel-search-or-error-drop','cancel-input-drop','prepared-drop']
    readings=[];times=[]
    for kind,reps in [('meter',2),('time',5)]:
        for r in range(reps):
            d=json.loads((p/kind/f'{r}-{mode}-{family}-{k}-{depth}-{reuse}-{schedule}.json').read_text())
            assert (d['mode'],d['family'],d['choices'],d['depth'],d['reuse'],d['schedule'],d['meter'])==(mode,family,k,depth,reuse,schedule,kind=='meter')
            rejected=reuse//2+1 if mode=='declared' and schedule=='unknown' else 0
            assert d['rejected']==rejected
            xs=d['phases'];assert [x['phase'] for x in xs]==names
            if kind=='time':assert all(x['memory'] is None for x in xs);times.append(sum(x['ns'] for x in xs));continue
            ms=[x['memory'] for x in xs];assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]));assert ms[0]['live_start']==ms[-1]['live_end']
            for x in xs:
                if x['phase'] in ['input-drop','cancel-input-drop']:assert x['memory']['live_end']==ms[0]['live_end']
                if x['phase']=='prepared-drop':assert x['memory']['live_end']==ms[0]['live_start']
            readings.append(ms)
    assert readings[0]==readings[1]
    ms=readings[0]
    rows.append(dict(mode=mode,family=family,choices=k,depth=depth,reuse=reuse,schedule=schedule,requested_bytes=sum(x['requested_bytes'] for x in ms),peak_above_start=max(x['peak_live'] for x in ms)-ms[0]['live_start'],wall_median_ns=statistics.median(times),wall_min_ns=min(times),wall_max_ns=max(times)))
for kind,reps in [('meter',2),('time',5)]:
    order=json.loads((p/f'{kind}-order.json').read_text());assert len(order)==reps*len(cells);assert set(map(tuple,order))=={(r,*c) for r in range(reps) for c in cells};assert len(list((p/kind).glob('*.json')))==reps*len(cells)
with (p/'summary.csv').open('w') as f:
    w=csv.DictWriter(f,fieldnames=rows[0],lineterminator="\n");w.writeheader();w.writerows(rows)
print('672 processes; 96 exact allocation pairs; complete observations, rejection counts, phase order and all owner checks passed; frozen sources unchanged; timing exploratory')
