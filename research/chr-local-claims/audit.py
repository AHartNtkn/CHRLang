"""Replay frozen cells and verify the claimed protocol/atomic-model relation."""
import hashlib,json,sys
from pathlib import Path
sys.dont_write_bytecode=True
from model import CASES,explore
from oracle import outcomes
ROOT=Path(__file__).resolve().parents[2];RAW=ROOT/'docs/experiments/results/s03-local-claims'
m=json.loads((RAW/'manifest.json').read_text());r=json.loads((RAW/'results.json').read_text())
for p,h in m['sha256'].items():assert hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h,p
assert r['status']=='passed' and len(r['rows'])==40
keys=set();states=edges=partial=0
for row in r['rows']:
    key=(row['case'],row['cancel'],row['priority_admission']);assert key not in keys;keys.add(key)
    case=CASES[key[0]];got,n,e,c,traces=explore(case,key[1],key[2]);assert got==outcomes(case,key[1],key[2])
    assert (n,e,c)==(row['states'],row['edges'],row['partial_cancel_edges'])
    difference=got-outcomes(case,key[1],True);assert len(difference)==row['outside_source_priority']
    assert len(got)==row['terminal_outcomes']
    witnesses=[dict(observation=o,trace=traces[o]) for o in sorted(difference)]
    assert json.loads(json.dumps(witnesses))==row['witnesses']
    states+=n;edges+=e;partial+=c
assert states==r['states']==414 and edges==581 and partial==51
assert keys=={(name,c,p) for name,case in CASES.items() for c in [None,*range(len(case[2]))] for p in [False,True]}
print('Verified 40 cells, 414 states, 581 transitions, 51 partial-claim cancellation edges; all atomic outcomes and priority counterexamples replay.')
