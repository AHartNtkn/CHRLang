#!/usr/bin/env python3
import csv
import hashlib
import itertools
import json
from pathlib import Path
import statistics
import sys
p=Path(sys.argv[1])
cells=list(itertools.product(['scan','contextual','demand','resumable','conditional'],['independent','shared','delayed','early','late','history','deep','stable_history','reset_history','dense'],[0,1,3],[1,4],['query','prepared']))
for name,h in json.loads((p/'freeze.json').read_text()).items():
    data=Path(name).read_bytes()
    if hashlib.sha256(data).hexdigest()!=h:
        snapshots=json.loads((p/'source-snapshots.json').read_text())
        data=Path(snapshots[name]).read_bytes()
    assert hashlib.sha256(data).hexdigest()==h,name
rows=[]
for mode,family,n,reuse,consumer in cells:
    names=['prepare']+['input','setup','first-delivery-or-exhaustion','remaining-delivery','search-drop','answer-policy','input-drop']*reuse+['cancel-input','cancel-setup','cancel-unit','cancel-search-drop','cancel-input-drop','prepared-drop','retained-answers-drop']
    readings=[];times=[];retained=0
    for kind,reps in [('meter',2),('time',5)]:
        for r in range(reps):
            d=json.loads((p/kind/f'{r}-{mode}-{family}-{n}-{reuse}-{consumer}.json').read_text())
            assert (d['mode'],d['family'],d['n'],d['reuse'],d['consumer'],d['meter'])==(mode,family,n,reuse,consumer,kind=='meter')
            xs=d['phases'];assert [x['phase'] for x in xs]==names
            if kind=='time':
                assert all(x['memory'] is None for x in xs);times.append(sum(x['ns'] for x in xs));continue
            ms=[x['memory'] for x in xs];assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]));assert ms[0]['live_start']==ms[-1]['live_end']
            kept=0
            for i in range(reuse):
                q=ms[1+7*i:8+7*i];qbytes=q[0]['live_end']-q[0]['live_start']
                abytes=q[4]['live_end']-ms[0]['live_end']-kept-qbytes
                assert abytes>=0
                if consumer=='prepared':kept+=abytes
                assert q[-1]['live_end']==ms[0]['live_end']+kept
            assert ms[-3]['live_end']==ms[0]['live_end']+kept
            assert ms[-2]['live_end']==ms[0]['live_start']+kept
            readings.append(ms);retained=kept
    assert readings[0]==readings[1]
    ms=readings[0]
    rows.append(dict(mode=mode,family=family,n=n,reuse=reuse,consumer=consumer,requested_bytes=sum(x['requested_bytes'] for x in ms),peak_above_start=max(x['peak_live'] for x in ms)-ms[0]['live_start'],consumer_retained_bytes=retained,wall_median_ns=statistics.median(times),wall_min_ns=min(times),wall_max_ns=max(times)))
for kind,reps in [('meter',2),('time',5)]:
    order=json.loads((p/f'{kind}-order.json').read_text());assert len(order)==reps*len(cells);assert set(map(tuple,order))=={(r,*c) for r in range(reps) for c in cells};assert len(list((p/kind).glob('*.json')))==reps*len(cells)
with (p/'summary.csv').open('w') as f:
    w=csv.DictWriter(f,fieldnames=rows[0].keys(),lineterminator="\n");w.writeheader();w.writerows(rows)
print('4200 processes; 600 exact allocation pairs; all phase, query, consumer, cancellation and prepared owner checks pass; frozen input hashes verified; timing exploratory')
