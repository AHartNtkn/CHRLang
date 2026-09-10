#!/usr/bin/env python3
"""Independent reconstruction and ownership audit of the order pilot."""
import hashlib,json,random,statistics
from pathlib import Path
R=Path('docs/experiments/results/s08-order-lifecycle')
controls=[('ordinary','inferred'),('combined','inferred'),('general','inferred'),('reverse','inferred'),('ordinary','scan'),('ordinary','resumable')]
cases=[]
for family,depth in [('aliases',64),('distinct',64),('oldest-first',8),('newest-first',8)]:
 cases += [(family,0,'0',0,1,0),(family,depth,'0',0,4,0),(family,depth,'all',0,4,0)]
cases += [('aliases',64,'4',0,4,0),('aliases',64,'0',0,4,1),('aliases',64,'0',1,4,0),('aliases',64,'all',1,4,0),('oldest-first',8,'0',0,4,1),('newest-first',8,'0',0,4,1)]

def strip(v):
 if isinstance(v,dict): return {k:strip(x) for k,x in v.items() if k not in ('ns','first_answer_ns')}
 if isinstance(v,list): return list(map(strip,v))
 return v

def phases(d):
 a=[d['source_build'],d['preparation']]
 for s in d['samples']: a.extend(s[k] for k in ('input_build','setup','execute_observe','engine_drop'))
 return a+[d['consumer_drop'],d['prepared_drop']]

for file,digest in json.loads((R/'freeze.json').read_text()).items():
 assert hashlib.sha256(Path(file).read_bytes()).hexdigest()==digest,file
layout=(R/'layout.log').read_text()
for mode,cap in [(0,1),(5,4),(7,1)]:
 for q in range(4):
  assert f'mode={mode} query={q} outputs_len=2 outputs_capacity=2 residual_len=1 residual_capacity={cap} constraint_bytes=48' in layout
stats={};output_groups={};engine_groups={}
for kind,reps,seed in [('meter',2,7890),('time',5,7891)]:
 jobs=[(rep,*control,*case) for rep in range(reps) for control in controls for case in cases]
 random.Random(seed).shuffle(jobs)
 assert json.loads((R/f'{kind}-order.json').read_text())==[list(j) for j in jobs]
 assert len(list((R/kind).glob('*.json')))==len(jobs)
 for i,(rep,policy,mode,f,n,k,c,reuse,fail) in enumerate(jobs):
  raw=json.loads((R/kind/f'{i:03}.json').read_text());assert raw['returncode']==0
  assert raw['command'][1:]==[mode,f,str(n),'4','8','1',k,str(c),str(reuse),str(fail)]
  assert Path(raw['command'][0]).name==f'{policy}-{kind}'
  d=json.loads(raw['stdout'].splitlines()[-1]);assert not d['counters'] and d['meter']==(kind=='meter')
  assert (d['mode'],d['family'],d['depth'],d['keep'],d['cancel'],d['reuse'],d['fail_tail'])==(mode,f,n,k,bool(c),reuse,bool(fail))
  assert d['policy']==('ordinary' if policy=='ordinary' else 'combined')
  assert d['order']==('reverse' if policy=='reverse' else 'ascending')
  assert d['observer']==('general' if policy in ('reverse','general') else 'direct')
  key=(policy,mode,f,n,k,c,reuse,fail);row=stats.setdefault(key,{'timings':[]})
  expected_retained=0
  for q,s in enumerate(d['samples']):
   assert s['depth']==n+q and s['complete']==(not(c and q==0))
   count=4 if c and q==0 else ((1-int(fail)) if f.endswith('-first') else n+q+1-int(fail))
   assert s['answers']==count and (s['first_answer_ns'] is None)==(count==0)
   expected_retained+=count
   if k!='all': expected_retained=min(int(k),expected_retained)
  assert len(d['samples'])==reuse
  pp=phases(d)
  if kind=='time':
   assert all(p['memory'] is None for p in pp)
   row['timings'].append(sum(p['ns'] for p in pp));continue
  if 'meter' in row: assert row['meter']==strip(d),key
  row['meter']=strip(d)
  mem=[p['memory'] for p in pp]
  for before,after in zip(mem,mem[1:]): assert before['live_end']==after['live_start'],key
  root=mem[0]['live_start'];prep=d['preparation']['memory']['live_end']
  assert mem[-1]['live_end']==root
  assert d['consumer_drop']['memory']['live_end']==prep==d['prepared_drop']['memory']['live_start']
  row['requested']=sum(p['requested_bytes'] for p in mem)
  row['peak']=max(p['peak_live'] for p in mem)-root
  row['execution_requested']=sum(s['execute_observe']['memory']['requested_bytes'] for s in d['samples'])
  row['retained']=d['consumer_drop']['memory']['live_start']-prep
  ends=[s for s in d['snapshots'] if s['label']=='engine-disposed']
  assert len(ends)==reuse and ends[-1]['retained']==expected_retained
  # Post-run layout attribution: conditional residual Vec has capacity4,
  # explicit controls capacity1; each Constraint is48 bytes on this host.
  # Preserve actual owner costs in summaries; compare equal semantic payload
  # only after independently observed spare storage is accounted for.
  spare = expected_retained*3*48 if f.endswith('-first') and mode=='inferred' else 0
  og=(f,n,k,c,reuse,fail);output_groups.setdefault(og,set()).add(row['retained']-spare)
  for q,s in enumerate(d['samples']):
   end=[x for x in d['snapshots'] if x['query']==q and x['label'] in ('exhausted','cancelled')];assert len(end)==1
   eg=(policy,mode,f,n,c,reuse,fail,q)
   engine_groups.setdefault(eg,set()).add(end[0]['live']-ends[q]['live'])
assert all(len(v)==1 for v in output_groups.values()),output_groups
assert all(len(v)==1 for v in engine_groups.values()),engine_groups
summary=[]
for key,v in stats.items():
 assert len(v['timings'])==5
 summary.append(dict(key=key,requested=v['requested'],peak=v['peak'],execution_requested=v['execution_requested'],retained=v['retained'],median_ns=statistics.median(v['timings']),min_ns=min(v['timings']),max_ns=max(v['timings'])))
(R/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(dict(cells=len(stats),exact_allocation_pairs=len(stats),output_groups=len(output_groups),engine_groups=len(engine_groups),source_hashes='pass',phase_continuity='pass',ownership='pass')))
