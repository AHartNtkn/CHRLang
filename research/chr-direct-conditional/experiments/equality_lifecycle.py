#!/usr/bin/env python3
"""Registered paired equality-control pilot; fail closed on any invalid run."""
import hashlib
import itertools
import json
from pathlib import Path
import random
import resource
import subprocess
import sys

out = Path(sys.argv[1])
binroot = Path(sys.argv[2]).resolve()
out.mkdir(exist_ok=False)
cells = list(itertools.product(['scan','contextual','demand','resumable'], ['stable_history','reset_history','constructor','dynamic','delayed','independent'], [0,3], [1,4], ['query','prepared'], ['original','eliminated'], ['conservative','precise']))
sources = [Path(__file__), Path('docs/experiments/registrations/S10-equality-lifecycle.md'), Path('Cargo.lock')]
for root in ['research/chr-direct-conditional','research/chr-compiled','research/chr-relational','research/chr-persistent','research/chr-observe','crates/chr-syntax']:
    sources.extend(Path(root).rglob('*.rs'))
    sources.append(Path(root)/'Cargo.toml')
binaries = [binroot/f'{b}-{k}' for b in ['conservative','precise'] for k in ['meter','time']]
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
freeze = {str(p):sha(p) for p in sources+binaries}
(out/'freeze.json').write_text(json.dumps(freeze,indent=2))
(out/'toolchain.txt').write_text(subprocess.check_output(['rustc','--version','--verbose'],text=True))
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3))
    resource.setrlimit(resource.RLIMIT_CPU,(60,60))
for kind,reps,seed in [('meter',2,7820),('time',5,7821)]:
    jobs=[(r,*c) for r in range(reps) for c in cells]
    random.Random(seed).shuffle(jobs)
    (out/f'{kind}-order.json').write_text(json.dumps(jobs))
    (out/kind).mkdir()
    prior={}
    for i,(r,mode,family,n,reuse,consumer,form,build) in enumerate(jobs):
        dest=out/kind/f'{r}-{mode}-{family}-{n}-{reuse}-{consumer}-{form}-{build}.json'
        run=subprocess.run([str(binroot/f'{build}-{kind}'),mode,family,str(n),str(reuse),consumer,form],capture_output=True,text=True,timeout=75,preexec_fn=limits)
        if run.returncode:
            dest.with_suffix('.error').write_text(run.stdout+run.stderr)
            raise RuntimeError(str(dest))
        dest.write_text(run.stdout)
        d=json.loads(run.stdout)
        assert (d['mode'],d['family'],d['n'],d['reuse'],d['consumer'],d['form'],d['precise'],d['meter']) == (mode,family,n,reuse,consumer,form,build=='precise',kind=='meter')
        assert len(d['phases'])==9+7*reuse
        if kind=='meter':
            ms=[x['memory'] for x in d['phases']]
            assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]))
            assert ms[0]['live_start']==ms[-1]['live_end']
            key=(mode,family,n,reuse,consumer,form,build)
            if key in prior: assert prior[key]==ms,key
            prior[key]=ms
        if (i+1)%256==0: print(kind,i+1,'/',len(jobs),flush=True)
    (out/f'{kind}-complete.json').write_text(json.dumps({'processes':len(jobs),'cells':len(cells)}))
assert all(sha(Path(p))==h for p,h in freeze.items())
(out/'completion.json').write_text(json.dumps({'processes':5376,'cells':768,'exact_allocation_pairs':768,'timing':'exploratory'}))
