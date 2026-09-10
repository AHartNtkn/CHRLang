"""Bounded eager/control regressions followed by registered allocation ownership."""
from pathlib import Path
import hashlib,itertools,json,random,resource,subprocess
ROOT=Path(__file__).resolve().parents[3];RAW=ROOT/'docs/experiments/results/s04-adaptive-ownership'
MODES=['copy','reunion','eager','scheduled','fixed1','fixed8','backoff1','backoff8']
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def bounds():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def run(command,path):
    assert not path.exists(),path
    try:
        p=subprocess.run(command,capture_output=True,text=True,timeout=60,preexec_fn=bounds)
        r=dict(command=command,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
    except subprocess.TimeoutExpired as e:r=dict(command=command,timeout=True,stdout=(e.stdout or b'').decode(),stderr=(e.stderr or b'').decode())
    path.write_text(json.dumps(r,indent=2)+'\n');assert r.get('returncode')==0 and not r['stderr'],path
if __name__=='__main__':
    binaries=[]
    for build in ['primary','diagnostic']:
        records=[json.loads(s) for s in (RAW/f'{build}-build.jsonl').read_text().splitlines()]
        found=[r for r in records if r.get('reason')=='compiler-artifact' and r.get('executable') and r['profile']['test'] and r['target']['src_path'].startswith(str(ROOT/'research/chr-restoration')+'/')]
        assert len(found)>=3
        binaries.extend((build,r['target']['name'],Path(r['executable'])) for r in found)
    owner=ROOT/'target/s04-adaptive-owner-meter/release/examples/adaptive_ownership'
    sources=[Path(__file__),ROOT/'research/chr-restoration/experiments/audit_adaptive_ownership.py',ROOT/'research/chr-restoration/src/reunion.rs',ROOT/'research/chr-restoration/src/lib.rs',ROOT/'research/chr-restoration/Cargo.toml',ROOT/'research/chr-restoration/examples/adaptive_ownership.rs',ROOT/'research/chr-restoration/examples/support/repeated_source.rs',ROOT/'research/chr-restoration/tests/eager_schedule.rs',ROOT/'research/chr-restoration/tests/adaptive_reunion.rs',ROOT/'research/chr-restoration/tests/reunion.rs',ROOT/'research/chr-compiled/experiments/meter.rs',ROOT/'research/chr-direct-conditional/tests/runtime_support/mod.rs',ROOT/'docs/experiments/registrations/S04-adaptive-ownership.md',ROOT/'Cargo.lock',owner]+[p for _,_,p in binaries]
    manifest=RAW/'manifest.json';assert not manifest.exists()
    manifest.write_text(json.dumps(dict(binaries=[(b,t,str(p.relative_to(ROOT))) for b,t,p in binaries],owner=str(owner.relative_to(ROOT)),sha256={str(p.relative_to(ROOT)):digest(p) for p in sources}),indent=2)+'\n')
    for build,test,binary in binaries:
        command=[str(binary),'--test-threads=1']+(['--nocapture'] if test=='eager_schedule' else [])
        run(command,RAW/f'{build}-{test}.json');print(build,test,'passed',flush=True)
    run([str(owner),'backoff8','late','4','all','0'],RAW/'owner-preflight.json')
    cells=list(itertools.product(MODES,['plain','history','late'],[0,4],['0','4','all'],[0,1]));assert len(cells)==288
    jobs=[(r,c) for r in range(2) for c in cells];random.Random(770901).shuffle(jobs)
    (RAW/'schedule.json').write_text(json.dumps(jobs)+'\n')
    for i,(repeat,cell) in enumerate(jobs):
        run([str(owner),*map(str,cell)],RAW/f'owner-{i:03}.json')
        if i%96==95:print(i+1,'/576 owner processes passed',flush=True)
