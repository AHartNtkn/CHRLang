#!/usr/bin/env python3
import hashlib,json,random,re,statistics
from pathlib import Path
R=Path('docs/experiments/results/s10-arrival-lifecycle')
controls=[('ordinary','scan'),('ordinary','resumable'),('ordinary','specialized'),('ordinary','prefix'),('ordinary','prepared-prefix'),('combined','inferred'),('reverse','inferred')]
cells=[]
for family,depth in [('oldest-first',8),('newest-first',8),('aliases',64),('distinct',64)]:
 cases=[(0,'0',0,1,0,'same'),(depth,'0',0,1,0,'same')]
 for signature in ['same','changing']:
  for keep in ['0','all']:cases.append((depth,keep,0,4,0,signature))
 cases += [(depth,'0',0,4,1,'changing'),(depth,'all',2 if depth==8 else 1,4,0,'same' if depth==8 else 'changing')]
 for policy,mode in controls:
  if depth==64 and mode in ('prefix','prepared-prefix'):continue
  for c in cases:cells.append((policy,mode,family,*c))
assert len(cells)==192
for file,digest in json.loads((R/'freeze.json').read_text()).items():assert hashlib.sha256(Path(file).read_bytes()).hexdigest()==digest,file

def clean(x):
 if isinstance(x,dict):return {k:clean(v) for k,v in x.items() if k not in ('ns','first_answer_ns')}
 if isinstance(x,list):return list(map(clean,x))
 return x

def phases(d):
 return [d['source_build'],d['preparation']]+[s[k] for s in d['samples'] for k in ['input_build','setup','execute_observe','engine_drop']]+[d['artifact_drop'],d['consumer_drop'],d['prepared_drop']]
rows={};consumers={};engines={}
for kind,reps,seed in [('meter',2,7910),('time',5,7911)]:
 jobs=[(rep,*c) for rep in range(reps) for c in cells];random.Random(seed).shuffle(jobs)
 assert json.loads((R/f'{kind}-order.json').read_text())==list(map(list,jobs))
 assert len(list((R/kind).glob('*.json')))==len(jobs)
 for i,(rep,p,m,f,n,k,c,reuse,fail,sig) in enumerate(jobs):
  raw=json.loads((R/kind/f'{i:03}.json').read_text());assert raw['returncode']==0
  assert raw['command'][1:]==[m,f,str(n),'4','8','1',k,str(c),str(reuse),str(fail),sig]
  assert Path(raw['command'][0]).name==f'{p}-{kind}'
  d=json.loads(raw['stdout'].splitlines()[-1]);assert (d['mode'],d['family'],d['depth'],d['keep'],d['cancel'],d['reuse'],d['fail_tail'],d['signature'])==(m,f,n,k,bool(c),reuse,bool(fail),sig)
  assert d['policy']==('ordinary' if p=='ordinary' else 'combined')
  assert d['order']==('reverse' if p=='reverse' else 'ascending')
  assert d['observer']==('general' if p=='reverse' else 'direct')
  assert not d['counters'] and d['meter']==(kind=='meter')
  assert d['artifacts']==((1 if sig=='same' else reuse) if m=='prepared-prefix' else 0)
  assert len(d['samples'])==reuse
  counts=[]
  for q,s in enumerate(d['samples']):
   depth=n if sig=='same' else n+q;assert s['depth']==depth
   assert s['complete']==(not(c and q==0))
   expected=(1-int(fail)) if f.endswith('-first') else depth+1-int(fail)
   if c==1 and q==0:expected=4
   if c==2 and q==0:assert 0<=s['answers']<=expected
   else:assert s['answers']==expected
   assert (s['first_answer_ns'] is None)==(s['answers']==0);counts.append(s['answers'])
  key=(p,m,f,n,k,c,reuse,fail,sig);row=rows.setdefault(key,{'time':[],'setup_time':[],'execute_time':[]})
  pp=phases(d)
  if kind=='time':
   assert all(x['memory'] is None for x in pp)
   row['time'].append(sum(x['ns'] for x in pp));row['setup_time'].append(sum(s['setup']['ns'] for s in d['samples']));row['execute_time'].append(sum(s['execute_observe']['ns'] for s in d['samples']));continue
  if 'meter' in row:assert row['meter']==clean(d)
  row['meter']=clean(d);mm=[x['memory'] for x in pp]
  for a,b in zip(mm,mm[1:]):assert a['live_end']==b['live_start'],key
  root=mm[0]['live_start'];prep=d['preparation']['memory']['live_end']
  assert mm[-1]['live_end']==root and d['consumer_drop']['memory']['live_end']==prep
  row['requested']=sum(x['requested_bytes'] for x in mm);row['peak']=max(x['peak_live'] for x in mm)-root
  row['artifact_bytes']=d['artifact_drop']['memory']['live_start']-d['artifact_drop']['memory']['live_end']
  assert row['artifact_bytes']>=0
  if m!='prepared-prefix':assert row['artifact_bytes']==0
  row['retained']=d['consumer_drop']['memory']['live_start']-prep
  row['setup_bytes']=[s['setup']['memory']['requested_bytes'] for s in d['samples']]
  row['execute_bytes']=sum(s['execute_observe']['memory']['requested_bytes'] for s in d['samples'])
  retained=sum(counts) if k=='all' else 0
  end=[x for x in d['snapshots'] if x['label']=='engine-disposed'];assert len(end)==reuse and end[-1]['retained']==retained
  # Actual output costs remain unadjusted; compare payload after known spare
  # capacity, established by the preceding independent layout probe.
  spare=retained*3*48 if f.endswith('-first') and m=='inferred' else 0
  consumers.setdefault((f,n,k,c,reuse,fail,sig,tuple(counts)),set()).add(row['retained']-spare)
  for q in range(reuse):
   before=[x for x in d['snapshots'] if x['query']==q and x['label'] in ['exhausted','cancelled']];assert len(before)==1
   engines.setdefault((p,m,f,n,c,reuse,fail,sig,q),set()).add(before[0]['live']-end[q]['live'])
assert all(len(v)==1 for v in consumers.values()),consumers
assert all(len(v)==1 for v in engines.values()),engines
out=[]
for key,v in rows.items():
 assert len(v['time'])==5
 out.append(dict(key=key,requested=v['requested'],peak=v['peak'],artifact_bytes=v['artifact_bytes'],retained=v['retained'],setup_bytes=v['setup_bytes'],execute_bytes=v['execute_bytes'],median_ns=statistics.median(v['time']),min_ns=min(v['time']),max_ns=max(v['time']),setup_median_ns=statistics.median(v['setup_time']),execute_median_ns=statistics.median(v['execute_time'])))
(R/'summary.json').write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps(dict(cells=len(out),allocation_pairs=len(out),consumer_groups=len(consumers),engine_groups=len(engines),phase_continuity='pass',owners='pass',frozen_hashes='pass')))
