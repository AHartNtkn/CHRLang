"""Registered work comparison of tick-local completed traversal sharing."""
import itertools
import json
import shutil
import subprocess
import zipfile
from pathlib import Path
from continuing_lifecycle_entry import ROOT, invoke, sha

OUT=ROOT/'docs/experiments/results/s08-completed-traversal'

def main():
    assert not (OUT/'freeze.json').exists()
    binaries={};builds=[]
    for enabled in [False,True]:
        features=['continuing-graph-diagnostics']
        if enabled:features+=['chr-direct-choice/completed-traversal']
        cmd=['cargo','build','-p','chr-reuse','--release','--no-default-features','--features',','.join(features),'--example','continuing_graph_attribution','--target-dir','target/s08-traversal-build']
        with (OUT/f'build-{enabled}.log').open('w') as log:
            subprocess.run(cmd,cwd=ROOT,stdout=log,stderr=subprocess.STDOUT,check=True)
        dest=ROOT/f'target/s08-traversal-binaries/graph-{enabled}'
        dest.parent.mkdir(exist_ok=True)
        shutil.copy2(ROOT/'target/s08-traversal-build/release/examples/continuing_graph_attribution',dest)
        binaries[str(enabled)]=dict(path=str(dest),sha256=sha(dest));builds.append(cmd)
    jobs=[dict(mode=m,resource=r,marker=q,demand=128,enabled=e,rep=i) for m,r,q,e,i in itertools.product(['dependencies','templates'],[False,True],[False,True],[False,True],range(2))]
    jobs += [dict(mode=m,resource=r,marker=True,demand=512,enabled=e,rep=0) for m,r,e in itertools.product(['dependencies','templates'],[False,True],[False,True])]
    assert len(jobs)==40
    paths=[]
    for folder in ['research/chr-reuse','research/chr-direct-choice','research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax']:
        paths+=list((ROOT/folder).rglob('*.rs'))+[ROOT/folder/'Cargo.toml']
    paths += [ROOT/'Cargo.toml',ROOT/'Cargo.lock',Path(__file__),ROOT/'docs/experiments/registrations/S08-completed-traversal.md']
    hashes={str(p.relative_to(ROOT)):sha(p) for p in paths}
    with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    (OUT/'freeze.json').write_text(json.dumps(dict(binaries=binaries,builds=builds,sources=hashes,archive_sha256=sha(OUT/'sources.zip'),jobs=jobs),indent=2)+'\n')
    for i,j in enumerate(jobs):
        raw=invoke([binaries[str(j['enabled'])]['path']]+[str(j[k]).lower() for k in ['mode','resource','marker','demand']],120 if j['demand']==512 else 60)
        (OUT/f'run-{i}.json').write_text(json.dumps(dict(job=j,raw=raw),indent=2)+'\n')
        print(i,j,raw['exit_code'],flush=True)

if __name__=='__main__':main()
