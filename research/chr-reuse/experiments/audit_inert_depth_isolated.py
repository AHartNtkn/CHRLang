"""Audit complete independent groups without treating memory failures as results."""
import collections,hashlib,itertools,json,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s05-inert-depth/isolated'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f=read(BASE/'freeze.json');assert sha(BASE/'sources.zip')==f['archive_sha256'] and sha(BASE/'jobs.json')==f['jobs_sha256']
with zipfile.ZipFile(BASE/'sources.zip') as z:
 assert set(z.namelist())==set(f['sources'])
 for p,h in f['sources'].items():assert hashlib.sha256(z.read(p)).hexdigest()==h
for kind,b in f['binaries'].items():
 assert sha(Path(b['path']))==b['sha256'];build=read(BASE/f'build-{kind}.json');assert build['exit_code']==0
 if kind=='plain':
  for l in build['stdout'].splitlines():
   x=json.loads(l)
   if x.get('reason')=='compiler-artifact' and x['target']['name'] in ['chr_reuse','chr_persistent','chr_observe','inert_work']:assert not set(x['features'])&{'metrics','kernel-metrics','alloc-meter'}
jobs=[dict(kind=k,rep=r,args=[fam,d,str(dist).lower(),m]) for k,r,fam,d,dist,m in itertools.product(['work','plain'],[0,1],range(6),[0,4,32,128],[False,True],['direct','whole','compact','separate','memo'])];assert read(BASE/'jobs.json')==jobs;assert len(list((BASE/'runs').glob('*.json')))==960
successful=collections.defaultdict(list);failed=[];rows_out=[]
for i,j in enumerate(jobs):
 raw=read(BASE/'runs'/f'{i}.json');assert raw['command']==[f['binaries'][j['kind']]['path'],*map(str,j['args'])]
 if raw['exit_code']!=0 or raw['timeout']:
  assert raw['exit_code']==-6 and not raw['timeout'] and 'memory allocation of ' in raw['stderr'] and 'bytes failed' in raw['stderr'],(i,raw)
  failed.append(dict(index=i,**j,completed_rows=len(raw['stdout'].splitlines()),kind_of_failure='requested allocation failed under 1 GiB address-space limit'));continue
 assert not raw['stderr'];rows=[json.loads(l) for l in raw['stdout'].splitlines()];assert len(rows)==2;fam,depth,distinct,mode=j['args']
 for r,offset in zip(rows,[7,1007]):
  assert {k:r[k] for k in ['family','depth','distinct','offset','mode']}==dict(family=fam,depth=depth,distinct=distinct=='true',offset=offset,mode=mode);c=r['counters'];assert len(c)==8
  if j['kind']=='plain':assert c==[0]*8
  else:assert c[0]==c[1]+c[2] and c[5]==(1 if fam==5 else 2)
 successful[j['kind'],*j['args']].append(rows)
 if j['kind']=='work' and j['rep']==0:rows_out.extend(rows)
for key,rs in successful.items():assert len(rs)==2 and rs[0]==rs[1],key
fail_groups=collections.Counter((x['kind'],*x['args']) for x in failed);assert all(v==2 for v in fail_groups.values())
assert sum(len(rs)*2 for rs in successful.values())+sum(2-x['completed_rows'] for x in failed)==1920
campaign=read(BASE/'campaign.json');assert campaign['processes']==960 and campaign['seconds']<1800
print(json.dumps(dict(processes=960,successful_processes=sum(len(rs) for rs in successful.values()),complete_control_rows=sum(len(rs)*2 for rs in successful.values()),failed_processes=len(failed),failures=failed,exact_successful_repeats=True,plain_counters_zero=True,counter_order=['logical','executed','hits','key_requests','states','completed','failed','max_frontier'],work_rows=rows_out),indent=2))
