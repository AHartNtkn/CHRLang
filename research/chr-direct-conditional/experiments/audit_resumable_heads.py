#!/usr/bin/env python3
import csv,hashlib,itertools,json,sys
from pathlib import Path
p=Path(sys.argv[1]);baseline=Path(sys.argv[2])
cells=list(itertools.product(['contextual','demand','resumable'],['independent','shared','delayed','early','late','history','deep','stable_history','reset_history','dense'],[0,1,3],[1,4],['query','prepared']))
for name,h in json.loads((p/'freeze.json').read_text()).items():assert hashlib.sha256(Path(name).read_bytes()).hexdigest()==h,name
rows=[]
for mode,family,n,reuse,consumer in cells:
 readings=[];names=['prepare']+['input','setup','first-delivery-or-exhaustion','remaining-delivery','search-drop','answer-policy','input-drop']*reuse+['cancel-input','cancel-setup','cancel-unit','cancel-search-drop','cancel-input-drop','prepared-drop','retained-answers-drop']
 for r in range(2):
  filename=f'{r}-{mode}-{family}-{n}-{reuse}-{consumer}.json';d=json.loads((p/'meter'/filename).read_text());xs=d['phases']
  assert (d['mode'],d['family'],d['n'],d['reuse'],d['consumer'],d['meter'])==(mode,family,n,reuse,consumer,True)
  assert [x['phase'] for x in xs]==names
  ms=[x['memory'] for x in xs];assert ms[0]['live_start']==ms[-1]['live_end'];assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]))
  retained=0
  for i in range(reuse):
   q=ms[1+7*i:8+7*i];input_bytes=q[0]['live_end']-q[0]['live_start'];answer_bytes=q[4]['live_end']-ms[0]['live_end']-retained-input_bytes;assert answer_bytes>=0
   if consumer=='prepared':retained+=answer_bytes
   assert q[-1]['live_end']==ms[0]['live_end']+retained
  assert ms[-3]['live_end']==ms[0]['live_end']+retained;assert ms[-2]['live_end']==ms[0]['live_start']+retained
  if mode!='resumable':assert ms==[x['memory'] for x in json.loads((baseline/'meter'/filename).read_text())['phases']],filename
  readings.append(ms)
 assert readings[0]==readings[1]
 rows.append(dict(mode=mode,family=family,n=n,reuse=reuse,consumer=consumer,requested_bytes=sum(x['requested_bytes'] for x in ms),peak_above_start=max(x['peak_live'] for x in ms)-ms[0]['live_start'],consumer_retained_bytes=retained))
order=json.loads((p/'meter-order.json').read_text());assert len(order)==720 and set(map(tuple,order))=={(r,*c) for r in range(2) for c in cells};assert len(list((p/'meter').glob('*.json')))==720
with (p/'summary.csv').open('w') as f:
 w=csv.DictWriter(f,fieldnames=rows[0],lineterminator='\n');w.writeheader();w.writerows(rows)
print('720 allocation processes; 360 exact pairs; complete phase and owner checks; all eager/demand readings identical to baseline; revised timing not measured')
