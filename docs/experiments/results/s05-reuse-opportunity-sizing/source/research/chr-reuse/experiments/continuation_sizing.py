#!/usr/bin/env python3
"""Prospectively registered exploratory lifecycle sizing."""
import hashlib,itertools,json,random,shutil,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'research/chr-direct-conditional/experiments'))
import derivation_sizing as runner
from shared_template_analysis import phases,primary
OUT=ROOT/'docs/experiments/results/s05-continuation-lifecycle-sizing'
BIN=Path('/tmp/chr-continuation-cost-ca5230779')
MODES=['direct','exact','alpha','live','scan','sealed','dependencies']
CONFIGS=list(itertools.product(['exact','rename','history','distinct'],[0,16,64],[1,4],[False,True],[False,True]))
def main():
 OUT.mkdir(exist_ok=False);runner.OUT=OUT;runner.BIN=BIN
 paths=['Cargo.toml','Cargo.lock','docs/experiments/registrations/S05-continuation-lifecycle-sizing.md',str(Path(__file__).relative_to(ROOT))]
 for d in ['crates/chr-syntax','research/chr-reuse','research/chr-persistent','research/chr-compiled','research/chr-observe','research/chr-direct-choice','research/chr-direct-conditional']:
  paths += [str(p.relative_to(ROOT)) for p in (ROOT/d).rglob('*') if p.is_file() and p.suffix in ['.rs','.toml','.py']]
 f={'commit':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'cpu':runner.CPU,'configs':CONFIGS,'modes':MODES,'sources':{},'binaries':{}}
 for p in sorted(set(paths)):
  dst=OUT/'source'/p;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(ROOT/p,dst);f['sources'][p]=hashlib.sha256(dst.read_bytes()).hexdigest()
 for name in ['ordinary','meter']:
  p=BIN/name;f['binaries'][str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
 (OUT/'freeze.json').write_text(json.dumps(f,indent=2)+'\n')
 check=subprocess.run([str(BIN/'meter'),'meter-check'],capture_output=True,text=True,timeout=60,preexec_fn=runner.limits);(OUT/'meter-check.log').write_text(check.stdout+check.stderr);assert check.returncode==0
 saved={}
 for rep in range(2):
  for i,c in enumerate(CONFIGS):
   for j,m in enumerate(MODES):
    index=i*7+j;v=runner.run('allocation',rep*672+index,(m,*c));record=runner.allocation_records(v)
    if rep==0:saved[index]=record
    else:assert saved[index]==record
  print(f'allocation {rep+1}/2',flush=True)
 order=list(range(672));random.Random(7811).shuffle(order)
 for index in order:
  i,j=divmod(index,7);runner.run('ordinary',index,(MODES[j],*CONFIGS[i]))
 for j,m in enumerate(MODES):
  for cancel in [0,1]:
   for kind in ['cancel','cancel-meter']:runner.run(kind,2*j+cancel,(m,'rename',16,2,True,False),cancel)
 results=[]
 for i,c in enumerate(CONFIGS):
  row={'config':c,'modes':{}}
  for j,m in enumerate(MODES):
   index=i*7+j
   def read(kind):return json.loads(json.loads((OUT/f'{kind}-{index:04}.json').read_text())['stdout'].splitlines()[-1])
   v,a=read('ordinary'),read('allocation');ps=list(phases(a).values());all_ps=ps+[a['source_build']]+[x['input_build'] for x in a['samples']]
   row['modes'][m]={'primary_ms':primary(v)/1e6,'inclusive_ms':(primary(v)+v['source_build']['ns']+sum(x['input_build']['ns'] for x in v['samples']))/1e6,'requested_bytes':sum(p['memory']['requested_bytes'] for p in ps),'peak_requested_growth':max(p['memory']['peak_live'] for p in all_ps)-a['source_build']['memory']['live_start'],'phase_ms':{k:p['ns']/1e6 for k,p in phases(v).items()}}
  results.append(row)
 (OUT/'summary.json').write_text(json.dumps({'exploratory':True,'results':results},indent=2)+'\n');print('2044 successful processes; 672 exact allocation replays',flush=True)
if __name__=='__main__':main()
