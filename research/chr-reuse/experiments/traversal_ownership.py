"""Registered continuing/flat ownership gate for completed traversal reuse."""
import gzip
import itertools
import json
import shutil
import subprocess
import zipfile
from pathlib import Path
from continuing_lifecycle_entry import ROOT, invoke, sha
OUT=ROOT/'docs/experiments/results/s08-traversal-ownership'

def main():
    assert not (OUT/'freeze.json').exists()
    binaries={};builds=[]
    for enabled,kind in itertools.product([False,True],['time','meter']):
        features=[]
        if enabled:features+=['chr-direct-choice/completed-traversal']
        if kind=='meter':features+=['alloc-meter']
        cmd=['cargo','build','-p','chr-reuse','--release','--no-default-features','--example','continuing_lifecycle','--example','flat_lifecycle','--target-dir','target/s08-traversal-owner-build']
        if features:cmd+=['--features',','.join(features)]
        with (OUT/f'build-{enabled}-{kind}.log').open('w') as log:subprocess.run(cmd,cwd=ROOT,stdout=log,stderr=subprocess.STDOUT,check=True)
        binaries[str(enabled),kind]={}
        for source,example in [('continuing','continuing_lifecycle'),('flat','flat_lifecycle')]:
            dest=ROOT/f'target/s08-traversal-owner-binaries/{enabled}-{kind}-{source}';dest.parent.mkdir(exist_ok=True)
            shutil.copy2(ROOT/f'target/s08-traversal-owner-build/release/examples/{example}',dest)
            binaries[str(enabled),kind][source]=dict(path=str(dest),sha256=sha(dest))
        builds.append(cmd);print('built',enabled,kind,flush=True)
    cases=[]
    for source,demands,modes in [('continuing',[32,128],['dependencies','templates','dependencies-reclaim','templates-reclaim']),('flat',[1,32],['dependencies','templates'])]:
        cases += [dict(source=source,mode=m,enabled=e,resource=r,demand=n,keep=k) for m,e,r,n,k in itertools.product(modes,[False,True],[False,True],demands,['0','all'])]
        cases += [dict(source=source,mode='direct',enabled=False,resource=r,demand=n,keep=k) for r,n,k in itertools.product([False,True],demands,['0','all'])]
    jobs=[dict(case=c,kind=k,rep=rep) for k,reps in [('time',1),('meter',2)] for rep in range(reps) for c in cases]
    assert len(jobs)==336
    paths=[]
    for folder in ['research/chr-reuse','research/chr-direct-choice','research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax']:
        paths+=list((ROOT/folder).rglob('*.rs'))+[ROOT/folder/'Cargo.toml']
    paths += [ROOT/'Cargo.toml',ROOT/'Cargo.lock',Path(__file__),ROOT/'docs/experiments/registrations/S08-traversal-ownership.md']
    hashes={str(p.relative_to(ROOT)):sha(p) for p in paths}
    with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    entries=[dict(enabled=e,kind=k,sources=v) for (e,k),v in binaries.items()]
    (OUT/'freeze.json').write_text(json.dumps(dict(binaries=entries,builds=builds,sources=hashes,archive_sha256=sha(OUT/'sources.zip'),jobs=jobs),indent=2)+'\n')
    with gzip.open(OUT/'runs.jsonl.gz','wt') as f:
        for i,j in enumerate(jobs):
            c=j['case'];binary=binaries[str(c['enabled']),j['kind']][c['source']]['path']
            args=[c['mode'],c['resource'],c['demand'],c['keep']]
            if c['source']=='continuing':args += [4,False,False]
            raw=invoke([binary]+[str(x).lower() for x in args],60)
            f.write(json.dumps(dict(job=j,raw=raw))+'\n');f.flush()
            print(i,c,j['kind'],j['rep'],raw['exit_code'],flush=True)
            assert raw['exit_code']==0,(j,raw['stderr'])

if __name__=='__main__':main()
