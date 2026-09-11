"""Attribute completed allocation only, preserving the prior capacity obstruction."""
import collections,hashlib,itertools,json,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s05-key-allocation';PARENT=ROOT/'docs/experiments/results/s05-inert-depth/isolated'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f=read(BASE/'freeze.json');assert sha(BASE/'sources.zip')==f['archive_sha256'] and sha(BASE/'jobs.json')==f['jobs_sha256'] and sha(PARENT/'audit.json')==f['parent_audit_sha256']
with zipfile.ZipFile(BASE/'sources.zip') as z:
 assert set(z.namelist())==set(f['sources'])
 for p,h in f['sources'].items():assert hashlib.sha256(z.read(p)).hexdigest()==h
for kind,b in f['binaries'].items():
 assert sha(Path(b['path']))==b['sha256'];build=read(BASE/f'build-{kind}.json');assert build['exit_code']==0
 for l in build['stdout'].splitlines():
  x=json.loads(l)
  if x.get('reason')=='compiler-artifact' and x['target']['name'] in ['chr_reuse','chr_persistent','chr_observe','inert_work']:
   assert not set(x['features'])&{'metrics','kernel-metrics'}
   if kind=='plain':assert not set(x['features'])&{'stage-alloc','alloc-meter'}
work={(x['family'],x['depth'],x['distinct'],x['offset'],x['mode']):x['counters'] for x in read(PARENT/'audit.json')['work_rows']}
plain={}
for i,j in enumerate(read(PARENT/'jobs.json')):
 if j['kind']=='plain' and j['rep']==0:
  raw=read(PARENT/'runs'/f'{i}.json')
  if raw['exit_code']==0:plain[tuple(j['args'])]=[json.loads(l) for l in raw['stdout'].splitlines()]
jobs=[dict(kind=k,rep=r,args=[fam,d,str(dist).lower(),m]) for k,reps in [('profile',2),('plain',1)] for r,fam,d,dist,m in itertools.product(range(reps),range(6),[0,4,32,128],[False,True],['direct','whole','compact','separate','memo'])];assert read(BASE/'jobs.json')==jobs and len(list((BASE/'runs').glob('*.json')))==720
completed=collections.defaultdict(list);failed=[];summaries=[]
for i,j in enumerate(jobs):
 raw=read(BASE/'runs'/f'{i}.json');assert raw['command']==[f['binaries'][j['kind']]['path'],*map(str,j['args'])]
 if raw['exit_code']!=0 or raw['timeout']:
  assert raw['exit_code']==-6 and not raw['timeout'] and 'memory allocation of ' in raw['stderr'] and 'bytes failed' in raw['stderr'];assert j['args']==[4,128,'true','whole'];failed.append(dict(index=i,**j));continue
 assert not raw['stderr'];rows=[json.loads(l) for l in raw['stdout'].splitlines()];args=tuple(j['args']);completed[j['kind'],*args].append(rows)
 if j['kind']=='plain':assert rows==plain[args];continue
 assert len(rows)==4
 for at,offset in zip([0,2],[7,1007]):
  alloc,counters=rows[at:at+2];fam,depth,distinct,mode=args;identity=dict(family=fam,depth=depth,distinct=distinct=='true',offset=offset,mode=mode);assert {k:alloc[k] for k in identity}==identity and alloc['event']=='allocation';assert counters==dict(**identity,counters=[0]*8)
  old=work[fam,depth,distinct=='true',offset,mode];assert alloc['stage_calls'][0]==old[3] and alloc['stage_calls'][2]==old[1] and alloc['stage_calls'][4]==old[0]
  assert len(alloc['stage_bytes'])==len(alloc['stage_calls'])==len(alloc['stage_allocations'])==5
  assert all(x>=0 for x in alloc['stage_bytes']);assert sum(alloc['stage_bytes'])<=alloc['requested'];assert all(x==0 for x in alloc['stage_bytes'][0:2]) if mode=='direct' else True
  if j['rep']==0:summaries.append(dict(**alloc,remainder=alloc['requested']-sum(alloc['stage_bytes'])))
for key,rs in completed.items():assert len(rs)==(2 if key[0]=='profile' else 1) and all(r==rs[0] for r in rs)
assert len(failed)==3 and len(summaries)==478
campaign=read(BASE/'campaign.json');assert campaign['processes']==720 and campaign['seconds']<1800
print(json.dumps(dict(processes=720,successful_processes=717,profile_queries_per_repeat=478,plain_queries=478,failed_processes=failed,exact_profile_repeats=True,plain_parent_correspondence=True,stage_invocations_match_independent_work=True,stages=['key_export_normalize','inert_detach_export','machine_step','cached_edge_copy','frontier_output_assembly'],scope='Requested traffic from query engine start through complete collection; not full lifecycle or peak/RSS.',summary=summaries),indent=2))
