#!/usr/bin/env python3
import hashlib
import itertools
import json
from pathlib import Path
import random
import resource
import subprocess
import sys
out=Path(sys.argv[1]);meter,timing=[Path(x).resolve() for x in sys.argv[2:4]];out.mkdir(exist_ok=False)
cells=list(itertools.product(['inferred','declared'],['common','independent'],[0,3],[0,16],[1,4],['eligible','missing-permit','unknown']))
files=[Path(__file__),Path('docs/experiments/registrations/S07-resource-contract-lifecycle.md'),Path('Cargo.lock'),meter,timing]
for root in ['research/chr-direct-conditional','research/chr-compiled','research/chr-relational','research/chr-persistent','research/chr-observe','crates/chr-syntax']:
    files.extend(Path(root).rglob('*.rs'));files.append(Path(root)/'Cargo.toml')
freeze={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in files};(out/'freeze.json').write_text(json.dumps(freeze,indent=2));(out/'toolchain.txt').write_text(subprocess.check_output(['rustc','--version','--verbose'],text=True))
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def group(kind,binary,reps,seed):
    jobs=[(r,*c) for r in range(reps) for c in cells];random.Random(seed).shuffle(jobs);(out/f'{kind}-order.json').write_text(json.dumps(jobs));(out/kind).mkdir();prior={}
    for i,(r,mode,family,k,depth,reuse,schedule) in enumerate(jobs):
        p=out/kind/f'{r}-{mode}-{family}-{k}-{depth}-{reuse}-{schedule}.json'
        run=subprocess.run([str(binary),mode,family,str(k),str(depth),str(reuse),schedule],capture_output=True,text=True,timeout=75,preexec_fn=limits)
        if run.returncode:p.with_suffix('.error').write_text(run.stdout+run.stderr);raise RuntimeError(str(p))
        p.write_text(run.stdout);d=json.loads(run.stdout)
        assert (d['mode'],d['family'],d['choices'],d['depth'],d['reuse'],d['schedule'],d['meter'])==(mode,family,k,depth,reuse,schedule,kind=='meter')
        rejected=reuse//2+1 if mode=='declared' and schedule=='unknown' else 0
        assert d['rejected']==rejected
        assert len(d['phases'])==7+7*reuse
        if kind=='meter':
            ms=[x['memory'] for x in d['phases']];assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]));assert ms[0]['live_start']==ms[-1]['live_end']
            key=(mode,family,k,depth,reuse,schedule)
            if key in prior:assert prior[key]==ms,key
            prior[key]=ms
        if (i+1)%96==0:print(kind,i+1,'/',len(jobs),flush=True)
    return len(jobs)
n=group('meter',meter,2,7900);(out/'allocation-gate.json').write_text(json.dumps({'processes':n,'exact_pairs':len(cells),'ownership':'pass'}))
n+=group('time',timing,5,7901)
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in freeze.items())
(out/'completion.json').write_text(json.dumps({'processes':n,'cells':len(cells),'timing':'exploratory'}))
