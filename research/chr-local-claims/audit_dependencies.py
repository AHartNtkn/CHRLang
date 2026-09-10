"""Recompute all registered admission and staged-publication evidence."""
from pathlib import Path
import hashlib,json,sys
sys.dont_write_bytecode=True
from model import CASES
from oracle import outcomes
from dependencies import explore_components
from publication import explore_publication
ROOT=Path(__file__).resolve().parents[2];RAW=ROOT/'docs/experiments/results/s03-local-dependencies'
first=json.loads((RAW/'manifest.json').read_text())
for p,h in first.items():assert hashlib.sha256((RAW/'attempt-1-sources'/Path(p).name).read_bytes()).hexdigest()==h,p
assert 'deadlock' in json.loads((RAW/'failure.json').read_text())['error']
for p,h in json.loads((RAW/'attempt-2/manifest.json').read_text()).items():assert hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h,p
r=json.loads((RAW/'attempt-2/results.json').read_text());assert r['status']=='passed'
keys=set();total=0
for row in r['admission']:
    name,cancel,bindings=row['case'],row['cancel'],row['binding_edges'];keys.add((name,cancel,bindings))
    got,n,e,parallel,groups,dead=explore_components(CASES[name],cancel,bindings);expected=outcomes(CASES[name],cancel,True)
    result=dict(case=name,cancel=cancel,binding_edges=bindings,states=n,edges=e,parallel_admission_states=parallel,groups=groups,outcomes=len(got),deadlocks=dead,missing=len(expected-set(got)),differences=[dict(observation=o,trace=got[o]) for o in sorted(set(got)-expected)])
    assert json.loads(json.dumps(result))==row
    if bindings:assert not dead and set(got)==expected
    total+=n
assert keys=={(name,c,b) for name,case in CASES.items() for c in [None,*range(len(case[2]))] for b in [False,True]}
assert len(r['admission'])==40 and len(r['publication'])==4
keys=set()
for row in r['publication']:
    checked,cancel=row['validated_scan'],row['cancellation'];keys.add((checked,cancel))
    got=dict(cancellation=cancel,validated_scan=checked,**explore_publication(checked,cancel));assert json.loads(json.dumps(got))==row
    if checked:assert not got['mixed'] and got['published']==[(0,)*4,(1,)*4]
    else:assert got['mixed']
    total+=got['states']
assert len(keys)==4 and total==r['states']==3010
print('Verified both frozen attempts, 40 admission cells, 4 publication cells, 3010 states, deadlock/order witnesses and validated snapshots.')
