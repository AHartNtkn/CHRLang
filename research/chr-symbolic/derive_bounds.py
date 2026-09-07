"""Derive exploratory storage/transition bounds from independent direct traces.

Constructor counts follow source body instantiation, retaining matched handles;
all fresh rule variables count, including those in unselected OR arms. Equality
service bound eight is a starting probe, not a completeness theorem.
"""
from collections import deque
from copy import deepcopy
import json
import sys
from check_cases import freeze,equivalent
from machine import Branch,Rule


def constructors(term):
    return 0 if isinstance(term,int) else 1+sum(constructors(t) for t in term[1])


def goal_constructors(goal):
    if goal[0]=='post':return constructors(goal[1])
    if goal[0]=='eq':return constructors(goal[1])+constructors(goal[2])
    if goal[0] in ('and','or'):return sum(goal_constructors(g) for g in goal[1:])
    return 0


def derive(case):
    rules=[Rule(r['kept'],r['removed'],r['body'],r['guards']) for r in case['rules']]
    branch=Branch(case['constraints'],case['outputs'])
    branch.allocated_nodes=branch.next_var+sum(constructors(c) for c in case['constraints'])+sum(constructors(t) for t in case['outputs'])
    frontier=deque([branch]);answers=[];raw=0;steps=0
    bounds={'transitions':1,'nodes':max(1,branch.allocated_nodes),'occurrences':1,'pending':max(1,len(branch.pending)),'service':8}
    for _ in range(case['budget']):
        if not frontier:break
        branch=frontier.popleft();before=branch.next_var
        outcome=branch.step(rules);steps+=1
        if branch.trace[-1][0]=='apply':
            branch.allocated_nodes+=goal_constructors(rules[branch.trace[-1][1]].body)+branch.next_var-before
        bounds['nodes']=max(bounds['nodes'],branch.allocated_nodes)
        bounds['transitions']=max(bounds['transitions'],len(branch.trace))
        bounds['occurrences']=max(bounds['occurrences'],branch.next_occurrence)
        if outcome[0]=='continue':frontier.append(branch)
        elif outcome[0]=='answer':
            raw+=1
            if not any(equivalent(outcome[1],old) for old in answers):answers.append(outcome[1])
        elif outcome[0]=='split':
            sibling=deepcopy(branch)
            branch.pending.appendleft(outcome[1]);sibling.pending.appendleft(outcome[2])
            frontier.extend([branch,sibling])
        bounds['pending']=max([bounds['pending']]+[len(b.pending) for b in frontier])
        if case['limit'] is not None and len(answers)>=case['limit']:break
    assert (not frontier)==case['exhausted'],case['id']
    assert raw==case['raw'],case['id']
    assert len(answers)==len(case['expected']) and all(any(equivalent(a,b) for b in case['expected']) for a in answers),case['id']
    return {'case':case['id'],'bounds':bounds,'reference_exhausted':case['exhausted'],'direct_steps':steps,'direct_raw':raw}


if __name__=='__main__':
    for line in sys.stdin:print(json.dumps(derive(freeze(json.loads(line)))),flush=True)
