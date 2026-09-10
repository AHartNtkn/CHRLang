#!/usr/bin/env python3
"""Execute the prospectively registered finite-phase lifecycle pilot once."""
import hashlib,json,random,resource,subprocess
from pathlib import Path
R=Path('docs/experiments/results/s06-finite-lifecycle-extended')
B=Path('target/s06-finite-lifecycle-extended').resolve()
CONTROLS=[('ascending','finite'),('ascending','scan'),('ascending','specialized'),('ascending','prepared-prefix'),('ascending','conditional'),('reverse','conditional')]
CASES=[(0,1,'0','none',0,0),(4,1,'0','none',0,2),(4,4,'0','none',0,2),(4,4,'4','none',0,2),(4,4,'all','none',0,2),(4,4,'0','none',1,2),(4,4,'all','step4',0,2),(8,1,'0','none',0,2)]
CELLS=[(order,mode,f,*c) for f in ['oldest','newest','all','duplicates'] for order,mode in CONTROLS for c in CASES+([(4,4,'all','answer2',0,2)] if f in ['all','duplicates'] else [])]
assert len(CELLS)==204
FEATURES='experiment,head-dispatch,serial-body-accounting,equality-invalidation,support-identities,support-result-cache'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def bounds():
 resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def strip(x):
 if isinstance(x,dict):return {k:strip(v) for k,v in x.items() if k not in ['ns','first_answer_ns']}
 if isinstance(x,list):return list(map(strip,x))
 return x
def main():
 R.mkdir(exist_ok=False);B.mkdir(exist_ok=True)
 files=[Path(__file__),Path('research/chr-direct-conditional/experiments/audit_finite_lifecycle.py'),Path('Cargo.toml'),Path('Cargo.lock'),Path('docs/experiments/registrations/S06-finite-lifecycle.md')]
 for d in ['research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-relational','research/chr-direct-choice','research/chr-observe','crates/chr-syntax']:
  files.extend(Path(d).rglob('*.rs'));files.append(Path(d)/'Cargo.toml')
 freeze={str(p):sha(p) for p in files}
 for order in ['ascending','reverse']:
  for kind in ['meter','time']:
   features=FEATURES+(',support-reverse-order' if order=='reverse' else '')+(',alloc-meter' if kind=='meter' else '')
   cmd=['cargo','build','--release','-p','chr-direct-conditional','--no-default-features','--features',features,'--example','finite_lifecycle']
   run=subprocess.run(cmd,capture_output=True,text=True);(R/f'{order}-{kind}-build.json').write_text(json.dumps(dict(command=cmd,returncode=run.returncode,stdout=run.stdout,stderr=run.stderr)));assert run.returncode==0,run.stderr
   binary=B/f'{order}-{kind}';binary.write_bytes(Path('target/release/examples/finite_lifecycle').read_bytes());binary.chmod(0o755);freeze[str(binary)]=sha(binary)
   if kind=='meter':
    p=subprocess.run([str(binary),'meter-check'],capture_output=True,text=True);(R/f'{order}-meter-check.txt').write_text(p.stdout+p.stderr);assert p.returncode==0
   print('built',order,kind,flush=True)
 (R/'freeze.json').write_text(json.dumps(freeze,indent=2));(R/'environment.txt').write_text(subprocess.check_output(['rustc','-Vv'],text=True)+subprocess.check_output(['uname','-a'],text=True))
 for kind,reps,seed in [('meter',2,7940),('time',5,7941)]:
  jobs=[(rep,*c) for rep in range(reps) for c in CELLS];random.Random(seed).shuffle(jobs)
  (R/kind).mkdir();(R/f'{kind}-order.json').write_text(json.dumps(jobs));(R/f'{kind}-load.txt').write_text(Path('/proc/loadavg').read_text());seen={}
  for i,(rep,order,mode,f,n,reuse,keep,cancel,fail,work) in enumerate(jobs):
   cmd=[str(B/f'{order}-{kind}'),mode,f,str(n),str(reuse),keep,cancel,str(fail),str(work)]
   try:p=subprocess.run(cmd,capture_output=True,text=True,timeout=75,preexec_fn=bounds)
   except subprocess.TimeoutExpired as e:
    (R/kind/f'{i:04}.json').write_text(json.dumps(dict(command=cmd,timeout=True,stdout=str(e.stdout),stderr=str(e.stderr))));raise
   (R/kind/f'{i:04}.json').write_text(json.dumps(dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)));assert p.returncode==0,(i,cmd,p.stderr)
   d=json.loads(p.stdout.splitlines()[-1]);assert d['meter']==(kind=='meter') and not d['counters'] and d['order']==order
   if kind=='meter':
    k=tuple(cmd);v=strip(d)
    if k in seen:assert seen[k]==v,k
    seen[k]=v
   if (i+1)%48==0:print(kind,i+1,'/',len(jobs),flush=True)
 assert all(sha(Path(p))==h for p,h in freeze.items())
 (R/'completion.json').write_text(json.dumps(dict(cells=204,allocation_processes=408,timing_processes=1020,exact_allocation_pairs=204))+'\n')
if __name__=='__main__':main()
