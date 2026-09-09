#!/usr/bin/env python3
import hashlib
import itertools
import json
from pathlib import Path
import random
import resource
import subprocess
import sys

out=Path(sys.argv[1]); binroot=Path(sys.argv[2]).resolve()
out.mkdir(exist_ok=False)
controls=[('filtered','conditional'),('direct','conditional'),('prefix','conditional'),('filtered','scan'),('filtered','resumable')]
cells=[(build,mode,family,n,reuse,consumer) for (build,mode),family,n,reuse,consumer in itertools.product(controls,['incompatible','correlated','independent_suffix','dense_suffix','independent','delayed','early'],[0,1,3],[1,4],['query','prepared'])]
binaries=[binroot/f'{b}-{k}' for b in ['filtered','direct','prefix'] for k in ['meter','time']]
sources=[Path(__file__),Path('docs/experiments/registrations/S10-join-lifecycle.md'),Path('Cargo.lock')]
for root in ['research/chr-direct-conditional','research/chr-compiled','research/chr-relational','research/chr-persistent','research/chr-observe','crates/chr-syntax']:
    sources.extend(Path(root).rglob('*.rs'));sources.append(Path(root)/'Cargo.toml')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
freeze={str(p):sha(p) for p in sources+binaries}
(out/'freeze.json').write_text(json.dumps(freeze,indent=2))
(out/'toolchain.txt').write_text(subprocess.check_output(['rustc','--version','--verbose'],text=True))
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def group(kind,reps,seed):
    jobs=[(r,*c) for r in range(reps) for c in cells];random.Random(seed).shuffle(jobs)
    (out/f'{kind}-order.json').write_text(json.dumps(jobs));(out/kind).mkdir();prior={}
    for i,(r,build,mode,family,n,reuse,consumer) in enumerate(jobs):
        p=out/kind/f'{r}-{build}-{mode}-{family}-{n}-{reuse}-{consumer}.json'
        run=subprocess.run([str(binroot/f'{build}-{kind}'),mode,family,str(n),str(reuse),consumer],capture_output=True,text=True,timeout=75,preexec_fn=limits)
        if run.returncode:
            p.with_suffix('.error').write_text(run.stdout+run.stderr);raise RuntimeError(str(p))
        p.write_text(run.stdout);d=json.loads(run.stdout)
        assert (d['build'],d['mode'],d['family'],d['n'],d['reuse'],d['consumer'],d['meter'])==(build,mode,family,n,reuse,consumer,kind=='meter')
        assert len(d['phases'])==18+7*reuse
        if kind=='meter':
            ms=[x['memory'] for x in d['phases']]
            assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]));assert ms[0]['live_start']==ms[-1]['live_end']
            key=(build,mode,family,n,reuse,consumer)
            if key in prior:assert prior[key]==ms,key
            prior[key]=ms
        if (i+1)%200==0:print(kind,i+1,'/',len(jobs),flush=True)
    return len(jobs)
n=group('meter',2,7830)
(out/'allocation-gate.json').write_text(json.dumps({'processes':n,'exact_pairs':len(cells),'ownership':'pass'}))
n+=group('time',5,7831)
assert all(sha(Path(p))==h for p,h in freeze.items())
(out/'completion.json').write_text(json.dumps({'processes':n,'cells':len(cells),'timing':'exploratory'}))
