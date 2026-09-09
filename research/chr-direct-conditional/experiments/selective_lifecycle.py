#!/usr/bin/env python3
import hashlib,itertools,json,random,resource,subprocess,sys
from pathlib import Path
out=Path(sys.argv[1]); out.mkdir(exist_ok=False)
root=Path(sys.argv[2]).resolve()
binaries={(variant,kind):root/f'{variant}-{kind}' for variant in ['existing','filtered'] for kind in ['meter','time']}
cells=list(itertools.product(['existing','filtered','scan'],['original','counted'],['common','independent','early'],[0,3],[0,16],[1,4],['eligible','mixed']))
cells += [(variant,'original','dense',k,0,reuse,'eligible') for variant,k,reuse in itertools.product(['existing','filtered','scan'],[0,3],[1,4])]
assert len(cells)==300
files=[Path(__file__),Path('docs/experiments/registrations/S10-selective-discovery-lifecycle.md'),Path('Cargo.lock'),*binaries.values()]
for name in ['chr-direct-conditional','chr-compiled','chr-relational','chr-persistent','chr-observe']:
 p=Path('research')/name;files.extend(p.rglob('*.rs'));files.append(p/'Cargo.toml')
files.extend(Path('crates/chr-syntax').rglob('*.rs'));files.append(Path('crates/chr-syntax/Cargo.toml'))
freeze={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in files};(out/'freeze.json').write_text(json.dumps(freeze,indent=2));(out/'toolchain.txt').write_text(subprocess.check_output(['rustc','--version','--verbose'],text=True))
def limits():
 resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def group(kind,reps,seed):
 jobs=[(r,*c) for r in range(reps) for c in cells];random.Random(seed).shuffle(jobs);(out/f'{kind}-order.json').write_text(json.dumps(jobs));(out/kind).mkdir();prior={}
 for i,job in enumerate(jobs):
  r,variant,form,family,k,d,reuse,schedule=job;mode=form+'-'+('scan' if variant=='scan' else 'conditional');binary=binaries[('filtered' if variant=='filtered' else 'existing'),kind]
  p=out/kind/('-'.join(map(str,job))+'.json')
  run=subprocess.run([str(binary),mode,family,str(k),str(d),str(reuse),schedule],capture_output=True,text=True,timeout=75,preexec_fn=limits)
  if run.returncode:p.with_suffix('.error').write_text(run.stdout+run.stderr);raise RuntimeError(str(p))
  p.write_text(run.stdout);x=json.loads(run.stdout)
  assert (x['mode'],x['selective'],x['family'],x['choices'],x['depth'],x['reuse'],x['schedule'],x['meter'])==(mode,variant=='filtered',family,k,d,reuse,schedule,kind=='meter')
  admitted=0 if form=='original' else (reuse+1 if schedule=='eligible' else (reuse+1)//2)
  assert x['admitted']==admitted and len(x['phases'])==11+9*reuse
  if kind=='meter':
   ms=[p['memory'] for p in x['phases']];assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]));assert ms[0]['live_start']==ms[-1]['live_end']
   key=tuple(job[1:])
   if key in prior:assert prior[key]==ms,key
   prior[key]=ms
  if (i+1)%150==0:print(kind,i+1,'/',len(jobs),flush=True)
 return len(jobs)
n=group('meter',2,7802);(out/'allocation-gate.json').write_text(json.dumps({'processes':n,'exact_pairs':300}))
n+=group('time',5,7803)
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in freeze.items())
(out/'completion.json').write_text(json.dumps({'processes':n,'cells':300,'timing':'exploratory'}))
