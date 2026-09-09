#!/usr/bin/env python3
"""Registered forcing-copy attribution; requires both frozen build pairs."""
import hashlib, itertools, json, math, random, shutil, statistics, subprocess
from pathlib import Path
import derivation_sizing as runner
from shared_template_analysis import phases, primary
ROOT=runner.ROOT
OUT=ROOT/'docs/experiments/results/s08-force-copy-attribution'
BIN=Path('/tmp/chr-observation-force-93f78642')
CONFIGS=[(f,n,q,r,o) for f,q,r,o in itertools.product(runner.FAMILIES,[1,8],[False,True],[False,True]) for n in [0,12 if f=='grow' else 32]]
CELLS=list(itertools.product(['before','after'],['dependencies','templates']))
def main():
 OUT.mkdir(exist_ok=False)
 runner.OUT=OUT
 paths=['Cargo.toml','Cargo.lock','docs/experiments/registrations/S08-force-copy-attribution.md',str(Path(__file__).relative_to(ROOT))]
 for directory in ['crates/chr-syntax','research/chr-direct-choice','research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-observe','research/chr-boxes']:
  paths += [str(p.relative_to(ROOT)) for p in (ROOT/directory).rglob('*') if p.is_file() and p.suffix in ['.rs','.toml','.py']]
 freeze={'commit':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'cpu':runner.CPU,'configs':CONFIGS,'cells':CELLS,'sources':{},'binaries':{},'variants':{}}
 for p in sorted(set(paths)):
  target=OUT/'source'/p;target.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(ROOT/p,target);freeze['sources'][p]=hashlib.sha256(target.read_bytes()).hexdigest()
 for version in ['before','after']:
  source=BIN/version/'demand.rs';shutil.copyfile(source,OUT/(version+'-demand.rs'));freeze['variants'][version]=hashlib.sha256(source.read_bytes()).hexdigest()
  for name in ['ordinary','meter']:
   p=BIN/version/name;freeze['binaries'][str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
 (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
 for version in ['before','after']:
  p=subprocess.run([str(BIN/version/'meter'),'meter-check'],capture_output=True,text=True,timeout=60,preexec_fn=runner.limits)
  (OUT/(version+'-meter-check.log')).write_text(p.stdout+p.stderr);assert p.returncode==0
 saved={}
 for rep in range(2):
  for i,c in enumerate(CONFIGS):
   for j,(version,mode) in enumerate(CELLS):
    runner.BIN=BIN/version
    value=runner.allocation_records(runner.run('allocation',(rep*80+i)*4+j,(mode,*c)))
    if rep==0:saved[i,j]=value
    else:assert saved[i,j]==value
  print(f'allocation replay {rep+1}/2',flush=True)
 rng=random.Random(7741)
 for rep in range(7):
  order=list(range(80));rng.shuffle(order)
  for i in order:
   cells=list(enumerate(CELLS));rng.shuffle(cells)
   for j,(version,mode) in cells:
    runner.BIN=BIN/version;runner.run('ordinary',(rep*80+i)*4+j,(mode,*CONFIGS[i]))
  print(f'timing block {rep+1}/7',flush=True)
 for j,(version,mode) in enumerate(CELLS):
  runner.BIN=BIN/version
  for cancel in [0,1]:
   for kind in ['cancel','cancel-meter']:runner.run(kind,2*j+cancel,(mode,'choice',8,2,True,False),cancel)
 results=[]
 def read(kind,index):
  raw=json.loads((OUT/f'{kind}-{index:04}.json').read_text());assert raw['returncode']==0
  return json.loads(raw['stdout'].splitlines()[-1])
 for i,c in enumerate(CONFIGS):
  for j,mode in enumerate(['dependencies','templates']):
   values=[[read('ordinary',(rep*80+i)*4+j+offset) for rep in range(7)] for offset in [0,2]]
   logs=[math.log(primary(a)/primary(b)) for a,b in zip(values[1],values[0])]
   rng=random.Random(7742+2*i+j);draws=sorted(math.exp(statistics.mean(rng.choices(logs,k=7))) for _ in range(10000));lo,hi=draws[249],draws[9749]
   row={'config':c,'mode':mode,'ratio':math.exp(statistics.mean(logs)),'interval95':[lo,hi],'classification':'gain' if hi<.9 else 'loss' if lo>1.1 else 'unresolved','versions':{}}
   for offset,version,vs in zip([0,2],['before','after'],values):
    meter=read('allocation',i*4+j+offset)
    row['versions'][version]={'median_primary_ms':statistics.median(primary(v) for v in vs)/1e6,'requested_by_phase':{k:v['memory']['requested_bytes'] for k,v in phases(meter).items()},'requested_bytes':sum(v['memory']['requested_bytes'] for v in phases(meter).values())}
   results.append(row)
 counts={m:{k:sum(r['mode']==m and r['classification']==k for r in results) for k in ['gain','loss','unresolved']} for m in ['dependencies','templates']}
 (OUT/'summary.json').write_text(json.dumps({'counts':counts,'results':results},indent=2)+'\n');print(json.dumps(counts),flush=True)
if __name__=='__main__':main()
