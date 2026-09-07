"""E09 source transition gate. Read E00 exported JSONL on stdin."""
import copy
import json
from pathlib import Path
import sys
from collections import Counter, deque
from source import Rule, StepJob, initial

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'chr-symbolic'))
from machine import Branch, Rule as ControlRule
from check_cases import freeze, equivalent


def compare(state, branch):
    assert state.pending == tuple(branch.pending), 'pending'
    assert state.outputs == tuple(branch.outputs), 'outputs'
    assert state.next_var == branch.next_var, 'next_var'
    assert state.next_occurrence == branch.next_occurrence, 'next_occurrence'
    assert state.store == tuple(branch.store), 'store'
    assert dict(state.sub) == branch.sub, 'substitution'
    assert state.history == branch.history, 'history'


def check(case, quantum, selector="scan", equality=None, comparer=compare):
    rules = tuple(Rule(**r) for r in case['rules'])
    controls = tuple(ControlRule(**r) for r in case['rules'])
    state = initial(case['constraints'], case['outputs'])
    branch = Branch(case['constraints'], case['outputs'])
    comparer(state, branch)
    queue = deque([(state, branch)])
    counts = Counter()
    answers = []
    steps = raw = 0
    for _ in range(case['budget']):
        if not queue:
            break
        state, branch = queue.popleft()
        job = StepJob(state, rules, selector, equality)
        total = 0
        while not job.done:
            total += job.advance(quantum)
            if total > 2_000_000:
                raise AssertionError((case['id'], steps, 'service cap'))
        counts.update(job.counts)
        event = job.observe()
        control = branch.step(controls)
        assert event[0] == control[0], (case['id'], steps, event[0], control[0])
        assert job.action == branch.trace[-1]
        steps += 1
        if event[0] == 'continue':
            comparer(event[1], branch)
            queue.append((event[1], branch))
        elif event[0] == 'split':
            for child, goal in zip(event[1:], control[1:]):
                sibling = copy.deepcopy(branch)
                sibling.pending.appendleft(goal)
                comparer(child, sibling)
                queue.append((child, sibling))
        elif event[0] == 'answer':
            assert equivalent(event[1], control[1])
            raw += 1
            if not any(equivalent(event[1], old) for old in answers):
                answers.append(event[1])
        if case['limit'] is not None and len(answers) >= case['limit']:
            break
    assert (not queue) == case['exhausted'], (case['id'], 'exhaustion')
    assert raw == case['raw'], (case['id'], 'raw', raw, case['raw'])
    for key in ('expected', 'reference'):
        assert len(answers) == len(case[key]), (case['id'], key, 'count')
        assert all(any(equivalent(a, b) for b in case[key]) for a in answers), (case['id'], key)
    return dict(id=case['id'], quantum=quantum, steps=steps, raw=raw,
                unique=len(answers), exhausted=not queue, actions=dict(counts), status='pass')


if __name__ == '__main__':
    cases = [freeze(json.loads(line)) for line in sys.stdin]
    assert len(cases) == 64
    for case in cases:
        previous = None
        for quantum in (1, 8, 64):
            row = check(case, quantum)
            invariant = {k: v for k, v in row.items() if k != 'quantum'}
            assert previous is None or previous == invariant
            previous = invariant
            print(json.dumps(row, sort_keys=True), flush=True)
