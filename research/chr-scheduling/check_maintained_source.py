"""A3 actual-source gate. No comparative result files are written here.

The root runner owns registration, matrix execution, hashes and result records.
"""
import copy
from collections import Counter, deque
from source import StepJob, initial
from maintained_join import MaintainedJoin, Selector
from check_source import compare
from machine import Branch, Rule as ControlRule
from check_cases import equivalent


def check(case, mode, quantum=8, transitions=True):
    if mode not in ('prefix','full','selective'):
        raise ValueError('unknown maintained-source control')
    rules=case['rules']
    control_rules=tuple(ControlRule(r.kept,r.removed,r.body,r.guards) for r in rules)
    index=None if mode=='prefix' else MaintainedJoin(rules,mode,case['contexts'])
    state=initial(case['constraints'],case['outputs'])
    branch=Branch(case['constraints'],case['outputs']) if transitions else None
    queue=deque([(state,branch,0)])
    next_context=1
    counts=Counter(); answers=[]; raw=steps=failed=0
    peak_occurrences=peak_pending=0
    while queue and steps<case['budget']:
        state,branch,context=queue.popleft()
        peak_occurrences=max(peak_occurrences,len(state.store))
        peak_pending=max(peak_pending,len(state.pending))
        if transitions: compare(state,branch)
        selector='prefix' if index is None else Selector(index,context)
        job=StepJob(state,rules,selector)
        while not job.done:
            job.advance(quantum)
            if sum(counts.values())+sum(job.counts.values())>case['matcher_cap']:
                raise RuntimeError('registered instrumented-work bound reached')
        counts.update(job.counts)
        event=job.observe(); expected=branch.step(control_rules) if transitions else None
        if transitions:
            assert event[0]==expected[0],(case['id'],mode,steps,event,expected)
            assert job.action==branch.trace[-1],(case['id'],mode,steps,job.action,branch.trace[-1])
        steps+=1
        if event[0]=='continue':
            if transitions: compare(event[1],branch)
            queue.append((event[1],branch,context))
        elif event[0]=='split':
            if next_context>=case['contexts']: raise ValueError('finite context budget exceeded')
            if index:
                for label in index.fork(context,next_context):
                    counts[label]+=1
                    if sum(counts.values())>case['matcher_cap']:
                        raise RuntimeError('registered instrumented-work bound reached during fork')
            goals=expected[1:] if transitions else (None,None)
            for child,goal,ctx in zip(event[1:],goals,(context,next_context)):
                sibling=copy.deepcopy(branch) if transitions else None
                if transitions:
                    sibling.pending.appendleft(goal)
                    compare(child,sibling)
                queue.append((child,sibling,ctx))
            next_context+=1
        elif event[0]=='answer':
            if transitions: assert equivalent(event[1],expected[1])
            raw+=1
            if not any(equivalent(event[1],old) for old in answers): answers.append(event[1])
        elif event[0]=='failed':failed+=1
    if queue: raise RuntimeError('registered source-step bound reached')
    if 'expected' in case:
        assert raw==case['contexts'] and failed==0
        assert len(answers)==len(case['expected'])
        assert all(any(equivalent(a,b) for b in case['expected']) for a in answers)
    return dict(id=case['id'],mode=mode,quantum=quantum,transitions=transitions,steps=steps,raw=raw,failed=failed,
                answers=answers,counts=dict(counts),exhausted=True,
                peak_occurrences=peak_occurrences,peak_pending=peak_pending,
                peak_rows=0 if index is None else index.peak_rows,
                peak_tests=0 if index is None else index.peak_tests,
                peak_dependencies=0 if index is None else index.peak_dependencies,
                peak_index_edges=0 if index is None else index.peak_index_edges,
                retained_contexts=0 if index is None else len(index.contexts),
                retained_occurrences=0 if index is None else sum(len(c.store) for c in index.contexts.values()),
                retained_bindings=0 if index is None else sum(len(c.sub) for c in index.contexts.values()),
                retained_rows=0 if index is None else len(index.rows),
                retained_tests=0 if index is None else sum(len(c.tests) for c in index.contexts.values()))
