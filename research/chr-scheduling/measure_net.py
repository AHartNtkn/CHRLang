"""Fresh-process E15 service/session measurement; checks outside measured phases."""
import gc
import json
import resource
import sys
import time
import tracemalloc
from source import Rule
from scheduler import Search
from net_bridge import NetEquality
from check_net_source import vocabulary
from check_cases import freeze, equivalent


def validate_output(serialized, oracle):
    """Check delivered data only after timed search release."""
    answers = json.loads(serialized)
    for key, expected in oracle.items():
        assert len(answers) == len(expected), key
        assert all(any(equivalent(a,b) for b in expected) for a in answers), key
        assert all(any(equivalent(a,b) for a in answers) for b in expected), key


def measure(request):
    resource.setrlimit(resource.RLIMIT_AS, (1024**3, 1024**3))
    assert request['service'] in ('direct', 'scan', 'count')
    assert request['queries'] > 0
    traced = request['traced']
    payload = json.loads(request['case_json'])
    oracle = {key: payload.pop(key) for key in ('expected', 'reference') if key in payload}
    source_json = json.dumps(payload)
    payload = None
    gc.collect()
    if traced:
        tracemalloc.start()
    memory = {}

    def checkpoint(label):
        if traced:
            current, peak = tracemalloc.get_traced_memory()
            memory[label] = dict(current=current, peak=peak)
            tracemalloc.reset_peak()

    start = time.perf_counter_ns()
    case = freeze(json.loads(source_json))
    rules = tuple(Rule(**r) for r in case['rules'])
    decoded = time.perf_counter_ns()
    checkpoint('rules')
    backend_start = time.perf_counter_ns()
    backend = None if request['service'] == 'direct' else NetEquality(
        vocabulary(case), status=request['service'])
    prepared = time.perf_counter_ns()
    checkpoint('backend')
    queries, outputs = [], []
    for index in range(request['queries']):
        begin = time.perf_counter_ns()
        search = Search(rules, case['constraints'], case['outputs'], request['policy'],
                        request['grouping'], compare_mode='identity', equality=backend)
        initialized = time.perf_counter_ns()
        checkpoint(f'{index}.initialized')
        evaluation = time.perf_counter_ns()
        first = None
        while not search.exhausted and search.actions < 20_000_000:
            search.advance(8)
            if first is None and search.answers:
                first = time.perf_counter_ns()-begin
            if case['prefix'] and search.answers:
                break
        evaluated = time.perf_counter_ns()
        checkpoint(f'{index}.evaluated')
        serialization = time.perf_counter_ns()
        output = json.dumps(search.answers, separators=(',', ':'))
        serialized = time.perf_counter_ns()
        checkpoint(f'{index}.serialized')
        exhausted, total_actions = search.exhausted, search.actions
        queries.append(dict(actions=dict(search.counts), raw=search.raw,
                            unique=len(search.answers), source_jobs=search.source_jobs,
                            answer_actions=search.answer_actions,
                            timings_ns=dict(initialize=initialized-begin,
                                            evaluate=evaluated-evaluation,
                                            serialize=serialized-serialization,
                                            first_answer=first)))
        outputs.append(output)
        release_start = time.perf_counter_ns()
        search = None
        dropped = time.perf_counter_ns()
        gc.collect()
        collected = time.perf_counter_ns()
        queries[-1]['timings_ns'].update(release=collected-release_start,
                                           drop=dropped-release_start, collect=collected-dropped)
        # Includes immutable prepared data, outputs, and measurement metadata.
        checkpoint(f'{index}.released')
        assert total_actions < 20_000_000, 'action cap'
        assert exhausted != case['prefix'], 'exhaustion'
        assert queries[-1]['raw'] == case['raw'], 'raw multiplicity'
        validate_output(output, oracle)
        # The independent comparator creates recursive closure cycles. Collect
        # them outside measurement before the next query can inherit that work.
        gc.collect()
        checkpoint(f'{index}.validated')
    if traced:
        tracemalloc.stop()
    return dict(status='pass', id=case['id'], service=request['service'],
                policy=request['policy'], grouping=request['grouping'], traced=traced,
                preparation_ns=dict(rules=decoded-start, backend=prepared-backend_start),
                queries=queries, outputs=outputs, memory=memory,
                rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss)


if __name__ == '__main__':
    print(json.dumps(measure(json.load(sys.stdin)), sort_keys=True))
