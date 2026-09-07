"""Registered E09 comparator microcases, integration and semantic gate."""
from dataclasses import replace
import json
import sys
from scheduler import Job, state_equal
from source import initial
from probes import workloads
from run_probes import run
from check_scheduler import check
from check_cases import freeze


def fresh_tree(depth):
    t = tuple(['z', ()])
    for _ in range(depth):
        t = ('s', (t,))
    return t


def common(depth):
    result = []
    for _ in range(4):
        t = fresh_tree(depth)
        result.append(('eq', t, t))
    return tuple(result)


def micro():
    base = initial((), (0,))
    for kind in ('equal', 'first', 'last'):
        for depth in (4,16,64):
            for shared in (False, True):
                left = common(depth)
                right = left if shared else common(depth)
                if kind == 'first':
                    left = (('post', ('a', ())),) + left
                    right = (('post', ('b', ())),) + right
                elif kind == 'last':
                    left += (('post', ('a', ())),)
                    right += (('post', ('b', ())),)
                a, b = replace(base, pending=left), replace(base, pending=right)
                expected = a == b
                assert expected == (kind == 'equal')
                for mode in ('reverse', 'forward', 'identity'):
                    previous = None
                    for quantum in (1,8,64):
                        job = Job(state_equal(a, b, mode))
                        actions = 0
                        while not job.done:
                            actions += job.advance(quantum)
                            assert actions <= 2_000_000
                        assert job.observe() == expected
                        assert previous is None or previous == job.counts
                        previous = job.counts.copy()
                        yield dict(group='micro', kind=kind, depth=depth, shared=shared,
                                   mode=mode, quantum=quantum, equivalent=expected,
                                   actions=dict(job.counts), status='pass')


if __name__ == '__main__':
    for row in micro():
        print(json.dumps(row, sort_keys=True), flush=True)
    for c in workloads():
        for policy in ('round', 'async'):
            for mode in ('reverse', 'forward', 'identity'):
                for quantum in (1,8,64):
                    row = run(c, policy, True, quantum, mode)
                    row.update(group='integrated', mode=mode)
                    print(json.dumps(row, sort_keys=True), flush=True)
    cases = [freeze(json.loads(line)) for line in sys.stdin]
    assert len(cases) == 64
    for c in cases:
        if not c['exhausted']:
            continue
        for policy in ('round', 'async'):
            for mode in ('reverse', 'forward', 'identity'):
                row = check(c, policy, True, 8, mode)
                row.update(group='conformance', mode=mode)
                print(json.dumps(row, sort_keys=True), flush=True)
