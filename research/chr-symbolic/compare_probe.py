"""Fresh-process E11 control/candidate measurement, including full observation."""
import argparse
import json
import resource
import time
from pathlib import Path
from check_cases import freeze,equivalent
from machine import Rule

parser=argparse.ArgumentParser()
parser.add_argument('manifest',type=Path)
parser.add_argument('case')
parser.add_argument('engine',choices=['direct','terms'])
parser.add_argument('bounds',type=json.loads)
args=parser.parse_args()
case=next(freeze(json.loads(s)) for s in args.manifest.read_text().splitlines() if json.loads(s)['id']==args.case)
rules=[Rule(r['kept'],r['removed'],r['body'],r['guards']) for r in case['rules']]
start=time.monotonic()
if args.engine=='direct':
    from direct import Direct
    run=Direct(rules,case['constraints'],case['outputs'],**args.bounds)
else:
    from symbolic import Bounded
    run=Bounded(rules,case['constraints'],case['outputs'],representation='terms',**args.bounds)
constructed=time.monotonic()
answers=run.answers();status=run.boundary_status()
finished=time.monotonic()
full_equal=len(answers)==len(case['expected']) and all(any(equivalent(a,b) for b in case['expected']) for a in answers)
if case['exhausted']:
    assert all(any(equivalent(a,b) for b in case['expected']) for a in answers)
    if not any(status.values()):assert full_equal and run.models==case['raw']
result=dict(case=case['id'],engine=args.engine,bounds=args.bounds,answers=len(answers),models=run.models,full_equal=full_equal,**status,
            engine_seconds=finished-start,construct_or_execute_seconds=constructed-start,observe_and_boundary_seconds=finished-constructed,
            peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss)
if args.engine=='direct':result['source_steps']=run.steps
else:
    result.update(formula_bindings=run.e.serial,prune_checks=run.e.prune_checks,pruned=run.e.pruned,prune_seconds=run.e.prune_seconds,assertions=len(run.e.solver.assertions()))
print(json.dumps(result),flush=True)
