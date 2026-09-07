"""Registered certified early-guard comparison and no-pruning controls."""
import json
import sys
from source import Rule
from mixed import workloads
from run_mixed import control, run
from run_selector import matched_workloads, emit
from check_source import check
from check_cases import freeze


def no_pruning():
    for family, guard in [('early-success', (0,1)), ('late-success', (0,2)), ('local-success', (3,3))]:
        for n in (4,8,12):
            q = ('q', (('a', ()),))
            rule = Rule((), (('q', (0,)), ('q', (1,)), ('q', (2,))), ('true',), (guard,))
            yield dict(id=f'{family}-{n}', rules=(rule,), constraints=(q,)*n,
                       outputs=(), expected=[dict(outputs=[], residual=[q]*(n%3))], raw=1,
                       expected_steps=n+2*(n//3)+1)


if __name__ == '__main__':
    cases = [freeze(json.loads(line)) for line in sys.stdin]
    assert len(cases) == 64
    for case in cases:
        previous = None
        for quantum in (1,8,64):
            row = check(case, quantum, 'guard-prefix')
            invariant = {k:v for k,v in row.items() if k != 'quantum'}
            assert previous is None or previous == invariant
            previous = invariant
            emit(row, 'source-gate', 'guard-prefix')
    for case in workloads():
        steps = control(case)
        configs = [('fifo', False, 1)] + [(p,g,b) for p in ('round','async')
                                         for g in (False,True) for b in (1,8)]
        for policy, grouping, batch in configs:
            for quantum in (1,8,64):
                row = run(case, policy, grouping, batch, quantum, 'guard-prefix')
                assert row['logical_steps'] == steps
                emit(row, 'mixed', 'guard-prefix')
    for case in list(matched_workloads()) + list(no_pruning()):
        steps = control(case)
        if 'expected_steps' in case:
            assert steps == case['expected_steps']
        modes = ('prefix', 'guard-prefix') if 'expected_steps' in case else ('guard-prefix',)
        for mode in modes:
            for policy in ('fifo','round','async'):
                for quantum in (1,8,64):
                    row = run(case, policy, False, 1 if policy == 'fifo' else 8, quantum, mode)
                    assert row['logical_steps'] == steps
                    emit(row, 'no-pruning' if 'expected_steps' in case else 'matched', mode)
