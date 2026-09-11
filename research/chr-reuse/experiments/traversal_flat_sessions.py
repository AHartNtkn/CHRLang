"""Registered complete-session precision for single-alternative graph/direct costs."""
import gzip
import itertools
import json
import random
import shutil
import subprocess
import sys
import zipfile
from pathlib import Path
sys.dont_write_bytecode=True
from continuing_lifecycle_entry import ROOT, invoke, sha
OUT=ROOT/'docs/experiments/results/s08-traversal-flat-sessions'


def main():
    OUT.mkdir(exist_ok=True);assert not (OUT/'freeze.json').exists()
    builds=[];binaries={}
    for enabled,kind in itertools.product([False,True],['time','meter']):
        cmd=['cargo','build','-p','chr-reuse','--release','--no-default-features','--example','flat_lifecycle','--target-dir','target/s08-traversal-owner-build']
        features=(['chr-direct-choice/completed-traversal'] if enabled else [])+(['alloc-meter'] if kind=='meter' else [])
        if features:cmd+=['--features',','.join(features)]
        with (OUT/f'build-{enabled}-{kind}.log').open('w') as log:subprocess.run(cmd,cwd=ROOT,stdout=log,stderr=subprocess.STDOUT,check=True)
        dest=ROOT/f'target/s08-traversal-flat-binaries/{enabled}-{kind}';dest.parent.mkdir(exist_ok=True)
        shutil.copy2(ROOT/'target/s08-traversal-owner-build/release/examples/flat_lifecycle',dest)
        binaries[str(enabled),kind]=dict(path=str(dest),sha256=sha(dest));builds.append(cmd)
    cases=[dict(mode=m,enabled=e,resource=r,keep=k) for m,e,r,k in itertools.product(['dependencies','templates'],[False,True],[False,True],['0','all'])]+[dict(mode='direct',enabled=False,resource=r,keep=k) for r,k in itertools.product([False,True],['0','all'])]
    assert len(cases)==20
    rng=random.Random(7402);jobs=[]
    for kind,reps in [('time',9),('meter',2)]:
        for rep in range(reps):
            order=cases.copy();rng.shuffle(order);jobs += [dict(case=c,kind=kind,rep=rep) for c in order]
    previous=json.loads((ROOT/'docs/experiments/results/s08-traversal-lifecycle/freeze.json').read_text())
    paths=list(previous['sources'])+[str(Path(__file__).relative_to(ROOT)),'docs/experiments/registrations/S08-traversal-flat-sessions.md']
    hashes={p:sha(ROOT/p) for p in paths}
    with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    (OUT/'freeze.json').write_text(json.dumps(dict(builds=builds,binaries=[dict(enabled=e,kind=k,**b) for (e,k),b in binaries.items()],sources=hashes,archive_sha256=sha(OUT/'sources.zip'),jobs=jobs,seed=7402,cpu=0),indent=2)+'\n')
    with gzip.open(OUT/'runs.jsonl.gz','wt') as out:
        for i,j in enumerate(jobs):
            c=j['case'];b=binaries[str(c['enabled']),j['kind']]['path']
            command=[b,c['mode'],str(c['resource']).lower(),'1',c['keep'],'false','50']
            raw=invoke(command,60);out.write(json.dumps(dict(job=j,raw=raw))+'\n');out.flush()
            assert raw['exit_code']==0,(j,raw['stderr'])
            rows=[json.loads(s) for s in raw['stdout'].splitlines()];assert len(rows)==50 and all(d['validated'] for d in rows)
            if (i+1)%20==0:print(j['kind'],j['rep'],'complete',flush=True)


if __name__=='__main__':main()
