"""Registered full-service comparison; every public result is fully materialized."""
import argparse
import json
import time
import tracemalloc
from check_unification import canonical, encoded, listing, items, number
from direct_unification import Direct, decoded, extract_tables
from measure import nat
from net import Net, data_system, encode, read
from unification_workloads import cases


def measure(case, mode):
    result = dict(id=case['id'], mode=mode)
    expected = [canonical(a) for a in case['expected']]
    tracemalloc.start()
    def phase(name, action):
        baseline = tracemalloc.get_traced_memory()[0]
        tracemalloc.reset_peak()
        start = time.perf_counter_ns()
        value = action()
        elapsed = (time.perf_counter_ns() - start) / 1000
        current, peak = tracemalloc.get_traced_memory()
        result[name] = dict(us=elapsed, baseline=baseline, current=current, peak=peak)
        return value
    if mode == 'net':
        system = phase('compile', data_system)
        def initialize():
            net = Net(system)
            table = listing([('Pair', (nat(k), encoded(v))) for k, v in case['initial']])
            equations = listing([('Pair', (encoded(a), encoded(b))) for a, b in case['equations']])
            original, output = net.node('Out'), net.node('Out')
            dup, controller = net.node('Dup'), net.node('U')
            net.connect((dup, 0), (encode(net, table), 0))
            net.connect((dup, 1), (original, 0))
            net.connect((dup, 2), (controller, 1))
            net.connect((controller, 0), (encode(net, equations), 0))
            net.connect((controller, 2), (output, 0))
            return net, original, output
        net, original, output = phase('initialize', initialize)
        status = phase('execute', lambda: net.advance(200000))
        if status != 'quiescent':
            tracemalloc.stop()
            result.update(status='unfinished', interactions=net.interactions)
            return result
        def extract():
            before, option = read(net, original), read(net, output)
            def table(data):
                return [[number(pair[1][0]), decoded(pair[1][1])] for pair in items(data)]
            after = None if option[0] == 'None' else table(option[1][0])
            return extract_tables(table(before), after, case['selected'])
        before, after, values = phase('extract', extract)
        tracemalloc.stop()
        result.update(interactions=net.interactions, by_rule=net.by_rule,
                      created=net.created, live=net.live, peak_live=net.peak_live,
                      slots=len(net.nodes), rules=len(system.rules), types=len(system.arities))
    else:
        phase('compile', lambda: None)
        solver = phase('initialize', lambda: Direct(case['initial'], case['equations'], mode))
        success = phase('execute', solver.solve)
        before, after, values = phase('extract', lambda: solver.extract(success, case['selected']))
        tracemalloc.stop()
        result.update(work=solver.stats)
    actual = [] if after is None else [canonical(values)]
    passed = before == case['initial'] and actual == expected
    result.update(status='complete', passed=passed, answer=actual,
                  result_bindings=0 if after is None else len(after))
    assert passed, case['id']
    return result


if __name__ == '__main__':
    p = argparse.ArgumentParser()
    p.add_argument('case')
    p.add_argument('mode', choices=['map', 'table', 'encoded', 'net'])
    a = p.parse_args()
    case = next(c for c in cases() if c['id'] == a.case)
    print(json.dumps(measure(case, a.mode), sort_keys=True))
