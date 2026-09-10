#!/usr/bin/env python3
"""Independent reconstruction of the registered pilot and its owner accounting."""
import hashlib,json,random,statistics
from pathlib import Path
R=Path('docs/experiments/results/s06-finite-lifecycle')
controls=[('ascending','finite'),('ascending','scan'),('ascending','specialized'),('ascending','prepared-prefix'),('ascending','conditional'),('reverse','conditional')]
scenarios=[(0,1,'0','none',0,0),(4,1,'0','none',0,2),(4,4,'0','none',0,2),(4,4,'4','none',0,2),(4,4,'all','none',0,2),(4,4,'0','none',1,2),(4,4,'all','step4',0,2),(8,1,'0','none',0,2)]
cells=[]
for family in ['oldest','newest','all','duplicates']:
 for order,mode in controls:
  for case in scenarios:cells.append((order,mode,family,*case))
  if family in ('all','duplicates'):cells.append((order,mode,family,4,4,'all','answer2',0,2))
assert len(cells)==204
snapshots=json.loads((R/'source-snapshots.json').read_text()) if (R/'source-snapshots.json').exists() else {}
for name,h in json.loads((R/'freeze.json').read_text()).items():
 p=Path(name)
 if hashlib.sha256(p.read_bytes()).hexdigest()!=h:p=Path(snapshots[name])
 assert hashlib.sha256(p.read_bytes()).hexdigest()==h,name

def strip(x):
 if isinstance(x,dict):return {k:strip(v) for k,v in x.items() if k not in ('ns','first_answer_ns')}
 if isinstance(x,list):return list(map(strip,x))
 return x
rows={};work_signatures={};consumer_work={};owner_groups={}
for kind,reps,seed in [('meter',2,7940),('time',5,7941)]:
 jobs=[(rep,*c) for rep in range(reps) for c in cells];random.Random(seed).shuffle(jobs)
 assert json.loads((R/f'{kind}-order.json').read_text())==list(map(list,jobs))
 assert len(list((R/kind).glob('*.json')))==len(jobs)
 for i,(rep,order,mode,f,n,reuse,keep,cancel,fail,work) in enumerate(jobs):
  raw=json.loads((R/kind/f'{i:04}.json').read_text());assert raw['returncode']==0
  cmd=raw['command'];assert Path(cmd[0]).name==f'{order}-{kind}' and cmd[1:]==[mode,f,str(n),str(reuse),keep,cancel,str(fail),str(work)]
  d=json.loads(raw['stdout'].splitlines()[-1]);assert (d['mode'],d['family'],d['depth'],d['reuse'],d['keep'],d['cancel'],d['fail'],d['work'],d['order'])==(mode,f,n,reuse,keep,cancel,bool(fail),work,order)
  assert not d['counters'] and d['meter']==(kind=='meter')
  assert len(d['samples'])==reuse
  expected_count=0
  for q,s in enumerate(d['samples']):
   assert s['depth']==n+q and s['complete']==(q>0 or cancel=='none')
   if s['complete']:assert s['answers']==(0 if fail else (2**(n+q) if f in ('all','duplicates') else 1))
   elif cancel=='step4':assert s['calls']==4
   elif cancel=='answer2':assert s['answers']==2
   assert (s['first_answer_ns'] is None)==(s['answers']==0)
   assert s['artifacts']==(q+1 if mode=='prepared-prefix' else 0)
   expected_count+=s['answers']
  assert d['retained']==min(expected_count,expected_count if keep=='all' else int(keep))
  assert d['artifacts']==(reuse if mode=='prepared-prefix' else 0)
  phases=d['phases'];assert [x['name'] for x in phases[:2]]==['source_build','preparation']
  assert [x['name'] for x in phases[-3:]]==['consumer_drop','artifact_drop','prepared_drop']
  for q in range(reuse):
   names=[x['name'] for x in phases if x['query']==q]
   if mode=='finite':
    assert names[:4]==['input_build','solver_setup','private_solve','solver_drop'] and names[-1]=='query_drop'
    middle=names[4:-1];assert len(middle)%4==0 and middle==['transport','caller_setup','caller_execute_observe','caller_drop']*(len(middle)//4)
   else:assert names==['input_build','query_setup','execute_observe','query_drop']
  k=(order,mode,f,n,reuse,keep,cancel,fail,work);row=rows.setdefault(k,{'time':[],'first':[]})
  signature=[{k:v for k,v in s.items() if k!='first_answer_ns'} for s in d['samples']]
  signature.append([(p['query'],p['name']) for p in phases])
  if k in work_signatures:assert work_signatures[k]==signature,k
  work_signatures[k]=signature
  if kind=='time':
   assert all(x['memory'] is None for x in phases)
   row['time'].append(sum(x['ns'] for x in phases));row['first'].append([s['first_answer_ns'] for s in d['samples']]);continue
  value=strip(d)
  if 'meter' in row:assert row['meter']==value,k
  row['meter']=value
  mm=[x['memory'] for x in phases];root=mm[0]['live_start']
  assert all(a['live_end']==b['live_start'] for a,b in zip(mm,mm[1:])),k
  assert mm[-1]['live_end']==root and mm[-2]['live_end']==mm[1]['live_end'],k
  row['requested']=sum(x['requested_bytes'] for x in mm);row['peak']=max(x['peak_live'] for x in mm)-root
  row['retained']=mm[-3]['live_start']-mm[-3]['live_end'];row['artifact_bytes']=mm[-2]['live_start']-mm[-2]['live_end']
  by_phase={}
  for x in phases:by_phase[x['name']]=by_phase.get(x['name'],0)+x['memory']['requested_bytes']
  row['by_phase']=by_phase
  # Retention cannot change engine work/allocation on otherwise identical queries.
  g=(order,mode,f,n,reuse,cancel,fail,work)
  comparable={name:b for name,b in by_phase.items() if name not in ('consumer_drop','artifact_drop','prepared_drop')}
  if g in consumer_work:assert consumer_work[g]==comparable,g
  consumer_work[g]=comparable
  representation='conditional' if mode=='conditional' else ('finite-weighted' if mode=='finite' and f=='duplicates' else 'compiled')
  owner_groups.setdefault((representation,f,n,reuse,keep,cancel,fail,work,expected_count),set()).add(row['retained'])
# Different result Vec capacities may be real owner costs; retain, do not normalize them away.
summary=[]
for k,row in rows.items():
 assert len(row['time'])==5
 summary.append(dict(key=k,requested=row['requested'],peak=row['peak'],retained=row['retained'],artifact_bytes=row['artifact_bytes'],by_phase=row['by_phase'],median_ns=statistics.median(row['time']),min_ns=min(row['time']),max_ns=max(row['time']),first_answer_ns=row['first']))
(R/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(dict(cells=len(rows),exact_allocation_pairs=len(rows),exact_diagnostic_time_work=len(work_signatures),consumer_independent_groups=len(consumer_work),owner_groups_with_distinct_layouts=sum(len(x)>1 for x in owner_groups.values()),phase_continuity='pass',final_owners='pass',frozen_sources='pass')))
