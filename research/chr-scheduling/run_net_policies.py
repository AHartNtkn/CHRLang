"""Registered direct/net service policy conformance and work comparison."""
from dataclasses import asdict
import json
import sys
from source import Rule
from scheduler import Search
from net_bridge import NetEquality
from check_net_source import vocabulary
from check_cases import freeze, equivalent
from probes import workloads


def run(case, service, policy, grouping, quantum):
    prepared = NetEquality(vocabulary(case)) if service == 'net' else None
    search = Search(tuple(Rule(**r) for r in case['rules']), case['constraints'],
                    case['outputs'], policy, grouping, compare_mode='identity', equality=prepared)
    while not search.exhausted and search.actions < 20_000_000:
        search.advance(quantum)
        if case['prefix'] and search.answers:
            break
    assert search.actions < 20_000_000, (case['id'], service, policy, 'action cap')
    assert search.exhausted != case['prefix'], (case['id'], 'exhaustion')
    assert search.raw == case['raw'], (case['id'], 'raw', search.raw)
    for key in ('expected', 'reference'):
        if key not in case:
            continue
        assert len(search.answers) == len(case[key]), (case['id'], key, 'count')
        assert all(any(equivalent(a,b) for b in case[key]) for a in search.answers), (case['id'], key)
    return dict(id=case['id'], service=service, policy=policy, grouping=grouping,
                quantum=quantum, actions=dict(search.counts), answers=search.answers,
                answer_actions=search.answer_actions, source_jobs=search.source_jobs,
                logical_steps=search.logical_steps, raw=search.raw, unique=len(search.answers),
                failed=search.failed, prefix=case['prefix'], peak_jobs=search.peak_jobs,
                peak_ready=search.peak_ready, peak_observations=search.peak_observations,
                status='pass')


if __name__ == '__main__':
    cases = [freeze(json.loads(line)) for line in sys.stdin]
    assert len(cases) == 64
    jobs = []
    for c in cases:
        if c['exhausted']:
            jobs.append((dict(c, prefix=False), (8,), 'conformance'))
    for c in workloads():
        c = dict(c, rules=tuple(asdict(r) for r in c['rules']))
        jobs.append((c, (1,8,64), 'controlled'))
    configs = [('fifo', False), ('round', False), ('round', True), ('async', False), ('async', True)]
    for case, quanta, group in jobs:
        for service in ('direct','net'):
            for policy, grouping in configs:
                for quantum in quanta:
                    try:
                        row = run(case, service, policy, grouping, quantum)
                    except Exception as error:
                        print(json.dumps(dict(id=case['id'], service=service, policy=policy,
                                              grouping=grouping, quantum=quantum, status='error', detail=repr(error))), flush=True)
                        raise
                    row['group'] = group
                    print(json.dumps(row, sort_keys=True), flush=True)
