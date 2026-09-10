"""Freeze and confirm graph reduction and source correspondence independently."""
import hashlib,json,resource,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s06-graph-simplification/isolated-confirmation'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
 resource.setrlimit(resource.RLIMIT_CPU,(60,60));resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
def main():
 OUT.mkdir(exist_ok=True)
 assert not (OUT/'freeze.json').exists(),'already frozen'
 binaries=[]
 for profile in ['debug','release']:
  cmd=['cargo','test','-p','chr-structural','--test','graph_simplification','--test','diagram_source','--no-run','--message-format=json','--target-dir',str(ROOT/'target/s06-graph-simplification-confirmation')]
  if profile=='release':cmd.append('--release')
  p=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=300)
  (OUT/f'build-{profile}.jsonl').write_text(p.stdout);(OUT/f'build-{profile}.log').write_text(p.stderr);p.check_returncode()
  for line in p.stdout.splitlines():
   r=json.loads(line)
   if r.get('executable') and r['target']['name'] in ['graph_simplification','diagram_source']:
    path=Path(r['executable']);binaries.append({'profile':profile,'test':r['target']['name'],'path':str(path.relative_to(ROOT)),'sha256':sha(path)})
 assert len(binaries)==4
 files=[Path(__file__),ROOT/'research/chr-structural/tests/graph_simplification.rs',ROOT/'research/chr-structural/tests/diagram_source.rs',ROOT/'research/chr-structural/Cargo.toml',ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'docs/experiments/registrations/S06-graph-simplification.md']
 for directory in ['research/chr-structural/src','crates/chr-reference/src','crates/chr-syntax/src']:files+=list((ROOT/directory).rglob('*.rs'))
 freeze={'sources':{str(p.relative_to(ROOT)):sha(p) for p in files},'binaries':binaries,'rustc':subprocess.check_output(['rustc','-Vv'],text=True)}
 (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
 results=[]
 for b in binaries:
  p=subprocess.run([str(ROOT/b['path']),'--nocapture'],capture_output=True,text=True,timeout=60,preexec_fn=limits)
  (OUT/f"{b['profile']}-{b['test']}.log").write_text(p.stdout+p.stderr);p.check_returncode()
  assert '2 passed; 0 failed' in p.stdout
  expected='graph_projections=3072 union_projections=128' if b['test']=='graph_simplification' else 'source_configurations=360 candidate_set_checks=1800 multiplicity_cases=138'
  assert expected in p.stdout
  results.append({'profile':b['profile'],'test':b['test'],'checks':expected,'exit_code':p.returncode})
 (OUT/'audit.json').write_text(json.dumps(results,indent=2)+'\n');print(json.dumps(results))
if __name__=='__main__':main()
