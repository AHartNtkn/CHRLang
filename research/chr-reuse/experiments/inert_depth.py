"""Freeze and execute the registered substantive inert-call work gate."""
import hashlib,json,os,resource,subprocess,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s05-inert-depth'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
 os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_CPU,(180,180));resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
def main():
 assert not (BASE/'freeze.json').exists() and 0 in os.sched_getaffinity(0);binaries={}
 for kind,features in [('work',[]),('plain',['--no-default-features'])]:
  cmd=['cargo','build','-p','chr-reuse','--example','inert_work','--release',*features,'--target-dir',f'target/s05-inert-depth-{kind}','--message-format=json'];r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=300);(BASE/f'build-{kind}.json').write_text(json.dumps(dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr))+'\n');assert r.returncode==0
  artifacts=[json.loads(l) for l in r.stdout.splitlines() if json.loads(l).get('reason')=='compiler-artifact'];p=Path(next(x['executable'] for x in artifacts if x.get('executable')));binaries[kind]=dict(path=str(p),sha256=sha(p))
  if kind=='plain':
   for x in artifacts:
    if x['target']['name'] in ['chr_reuse','chr_persistent','chr_observe','inert_work']:assert not set(x['features'])&{'metrics','kernel-metrics','alloc-meter'}
 paths=subprocess.check_output(['git','ls-files','crates','research','Cargo.toml','Cargo.lock'],text=True).splitlines();paths=[p for p in paths if p.endswith(('.rs','.toml','.lock'))]+['research/chr-reuse/examples/inert_work.rs','research/chr-reuse/experiments/inert_depth.py','research/chr-reuse/experiments/audit_inert_depth.py','docs/experiments/registrations/S05-inert-depth.md'];hashes={p:sha(ROOT/p) for p in sorted(set(paths))}
 with zipfile.ZipFile(BASE/'sources.zip','w',compression=zipfile.ZIP_DEFLATED) as z:
  for p in hashes:z.write(ROOT/p,p)
 (BASE/'freeze.json').write_text(json.dumps(dict(sources=hashes,archive_sha256=sha(BASE/'sources.zip'),binaries=binaries,cpu=0,commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()),indent=2)+'\n')
 for kind in ['work','plain']:
  for rep in [0,1]:
   cmd=[binaries[kind]['path']]
   try:
    r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=180,preexec_fn=limits);raw=dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr,timeout=False)
   except subprocess.TimeoutExpired as e:
    dec=lambda x:x.decode() if isinstance(x,bytes) else x or ''
    raw=dict(command=cmd,exit_code=None,stdout=dec(e.stdout),stderr=dec(e.stderr),timeout=True)
   (BASE/f'run-{kind}-{rep}.json').write_text(json.dumps(raw)+'\n');assert raw['exit_code']==0 and not raw['stderr'] and not raw['timeout'];print(kind,rep,'complete',flush=True)
 assert all(sha(ROOT/p)==h for p,h in hashes.items())
if __name__=='__main__':main()
