"""Bounded finite joint-projection confirmation and existing structural regressions."""
from pathlib import Path
import hashlib,json,re,resource,subprocess
ROOT=Path(__file__).resolve().parents[3]
RAW=ROOT/'docs/experiments/results/s06-joint-projection'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def binaries(mode):
    out={}
    for line in (RAW/f'build-{mode}.jsonl').read_text().splitlines():
        r=json.loads(line)
        if r.get('reason')=='compiler-artifact' and r.get('executable') and r.get('profile',{}).get('test'):out[r['target']['name']]=r['executable']
    return out

def limits():
    resource.setrlimit(resource.RLIMIT_CPU,(60,60));resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
def run(path,label):
    p=subprocess.run([path,'--nocapture'],capture_output=True,text=True,timeout=60,preexec_fn=limits)
    with (RAW/f'{label}.json').open('x') as f:json.dump(dict(command=[path,'--nocapture'],returncode=p.returncode,stdout=p.stdout,stderr=p.stderr),f,indent=2)
    assert p.returncode==0,(label,p.stdout,p.stderr)
    count=int(re.search(r'test result: ok\. (\d+) passed',p.stdout)[1]);print(label,count,flush=True)
    return count,p.stdout
if __name__=='__main__':
    builds={m:binaries(m) for m in ['default','off']}
    files=[Path(__file__),ROOT/'Cargo.lock',ROOT/'docs/experiments/registrations/S06-joint-projection.md',ROOT/'research/chr-structural/Cargo.toml']
    for directory in ['research/chr-structural/src','research/chr-structural/tests','crates/chr-reference/src','crates/chr-programs/src','crates/chr-syntax/src','research/chr-observe/src']:
        files.extend((ROOT/directory).rglob('*.rs'))
    freeze=dict(inputs={str(p.relative_to(ROOT)):sha(p) for p in files},binaries={f'{mode}-{name}':dict(path=p,sha256=sha(Path(p))) for mode,bs in builds.items() for name,p in bs.items()})
    with (RAW/'freeze.json').open('x') as f:json.dump(freeze,f,indent=2)
    for mode,bs in builds.items():
        for rep in range(2):
            n,out=run(bs['joint_projection'],f'{mode}-{rep}');assert n==3 and 'joint_projection_sets=1536' in out
    regression={}
    for name,path in builds['default'].items():
        if name!='joint_projection':regression[name]=run(path,f'regression-{name}')[0]
    for p,h in freeze['inputs'].items():assert sha(ROOT/p)==h,p
    audit=dict(confirming_executions=4,tests_per_execution=3,exact_set_comparisons_per_execution=1536,regressions=regression,regression_tests=sum(regression.values()),frozen_inputs=len(files))
    (RAW/'audit.json').write_text(json.dumps(audit,indent=2)+'\n');print(audit)
