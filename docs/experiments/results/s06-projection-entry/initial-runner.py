"""Freeze and run projection/finite-path test binaries under registered bounds."""
from pathlib import Path
import hashlib,json,resource,subprocess
ROOT=Path(__file__).resolve().parents[3]
RAW=ROOT/'docs/experiments/results/s06-projection-entry'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def bounds():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
if __name__=='__main__':
    binaries=[]
    for build in ['default','counter-free']:
        records=[json.loads(l) for l in (RAW/f'{build}-build.jsonl').read_text().splitlines()]
        found=[r for r in records if r.get('reason')=='compiler-artifact' and r.get('executable') and r['target']['name'] in ['projection','finite_paths']]
        assert len(found)==2
        binaries.extend((build,r['target']['name'],Path(r['executable'])) for r in found)
    paths=[Path(__file__),ROOT/'research/chr-structural/src/lib.rs',ROOT/'research/chr-structural/src/projection.rs',ROOT/'research/chr-structural/src/finite.rs',ROOT/'research/chr-structural/tests/projection.rs',ROOT/'research/chr-structural/tests/finite_paths.rs',ROOT/'docs/experiments/registrations/S06-projection-entry.md',ROOT/'docs/experiments/registrations/S06-projection-order-gate.md',ROOT/'Cargo.lock']+[p for _,_,p in binaries]
    manifest=RAW/'manifest.json';assert not manifest.exists()
    manifest.write_text(json.dumps(dict(binaries=[(b,t,str(p.relative_to(ROOT))) for b,t,p in binaries],sha256={str(p.relative_to(ROOT)):digest(p) for p in paths}),indent=2)+'\n')
    for build,test,binary in binaries:
        command=[str(binary),'--test-threads=1','--nocapture']
        try:
            p=subprocess.run(command,timeout=60,preexec_fn=bounds,capture_output=True,text=True)
            r=dict(command=command,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
        except subprocess.TimeoutExpired as e:
            r=dict(command=command,timeout=True,stdout=(e.stdout or b'').decode(),stderr=(e.stderr or b'').decode())
        (RAW/f'{build}-{test}.json').write_text(json.dumps(r,indent=2)+'\n');print(build,test,r.get('returncode'),flush=True)
        assert r.get('returncode')==0 and not r['stderr'],r
