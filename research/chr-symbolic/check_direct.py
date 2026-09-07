"""Correctness gate for the finite arena control; no performance claims."""
import argparse
import json
from pathlib import Path
from check_cases import freeze,equivalent
from direct import Direct
from machine import Rule

parser=argparse.ArgumentParser()
parser.add_argument('manifest',type=Path)
parser.add_argument('--bounds',type=Path)
parser.add_argument('--compare',type=Path)
args=parser.parse_args()
bounds={} if args.bounds is None else {r['case']:r['bounds'] for r in map(json.loads,args.bounds.read_text().splitlines())}
comparisons={} if args.compare is None else {r['case']:r for r in map(json.loads,args.compare.read_text().splitlines())}
for line in args.manifest.read_text().splitlines():
    case=freeze(json.loads(line))
    b=bounds.get(case['id'],dict(transitions=8,nodes=6,occurrences=3,pending=3,service=4))
    run=Direct([Rule(r['kept'],r['removed'],r['body'],r['guards']) for r in case['rules']],case['constraints'],case['outputs'],**b)
    answers=run.answers();status=run.boundary_status()
    equal=len(answers)==len(case['expected']) and all(any(equivalent(a,e) for e in case['expected']) for a in answers)
    classification='bounded_incomplete' if any(status.values()) else 'complete_match' if case['exhausted'] and equal else 'mismatch' if case['exhausted'] else 'prefix_reference'
    row=dict(case=case['id'],bounds=b,answers=len(answers),models=run.models,full_equal=equal,classification=classification,**status)
    assert classification!='mismatch',row
    if case['exhausted']:
        assert all(any(equivalent(a,e) for e in case['expected']) for a in answers),row
        if not any(status.values()):assert run.models==case['raw'],row
    if case['id'] in comparisons:
        other=comparisons[case['id']]
        assert all(row[k]==other[k] for k in row if k in other),(row,other)
    print(json.dumps(row),flush=True)
