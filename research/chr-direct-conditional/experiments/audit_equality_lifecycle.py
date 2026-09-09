#!/usr/bin/env python3
import csv
import hashlib
import itertools
import json
from pathlib import Path
import statistics
import sys
p=Path(sys.argv[1])
cells=list(itertools.product(['scan','contextual','demand','resumable'],['stable_history','reset_history','constructor','dynamic','delayed','independent'],[0,3],[1,4],['query','prepared'],['original','eliminated'],['conservative','precise']))
for name,h in json.loads((p/'freeze.json').read_text()).items():
    assert hashlib.sha256(Path(name).read_bytes()).hexdigest()==h,name
rows=[]
for mode,family,n,reuse,consumer,form,build in cells:
    names=['source-analysis','prepare']+['input','setup','first-delivery-or-exhaustion','remaining-delivery','search-drop','answer-policy','input-drop']*reuse+['cancel-input','cancel-setup','cancel-unit','cancel-search-drop','cancel-input-drop','prepared-drop','retained-answers-drop']
    readings=[];times=[];retained=0
    for kind,reps in [('meter',2),('time',5)]:
        for r in range(reps):
            d=json.loads((p/kind/f'{r}-{mode}-{family}-{n}-{reuse}-{consumer}-{form}-{build}.json').read_text())
            assert (d['mode'],d['family'],d['n'],d['reuse'],d['consumer'],d['form'],d['precise'],d['meter'])==(mode,family,n,reuse,consumer,form,build=='precise',kind=='meter')
            xs=d['phases'];assert [x['phase'] for x in xs]==names
            if kind=='time':
                assert all(x['memory'] is None for x in xs);times.append(sum(x['ns'] for x in xs));continue
            ms=[x['memory'] for x in xs];assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]));assert ms[0]['live_start']==ms[-1]['live_end']
            kept=0
            for i in range(reuse):
                q=ms[2+7*i:9+7*i];qbytes=q[0]['live_end']-q[0]['live_start']
                abytes=q[4]['live_end']-ms[1]['live_end']-kept-qbytes
                assert abytes>=0
                if consumer=='prepared':kept+=abytes
                assert q[-1]['live_end']==ms[1]['live_end']+kept
            assert ms[-3]['live_end']==ms[1]['live_end']+kept
            assert ms[-2]['live_end']==ms[0]['live_start']+kept
            readings.append(ms);retained=kept
    assert readings[0]==readings[1]
    ms=readings[0]
    rows.append(dict(mode=mode,family=family,n=n,reuse=reuse,consumer=consumer,form=form,build=build,analysis_bytes=ms[0]['requested_bytes'],requested_bytes=sum(x['requested_bytes'] for x in ms),peak_above_start=max(x['peak_live'] for x in ms)-ms[0]['live_start'],consumer_retained_bytes=retained,wall_median_ns=statistics.median(times),wall_min_ns=min(times),wall_max_ns=max(times)))
for kind,reps in [('meter',2),('time',5)]:
    order=json.loads((p/f'{kind}-order.json').read_text());assert len(order)==reps*len(cells);assert set(map(tuple,order))=={(r,*c) for r in range(reps) for c in cells};assert len(list((p/kind).glob('*.json')))==reps*len(cells)
with (p/'summary.csv').open('w') as f:
    w=csv.DictWriter(f,fieldnames=rows[0].keys(),lineterminator="\n");w.writeheader();w.writerows(rows)
print('5376 processes; 768 exact allocation pairs; phase and owner checks pass; frozen inputs verified; timing exploratory')
