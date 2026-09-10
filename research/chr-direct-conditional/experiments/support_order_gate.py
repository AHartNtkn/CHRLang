#!/usr/bin/env python3
import hashlib,json,resource,subprocess
from pathlib import Path
root=Path('docs/experiments/results/s08-support-order-gate')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(120,120))
def main():
    root.mkdir(exist_ok=False)
    files=[Path(__file__),Path('Cargo.lock'),Path('docs/experiments/registrations/S08-support-order-gate.md')]
    for d in ['research/chr-direct-conditional','research/chr-reuse','crates/chr-syntax']:
        files.extend(Path(d).rglob('*.rs'));files.append(Path(d)/'Cargo.toml')
    freeze={str(p):sha(p) for p in files};rows=[]
    for order,feature in [('ascending',''),('ascending-general',',support-generic-histories'),('descending-general',',support-reverse-order')]:
        for policy,extra in [('ordinary',''),('combined',',support-identities,support-result-cache')]:
            features='experiment,support-trace,head-dispatch,serial-body-accounting,equality-invalidation'+feature+extra
            run=subprocess.run(['cargo','build','--release','-p','chr-direct-conditional','--no-default-features','--features',features,'--example','support_order'],capture_output=True,text=True)
            (root/f'{order}-{policy}-build.log').write_text(run.stdout+run.stderr);assert run.returncode==0
            binary=Path(f'target/s08-order/{order}-{policy}').resolve();binary.parent.mkdir(exist_ok=True);binary.write_bytes(Path('target/release/examples/support_order').read_bytes());binary.chmod(0o755);freeze[str(binary)]=sha(binary)
            (root/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
            for family in ['aliases','distinct']:
                for n in [0,16,64]:
                    cmd=[str(binary),'inferred',family,str(n)]
                    run=subprocess.run(cmd,capture_output=True,text=True,timeout=150,preexec_fn=limits)
                    (root/f'{order}-{policy}-{family}-{n}.json').write_text(json.dumps(dict(command=cmd,returncode=run.returncode,stdout=run.stdout,stderr=run.stderr),indent=2)+'\n');assert run.returncode==0,(order,policy,family,n)
                    d=json.loads(run.stdout);assert d['depth']==n and d['family']==family
                    row=dict(order=order,policy=policy,family=family,depth=n,ticks=d['ticks'],nodes=d['nodes'],jobs=len(d['trace']),frames=sum(t[4] for t in d['trace']))
                    rows.append(row);print(row,flush=True)
    assert all(sha(Path(p))==h for p,h in freeze.items())
    (root/'summary.json').write_text(json.dumps(rows,indent=2)+'\n');(root/'toolchain.txt').write_text(subprocess.check_output(['rustc','-Vv'],text=True))
    (root/'completion.json').write_text(json.dumps(dict(processes=36,source_answers='pass',timing='not_run',allocation='not_measured'))+'\n')
if __name__=='__main__':main()
