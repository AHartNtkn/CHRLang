"""One isolated exploratory conformance run. Not a comparative benchmark."""
import argparse
import json
import time
import sys
from pathlib import Path
from check_cases import freeze,equivalent
from machine import Rule
from symbolic import Bounded

parser=argparse.ArgumentParser()
parser.add_argument('manifest',type=Path)
parser.add_argument('case')
for field,default in [('transitions',8),('nodes',6),('occurrences',3),('pending',3),('service',4)]:
    parser.add_argument('--'+field,type=int,default=default)
args=parser.parse_args()
case=next(freeze(json.loads(line)) for line in args.manifest.read_text().splitlines() if json.loads(line)['id']==args.case)
bounds={f:getattr(args,f) for f in ['transitions','nodes','occurrences','pending','service']}
print(json.dumps({'phase':'build'}),file=sys.stderr,flush=True)
start=time.monotonic()
run=Bounded([Rule(r['kept'],r['removed'],r['body'],r['guards']) for r in case['rules']],case['constraints'],case['outputs'],**bounds)
built=time.monotonic()
print(json.dumps({'phase':'observe'}),file=sys.stderr,flush=True)
answers=run.answers()
observed=time.monotonic()
print(json.dumps({'phase':'boundary'}),file=sys.stderr,flush=True)
status=run.boundary_status()
expected=case['expected']
full_equal=len(answers)==len(expected) and all(any(equivalent(a,b) for b in expected) for a in answers)
classification='bounded_incomplete' if any(status.values()) else 'complete_match' if case['exhausted'] and full_equal else 'mismatch' if case['exhausted'] else 'prefix_reference'
result={'classification':classification,'reference_exhausted':case['exhausted'],'case':case['id'],'bounds':bounds,'answers':len(answers),'models':run.models,'expected_answers':len(expected),'full_equal':full_equal,**status,'formula_bindings':run.e.serial,'prune_checks':run.e.prune_checks,'pruned':run.e.pruned,'prune_seconds':run.e.prune_seconds,'assertions':len(run.e.solver.assertions()),'build_seconds':built-start,'observe_seconds':observed-built,'boundary_seconds':time.monotonic()-observed}
print(json.dumps(result),flush=True)

if classification=='mismatch':raise SystemExit(1)
