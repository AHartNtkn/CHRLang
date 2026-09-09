import hashlib,itertools,json,shutil,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'research/chr-direct-conditional/experiments'))
import derivation_sizing as runner
from shared_template_analysis import phases
OUT=ROOT/'docs/experiments/results/s05-compact-key-allocation-gate'
BIN=Path('/tmp/chr-compact-key-12af2f36f')
MODES=['direct','exact','alpha','live','scan','sealed','dependencies','compact-exact','compact-alpha','compact-live']
CONFIGS=[(f,n,2,r,False) for f,n,r in itertools.product(['exact','rename','history','history-early','distinct'],[0,64],[False,True])]
def main():
 OUT.mkdir(exist_ok=False);runner.OUT=OUT;runner.BIN=BIN
 paths=['Cargo.toml','Cargo.lock','docs/experiments/registrations/S05-compact-key-allocation-gate.md',str(Path(__file__).relative_to(ROOT))]
 for d in ['crates/chr-syntax','research/chr-reuse','research/chr-persistent','research/chr-compiled','research/chr-observe','research/chr-direct-choice','research/chr-direct-conditional']:
  paths += [str(p.relative_to(ROOT)) for p in (ROOT/d).rglob('*') if p.is_file() and p.suffix in ['.rs','.toml','.py']]
 freeze={'commit':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'cpu':runner.CPU,'configs':CONFIGS,'modes':MODES,'sources':{},'binaries':{}}
 for p in sorted(set(paths)):
  target=OUT/'source'/p;target.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(ROOT/p,target);freeze['sources'][p]=hashlib.sha256(target.read_bytes()).hexdigest()
 p=BIN/'meter';freeze['binaries'][str(p)]=hashlib.sha256(p.read_bytes()).hexdigest();(OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
 check=subprocess.run([str(p),'meter-check'],capture_output=True,text=True,timeout=60,preexec_fn=runner.limits);(OUT/'meter-check.log').write_text(check.stdout+check.stderr);assert check.returncode==0
 saved={};results=[]
 for rep in range(2):
  for i,c in enumerate(CONFIGS):
   row={'config':c,'modes':{}}
   for j,m in enumerate(MODES):
    index=10*i+j;v=runner.run('allocation',200*rep+index,(m,*c));record=runner.allocation_records(v)
    if rep==0:saved[index]=record
    else:assert record==saved[index]
    p=list(phases(v).values());all_p=p+[v['source_build']]+[s['input_build'] for s in v['samples']]
    row['modes'][m]={'requested_bytes':sum(x['memory']['requested_bytes'] for x in p),'peak_growth':max(x['memory']['peak_live'] for x in all_p)-v['source_build']['memory']['live_start'],'phase_requested':{k:p['memory']['requested_bytes'] for k,p in phases(v).items()}}
   if rep==0:results.append(row)
  print(f'allocation repetition {rep+1}/2',flush=True)
 for j,m in enumerate(MODES):runner.run('cancel-meter',j,(m,'history-early',16,2,True,False),1)
 (OUT/'summary.json').write_text(json.dumps({'results':results},indent=2)+'\n');print('410 successful processes, 200 exact allocation replays',flush=True)
if __name__=='__main__':main()
