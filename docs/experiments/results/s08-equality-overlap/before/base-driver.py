"""Frozen repeated source attribution, with explicit physical/service bounds."""
from pathlib import Path
import hashlib,json,resource,subprocess,sys
ROOT=Path(__file__).resolve().parents[3];RAW=ROOT/'docs/experiments/results/s08-equality-overlap'
def h(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
 resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
if __name__=='__main__':
 mode=sys.argv[1];assert mode in ['base','reverse','shortcut','shortcut-reverse'];binary=ROOT/f'target/s08-overlap-{mode}/release/examples/equality_overlap'
 files=[Path(__file__),binary,ROOT/'research/chr-direct-conditional/examples/equality_overlap.rs',ROOT/'research/chr-direct-conditional/Cargo.toml',ROOT/'docs/experiments/registrations/S08-equality-overlap.md',ROOT/'Cargo.lock']+list((ROOT/'research/chr-direct-conditional/src').glob('*.rs'))
 with (RAW/f'{mode}-manifest.json').open('x') as f:json.dump({str(p.relative_to(ROOT)):h(p) for p in files},f,indent=2)
 outcomes=[]
 for rep in range(2):
  cmd=[str(binary)];p=subprocess.run(cmd,capture_output=True,text=True,timeout=60,preexec_fn=limits)
  result=dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
  with (RAW/f'{mode}-{rep}.json').open('x') as f:json.dump(result,f,indent=2)
  assert p.returncode==0 and not p.stderr,result
  rows=[json.loads(l) for l in p.stdout.splitlines()];assert [r['answers'] for r in rows]==[1,8,32,64,128]
  assert all(sum(r['stages'])==r['calls'] for r in rows);outcomes.append(rows)
 assert outcomes[0]==outcomes[1]
 print(mode,json.dumps(outcomes[0][-1]))
