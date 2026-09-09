#!/usr/bin/env python3
import csv
import hashlib
import itertools
import json
from pathlib import Path
import random
import resource
import subprocess
import sys
base=Path(sys.argv[1]);out=Path(sys.argv[2]);binary=Path(sys.argv[3]).resolve()
out.mkdir(exist_ok=False)
files=[Path('research/chr-direct-conditional/src/engine.rs'),Path('research/chr-direct-conditional/examples/broad_mixed_cost.rs'),Path('docs/experiments/registrations/S10-known-arity-export-repair.md'),Path(__file__),binary]
(out/'freeze.json').write_text(json.dumps({str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in files},indent=2))
cells=list(itertools.product(['independent','shared','delayed','early','late','history','deep'],[0,1,3],[1,4],['query','prepared']))
jobs=[(r,*c) for r in range(2) for c in cells];random.Random(7832).shuffle(jobs)
(out/'order.json').write_text(json.dumps(jobs));(out/'meter').mkdir();prior={};rows=[]
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
for r,family,n,reuse,consumer in jobs:
    path=out/'meter'/f'{r}-conditional-{family}-{n}-{reuse}-{consumer}.json'
    run=subprocess.run([str(binary),'conditional',family,str(n),str(reuse),consumer],capture_output=True,text=True,timeout=75,preexec_fn=limits)
    if run.returncode:path.with_suffix('.error').write_text(run.stdout+run.stderr);raise RuntimeError(str(path))
    path.write_text(run.stdout);d=json.loads(run.stdout)
    old=json.loads((base/'meter'/path.name).read_text());xs=d['phases'];oxs=old['phases'];assert [x['phase'] for x in xs]==[x['phase'] for x in oxs]
    ms=[x['memory'] for x in xs];assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]));assert ms[0]['live_start']==ms[-1]['live_end']
    kept=0
    for i in range(reuse):
        q=ms[1+7*i:8+7*i];abytes=q[4]['live_end']-ms[0]['live_end']-kept-(q[0]['live_end']-q[0]['live_start']);assert abytes>=0
        if consumer=='prepared':kept+=abytes
        assert q[-1]['live_end']==ms[0]['live_end']+kept
    assert ms[-3]['live_end']==ms[0]['live_end']+kept;assert ms[-2]['live_end']==ms[0]['live_start']+kept
    for x,o in zip(xs,oxs):
        if x['phase'] not in ['first-delivery-or-exhaustion','remaining-delivery']:
            assert x['memory']['requested_bytes']==o['memory']['requested_bytes'],x['phase']
    key=(family,n,reuse,consumer)
    if key in prior:assert prior[key]==ms
    else:
        prior[key]=ms
        rows.append(dict(family=family,n=n,reuse=reuse,consumer=consumer,before_bytes=sum(x['memory']['requested_bytes'] for x in oxs),after_bytes=sum(x['requested_bytes'] for x in ms),after_peak=max(x['peak_live'] for x in ms)-ms[0]['live_start'],retained_bytes=kept))
with (out/'summary.csv').open('w') as f:
    w=csv.DictWriter(f,fieldnames=rows[0]);w.writeheader();w.writerows(rows)
print('168 processes; 84 exact pairs; complete answer and all owner checks pass; allocation traffic changes only in delivery')
