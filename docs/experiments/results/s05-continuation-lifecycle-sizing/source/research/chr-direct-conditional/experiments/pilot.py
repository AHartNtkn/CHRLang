#!/usr/bin/env python3
"""Execute prospectively registered R03 conditional lifecycle pilot serially."""
import hashlib, itertools, json, os, pathlib, random, resource, subprocess, time
ROOT=pathlib.Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/r03-conditional-lifecycle'
OUT.mkdir(exist_ok=True)
cpu=min(os.sched_getaffinity(0))
start=time.monotonic()
meta={'affinity_cpu':cpu,'uname':list(os.uname()),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'base_commit':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()}
files=[ROOT/'Cargo.lock',ROOT/'Cargo.toml',ROOT/'docs/experiments/registrations/R03-conditional-lifecycle.md']
for base in ['research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax']:
 files += [p for p in (ROOT/base).rglob('*') if p.suffix in ['.rs','.toml','.py']]
meta['sources']={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(files))}
binaries={}
for mode,features in [('primary',['--no-default-features','--features','experiment']),('allocation',['--no-default-features','--features','alloc-meter']),('work',['--features','experiment'])]:
 target=ROOT/'target'/('r03-conditional-'+mode)
 command=['cargo','build','--release','-p','chr-direct-conditional','--bin','chr-conditional-cost','--target-dir',str(target),*features]
 with (OUT/(mode+'-build.log')).open('w') as log: subprocess.run(command,cwd=ROOT,stdout=log,stderr=subprocess.STDOUT,check=True)
 binaries[mode]=str(target/'release/chr-conditional-cost')
 meta.setdefault('builds',{})[mode]={'command':command,'binary_sha256':hashlib.sha256(pathlib.Path(binaries[mode]).read_bytes()).hexdigest()}
(OUT/'metadata.json').write_text(json.dumps(meta,indent=2)+'\n')
def limits():
 os.sched_setaffinity(0,{cpu});resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3))
check=subprocess.run([binaries['allocation'],'meter-check'],capture_output=True,text=True,timeout=30,preexec_fn=limits)
(OUT/'meter-check.log').write_text(check.stdout+check.stderr);check.check_returncode()
cells=[(backend,family,n,q) for backend in ['conditional','explicit'] for family in ['plain','shared','discriminate','output'] for n in ([4,6] if family=='output' else [16,64]) for q in [1,4]]
rng=random.Random(35036)
jobs=[]
for mode,reps in [('warmup',1),('primary',5),('allocation',1),('work',1)]:
 batch=[(mode,r,cell) for r in range(reps) for cell in cells];rng.shuffle(batch);jobs.extend(batch)
with (OUT/'runs.jsonl').open('x') as log:
 for i,(mode,rep,cell) in enumerate(jobs):
  if time.monotonic()-start>1800: raise RuntimeError('registered total wall bound')
  command=[binaries['primary' if mode=='warmup' else mode],*map(str,cell)]
  before=time.monotonic()
  try:
   result=subprocess.run(command,capture_output=True,text=True,timeout=30,preexec_fn=limits)
   row={'mode':mode,'rep':rep,'cell':cell,'exit':result.returncode,'stderr':result.stderr,'wall_s':time.monotonic()-before}
   row['stdout']=result.stdout
   if result.returncode==0:
    try: row['result']=json.loads(result.stdout)
    except json.JSONDecodeError as e: row['decode_error']=str(e)
  except subprocess.TimeoutExpired as e: row={'mode':mode,'rep':rep,'cell':cell,'timeout':True,'stdout':str(e.stdout),'stderr':str(e.stderr)}
  log.write(json.dumps(row)+'\n');log.flush()
  if i%32==31: print(f'{i+1}/{len(jobs)} processes recorded',flush=True)
for name,digest in meta['sources'].items():
 assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==digest,('source changed',name)
(OUT/'source-check.log').write_text('All registered source hashes unchanged after runs.\n')
