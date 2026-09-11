"""Registered complete traversal timing, heap and residency sizing."""
import gzip
import itertools
import json
import random
import shutil
import subprocess
import sys
import zipfile
from pathlib import Path
sys.dont_write_bytecode = True
from continuing_lifecycle_entry import ROOT, invoke, sha

OUT=ROOT/'docs/experiments/results/s08-context-lifecycle'


def args(case, resident=False):
    values=[case['mode'],case['resource'],case['demand'],case['keep']]
    if case['source']=='continuing': values += [4,False,resident]
    else: values += [resident]
    return [str(x).lower() for x in values]


def main():
    assert not (OUT/'freeze.json').exists()
    binaries={};builds=[]
    for enabled,kind in itertools.product(['lookup','ordered','seeking'],['time','meter']):
        features=['chr-direct-choice/completed-traversal']
        if enabled=='ordered':features+=['chr-direct-choice/ordered-context']
        if enabled=='seeking':features+=['chr-direct-choice/seek-context']
        if kind=='meter':features+=['alloc-meter']
        cmd=['cargo','build','-p','chr-reuse','--release','--no-default-features','--example','continuing_lifecycle','--example','flat_lifecycle','--target-dir','target/s08-context-owner-build']
        if features:cmd+=['--features',','.join(features)]
        with (OUT/f'build-{enabled}-{kind}.log').open('w') as log:
            subprocess.run(cmd,cwd=ROOT,stdout=log,stderr=subprocess.STDOUT,check=True)
        binaries[str(enabled),kind]={}
        for source,example in [('continuing','continuing_lifecycle'),('flat','flat_lifecycle')]:
            dest=ROOT/f'target/s08-context-lifecycle-binaries/{enabled}-{kind}-{source}';dest.parent.mkdir(exist_ok=True)
            shutil.copy2(ROOT/f'target/s08-context-owner-build/release/examples/{example}',dest)
            binaries[str(enabled),kind][source]=dict(path=str(dest),sha256=sha(dest))
        builds.append(cmd);print('built',enabled,kind,flush=True)
    cases=[]
    for source,demands,modes in [('continuing',[32,128],['dependencies','templates','dependencies-reclaim','templates-reclaim']),('flat',[1,32],['dependencies','templates'])]:
        cases += [dict(source=source,mode=m,enabled=e,resource=r,demand=n,keep=k) for m,e,r,n,k in itertools.product(modes,['lookup','ordered','seeking'],[False,True],demands,['0','all'])]
        cases += [dict(source=source,mode='direct',enabled='lookup',resource=r,demand=n,keep=k) for r,n,k in itertools.product([False,True],demands,['0','all'])]
    assert len(cases)==160
    entries=[c for c in cases if not c['resource'] and c['keep']=='0' and c['mode'] in ['dependencies','templates'] and c['demand']==(32 if c['source']=='continuing' else 1)]
    assert len(entries)==12
    entry_runs=[]
    for kind,case in itertools.product(['time','meter'],entries):
        binary=binaries[str(case['enabled']),'time' if kind=='rss' else kind][case['source']]['path']
        raw=invoke([binary]+args(case,kind=='rss'),60)
        entry_runs.append(dict(case=case,kind=kind,raw=raw))
        (OUT/'entry.json').write_text(json.dumps(entry_runs,indent=2)+'\n')
        assert raw['exit_code']==0,(case,raw)
        data=json.loads(raw['stdout']);assert data['validated']
        if kind=='rss':assert all(s['rss_kib']>0 for s in data['snapshots'])
    print('24 ordinary/heap entries passed',flush=True)
    rng=random.Random(7403);jobs=[]
    for kind,reps in [('time',5),('meter',2)]:
        for rep in range(reps):
            order=cases.copy();rng.shuffle(order)
            jobs += [dict(case=c,kind=kind,rep=rep) for c in order]
    paths=[]
    for folder in ['research/chr-reuse','research/chr-direct-choice','research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax']:
        paths+=list((ROOT/folder).rglob('*.rs'))+[ROOT/folder/'Cargo.toml']
    paths += [ROOT/'Cargo.toml',ROOT/'Cargo.lock',Path(__file__),ROOT/'research/chr-reuse/experiments/continuing_lifecycle_entry.py',ROOT/'docs/experiments/registrations/S08-context-lifecycle.md']
    hashes={str(p.relative_to(ROOT)):sha(p) for p in paths}
    with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    binary_entries=[dict(enabled=e,kind=k,sources=v) for (e,k),v in binaries.items()]
    (OUT/'freeze.json').write_text(json.dumps(dict(binaries=binary_entries,builds=builds,sources=hashes,archive_sha256=sha(OUT/'sources.zip'),jobs=jobs,seed=7403,cpu=0,toolchain=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2)+'\n')
    warmups=[]
    for case in entries:
        binary=binaries[str(case['enabled']),'time'][case['source']]['path']
        raw=invoke([binary]+args(case),60);warmups.append(dict(case=case,raw=raw));assert raw['exit_code']==0
    (OUT/'warmups.json').write_text(json.dumps(warmups,indent=2)+'\n')
    with gzip.open(OUT/'runs.jsonl.gz','wt') as f:
        for i,j in enumerate(jobs):
            c=j['case'];kind='time' if j['kind']=='rss' else j['kind']
            binary=binaries[str(c['enabled']),kind][c['source']]['path']
            raw=invoke([binary]+args(c,j['kind']=='rss'),60)
            f.write(json.dumps(dict(job=j,raw=raw))+'\n');f.flush()
            assert raw['exit_code']==0,(j,raw['stderr'])
            if (i+1)%160==0:print(j['kind'],'block',j['rep'],'complete',flush=True)
    print('1,120 complete context lifecycle/heap processes',flush=True)


if __name__=='__main__':main()
