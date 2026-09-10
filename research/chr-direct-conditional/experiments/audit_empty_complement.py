#!/usr/bin/env python3
from pathlib import Path
import hashlib,json,random,statistics
R=Path('docs/experiments/results/s06-empty-complement-attribution')
CASES=[]
for f in ['oldest','newest','all','duplicates']:
 for c in [(0,1,'0','none',0,0),(4,4,'0','none',0,2),(4,4,'all','none',0,2),(8,1,'0','none',0,2)]:CASES.append(('finite',f,*c))
CASES.extend(('scan',f,4,4,'0','none',0,2) for f in ['oldest','all'])
for p,h in json.loads((R/'freeze.json').read_text()).items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h,p
def strip(x):
 if isinstance(x,dict):return {k:strip(v) for k,v in x.items() if k not in ('ns','first_answer_ns')}
 if isinstance(x,list):return list(map(strip,x))
 return x
rows={};signatures={}
for kind,reps,seed in [('meter',2,7942),('time',5,7943)]:
 jobs=[(rep,version,*c) for rep in range(reps) for version in ['baseline','candidate'] for c in CASES];random.Random(seed).shuffle(jobs)
 assert json.loads((R/f'{kind}-order.json').read_text())==list(map(list,jobs))
 assert len(list((R/kind).glob('*.json')))==len(jobs)
 for i,(rep,version,*case) in enumerate(jobs):
  r=json.loads((R/kind/f'{i:03}.json').read_text());assert r['returncode']==0 and r['version']==version
  assert r['command'][1:]==list(map(str,case))
  expected=Path('target/s06-finite-lifecycle-extended')/f'ascending-{kind}' if version=='baseline' else Path('target/s06-empty-complement')/kind
  assert Path(r['command'][0])==expected.resolve()
  d=json.loads(r['stdout'].splitlines()[-1]);assert d['meter']==(kind=='meter') and not d['counters']
  k=(version,*case);row=rows.setdefault(k,{'times':[]})
  sig=([{k:v for k,v in s.items() if k!='first_answer_ns'} for s in d['samples']],[(p['query'],p['name']) for p in d['phases']])
  key=tuple(case)
  if key in signatures:assert signatures[key]==sig,key
  signatures[key]=sig
  for q,s in enumerate(d['samples']):
   assert s['complete'] and s['depth']==case[2]+q and s['answers']==(2**s['depth'] if case[1] in ('all','duplicates') else 1)
  if kind=='time':assert all(p['memory'] is None for p in d['phases']);row['times'].append(sum(p['ns'] for p in d['phases']));continue
  data=strip(d)
  if 'data' in row:assert row['data']==data,k
  row['data']=data;ms=[p['memory'] for p in d['phases']];root=ms[0]['live_start']
  assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:])),k
  assert ms[-1]['live_end']==root and ms[-2]['live_end']==ms[1]['live_end']
  row['requested']=sum(m['requested_bytes'] for m in ms);row['peak']=max(m['peak_live'] for m in ms)-root
  phases={}
  for p in d['phases']:phases[p['name']]=phases.get(p['name'],0)+p['memory']['requested_bytes']
  row['phases']=phases
summary=[]
for case in CASES:
 a=rows[('baseline',*case)];b=rows[('candidate',*case)]
 assert len(a['times'])==len(b['times'])==5
 for name,value in a['phases'].items():
  if name!='private_solve':assert b['phases'][name]==value,(case,name)
 if case[0]=='scan':assert (a['requested'],a['peak'])==(b['requested'],b['peak']),case
 else:assert b['requested']<=a['requested'],case
 summary.append(dict(case=case,baseline_requested=a['requested'],candidate_requested=b['requested'],baseline_peak=a['peak'],candidate_peak=b['peak'],baseline_median_ns=statistics.median(a['times']),candidate_median_ns=statistics.median(b['times']),baseline_range_ns=[min(a['times']),max(a['times'])],candidate_range_ns=[min(b['times']),max(b['times'])],baseline_phases=a['phases'],candidate_phases=b['phases']))
(R/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(dict(cells=len(rows),exact_allocation_pairs=len(rows),matching_work_cases=len(signatures),unchanged_scan_calibrations=2,allocation_change_only_private_solve=True,owners='pass',frozen_sources='pass')))
