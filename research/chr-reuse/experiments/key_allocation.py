"""Frozen allocation attribution and plain correspondence, retaining capacity failures."""
import hashlib,itertools,json,os,resource,subprocess,time,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s05-key-allocation'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def bounds():
 os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(180,180))
def main():
 assert not (BASE/'freeze.json').exists();bins={}
 for kind,features in [('profile',['--features','stage-alloc']),('plain',[])]:
  cmd=['cargo','build','-p','chr-reuse','--example','inert_work','--release','--no-default-features',*features,'--target-dir',f'target/s05-key-allocation-{kind}','--message-format=json'];r=subprocess.run(cmd,capture_output=True,text=True,timeout=300);(BASE/f'build-{kind}.json').write_text(json.dumps(dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr))+'\n');assert r.returncode==0;p=Path(next(json.loads(l)['executable'] for l in r.stdout.splitlines() if json.loads(l).get('executable')));bins[kind]=dict(path=str(p),sha256=sha(p))
  for l in r.stdout.splitlines():
   x=json.loads(l)
   if x.get('reason')=='compiler-artifact' and x['target']['name'] in ['chr_reuse','chr_persistent','chr_observe','inert_work']:
    assert not set(x['features'])&{'metrics','kernel-metrics'}
    if kind=='plain':assert not set(x['features'])&{'alloc-meter','stage-alloc'}
 paths=subprocess.check_output(['git','ls-files','crates','research','Cargo.toml','Cargo.lock'],text=True).splitlines();paths=[p for p in paths if p.endswith(('.rs','.toml','.lock'))]+['research/chr-reuse/src/allocation_profile.rs','research/chr-reuse/tests/allocation_profile.rs','research/chr-reuse/tests/inert_compiled.rs','research/chr-reuse/experiments/key_allocation.py','research/chr-reuse/experiments/audit_key_allocation.py','docs/experiments/registrations/S05-key-allocation.md'];hashes={p:sha(ROOT/p) for p in sorted(set(paths))}
 with zipfile.ZipFile(BASE/'sources.zip','w',compression=zipfile.ZIP_DEFLATED) as z:
  for p in hashes:z.write(ROOT/p,p)
 jobs=[dict(kind=k,rep=r,args=[fam,d,str(dist).lower(),m]) for k,reps in [('profile',2),('plain',1)] for r,fam,d,dist,m in itertools.product(range(reps),range(6),[0,4,32,128],[False,True],['direct','whole','compact','separate','memo'])];(BASE/'jobs.json').write_text(json.dumps(jobs)+'\n');(BASE/'freeze.json').write_text(json.dumps(dict(binaries=bins,sources=hashes,archive_sha256=sha(BASE/'sources.zip'),jobs_sha256=sha(BASE/'jobs.json'),parent_audit_sha256=sha(ROOT/'docs/experiments/results/s05-inert-depth/isolated/audit.json'),cpu=0),indent=2)+'\n');(BASE/'runs').mkdir();start=time.monotonic()
 for i,j in enumerate(jobs):
  assert time.monotonic()-start<1800;cmd=[bins[j['kind']]['path'],*map(str,j['args'])]
  try:
   r=subprocess.run(cmd,capture_output=True,text=True,timeout=180,preexec_fn=bounds);raw=dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr,timeout=False)
  except subprocess.TimeoutExpired as e:
   dec=lambda x:x.decode() if isinstance(x,bytes) else x or ''
   raw=dict(command=cmd,exit_code=None,stdout=dec(e.stdout),stderr=dec(e.stderr),timeout=True)
  (BASE/'runs'/f'{i}.json').write_text(json.dumps(raw)+'\n')
  if (i+1)%120==0:print(i+1,'/',len(jobs),flush=True)
 assert all(sha(ROOT/p)==h for p,h in hashes.items());(BASE/'campaign.json').write_text(json.dumps(dict(processes=720,seconds=time.monotonic()-start))+'\n')
if __name__=='__main__':main()
