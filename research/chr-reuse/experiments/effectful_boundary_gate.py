"""Build and run the bounded effectful boundary gate; refuse receipt replacement."""
from pathlib import Path
import hashlib,json,resource,subprocess
ROOT=Path(__file__).resolve().parents[3]
RAW=ROOT/'docs/experiments/results/s05-effectful-boundary-gate/final'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
    resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def save(path,text):
    with path.open('x') as f:f.write(text)
if __name__=='__main__':
    RAW.mkdir(exist_ok=False)
    records=[]
    for kind,features in [('metrics',[]),('primary',['--no-default-features'])]:
        target=f'target/s05-effectful-boundary-{kind}'
        cmd=['cargo','test','--offline','-p','chr-reuse','--test','effectful_boundary','--no-run','--message-format=json','--target-dir',target,*features]
        build=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True)
        save(RAW/f'{kind}-build.jsonl',build.stdout);save(RAW/f'{kind}-build.log',build.stderr)
        assert build.returncode==0,build.stderr
        artifacts=[json.loads(l) for l in build.stdout.splitlines() if l.startswith('{')]
        paths=[a['executable'] for a in artifacts if a.get('reason')=='compiler-artifact' and a.get('executable') and a['target']['name']=='effectful_boundary']
        assert len(paths)==1
        cmd=[paths[0],'--nocapture','--test-threads=1']
        run=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,preexec_fn=limits,timeout=60)
        save(RAW/f'{kind}-run.log',run.stdout+run.stderr)
        assert run.returncode==0,run.stdout+run.stderr
        records.append(dict(kind=kind,command=cmd,returncode=run.returncode,binary_sha256=digest(Path(paths[0]))))
        cmd=['cargo','clippy','--offline','-p','chr-reuse','--test','effectful_boundary','--target-dir',target,*features,'--','-D','warnings']
        lint=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True)
        save(RAW/f'{kind}-clippy.log',lint.stdout+lint.stderr)
        assert lint.returncode==0,lint.stderr
    sources=[Path(__file__),ROOT/'research/chr-reuse/tests/effectful_boundary.rs',ROOT/'research/chr-reuse/tests/support/effectful_probe.rs',ROOT/'research/chr-direct-conditional/tests/runtime_support/mod.rs',ROOT/'research/chr-reuse/Cargo.toml',ROOT/'Cargo.lock',ROOT/'docs/experiments/registrations/S05-effectful-boundary-gate.md',ROOT/'research/chr-reuse/src/continuations.rs']
    sources+=list((ROOT/'research/chr-persistent/src').rglob('*.rs'))+list((ROOT/'research/chr-observe/src').rglob('*.rs'))+list((ROOT/'crates/chr-syntax/src').rglob('*.rs'))
    save(RAW/'manifest.json',json.dumps(dict(status='passed',records=records,sha256={str(p.relative_to(ROOT)):digest(p) for p in sources},receipts={p.name:digest(p) for p in RAW.glob('*.log')}),indent=2)+'\n')
    print('Both bounded source suites and strict Clippy passed; receipts and source hashes frozen.')
