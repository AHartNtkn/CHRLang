"""E09 policy conformance; exhausted E00 fixtures on stdin."""
import json
from pathlib import Path
import sys
from scheduler import Search
from source import Rule
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'chr-symbolic'))
from check_cases import equivalent, freeze


def check(case, policy, grouping, quantum, compare_mode="reverse"):
    rules = tuple(Rule(**r) for r in case['rules'])
    search = Search(rules, case['constraints'], case['outputs'], policy, grouping, compare_mode=compare_mode)
    while not search.exhausted and search.actions < 5_000_000:
        search.advance(quantum)
    assert search.exhausted, (case['id'], policy, grouping, quantum, 'action cap')
    assert search.raw == case['raw'], (case['id'], 'raw', search.raw, case['raw'])
    for key in ('expected', 'reference'):
        assert len(search.answers) == len(case[key]), (case['id'], key, 'count')
        assert all(any(equivalent(a, b) for b in case[key]) for a in search.answers), (case['id'], key)
    return dict(id=case['id'], policy=policy, grouping=grouping, quantum=quantum,
                raw=search.raw, unique=len(search.answers), failed=search.failed,
                logical_steps=search.logical_steps, source_jobs=search.source_jobs,
                actions=dict(search.counts), answer_actions=search.answer_actions,
                peak_jobs=search.peak_jobs, peak_ready=search.peak_ready,
                peak_observations=search.peak_observations, status='pass')


if __name__ == '__main__':
    cases = [freeze(json.loads(line)) for line in sys.stdin]
    assert len(cases) == 64
    for c in cases:
        if not c['exhausted']:
            continue
        for policy, grouping in [('fifo', False), ('round', False), ('round', True), ('async', False), ('async', True)]:
            for quantum in (1, 8, 64):
                print(json.dumps(check(c, policy, grouping, quantum), sort_keys=True), flush=True)
