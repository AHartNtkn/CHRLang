#!/usr/bin/env python3
import hashlib,json,random,statistics
from pathlib import Path
R=Path('docs/experiments/results/s07-effect-lifecycle')
controls=[('off','ordinary'),('on','ordinary'),('on','inferred'),('on','declared'),('on','required'),('off','scan'),('off','specialized')]
scenarios=[(0,'0',0,1,0),(64,'0',0,1,0),(64,'0',0,4,0),(64,'all',0,4,0),(64,'0',0,4,1),(64,'all',1,4,0)]
cells=[]
for f in ['blocked','rewrite','writer']:
 for p,m in controls:
  if f=='writer' and m in ['declared','required']:continue
  cells.extend((p,m,f,*c) for c in scenarios)
admissions=[('off','ordinary'),('on','ordinary'),('on','inferred'),('on','declared'),('on','required')]
snapshots=json.loads((R/'source-snapshots.json').read_text()) if (R/'source-snapshots.json').exists() else {}
for name,h in json.loads((R/'freeze.json').read_text()).items():
 path=Path(name)
 if hashlib.sha256(path.read_bytes()).hexdigest()!=h:path=Path(snapshots[name])
 assert hashlib.sha256(path.read_bytes()).hexdigest()==h,name

def strip(v):
 if isinstance(v,dict):return {k:strip(x) for k,x in v.items() if k not in ('ns','first_answer_ns')}
 if isinstance(v,list):return list(map(strip,v))
 return v

def normalize(v,root):
 if isinstance(v,dict):return {k:((x-root) if k in ('live_start','live_end','peak_live','live','baseline') else normalize(x,root)) for k,x in v.items() if k not in ('ns','first_answer_ns','mode')}
 if isinstance(v,list):return [normalize(x,root) for x in v]
 return v
rows={};consumers={};engines={};declarations={}
for kind,reps,seed in [('meter',2,7930),('time',5,7931)]:
 jobs=[(rep,'life',*c) for rep in range(reps) for c in cells]+[(rep,'admission',*a) for rep in range(reps) for a in admissions];random.Random(seed).shuffle(jobs)
 assert json.loads((R/f'{kind}-order.json').read_text())==list(map(list,jobs))
 assert len(list((R/kind).glob('*.json')))==len(jobs)
 for i,(rep,action,p,m,*rest) in enumerate(jobs):
  raw=json.loads((R/kind/f'{i:03}.json').read_text());assert raw['returncode']==0
  cmd=raw['command'];assert Path(cmd[0]).name==f'{p}-{kind}'
  d=json.loads(raw['stdout'].splitlines()[-1]);assert d['mode']==m
  key=(action,p,m,*rest);row=rows.setdefault(key,{'time':[],'prep_time':[],'execute_time':[]})
  pp=[d['source_build'],d['preparation']]
  if action=='admission':
   assert cmd[1:]==['admission',m] and d['admission'] and d['accepted']==(m not in ('declared','required'))
  else:
   f,n,k,c,reuse,fail=rest;assert cmd[1:]==[m,f,str(n),'4','8','1',k,str(c),str(reuse),str(fail)]
   assert (d['family'],d['depth'],d['keep'],d['cancel'],d['reuse'],d['fail_tail'])==(f,n,k,bool(c),reuse,bool(fail))
   assert d['effect_feature']==(p=='on') and d['meter']==(kind=='meter') and not d['counters']
   assert len(d['samples'])==reuse
   counts=[]
   for q,s in enumerate(d['samples']):
    assert s['depth']==n+q and s['complete']==(not(c and q==0))
    if c and q==0:assert 0<=s['answers']<=1
    else:assert s['answers']==1-int(fail)
    assert (s['first_answer_ns'] is None)==(s['answers']==0);counts.append(s['answers'])
    pp.extend(s[t] for t in ['input_build','setup','execute_observe','engine_drop'])
   pp.append(d['consumer_drop'])
  pp.append(d['prepared_drop'])
  if kind=='time':
   assert all(x['memory'] is None for x in pp)
   row['time'].append(sum(x['ns'] for x in pp));row['prep_time'].append(d['preparation']['ns']);row['execute_time'].append(sum(s['execute_observe']['ns'] for s in d.get('samples',[])));continue
  if 'meter' in row:assert row['meter']==strip(d),key
  row['meter']=strip(d);mm=[x['memory'] for x in pp];root=mm[0]['live_start']
  for a,b in zip(mm,mm[1:]):assert a['live_end']==b['live_start'],key
  assert mm[-1]['live_end']==root
  row['requested']=sum(x['requested_bytes'] for x in mm);row['peak']=max(x['peak_live'] for x in mm)-root
  row['preparation_bytes']=d['preparation']['memory']['requested_bytes']
  if action=='life':
   prep=d['preparation']['memory']['live_end'];assert d['consumer_drop']['memory']['live_end']==prep
   row['retained']=d['consumer_drop']['memory']['live_start']-prep
   row['execute_bytes']=sum(s['execute_observe']['memory']['requested_bytes'] for s in d['samples'])
   ends=[x for x in d['snapshots'] if x['label']=='engine-disposed'];assert len(ends)==reuse
   assert ends[-1]['retained']==(sum(counts) if k=='all' else 0)
   representation='compiled' if m in ('scan','specialized') else 'conditional'
   consumers.setdefault((f,n,k,c,reuse,fail,tuple(counts),representation),set()).add(row['retained'])
   for q in range(reuse):
    before=[x for x in d['snapshots'] if x['query']==q and x['label'] in ('exhausted','cancelled')];assert len(before)==1
    engines.setdefault((p,m,f,n,c,reuse,fail,q),set()).add(before[0]['live']-ends[q]['live'])
   if m in ('inferred','declared','required') and f!='writer':
    group=(f,n,k,c,reuse,fail);value=normalize(d,root)
    if group in declarations:assert declarations[group]==value,group
    declarations[group]=value
assert all(len(v)==1 for v in consumers.values())
assert all(len(v)==1 for v in engines.values())
summary=[]
for k,v in rows.items():
 assert len(v['time'])==5
 summary.append(dict(key=k,requested=v['requested'],peak=v['peak'],preparation_bytes=v['preparation_bytes'],retained=v.get('retained'),execute_bytes=v.get('execute_bytes'),median_ns=statistics.median(v['time']),min_ns=min(v['time']),max_ns=max(v['time']),preparation_median_ns=statistics.median(v['prep_time']),execution_median_ns=statistics.median(v['execute_time'])))
(R/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(dict(lifecycle_cells=114,admission_cells=5,exact_allocation_pairs=len(summary),consumer_groups=len(consumers),engine_groups=len(engines),equivalent_declaration_groups=len(declarations),frozen_hashes='pass',owners='pass',phase_continuity='pass')))
