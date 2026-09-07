"""Check independent control, then registered E09 mixed scheduler matrix."""
import json
from pathlib import Path
import sys
from mixed import workloads
from scheduler import Search
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'chr-symbolic'))
from check_cases import equivalent
from machine import Search as Control, Rule as ControlRule


def validate(answers, raw, case):
    assert raw == case['raw'], (case['id'], 'raw', raw)
    assert len(answers) == len(case['expected']), (case['id'], 'unique', len(answers))
    assert all(any(equivalent(a, b) for b in case['expected']) for a in answers), (case['id'], 'answer')


def control(case):
    rules = [ControlRule(r.kept, r.removed, r.body, r.guards) for r in case['rules']]
    search = Control(rules, case['constraints'], case['outputs'])
    answers = []
    for _ in range(30000):
        for a in search.advance(1):
            if not any(equivalent(a, b) for b in answers):
                answers.append(a)
        if search.exhausted:
            break
    assert search.exhausted, (case['id'], 'control step cap')
    validate(answers, len(search.completed), case)
    return search.steps


def run(case, policy, grouping, batch, quantum):
    search = Search(case['rules'], case['constraints'], case['outputs'], policy, grouping,
                    batch_size=batch, compare_mode='identity')
    while not search.exhausted and search.actions < 5_000_000:
        search.advance(quantum)
    assert search.exhausted, (case['id'], 'action cap')
    validate(search.answers, search.raw, case)
    return dict(id=case['id'], policy=policy, grouping=grouping, batch=batch,
                quantum=quantum, actions=dict(search.counts), answers=search.answers,
                answer_actions=search.answer_actions, source_jobs=search.source_jobs,
                logical_steps=search.logical_steps, raw=search.raw, unique=len(search.answers),
                peak_jobs=search.peak_jobs, peak_ready=search.peak_ready,
                peak_observations=search.peak_observations, status='pass')


if __name__ == '__main__':
    cases = list(workloads())
    controls = {c['id']: control(c) for c in cases}
    for case in cases:
        configurations = [('fifo', False, 1)] + [(p, g, b) for p in ('round','async')
                                                for g in (False,True) for b in (1,8)]
        for policy, grouping, batch in configurations:
            for quantum in (1,8,64):
                row = run(case, policy, grouping, batch, quantum)
                row['control_steps'] = controls[case['id']]
                assert row['logical_steps'] == row['control_steps']
                print(json.dumps(row, sort_keys=True), flush=True)
