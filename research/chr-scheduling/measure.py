"""Single fresh-process E09 measurement; JSON request on stdin."""
import gc
import json
import resource
import sys
import time
import tracemalloc
from pathlib import Path
from scheduler import Search
from source import Rule
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'chr-symbolic'))
from check_cases import equivalent, freeze


def measure(request):
    resource.setrlimit(resource.RLIMIT_AS, (1024**3, 1024**3))
    traced = request['traced']
    gc.collect()
    if traced:
        tracemalloc.start()
    memory = {}

    def checkpoint(phase):
        if traced:
            current, peak = tracemalloc.get_traced_memory()
            memory[phase] = dict(current=current, peak=peak)
            tracemalloc.reset_peak()

    start = time.perf_counter_ns()
    case = freeze(json.loads(request['case_json']))
    rules = tuple(Rule(**r) for r in case['rules'])
    search = Search(rules, case['constraints'], case['outputs'], request['policy'],
                    request['grouping'], compare_mode=request['mode'])
    prepared = time.perf_counter_ns()
    checkpoint('prepared')
    first_ns = None
    begin_search = time.perf_counter_ns()
    while not search.exhausted and search.actions < 5_000_000:
        search.advance(8)
        if first_ns is None and search.answers:
            first_ns = time.perf_counter_ns() - start
        if case['prefix'] and search.answers:
            break
    searched = time.perf_counter_ns()
    checkpoint('searched')
    serialization_start = time.perf_counter_ns()
    serialized = json.dumps(search.answers, separators=(',', ':'))
    serialized_end = time.perf_counter_ns()
    checkpoint('serialized')
    # Semantic checks are outside timing and memory phase peaks.
    assert search.actions < 5_000_000, 'action cap'
    assert search.exhausted != case['prefix'], 'exhaustion'
    assert search.raw == case['raw'], 'raw multiplicity'
    assert len(search.answers) == len(case['expected']), 'unique count'
    assert all(any(equivalent(a, b) for b in case['expected']) for a in search.answers), 'expected answers'
    if 'reference' in case:
        assert len(search.answers) == len(case['reference'])
        assert all(any(equivalent(a, b) for b in case['reference']) for a in search.answers)
    result = dict(id=case['id'], policy=request['policy'], grouping=request['grouping'],
                  mode=request['mode'], traced=traced, prefix=case['prefix'], raw=search.raw,
                  unique=len(search.answers), source_jobs=search.source_jobs,
                  actions=dict(search.counts), answer_actions=search.answer_actions,
                  timings_ns=dict(total=serialized_end-start, prepare=prepared-start, search=searched-begin_search,
                                  serialize=serialized_end-serialization_start,
                                  first_answer=first_ns), memory=memory, status='pass')
    answers = search.answers
    search = case = rules = None
    gc.collect()
    if traced:
        # Current includes retained answers/serialized output and measurement metadata.
        memory['released'] = {'current': tracemalloc.get_traced_memory()[0]}
        tracemalloc.stop()
    result['output_bytes'] = len(serialized.encode())
    result['answers'] = answers
    result['rss_kib'] = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    return result


if __name__ == '__main__':
    print(json.dumps(measure(json.load(sys.stdin)), sort_keys=True))
