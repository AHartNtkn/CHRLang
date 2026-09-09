#!/usr/bin/env python3
import csv,hashlib,itertools,json,statistics,sys
from pathlib import Path
p=Path(sys.argv[1])
cells=list(itertools.product(['existing','filtered','scan'],['original','counted'],['common','independent','early'],[0,3],[0,16],[1,4],['eligible','mixed']))
cells += [(v,'original','dense',k,0,n,'eligible') for v,k,n in itertools.product(['existing','filtered','scan'],[0,3],[1,4])]
for name,h in json.loads((p/'freeze.json').read_text()).items():assert hashlib.sha256(Path(name).read_bytes()).hexdigest()==h,name
rows=[]
for variant,form,family,k,depth,reuse,schedule in cells:
 names=['inference','prepare']+['input','transform','setup','first-delivery-or-exhaustion','remaining-delivery','search-drop','answers-drop','transformed-drop','input-drop']*reuse+['cancel-input','cancel-transform','cancel-setup','cancel-unit','cancel-search-drop','cancel-transformed-drop','cancel-input-drop','prepared-drop','inference-drop']
 readings=[];times=[]
 for kind,reps in [('meter',2),('time',5)]:
  for r in range(reps):
   job=(r,variant,form,family,k,depth,reuse,schedule);x=json.loads((p/kind/('-'.join(map(str,job))+'.json')).read_text())
   mode=form+'-'+('scan' if variant=='scan' else 'conditional')
   assert (x['mode'],x['selective'],x['family'],x['choices'],x['depth'],x['reuse'],x['schedule'],x['meter'])==(mode,variant=='filtered',family,k,depth,reuse,schedule,kind=='meter')
   assert x['admitted']==(0 if form=='original' else (reuse+1 if schedule=='eligible' else (reuse+1)//2))
   xs=x['phases'];assert [a['phase'] for a in xs]==names
   if kind=='time':assert all(a['memory'] is None for a in xs);times.append(sum(a['ns'] for a in xs));continue
   ms=[a['memory'] for a in xs];assert ms[0]['live_start']==ms[-1]['live_end'];assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]))
   for a in xs:
    if a['phase'] in ['input-drop','cancel-input-drop']:assert a['memory']['live_end']==ms[1]['live_end']
    if a['phase']=='prepared-drop':assert a['memory']['live_end']==ms[0]['live_end']
   readings.append(ms)
 assert readings[0]==readings[1]
 ms=readings[0];rows.append(dict(variant=variant,form=form,family=family,choices=k,depth=depth,reuse=reuse,schedule=schedule,requested_bytes=sum(a['requested_bytes'] for a in ms),peak_above_start=max(a['peak_live'] for a in ms)-ms[0]['live_start'],wall_median_ns=statistics.median(times),wall_min_ns=min(times),wall_max_ns=max(times)))
for kind,reps in [('meter',2),('time',5)]:
 order=json.loads((p/f'{kind}-order.json').read_text());assert len(order)==reps*len(cells);assert set(map(tuple,order))=={(r,*c) for r in range(reps) for c in cells};assert len(list((p/kind).glob('*.json')))==reps*len(cells)
assert json.loads((p/'completion.json').read_text())['processes']==2100
with (p/'summary.csv').open('w') as f:
 w=csv.DictWriter(f,fieldnames=rows[0],lineterminator='\n');w.writeheader();w.writerows(rows)
print('2100 processes; 300 exact allocation pairs; complete observations, build identity, admission, phase order and owner checks pass; timing exploratory')
