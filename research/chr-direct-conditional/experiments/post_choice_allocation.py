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
out=Path(sys.argv[1]);binary=Path(sys.argv[2]).resolve();out.mkdir(exist_ok=False);(out/'meter').mkdir()
cells=list(itertools.product(['scan','indexed','special','contextual','demand','shared','conditional'],['common','independent','early'],[0,3],[0,4,16,64],[1,4]))
files=[Path(__file__),Path('docs/experiments/registrations/S10-post-choice-allocation.md'),binary]
for root in ['research/chr-direct-conditional','research/chr-compiled','research/chr-relational','research/chr-persistent','research/chr-observe','crates/chr-syntax']:
    files.extend(Path(root).rglob('*.rs'));files.append(Path(root)/'Cargo.toml')
freeze={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in files};(out/'freeze.json').write_text(json.dumps(freeze,indent=2))
jobs=[(r,*c) for r in range(2) for c in cells];random.Random(7840).shuffle(jobs);(out/'order.json').write_text(json.dumps(jobs));prior={};rows=[]
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
for i,(r,mode,family,k,depth,reuse) in enumerate(jobs):
    p=out/'meter'/f'{r}-{mode}-{family}-{k}-{depth}-{reuse}.json'
    run=subprocess.run([str(binary),mode,family,str(k),str(depth),str(reuse),'query'],capture_output=True,text=True,timeout=75,preexec_fn=limits)
    if run.returncode:p.with_suffix('.error').write_text(run.stdout+run.stderr);raise RuntimeError(str(p))
    p.write_text(run.stdout);d=json.loads(run.stdout)
    assert (d['mode'],d['family'],d['choices'],d['depth'],d['reuse'],d['consumer'],d['meter'])==(mode,family,k,depth,reuse,'query',True)
    names=['prepare']+['input','setup','first-delivery-or-exhaustion','remaining-delivery','search-drop','answer-policy','input-drop']*reuse+['cancel-input','cancel-setup','cancel-unit','cancel-search-drop','cancel-input-drop','prepared-drop','retained-answers-drop']
    xs=d['phases'];assert [x['phase'] for x in xs]==names;ms=[x['memory'] for x in xs]
    assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]));assert ms[0]['live_start']==ms[-1]['live_end']
    for x in xs:
        if x['phase'] in ['input-drop','cancel-input-drop']:assert x['memory']['live_end']==ms[0]['live_end']
        if x['phase']=='prepared-drop':assert x['memory']['live_end']==ms[0]['live_start']
    key=(mode,family,k,depth,reuse)
    if key in prior:assert prior[key]==ms,key
    else:
        prior[key]=ms;rows.append(dict(mode=mode,family=family,choices=k,depth=depth,reuse=reuse,requested_bytes=sum(x['requested_bytes'] for x in ms),peak_above_start=max(x['peak_live'] for x in ms)-ms[0]['live_start']))
    if (i+1)%112==0:print(i+1,'/',len(jobs),flush=True)
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in freeze.items())
with (out/'summary.csv').open('w') as f:
    w=csv.DictWriter(f,fieldnames=rows[0]);w.writeheader();w.writerows(rows)
(out/'completion.json').write_text(json.dumps({'processes':len(jobs),'exact_pairs':len(cells),'ownership':'pass','timing':'not compared'}))
