"""Bounded positive-guard validation for local-graph search."""
from pathlib import Path
import hashlib,json,re,resource,subprocess
ROOT=Path(__file__).resolve().parents[3]
RAW=ROOT/'docs/experiments/results/s02-local-guard-entry'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def binaries(mode):
    out={}
    for line in (RAW/f'build-{mode}.jsonl').read_text().splitlines():
        r=json.loads(line)
        if r.get('reason')=='compiler-artifact' and r.get('executable') and r.get('profile',{}).get('test'):out[r['target']['name']]=r['executable']
    return out

def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def run(label,binary):
    cmd=[binary,'--nocapture'];p=subprocess.run(cmd,capture_output=True,text=True,preexec_fn=limits,timeout=60)
    with (RAW/f'{label}.json').open('x') as f:json.dump(dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr),f,indent=2)
    assert p.returncode==0,(label,p.stdout,p.stderr)
    print(label,re.findall(r'test result:.*',p.stdout),flush=True)
if __name__=='__main__':
    builds={mode:binaries(mode) for mode in ['on','off']}
    files=[Path(__file__),ROOT/'Cargo.lock',ROOT/'docs/experiments/registrations/S02-local-guard-entry.md',ROOT/'research/chr-relational/Cargo.toml',ROOT/'research/chr-relational/tests/local_guards.rs',ROOT/'research/chr-relational/tests/support/local_ports.rs',ROOT/'research/chr-relational/tests/support/local_multihead.rs',ROOT/'research/chr-relational/tests/support/local_search.rs',ROOT/'research/chr-direct-conditional/tests/runtime_support/mod.rs']
    files+=list((ROOT/'research/chr-relational/src').glob('*.rs'))
    for name in ['local_choices','multihead','contextual','contextual_source']:files.append(ROOT/f'research/chr-relational/tests/{name}.rs')
    freeze={'inputs':{str(p.relative_to(ROOT)):digest(p) for p in files},'binaries':{f'{mode}-{name}':dict(path=p,sha256=digest(Path(p))) for mode,bs in builds.items() for name,p in bs.items()}}
    with (RAW/'freeze.json').open('x') as f:json.dump(freeze,f,indent=2)
    for mode,bs in builds.items():
        for rep in range(2):run(f'{mode}-{rep}',bs['local_guards'])
    for name,binary in builds['on'].items():
        if name!='local_guards':run(f'regression-{name}',binary)
