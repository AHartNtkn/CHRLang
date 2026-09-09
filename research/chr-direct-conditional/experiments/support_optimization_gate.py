#!/usr/bin/env python3
import hashlib,json,resource,subprocess
from pathlib import Path
root=Path('docs/experiments/results/s08-support-optimization-gate')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(120,120))
def main():
    root.mkdir(exist_ok=False)
    files=[Path(__file__),Path('Cargo.lock'),Path('docs/experiments/registrations/S08-support-optimization-gate.md')]
    for d in ['research/chr-direct-conditional','research/chr-reuse','crates/chr-syntax']:
        files.extend(Path(d).rglob('*.rs'));files.append(Path(d)/'Cargo.toml')
    freeze={str(p):sha(p) for p in files};summary=[]
    for policy,extra in [('ordinary',''),('identities',',support-identities'),('cache',',support-result-cache'),('combined',',support-identities,support-result-cache')]:
        features='experiment,support-trace,serial-body-accounting,head-dispatch,equality-invalidation'+extra
        run=subprocess.run(['cargo','build','--release','-p','chr-direct-conditional','--no-default-features','--features',features,'--example','support_repetition'],capture_output=True,text=True)
        (root/f'{policy}-build.log').write_text(run.stdout+run.stderr);assert run.returncode==0
        binary=Path(f'target/s08-optimization/{policy}').resolve();binary.parent.mkdir(exist_ok=True)
        binary.write_bytes(Path('target/release/examples/support_repetition').read_bytes());binary.chmod(0o755);freeze[str(binary)]=sha(binary)
        (root/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
        for n in [0,64]:
            cmd=[str(binary),'inferred','aliases',str(n)]
            r=subprocess.run(cmd,capture_output=True,text=True,timeout=150,preexec_fn=limits)
            (root/f'{policy}-{n}.json').write_text(json.dumps(dict(command=cmd,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr),indent=2)+'\n');assert r.returncode==0
            d=json.loads(r.stdout);assert (d['mode'],d['family'],d['depth'])==('inferred','aliases',n)
            assert all(row[5]!=2**64-1 for row in d['trace'])
            if policy=='ordinary':
                old=json.loads(Path('docs/experiments/results/s08-support-repetition/summary.json').read_text())
                expected=next(x for x in old if (x['mode'],x['family'],x['depth'])==('inferred','aliases',n));assert d['ticks']==expected['ticks']
            summary.append(dict(policy=policy,depth=n,ticks=d['ticks'],jobs=len(d['trace']),frames=sum(row[4] for row in d['trace']),direct_results=sum(row[4]==0 for row in d['trace'])))
    assert all(sha(Path(p))==h for p,h in freeze.items())
    (root/'toolchain.txt').write_text(subprocess.check_output(['rustc','-Vv'],text=True))
    (root/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
if __name__=='__main__':main()
