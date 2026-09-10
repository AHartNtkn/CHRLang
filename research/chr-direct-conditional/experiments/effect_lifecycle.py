#!/usr/bin/env python3
import hashlib,json,random,resource,subprocess
from pathlib import Path
R=Path('docs/experiments/results/s07-effect-lifecycle');B=Path('target/s07-effect-lifecycle').resolve()
CONTROLS=[('off','ordinary'),('on','ordinary'),('on','inferred'),('on','declared'),('on','required'),('off','scan'),('off','specialized')]
CASES=[(0,'0',0,1,0),(64,'0',0,1,0),(64,'0',0,4,0),(64,'all',0,4,0),(64,'0',0,4,1),(64,'all',1,4,0)]
CELLS=[(p,m,f,*c) for f in ['blocked','rewrite','writer'] for p,m in CONTROLS if not(f=='writer' and m in ['declared','required']) for c in CASES]
ADMISSIONS=[('off','ordinary'),('on','ordinary'),('on','inferred'),('on','declared'),('on','required')]
assert len(CELLS)==114

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def clean(x):
 if isinstance(x,dict):return {k:clean(v) for k,v in x.items() if k not in ['ns','first_answer_ns']}
 if isinstance(x,list):return list(map(clean,x))
 return x

def main():
 R.mkdir(exist_ok=False);B.mkdir(exist_ok=True)
 files=[Path(__file__),Path('Cargo.lock'),Path('Cargo.toml'),Path('docs/experiments/registrations/S07-effect-lifecycle.md')]
 for d in ['research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-relational','research/chr-direct-choice','research/chr-observe','crates/chr-syntax']:
  files.extend(Path(d).rglob('*.rs'));files.append(Path(d)/'Cargo.toml')
 freeze={str(p):sha(p) for p in files}
 for policy in ['off','on']:
  for kind in ['meter','time']:
   features='experiment,head-dispatch,serial-body-accounting,equality-invalidation,support-identities,support-result-cache'+(',effect-contract' if policy=='on' else '')+(',alloc-meter' if kind=='meter' else '')
   cmd=['cargo','build','--release','-p','chr-direct-conditional','--no-default-features','--features',features,'--example','effect_lifecycle']
   run=subprocess.run(cmd,capture_output=True,text=True);(R/f'{policy}-{kind}-build.json').write_text(json.dumps(dict(command=cmd,returncode=run.returncode,stdout=run.stdout,stderr=run.stderr)));assert run.returncode==0
   binary=B/f'{policy}-{kind}';binary.write_bytes(Path('target/release/examples/effect_lifecycle').read_bytes());binary.chmod(0o755);freeze[str(binary)]=sha(binary)
   if kind=='meter':
    check=subprocess.run([str(binary),'meter-check'],capture_output=True,text=True);(R/f'{policy}-check.txt').write_text(check.stdout+check.stderr);assert check.returncode==0
   print('built',policy,kind,flush=True)
 (R/'freeze.json').write_text(json.dumps(freeze,indent=2));(R/'toolchain.txt').write_text(subprocess.check_output(['rustc','-Vv'],text=True))
 for kind,reps,seed in [('meter',2,7930),('time',5,7931)]:
  jobs=[(rep,'life',*c) for rep in range(reps) for c in CELLS]+[(rep,'admission',*a) for rep in range(reps) for a in ADMISSIONS]
  random.Random(seed).shuffle(jobs);(R/kind).mkdir();(R/f'{kind}-order.json').write_text(json.dumps(jobs));(R/f'{kind}-load.txt').write_text(Path('/proc/loadavg').read_text());prior={}
  for i,job in enumerate(jobs):
   rep,action,p,m,*rest=job;cmd=[str(B/f'{p}-{kind}')]
   if action=='admission':cmd += ['admission',m]
   else:
    f,n,k,c,reuse,fail=rest;cmd += [m,f,str(n),'4','8','1',k,str(c),str(reuse),str(fail)]
   try:run=subprocess.run(cmd,capture_output=True,text=True,timeout=75,preexec_fn=limits)
   except subprocess.TimeoutExpired as e:
    (R/kind/f'{i:03}.json').write_text(json.dumps(dict(command=cmd,timeout=True,stdout=str(e.stdout),stderr=str(e.stderr))));raise
   (R/kind/f'{i:03}.json').write_text(json.dumps(dict(command=cmd,returncode=run.returncode,stdout=run.stdout,stderr=run.stderr)));assert run.returncode==0,(kind,i,cmd)
   data=json.loads(run.stdout.splitlines()[-1])
   if action=='admission':assert data['accepted']==(m not in ['declared','required'])
   else:assert data['effect_feature']==(p=='on') and data['meter']==(kind=='meter') and not data['counters']
   if kind=='meter':
    key=tuple(cmd);v=clean(data)
    if key in prior:assert prior[key]==v,key
    prior[key]=v
   if (i+1)%48==0:print(kind,i+1,'/',len(jobs),flush=True)
 assert all(sha(Path(p))==h for p,h in freeze.items())
 (R/'completion.json').write_text(json.dumps(dict(lifecycle_cells=114,admission_cells=5,allocation_processes=238,timing_processes=595,exact_allocation_pairs=119))+'\n')
if __name__=='__main__':main()
