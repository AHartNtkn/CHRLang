"""Registered action-count matrix; no timing/byte conclusions."""
import json
from pathlib import Path
import sys
from probes import workloads
from scheduler import Search
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'chr-symbolic'))
from check_cases import equivalent


def run(case, policy, grouping, quantum):
    search = Search(case['rules'], case['constraints'], case['outputs'], policy, grouping)
    while not search.exhausted and search.actions < 5_000_000:
        search.advance(quantum)
        if case['prefix'] and search.answers:
            break
    assert search.actions < 5_000_000, (case['id'], 'action cap')
    assert search.exhausted != case['prefix']
    assert search.raw == case['raw'], (case['id'], search.raw)
    assert len(search.answers) == len(case['expected'])
    assert all(any(equivalent(a, b) for b in case['expected']) for a in search.answers)
    return dict(id=case['id'], policy=policy, grouping=grouping, quantum=quantum,
                prefix=case['prefix'], raw=search.raw, unique=len(search.answers),
                source_jobs=search.source_jobs, logical_steps=search.logical_steps,
                actions=dict(search.counts), answers=search.answers,
                answer_actions=search.answer_actions, peak_jobs=search.peak_jobs,
                peak_ready=search.peak_ready, peak_observations=search.peak_observations,
                status='pass')


if __name__ == '__main__':
    for case in workloads():
        for policy, grouping in [('fifo', False), ('round', False), ('round', True), ('async', False), ('async', True)]:
            for quantum in (1,8,64):
                print(json.dumps(run(case, policy, grouping, quantum), sort_keys=True), flush=True)
