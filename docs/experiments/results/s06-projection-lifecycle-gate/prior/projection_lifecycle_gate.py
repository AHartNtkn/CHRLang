"""Freeze and execute the registered projection correctness/owner qualification."""
from pathlib import Path
import hashlib,json,resource,subprocess
ROOT=Path(__file__).resolve().parents[3]
RAW=ROOT/'docs/experiments/results/s06-projection-lifecycle-gate'
def digest(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def bounds():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
    resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def run(command,path):
    assert not path.exists(),path
    try:
        p=subprocess.run(command,capture_output=True,text=True,timeout=60,preexec_fn=bounds)
        result=dict(command=command,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
    except subprocess.TimeoutExpired as e:
        result=dict(command=command,timeout=True,stdout=(e.stdout or b'').decode(),stderr=(e.stderr or b'').decode())
    path.write_text(json.dumps(result,indent=2)+'\n')
    assert result.get('returncode')==0 and not result['stderr'],path
if __name__=='__main__':
    binaries=[]
    for build in ['default','counter-free']:
        records=[json.loads(s) for s in (RAW/f'{build}-build.jsonl').read_text().splitlines()]
        found=[r for r in records if r.get('reason')=='compiler-artifact' and r.get('executable') and r['target']['name'] in ['projection_lifecycle','projection','finite_paths']]
        assert len(found)==3
        binaries.extend((build,r['target']['name'],Path(r['executable'])) for r in found)
    ownership=ROOT/'target/s06-projection-lifecycle/debug/examples/projection_ownership'
    paths=[Path(__file__),ROOT/'research/chr-structural/experiments/audit_projection_lifecycle.py',ROOT/'research/chr-structural/src/projection.rs',ROOT/'research/chr-structural/src/finite.rs',ROOT/'research/chr-structural/src/lib.rs',ROOT/'research/chr-structural/tests/projection_lifecycle.rs',ROOT/'research/chr-structural/tests/projection.rs',ROOT/'research/chr-structural/tests/finite_paths.rs',ROOT/'research/chr-structural/examples/projection_ownership.rs',ROOT/'research/chr-compiled/experiments/meter.rs',ROOT/'docs/experiments/registrations/S06-projection-lifecycle-gate.md',ROOT/'Cargo.lock',ROOT/'research/chr-structural/Cargo.toml',ownership]+[p for _,_,p in binaries]
    manifest=RAW/'manifest.json'
    assert not manifest.exists()
    manifest.write_text(json.dumps(dict(binaries=[(b,t,str(p.relative_to(ROOT))) for b,t,p in binaries],ownership=str(ownership.relative_to(ROOT)),sha256={str(p.relative_to(ROOT)):digest(p) for p in paths}),indent=2)+'\n')
    for build,test,binary in binaries:
        run([str(binary),'--test-threads=1','--nocapture'],RAW/f'{build}-{test}.json')
        print(build,test,'passed',flush=True)
    for family in ['independent','star','dense']:
        for retain in ['0','4','all']:
            for cancel in ['0','1']:
                for repeat in range(2):
                    run([str(ownership),family,retain,cancel],RAW/f'owner-{family}-{retain}-{cancel}-{repeat}.json')
        print(family,'ownership passed',flush=True)
