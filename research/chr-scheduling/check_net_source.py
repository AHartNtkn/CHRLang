"""E15 projected source-state correspondence for net equality."""
import json
import sys
from check_source import check
from net_bridge import NetEquality
from check_cases import freeze, equivalent
from machine import resolve


def vocabulary(case):
    names = set()
    def term(t):
        if isinstance(t, int):
            return
        names.add(t[0])
        for c in t[1]:
            term(c)
    def goal(g):
        if g[0] == 'post':
            term(g[1])
        elif g[0] == 'eq':
            term(g[1]); term(g[2])
        elif g[0] in ('and','or'):
            for child in g[1:]:
                goal(child)
    for t in case['constraints'] + case['outputs']:
        term(t)
    for r in case['rules']:
        for t in r['kept'] + r['removed']:
            term(t)
        for a,b in r['guards']:
            term(a); term(b)
        goal(r['body'])
    return tuple(sorted(names))


def projected(state, branch):
    def allocated(term):
        pending = [term]
        while pending:
            t = pending.pop()
            if isinstance(t, int):
                assert 0 <= t < state.next_var, 'unallocated source variable'
            else:
                pending.extend(t[1])
    def allocated_goal(g):
        if g[0] == 'post':
            allocated(g[1])
        elif g[0] == 'eq':
            allocated(g[1]); allocated(g[2])
        elif g[0] in ('and','or'):
            for child in g[1:]:
                allocated_goal(child)
    for k,t in state.sub:
        allocated(k); allocated(t)
    for t in state.outputs:
        allocated(t)
    for _,t in state.store:
        allocated(t)
    for g in state.pending:
        allocated_goal(g)
    assert state.next_var == branch.next_var
    assert state.next_occurrence == branch.next_occurrence
    assert state.history == branch.history
    assert tuple(i for i,_ in state.store) == tuple(i for i,_ in branch.store)
    def projection(pending, store, outputs, sub, count):
        def goal(g):
            if g[0] == 'post':
                return ('post', (resolve(g[1], sub),))
            if g[0] == 'eq':
                return ('eq', (resolve(g[1], sub), resolve(g[2], sub)))
            if g[0] in ('and','or'):
                return (g[0], tuple(goal(c) for c in g[1:]))
            return (g[0], ())
        # Wrappers fix each sequence's position/length without multiset matching.
        terms = [('allocated', tuple(resolve(v, sub) for v in range(count))),
                 ('outputs', tuple(resolve(v, sub) for v in outputs)),
                 ('store', tuple(resolve(t, sub) for _,t in store)),
                 ('pending', tuple(goal(g) for g in pending))]
        return dict(outputs=terms, residual=[])
    a = projection(state.pending, state.store, state.outputs, dict(state.sub), state.next_var)
    b = projection(branch.pending, branch.store, branch.outputs, branch.sub, branch.next_var)
    assert equivalent(a,b), 'projected state differs'


if __name__ == '__main__':
    cases = [freeze(json.loads(line)) for line in sys.stdin]
    assert len(cases) == 64
    for case in cases:
        service = NetEquality(vocabulary(case), status=sys.argv[1] if len(sys.argv)>1 else 'scan')
        try:
            row = check(case, 8, equality=service, comparer=projected)
        except Exception as error:
            print(json.dumps(dict(id=case['id'], status='error', detail=repr(error))), flush=True)
            raise
        print(json.dumps(row, sort_keys=True), flush=True)
