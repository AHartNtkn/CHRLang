"""Registered predicate-filter gate and opposed mixed workloads."""
import json
import sys
from source import Rule
from mixed import workloads
from run_mixed import control, run
from check_source import check
from check_cases import freeze


def matched_workloads():
    for family in ('guard-tuples', 'acyclic-join'):
        for n in (4,8,12):
            if family == 'guard-tuples':
                store = tuple(('q', ((f'n{i}', ()),)) for i in range(n))
                # Distinct occurrence values make the guard false on every tuple.
                reject = Rule((), (('q', (0,)), ('q', (1,)), ('q', (2,))), ('fail',), ((0,1),))
            else:
                store = tuple(('edge', ((f'n{i}', ()), (f'n{i+1}', ()))) for i in range(n))
                reject = Rule((), (('edge', (0,1)), ('edge', (1,2)), ('edge', (2,0))), ('fail',))
            slow = ('and', *(('post', t) for t in store), ('eq', 0, ('b', ())))
            fast = ('and', *(('true',) for _ in store), ('post', ('finish', (0,))))
            rules = (Rule((), (('start', (0,)),), ('or', slow, fast)), reject,
                     Rule((), (('finish', (0,)),), ('eq', 0, ('a', ()))))
            yield dict(id=f'{family}-{n}', rules=rules, constraints=(('start', (0,)),),
                       outputs=(0,), expected=[dict(outputs=[('a', ())], residual=[]),
                                              dict(outputs=[('b', ())], residual=list(store))], raw=2)


def emit(row, group, selector):
    row.update(group=group, selector=selector)
    print(json.dumps(row, sort_keys=True), flush=True)


if __name__ == '__main__':
    cases = [freeze(json.loads(line)) for line in sys.stdin]
    assert len(cases) == 64
    for case in cases:
        previous = None
        for quantum in (1,8,64):
            row = check(case, quantum, 'predicate')
            invariant = {k: v for k, v in row.items() if k != 'quantum'}
            assert previous is None or previous == invariant
            previous = invariant
            emit(row, 'source-gate', 'predicate')
    for case in workloads():
        steps = control(case)
        configs = [('fifo', False, 1)] + [(p,g,b) for p in ('round','async')
                                         for g in (False,True) for b in (1,8)]
        for policy, grouping, batch in configs:
            for quantum in (1,8,64):
                row = run(case, policy, grouping, batch, quantum, 'predicate')
                assert row['logical_steps'] == steps
                emit(row, 'mixed', 'predicate')
    for case in matched_workloads():
        steps = control(case)
        for selector in ('scan', 'predicate'):
            for policy in ('fifo', 'round', 'async'):
                for quantum in (1,8,64):
                    row = run(case, policy, False, 1 if policy == 'fifo' else 8, quantum, selector)
                    assert row['logical_steps'] == steps
                    emit(row, 'matched', selector)
