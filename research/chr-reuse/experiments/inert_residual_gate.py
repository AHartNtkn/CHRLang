"""Bounded independent source qualification and existing-engine regression tests."""
from pathlib import Path
import hashlib,json,resource,subprocess
ROOT=Path(__file__).resolve().parents[3]
RAW=ROOT/'docs/experiments/results/s05-inert-residual-gate/final'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def save(path,value):
    with path.open('x') as f:f.write(value)
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
if __name__=='__main__':
    RAW.mkdir(exist_ok=False);records=[]
    for kind,features in [('metrics',[]),('primary',['--no-default-features'])]:
        target=f'target/s05-inert-{kind}'
        for package,selection in [('chr-reuse',['--test','inert_residuals','--test','effectful_calls','--test','effectful_boundary','--test','generalized_continuations']),('chr-persistent',['--lib','--tests'])]:
            cmd=['cargo','test','--offline','-p',package,*selection,'--no-run','--message-format=json','--target-dir',target,*features]
            build=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True)
            save(RAW/f'{kind}-{package}-build.jsonl',build.stdout);save(RAW/f'{kind}-{package}-build.log',build.stderr)
            assert build.returncode==0,build.stderr
            artifacts=[json.loads(l) for l in build.stdout.splitlines() if l.startswith('{')]
            artifacts=[a for a in artifacts if a.get('reason')=='compiler-artifact' and a.get('executable') and a.get('profile',{}).get('test')]
            for a in artifacts:
                binary=a['executable'];name=a['target']['name'];cmd=[binary,'--nocapture','--test-threads=1']
                run=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,preexec_fn=limits,timeout=60)
                log=f'{kind}-{package}-{name}.log';save(RAW/log,run.stdout+run.stderr)
                assert run.returncode==0,run.stdout+run.stderr
                records.append(dict(kind=kind,package=package,target=name,command=cmd,returncode=run.returncode,binary_sha256=digest(Path(binary)),log=log))
            cmd=['cargo','clippy','--offline','-p',package,*selection,'--target-dir',target,*features,'--','-D','warnings']
            lint=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True);save(RAW/f'{kind}-{package}-clippy.log',lint.stdout+lint.stderr);assert lint.returncode==0,lint.stderr
    sources=[Path(__file__),ROOT/'research/chr-reuse/tests/inert_residuals.rs',ROOT/'research/chr-reuse/src/residuals.rs',ROOT/'research/chr-reuse/src/continuations.rs',ROOT/'research/chr-reuse/src/lib.rs',ROOT/'research/chr-direct-conditional/tests/runtime_support/mod.rs',ROOT/'research/chr-reuse/Cargo.toml',ROOT/'Cargo.lock',ROOT/'docs/experiments/registrations/S05-inert-residual-gate.md']
    sources+=list((ROOT/'research/chr-persistent/src').rglob('*.rs'))+list((ROOT/'research/chr-observe/src').rglob('*.rs'))+list((ROOT/'crates/chr-syntax/src').rglob('*.rs'))
    sources+=list((ROOT/'research/chr-persistent/tests').glob('*.rs'))+[ROOT/f'research/chr-reuse/tests/{n}.rs' for n in ['effectful_calls','effectful_boundary','generalized_continuations']]+[ROOT/'research/chr-reuse/tests/support/effectful_probe.rs']
    prior={name:dict(snapshot=str((RAW.parent/'before'/snapshot).relative_to(ROOT)),sha256=digest(RAW.parent/'before'/snapshot)) for name,snapshot in [('research/chr-persistent/src/state.rs','state.rs'),('research/chr-persistent/src/continuations.rs','continuations.rs'),('research/chr-reuse/src/lib.rs','reuse-lib.rs')]}
    save(RAW/'manifest.json',json.dumps(dict(status='passed',records=records,sha256={str(p.relative_to(ROOT)):digest(p) for p in sources},prior_sources=prior,receipts={p.name:digest(p) for p in RAW.glob('*.log')}),indent=2)+'\n')
    print(f'Verified {len(records)} bounded test executables and four strict Clippy checks; sources and receipts frozen.')
