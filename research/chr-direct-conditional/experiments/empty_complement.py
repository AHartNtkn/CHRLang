#!/usr/bin/env python3
import hashlib,json,random,resource,subprocess
from pathlib import Path
R=Path('docs/experiments/results/s06-empty-complement-attribution')
OLD=Path('target/s06-finite-lifecycle-extended').resolve();NEW=Path('target/s06-empty-complement').resolve()
CASES=[('finite',f,*c) for f in ['oldest','newest','all','duplicates'] for c in [(0,1,'0','none',0,0),(4,4,'0','none',0,2),(4,4,'all','none',0,2),(8,1,'0','none',0,2)]]+[('scan',f,4,4,'0','none',0,2) for f in ['oldest','all']]
assert len(CASES)==18
FEATURES='experiment,head-dispatch,serial-body-accounting,equality-invalidation,support-identities,support-result-cache'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def strip(v):
 if isinstance(v,dict):return {k:strip(x) for k,x in v.items() if k not in ('ns','first_answer_ns')}
 if isinstance(v,list):return list(map(strip,v))
 return v
def main():
 R.mkdir(exist_ok=False);NEW.mkdir(exist_ok=True)
 files=[Path(__file__),Path('research/chr-direct-conditional/experiments/audit_empty_complement.py'),Path('docs/experiments/registrations/S06-empty-complement-attribution.md'),Path('Cargo.toml'),Path('Cargo.lock')]
 for d in ['research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-relational','research/chr-direct-choice','research/chr-observe','crates/chr-syntax']:
  files.extend(Path(d).rglob('*.rs'));files.append(Path(d)/'Cargo.toml')
 freeze={str(p):sha(p) for p in files}
 for kind in ['meter','time']:
  cmd=['cargo','build','--release','-p','chr-direct-conditional','--no-default-features','--features',FEATURES+(',alloc-meter' if kind=='meter' else ''),'--example','finite_lifecycle']
  p=subprocess.run(cmd,capture_output=True,text=True);(R/f'{kind}-build.json').write_text(json.dumps(dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)));assert p.returncode==0,p.stderr
  b=NEW/kind;b.write_bytes(Path('target/release/examples/finite_lifecycle').read_bytes());b.chmod(0o755)
  for path in [b,OLD/f'ascending-{kind}']:freeze[str(path)]=sha(path)
  if kind=='meter':
   p=subprocess.run([str(b),'meter-check'],capture_output=True,text=True);assert p.returncode==0;(R/'meter-check.log').write_text(p.stdout+p.stderr)
 (R/'freeze.json').write_text(json.dumps(freeze,indent=2));(R/'environment.txt').write_text(subprocess.check_output(['rustc','-Vv'],text=True))
 for kind,reps,seed in [('meter',2,7942),('time',5,7943)]:
  jobs=[(rep,version,*case) for rep in range(reps) for version in ['baseline','candidate'] for case in CASES];random.Random(seed).shuffle(jobs)
  (R/kind).mkdir();(R/f'{kind}-order.json').write_text(json.dumps(jobs));seen={}
  for i,(rep,version,*case) in enumerate(jobs):
   b=OLD/f'ascending-{kind}' if version=='baseline' else NEW/kind;cmd=[str(b),*map(str,case)]
   try:p=subprocess.run(cmd,capture_output=True,text=True,timeout=75,preexec_fn=limits)
   except subprocess.TimeoutExpired as e:
    (R/kind/f'{i:03}.json').write_text(json.dumps(dict(command=cmd,version=version,timeout=True,stdout=str(e.stdout),stderr=str(e.stderr))));raise
   (R/kind/f'{i:03}.json').write_text(json.dumps(dict(command=cmd,version=version,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)));assert p.returncode==0,(i,cmd,p.stderr)
   if kind=='meter':
    k=tuple(cmd);value=strip(json.loads(p.stdout.splitlines()[-1]))
    if k in seen:assert seen[k]==value,k
    seen[k]=value
   if (i+1)%36==0:print(kind,i+1,'/',len(jobs),flush=True)
 assert all(sha(Path(p))==h for p,h in freeze.items())
 (R/'completion.json').write_text(json.dumps(dict(cases=18,cells=36,allocation_processes=72,timing_processes=180))+'\n')
if __name__=='__main__':main()
