"""Verify frozen work repeats and all adverse source configurations."""
import hashlib,itertools,json,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s05-inert-depth'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f=read(BASE/'freeze.json');assert sha(BASE/'sources.zip')==f['archive_sha256']
with zipfile.ZipFile(BASE/'sources.zip') as z:
 assert set(z.namelist())==set(f['sources'])
 for n,h in f['sources'].items():assert hashlib.sha256(z.read(n)).hexdigest()==h
expected=set(itertools.product(range(6),[0,4,32,128],[False,True],[7,1007],['direct','whole','compact','separate','memo']));runs={}
for kind,b in f['binaries'].items():
 assert sha(Path(b['path']))==b['sha256'];build=read(BASE/f'build-{kind}.json');assert build['exit_code']==0
 if kind=='plain':
  for l in build['stdout'].splitlines():
   x=json.loads(l)
   if x.get('reason')=='compiler-artifact' and x['target']['name'] in ['chr_reuse','chr_persistent','chr_observe','inert_work']:assert not set(x['features'])&{'metrics','kernel-metrics','alloc-meter'}
 for rep in [0,1]:
  raw=read(BASE/f'run-{kind}-{rep}.json');assert raw['command']==[b['path']] and raw['exit_code']==0 and not raw['stderr'] and not raw['timeout'];rows=[json.loads(l) for l in raw['stdout'].splitlines()];assert len(rows)==480
  keys={(r['family'],r['depth'],r['distinct'],r['offset'],r['mode']) for r in rows};assert keys==expected
  for r in rows:
   s=r['counters'];assert len(s)==8
   if kind=='plain':assert s==[0]*8
   else:assert s[0]==s[1]+s[2] and s[5]==(1 if r['family']==5 else 2)
  runs[kind,rep]=rows
 assert runs[kind,0]==runs[kind,1]
lookup={(r['family'],r['depth'],r['distinct'],r['offset'],r['mode']):r['counters'] for r in runs['work',0]};contrasts=[]
for family,depth,distinct,offset in itertools.product(range(6),[0,4,32,128],[False,True],[7,1007]):
 counts={m:lookup[family,depth,distinct,offset,m] for m in ['direct','whole','compact','separate','memo']};contrasts.append(dict(family=family,depth=depth,distinct=distinct,offset=offset,counters=counts,avoided_vs_separate=counts['separate'][1]-counts['memo'][1]))
print(json.dumps(dict(recorded_complete_executions=1920,independent_direct_preflights=384,cancellation_sessions=1920,source_configurations=96,exact_work_repeats=True,plain_counters_zero=True,counter_order=['logical','executed','hits','key_requests','states','completed','failed','max_frontier'],summary=contrasts),indent=2))
