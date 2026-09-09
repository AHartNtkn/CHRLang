#!/usr/bin/env python3
"""Registered history-order confirmation and independent key allocation attribution."""
import hashlib,itertools,json,math,random,shutil,statistics,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'research/chr-direct-conditional/experiments'))
import derivation_sizing as runner
from shared_template_analysis import phases,primary
OUT=ROOT/'docs/experiments/results/s05-history-order-attribution'
BIN=Path('/tmp/chr-history-order-98e319976')
MODES=['direct','exact','alpha','live','scan','sealed','dependencies']
CONFIGS=list(itertools.product(['history','history-early'],[0,64],[1,4],[False,True],[False,True]))
CONTRASTS=[('live','alpha'),('live','exact'),('live','direct'),('live','sealed'),('live','dependencies'),('sealed','direct')]
def main():
 OUT.mkdir(exist_ok=False);runner.OUT=OUT;runner.BIN=BIN
 paths=['Cargo.toml','Cargo.lock','docs/experiments/registrations/S05-history-order-attribution.md',str(Path(__file__).relative_to(ROOT))]
 for d in ['crates/chr-syntax','research/chr-reuse','research/chr-persistent','research/chr-compiled','research/chr-observe','research/chr-direct-choice','research/chr-direct-conditional']:
  paths += [str(p.relative_to(ROOT)) for p in (ROOT/d).rglob('*') if p.is_file() and p.suffix in ['.rs','.toml','.py']]
 f={'commit':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'cpu':runner.CPU,'configs':CONFIGS,'modes':MODES,'sources':{},'binaries':{}}
 for p in sorted(set(paths)):
  dst=OUT/'source'/p;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(ROOT/p,dst);f['sources'][p]=hashlib.sha256(dst.read_bytes()).hexdigest()
 for name in ['ordinary','meter','key-alloc']:
  p=BIN/name;f['binaries'][str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
 (OUT/'freeze.json').write_text(json.dumps(f,indent=2)+'\n')
 check=subprocess.run([str(BIN/'meter'),'meter-check'],capture_output=True,text=True,timeout=60,preexec_fn=runner.limits);(OUT/'meter-check.log').write_text(check.stdout+check.stderr);assert check.returncode==0
 saved={}
 for rep in range(2):
  for i,c in enumerate(CONFIGS):
   for j,m in enumerate(MODES):
    index=i*7+j;v=runner.run('allocation',rep*224+index,(m,*c));record=runner.allocation_records(v)
    if rep==0:saved[index]=record
    else:assert saved[index]==record
  print(f'allocation {rep+1}/2',flush=True)
 rng=random.Random(7821)
 for rep in range(7):
  order=list(range(32));rng.shuffle(order)
  for i in order:
   modes=list(enumerate(MODES));rng.shuffle(modes)
   for j,m in modes:runner.run('ordinary',(rep*32+i)*7+j,(m,*CONFIGS[i]))
  print(f'timing {rep+1}/7',flush=True)
 for j,m in enumerate(MODES):
  for cancel in [0,1]:
   for kind in ['cancel','cancel-meter']:runner.run(kind,2*j+cancel,(m,'history-early',16,2,True,False),cancel)
 diagnostics=[]
 for rep in range(2):
  p=subprocess.run([str(BIN/'key-alloc')],capture_output=True,text=True,timeout=60,preexec_fn=runner.limits)
  (OUT/f'key-alloc-{rep}.json').write_text(json.dumps({'command':[str(BIN/'key-alloc')],'returncode':p.returncode,'stdout':p.stdout,'stderr':p.stderr},indent=2)+'\n');assert p.returncode==0
  rows=[json.loads(l) for l in p.stdout.splitlines() if l.startswith('{')];assert len(rows)==90;diagnostics.append(rows)
 assert diagnostics[0]==diagnostics[1]
 def read(kind,index):return json.loads(json.loads((OUT/f'{kind}-{index:04}.json').read_text())['stdout'].splitlines()[-1])
 results=[]
 for i,c in enumerate(CONFIGS):
  row={'config':c,'modes':{},'contrasts':{}};values={}
  for j,m in enumerate(MODES):
   vs=[read('ordinary',(rep*32+i)*7+j) for rep in range(7)];values[m]=vs;a=read('allocation',i*7+j);ps=list(phases(a).values());all_ps=ps+[a['source_build']]+[x['input_build'] for x in a['samples']]
   row['modes'][m]={'primary_median_ms':statistics.median(primary(v) for v in vs)/1e6,'requested_bytes':sum(p['memory']['requested_bytes'] for p in ps),'peak_requested_growth':max(p['memory']['peak_live'] for p in all_ps)-a['source_build']['memory']['live_start']}
  for j,(a,b) in enumerate(CONTRASTS):
   logs=[math.log(primary(x)/primary(y)) for x,y in zip(values[a],values[b])];rng=random.Random(7822+6*i+j);draws=sorted(math.exp(statistics.mean(rng.choices(logs,k=7))) for _ in range(10000));lo,hi=draws[249],draws[9749]
   row['contrasts'][f'{a}/{b}']={'ratio':math.exp(statistics.mean(logs)),'interval95':[lo,hi],'classification':'gain' if hi<.9 else 'loss' if lo>1.1 else 'unresolved'}
  results.append(row)
 counts={f'{a}/{b}':{c:sum(r['contrasts'][f'{a}/{b}']['classification']==c for r in results) for c in ['gain','loss','unresolved']} for a,b in CONTRASTS}
 (OUT/'summary.json').write_text(json.dumps({'counts':counts,'results':results,'key_allocation':diagnostics[0]},indent=2)+'\n');print(json.dumps(counts),flush=True)
if __name__=='__main__':main()
