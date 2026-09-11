"""Independent resource bounds for every candidate; preserve failed cells."""
import hashlib,itertools,json,os,resource,subprocess,time,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s05-inert-depth/isolated'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def bounds():
 os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(180,180))
def main():
 BASE.mkdir(exist_ok=True);assert not (BASE/'freeze.json').exists();bins={}
 for kind,features in [('work',[]),('plain',['--no-default-features'])]:
  cmd=['cargo','build','-p','chr-reuse','--example','inert_work','--release',*features,'--target-dir',f'target/s05-inert-depth-isolated-{kind}','--message-format=json'];r=subprocess.run(cmd,capture_output=True,text=True,timeout=300);(BASE/f'build-{kind}.json').write_text(json.dumps(dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr))+'\n');assert r.returncode==0;p=Path(next(json.loads(l)['executable'] for l in r.stdout.splitlines() if json.loads(l).get('executable')));bins[kind]=dict(path=str(p),sha256=sha(p))
 paths=subprocess.check_output(['git','ls-files','crates','research','Cargo.toml','Cargo.lock'],text=True).splitlines();paths=[p for p in paths if p.endswith(('.rs','.toml','.lock'))]+['research/chr-reuse/examples/inert_work.rs','research/chr-reuse/experiments/inert_depth_isolated.py','research/chr-reuse/experiments/audit_inert_depth_isolated.py','docs/experiments/registrations/S05-inert-depth-isolation.md'];hashes={p:sha(ROOT/p) for p in sorted(set(paths))}
 with zipfile.ZipFile(BASE/'sources.zip','w',compression=zipfile.ZIP_DEFLATED) as z:
  for p in hashes:z.write(ROOT/p,p)
 jobs=[dict(kind=k,rep=r,args=[f,d,str(dist).lower(),m]) for k,r,f,d,dist,m in itertools.product(['work','plain'],[0,1],range(6),[0,4,32,128],[False,True],['direct','whole','compact','separate','memo'])];(BASE/'jobs.json').write_text(json.dumps(jobs)+'\n');(BASE/'freeze.json').write_text(json.dumps(dict(binaries=bins,sources=hashes,archive_sha256=sha(BASE/'sources.zip'),jobs_sha256=sha(BASE/'jobs.json'),cpu=0),indent=2)+'\n');start=time.monotonic();(BASE/'runs').mkdir()
 for i,j in enumerate(jobs):
  assert time.monotonic()-start<1800;cmd=[bins[j['kind']]['path'],*map(str,j['args'])]
  try:
   r=subprocess.run(cmd,capture_output=True,text=True,timeout=180,preexec_fn=bounds);raw=dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr,timeout=False)
  except subprocess.TimeoutExpired as e:
   dec=lambda x:x.decode() if isinstance(x,bytes) else x or ''
   raw=dict(command=cmd,exit_code=None,stdout=dec(e.stdout),stderr=dec(e.stderr),timeout=True)
  (BASE/'runs'/f'{i}.json').write_text(json.dumps(raw)+'\n')
  if (i+1)%120==0:print(i+1,'/',len(jobs),flush=True)
 assert all(sha(ROOT/p)==h for p,h in hashes.items());(BASE/'campaign.json').write_text(json.dumps(dict(processes=len(jobs),seconds=time.monotonic()-start))+'\n')
if __name__=='__main__':main()
