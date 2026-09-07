"""Validate the full registered matrix and summarize untraced session costs."""
import json
from pathlib import Path
import statistics
import sys
from run_net_measurements import signature
from run_measurements import cases
from check_net_source import equivalent


def key(row):
    return row['id'], row['service'], row['policy'], row['grouping']


def measures(row):
    prep = sum(row['preparation_ns'].values())
    queries = row['queries']
    totals = [sum(q['timings_ns'][name] for name in ('initialize','evaluate','serialize','release'))
              for q in queries]
    return dict(prepare=prep, cold_total=prep+totals[0],
                cold_first=(prep+queries[0]['timings_ns']['first_answer']
                            if queries[0]['timings_ns']['first_answer'] is not None else None),
                prepared_query_2=totals[1], prepared_query_3=totals[2],
                prepared_first_2=queries[1]['timings_ns']['first_answer'],
                prepared_first_3=queries[2]['timings_ns']['first_answer'],
                session_total=prep+sum(totals), amortized_query=(prep+sum(totals))/3,
                process=row['process_ns'])


def summarize(time_path, memory_path, fixtures, diagnostic_path):
    oracles = {c['id']: c for c in cases(fixtures)}
    timed = [json.loads(l) for l in Path(time_path).read_text().splitlines()]
    memory = [json.loads(l) for l in Path(memory_path).read_text().splitlines()]
    assert len(timed) == 810 and len(memory) == 270
    assert all(r['warmup'] for r in timed[:135])
    assert all(not r['warmup'] for r in timed[135:])
    diagnostic = [json.loads(l) for l in Path(diagnostic_path).read_text().splitlines()]
    assert len(diagnostic) == 810 and all(r['status'] == 'pass' for r in diagnostic)
    controls = {key(r):signature(r['queries'][0]) for r in diagnostic}
    expected_configs = {(name, service, 'async', False) for name in oracles
                        for service in ('direct','scan','count')}
    expected_configs |= {(name, service, policy, grouping)
                         for name in ('lag-64','duplicates-6','opaque-64','recursive-64','app-sk-identity')
                         for service in ('direct','scan','count')
                         for policy, grouping in (('fifo',False),('round',False),('round',True),('async',True))}
    assert len(expected_configs) == 135
    assert {key(r) for r in timed} == expected_configs
    assert {key(r) for r in timed} == {key(r) for r in memory}
    result = []
    for config in sorted({key(r) for r in timed}):
        times = [r for r in timed if key(r) == config]
        memories = [r for r in memory if key(r) == config]
        assert sorted(r['repeat'] for r in times) == list(range(6))
        assert sorted(r['repeat'] for r in memories) == [0,1]
        baseline = signature(times[0]['queries'][0])
        assert baseline == controls[config], 'engine work changed across measurement correction'
        for row in times+memories:
            assert row['status'] == 'pass'
            for output in row['outputs']:
                answers = json.loads(output)
                for name in ('expected', 'reference'):
                    if name not in oracles[row['id']]:
                        continue
                    expected = oracles[row['id']][name]
                    assert len(answers) == len(expected)
                    assert all(any(equivalent(a,b) for b in expected) for a in answers)
                    assert all(any(equivalent(a,b) for a in answers) for b in expected)
            assert len(row['queries']) == 3 and len(set(row['outputs'])) == 1
            assert all(signature(q) == baseline for q in row['queries'])
        assert all(not r['traced'] and r['warmup'] == (r['repeat'] == 0) for r in times)
        assert all(r['traced'] and not r['warmup'] for r in memories)
        phases = {'rules','backend'} | {f'{i}.{phase}' for i in range(3)
                                      for phase in ('initialized','evaluated','serialized','released','validated')}
        for row in memories:
            assert set(row['memory']) == phases
            assert all(0 <= point['current'] <= point['peak'] for point in row['memory'].values())
        for row in times+memories:
            for query in row['queries']:
                t = query['timings_ns']
                assert t['release'] == t['drop']+t['collect']
        samples = [measures(r) for r in times if not r['warmup']]
        metrics = {}
        for name in samples[0]:
            values = [s[name] for s in samples]
            if all(v is None for v in values):
                metrics[name] = None  # An exhausted failure has no first answer.
            else:
                assert all(v is not None for v in values)
                metrics[name] = dict(median=statistics.median(values), min=min(values), max=max(values))
        result.append(dict(id=config[0], service=config[1], policy=config[2], grouping=config[3],
                           timings_ns=metrics, memory=[r['memory'] for r in memories],
                           rss_kib=[r['rss_kib'] for r in memories], work=baseline))
    return result


if __name__ == '__main__':
    print(json.dumps(summarize(sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]), indent=2, sort_keys=True))
