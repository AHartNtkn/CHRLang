#!/usr/bin/env python3
import csv
import hashlib
import itertools
import json
from pathlib import Path
import statistics
import sys
p=Path(sys.argv[1])
controls=[('prefix','conditional')]
cells=[(build,mode,family,n,reuse,consumer) for (build,mode),family,n,reuse,consumer in itertools.product(controls,['incompatible','correlated','independent_suffix','dense_suffix','independent','delayed','early'],[0,1,3],[1,4],['query','prepared'])]
for name,h in json.loads((p/'freeze.json').read_text()).items():
    data=Path(name).read_bytes()
    if hashlib.sha256(data).hexdigest()!=h:
        snapshots=json.loads((p/'source-snapshots.json').read_text())
        data=Path(snapshots[name]).read_bytes()
    assert hashlib.sha256(data).hexdigest()==h,name
rows=[]
cancel_counts={}
cancel_examples={}
for build,mode,family,n,reuse,consumer in cells:
    names=['prepare']+['input','setup','first-delivery-or-exhaustion','remaining-delivery','search-drop','answer-policy','input-drop']*reuse+['cancel-input','cancel-setup','cancel-unit','cancel-search-drop','cancel-input-drop']*3+['prepared-drop','retained-answers-drop']
    readings=[];times=[];retained=0;cancel_signatures=[]
    for kind,reps in [('meter',2)]:
        for r in range(reps):
            d=json.loads((p/kind/f'{r}-{build}-{mode}-{family}-{n}-{reuse}-{consumer}.json').read_text())
            assert (d['build'],d['mode'],d['family'],d['n'],d['reuse'],d['consumer'],d['meter'])==(build,mode,family,n,reuse,consumer,kind=='meter')
            assert [x['steps'] for x in d['cancellation']]==[1,64,256]
            cancel_signatures.append(d['cancellation'])
            if kind=='meter' and r==0:
                for c in d['cancellation']:
                    for field in ['pools','checks','prefixes']:
                        if c[field]:
                            key=build+'-'+field
                            cancel_counts[key]=cancel_counts.get(key,0)+1
                            cancel_examples[key]={'mode':mode,'family':family,'n':n,'reuse':reuse,'consumer':consumer,**c}
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
            for j in range(3): assert ms[1+7*reuse+5*j+4]['live_end']==ms[0]['live_end']+kept
            assert ms[-2]['live_end']==ms[0]['live_start']+kept
            readings.append(ms);retained=kept
    assert all(x==cancel_signatures[0] for x in cancel_signatures)
    assert readings[0]==readings[1]
    ms=readings[0]
    rows.append(dict(build=build,mode=mode,family=family,n=n,reuse=reuse,consumer=consumer,requested_bytes=sum(x['requested_bytes'] for x in ms),peak_above_start=max(x['peak_live'] for x in ms)-ms[0]['live_start'],consumer_retained_bytes=retained,wall_median_ns=statistics.median(times) if times else None,wall_min_ns=min(times) if times else None,wall_max_ns=max(times) if times else None))
for kind,reps in [('meter',2)]:
    order=json.loads((p/f'{kind}-order.json').read_text());assert len(order)==reps*len(cells);assert set(map(tuple,order))=={(r,*c) for r in range(reps) for c in cells};assert len(list((p/kind).glob('*.json')))==reps*len(cells)
with (p/'summary.csv').open('w') as f:
    w=csv.DictWriter(f,fieldnames=rows[0].keys(),lineterminator="\n");w.writeheader();w.writerows(rows)
print('168 processes; 84 exact allocation pairs; all phase, query, consumer, cancellation and prepared owner checks pass; frozen input hashes verified; revised timing not run')

for build,mode in controls:
    if mode=='conditional':
        assert cancel_counts.get(build+'-pools',0)>0
        if build=='direct': assert cancel_counts.get('direct-checks',0)>0
        if build=='prefix': assert cancel_counts.get('prefix-prefixes',0)>0
(p/'cancellation-audit.json').write_text(json.dumps({'positive_snapshots':cancel_counts,'examples':cancel_examples,'repeats_agree':True},indent=2)+'\n')
