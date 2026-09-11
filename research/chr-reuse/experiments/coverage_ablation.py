"""Build, freeze and execute the registered coverage-responsibility ablation."""
import gzip
import hashlib
import itertools
import json
import random
import shutil
import subprocess
import time
import zipfile
from pathlib import Path
from continuing_lifecycle_entry import ROOT, invoke

OUT = ROOT/'docs/experiments/results/s08-coverage-ablation'
STRONG = ['serial-body-accounting','equality-invalidation','support-identities',
          'support-result-cache','selective-discovery','equality-overlap-shortcut',
          'observation-backpressure']
VARIANTS = dict(off=[], equality=['equality-binding-coverage'], both=['matching-binding-coverage'])

def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

def main():
    assert not (OUT/'freeze.json').exists()
    binaries = {}; builds = []
    for variant, extra in VARIANTS.items():
        binaries[variant] = {}
        for kind in ['time','meter','diagnostic']:
            example = 'continuing_resource_stages' if kind == 'diagnostic' else 'continuing_lifecycle'
            features = ['chr-direct-conditional/'+x for x in STRONG+extra]
            if kind == 'meter': features += ['alloc-meter']
            if kind == 'diagnostic': features += ['continuing-stage-diagnostics','continuing-equality-diagnostics']
            cmd = ['cargo','build','-p','chr-reuse','--release','--no-default-features','--features',','.join(features),'--example',example,'--target-dir','target/s08-ablation-build']
            with (OUT/f'build-{variant}-{kind}.log').open('w') as log:
                subprocess.run(cmd,cwd=ROOT,stdout=log,stderr=subprocess.STDOUT,check=True)
            dest = ROOT/f'target/s08-ablation-binaries/{variant}-{kind}'
            dest.parent.mkdir(exist_ok=True)
            shutil.copy2(ROOT/f'target/s08-ablation-build/release/examples/{example}',dest)
            binaries[variant][kind] = dict(path=str(dest),sha256=sha(dest))
            builds.append(cmd)
            print('built',variant,kind,flush=True)
    diagnostic = [dict(variant=v,resource=r,rep=rep) for v,r,rep in itertools.product(VARIANTS,[False,True],range(2))]
    cases = list(itertools.product(VARIANTS,[False,True],[32,128]))
    rng = random.Random(740917); jobs = []
    for stage,kind,reps in [('warmup','time',1),('primary','time',7),('allocation','meter',2)]:
        for rep in range(reps):
            order = cases.copy(); rng.shuffle(order)
            jobs += [dict(stage=stage,kind=kind,rep=rep,case=c) for c in order]
    assert len(jobs)==120 and len(diagnostic)==12
    paths=[]
    for folder in ['research/chr-reuse','research/chr-direct-choice','research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax']:
        paths += list((ROOT/folder).rglob('*.rs')) + [ROOT/folder/'Cargo.toml']
    paths += [ROOT/'Cargo.toml',ROOT/'Cargo.lock',Path(__file__),ROOT/'docs/experiments/registrations/S08-coverage-ablation.md']
    hashes={str(p.relative_to(ROOT)):sha(p) for p in paths}
    with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    (OUT/'freeze.json').write_text(json.dumps(dict(binaries=binaries,builds=builds,sources=hashes,archive_sha256=sha(OUT/'sources.zip'),diagnostic=diagnostic,jobs=jobs),indent=2)+'\n')
    with gzip.open(OUT/'diagnostic.jsonl.gz','wt') as f:
        for j in diagnostic:
            raw=invoke([binaries[j['variant']]['diagnostic']['path'],str(j['resource']).lower(),'128'],60)
            f.write(json.dumps(dict(job=j,raw=raw))+'\n');f.flush()
            assert raw['exit_code']==0,(j,raw['stderr'])
    print('diagnostics complete',flush=True)
    start=time.monotonic()
    with gzip.open(OUT/'runs.jsonl.gz','wt') as f:
        for i,j in enumerate(jobs):
            assert time.monotonic()-start<900
            v,r,n=j['case']
            raw=invoke([binaries[v][j['kind']]['path'],'conditional',str(r).lower(),str(n),'all','4','false','false'],60)
            f.write(json.dumps(dict(job=j,raw=raw))+'\n');f.flush()
            assert raw['exit_code']==0,(j,raw['stderr'])
            if (i+1)%12==0:print(i+1,round(time.monotonic()-start,2),flush=True)
    (OUT/'campaign.json').write_text(json.dumps(dict(processes=120,diagnostics=12,seconds=time.monotonic()-start),indent=2)+'\n')

if __name__=='__main__':
    main()
