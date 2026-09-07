"""Registered distinct-prefix selector conformance and work comparison."""
import json
import sys
from mixed import workloads
from run_mixed import control, run
from run_selector import matched_workloads, emit
from check_source import check
from check_cases import freeze


if __name__ == '__main__':
    cases = [freeze(json.loads(line)) for line in sys.stdin]
    assert len(cases) == 64
    for case in cases:
        previous = None
        for quantum in (1,8,64):
            row = check(case, quantum, 'prefix')
            invariant = {k: v for k, v in row.items() if k != 'quantum'}
            assert previous is None or previous == invariant
            previous = invariant
            emit(row, 'source-gate', 'prefix')
    for case in workloads():
        steps = control(case)
        configs = [('fifo', False, 1)] + [(p,g,b) for p in ('round','async')
                                         for g in (False,True) for b in (1,8)]
        for policy, grouping, batch in configs:
            for quantum in (1,8,64):
                row = run(case, policy, grouping, batch, quantum, 'prefix')
                assert row['logical_steps'] == steps
                emit(row, 'mixed', 'prefix')
    for case in matched_workloads():
        steps = control(case)
        for policy in ('fifo','round','async'):
            for quantum in (1,8,64):
                row = run(case, policy, False, 1 if policy == 'fifo' else 8, quantum, 'prefix')
                assert row['logical_steps'] == steps
                emit(row, 'matched', 'prefix')
