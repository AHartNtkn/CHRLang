#!/usr/bin/env python3
import hashlib,itertools,json,random,resource,subprocess
from pathlib import Path
R=Path('docs/experiments/results/s10-arrival-lifecycle'); B=Path('target/s10-arrival-lifecycle').resolve()
CONTROLS=[('ordinary','scan'),('ordinary','resumable'),('ordinary','specialized'),('ordinary','prefix'),('ordinary','prepared-prefix'),('combined','inferred'),('reverse','inferred')]
CELLS=[]
for family,n in [('oldest-first',8),('newest-first',8),('aliases',64),('distinct',64)]:
 cases=[(0,'0',0,1,0,'same'),(n,'0',0,1,0,'same')]
 cases += [(n,k,0,4,0,s) for s,k in itertools.product(['same','changing'],['0','all'])]
 cases += [(n,'0',0,4,1,'changing'),(n,'all',2 if n==8 else 1,4,0,'same' if n==8 else 'changing')]
 for policy,mode in CONTROLS:
  if n==64 and mode in ['prefix','prepared-prefix']:continue
  CELLS += [(policy,mode,family,*case) for case in cases]
assert len(CELLS)==len(set(CELLS))==192

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
 resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def clean(x):
 if isinstance(x,dict):return {k:clean(v) for k,v in x.items() if k not in ['ns','first_answer_ns']}
 if isinstance(x,list):return list(map(clean,x))
 return x

def main():
 R.mkdir(exist_ok=False);B.mkdir(exist_ok=True)
 files=[Path(__file__),Path('Cargo.lock'),Path('Cargo.toml'),Path('docs/experiments/registrations/S10-arrival-lifecycle.md')]
 for d in ['research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-reuse','research/chr-direct-choice','research/chr-relational','research/chr-observe','crates/chr-syntax']:
  files.extend(Path(d).rglob('*.rs'));files.append(Path(d)/'Cargo.toml')
 freeze={str(p):sha(p) for p in files}
 for policy in ['ordinary','combined','reverse']:
  extra='' if policy=='ordinary' else ',support-identities,support-result-cache'+(',support-reverse-order' if policy=='reverse' else '')
  for kind in ['meter','time']:
   f='experiment,head-dispatch,serial-body-accounting,equality-invalidation'+extra+(',alloc-meter' if kind=='meter' else '')
   cmd=['cargo','build','--release','-p','chr-direct-conditional','--no-default-features','--features',f,'--example','arrival_lifecycle']
   run=subprocess.run(cmd,capture_output=True,text=True);(R/f'{policy}-{kind}-build.json').write_text(json.dumps(dict(command=cmd,returncode=run.returncode,stdout=run.stdout,stderr=run.stderr)));assert run.returncode==0
   binary=B/f'{policy}-{kind}';binary.write_bytes(Path('target/release/examples/arrival_lifecycle').read_bytes());binary.chmod(0o755);freeze[str(binary)]=sha(binary)
   if kind=='meter':
    check=subprocess.run([str(binary),'meter-check'],capture_output=True,text=True);(R/f'{policy}-meter-check.txt').write_text(check.stdout+check.stderr);assert check.returncode==0
   print('built',policy,kind,flush=True)
 (R/'freeze.json').write_text(json.dumps(freeze,indent=2));(R/'toolchain.txt').write_text(subprocess.check_output(['rustc','-Vv'],text=True))
 for kind,reps,seed in [('meter',2,7910),('time',5,7911)]:
  jobs=[(rep,*c) for rep in range(reps) for c in CELLS];random.Random(seed).shuffle(jobs)
  (R/kind).mkdir();(R/f'{kind}-order.json').write_text(json.dumps(jobs));(R/f'{kind}-load.txt').write_text(Path('/proc/loadavg').read_text());prior={}
  for i,(rep,p,m,f,n,k,c,reuse,fail,sig) in enumerate(jobs):
   cmd=[str(B/f'{p}-{kind}'),m,f,str(n),'4','8','1',k,str(c),str(reuse),str(fail),sig]
   try:run=subprocess.run(cmd,text=True,capture_output=True,timeout=75,preexec_fn=limits)
   except subprocess.TimeoutExpired as e:
    (R/kind/f'{i:03}.json').write_text(json.dumps(dict(command=cmd,timeout=True,stdout=str(e.stdout),stderr=str(e.stderr))));raise
   (R/kind/f'{i:03}.json').write_text(json.dumps(dict(command=cmd,returncode=run.returncode,stdout=run.stdout,stderr=run.stderr)));assert run.returncode==0,(kind,i,cmd)
   d=json.loads(run.stdout.splitlines()[-1]);assert not d['counters'] and d['meter']==(kind=='meter')
   assert d['signature']==sig and d['artifacts']==((1 if sig=='same' else reuse) if m=='prepared-prefix' else 0)
   if kind=='meter':
    key=tuple(cmd);v=clean(d)
    if key in prior:assert prior[key]==v,key
    prior[key]=v
   if (i+1)%48==0:print(kind,i+1,'/',len(jobs),flush=True)
 assert all(sha(Path(p))==h for p,h in freeze.items())
 (R/'completion.json').write_text(json.dumps(dict(cells=192,allocation_processes=384,timing_processes=960,exact_allocation_pairs=192))+'\n')
if __name__=='__main__':main()
