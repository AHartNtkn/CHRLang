"""Read-only backlog attribution; preserve the known adverse cutoff."""
from pathlib import Path
import hashlib,json,resource,subprocess
ROOT=Path(__file__).resolve().parents[3];RAW=ROOT/'docs/experiments/results/s08-equality-overlap'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
 resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
if __name__=='__main__':
 for mode,prior in [('base','reverse'),('shortcut','shortcut-reverse')]:
  binary=ROOT/f'target/s08-backlog-{mode}/release/examples/equality_backlog'
  files=[Path(__file__),binary,ROOT/'research/chr-direct-conditional/examples/equality_backlog.rs',ROOT/'research/chr-direct-conditional/Cargo.toml',ROOT/'docs/experiments/registrations/S08-equality-backlog.md']+list((ROOT/'research/chr-direct-conditional/src').glob('*.rs'))
  with (RAW/f'backlog-{mode}-manifest.json').open('x') as f:json.dump({str(p.relative_to(ROOT)):digest(p) for p in files},f,indent=2)
  expected=json.loads((RAW/f'{prior}-0.json').read_text());outcomes=[]
  for rep in range(2):
   cmd=[str(binary)];p=subprocess.run(cmd,capture_output=True,text=True,preexec_fn=limits,timeout=60)
   r=dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
   with (RAW/f'backlog-{mode}-{rep}.json').open('x') as f:json.dump(r,f,indent=2)
   assert p.returncode==expected['returncode']
   rows=[l for l in p.stdout.splitlines() if l.startswith('{')];assert '\n'.join(rows)+'\n'==expected['stdout']
   if mode=='shortcut':assert 'cutoff after 116' in p.stderr
   else:assert not p.stderr
   outcomes.append(p.stdout)
  assert outcomes[0]==outcomes[1]
  print(mode,'\n'.join(l for l in outcomes[0].splitlines() if l.startswith('backlog')))
