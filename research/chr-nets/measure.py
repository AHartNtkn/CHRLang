"""Instrumented lookup probe. Workload construction is outside traced phases."""
import argparse
import json
import time
import tracemalloc
from net import DATA, Net, data_system, encode, read


def nat(n):
    result = ('Z', ())
    for _ in range(n):
        result = ('S', (result,))
    return result


def workload(v, depth, position):
    entries = []
    for i in range(v):
        value = ('Ref', (nat(i),))
        for _ in range(depth):
            value = ('S', (value,))
        entries.append((nat(i), value))
    key = nat(0 if position == 'first' else max(0, v - 1) if position == 'last' else v)
    table = ('Nil', ())
    for k, value in reversed(entries):
        table = ('Cons', (('Pair', (k, value)), table))
    expected = next((('Some', (value,)) for k, value in entries if k == key), ('None', ()))
    return table, key, expected


def materialize(tree):
    return tree[0], tuple(materialize(child) for child in tree[1])


def size(tree):
    return 1 + sum(size(child) for child in tree[1])


def direct(table, key):
    entries = pairs = 0
    while table[0] == 'Cons':
        entry, table = table[1]
        k, value = entry[1]
        entries += 1
        pending = [(k, key)]
        equal = True
        while pending:
            a, b = pending.pop()
            pairs += 1
            if a[0] != b[0] or len(a[1]) != len(b[1]):
                equal = False
                break
            pending.extend(zip(a[1], b[1]))
        if equal:
            return ('Some', (value,)), entries, pairs
    assert table[0] == 'Nil'
    return ('None', ()), entries, pairs


def measure(v, depth, position, mode):
    table, key, expected = workload(v, depth, position)
    result = dict(v=v, depth=depth, position=position, mode=mode,
                  input_nodes=size(table) + size(key))
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

        def construct():
            net = Net(system)
            kept, output = net.node('Out'), net.node('Out')
            dup, look = net.node('Dup'), net.node('Look')
            net.connect((dup, 0), (encode(net, table), 0))
            net.connect((dup, 1), (kept, 0))
            net.connect((dup, 2), (look, 0))
            net.connect((look, 1), (encode(net, key), 0))
            net.connect((look, 2), (output, 0))
            return net, kept, output

        net, kept, output = phase('initialize', construct)
        status = phase('execute', lambda: net.advance(200000))
        if status != 'quiescent':
            result.update(status='unfinished', interactions=net.interactions)
            tracemalloc.stop()
            return result
        observed = phase('extract', lambda: (read(net, kept), read(net, output)))
        result.update(interactions=net.interactions, by_rule=net.by_rule,
                      created=net.created, live=net.live, peak_live=net.peak_live,
                      retained_slots=len(net.nodes), rules=len(system.rules),
                      agent_types=len(system.arities),
                      entries=net.by_rule.get('Entry/Pair', 0))
    else:
        phase('compile', lambda: None)
        data, query = phase('initialize', lambda: (materialize(table), materialize(key))
                            if mode == 'copied' else (table, key))
        answer, entries, pairs = phase('execute', lambda: direct(data, query))
        observed = phase('extract', lambda: (materialize(data), materialize(answer)))
        result.update(entries=entries, pairs=pairs)
    tracemalloc.stop()
    result.update(status='complete', passed=observed == (table, expected),
                  output_nodes=sum(size(x) for x in observed))
    assert result['passed']
    return result


if __name__ == '__main__':
    p = argparse.ArgumentParser()
    p.add_argument('v', type=int, choices=[0, 1, 4, 16, 64])
    p.add_argument('depth', type=int, choices=[0, 8, 64])
    p.add_argument('position', choices=['first', 'last', 'absent'])
    p.add_argument('mode', choices=['net', 'borrowed', 'copied'])
    a = p.parse_args()
    print(json.dumps(measure(a.v, a.depth, a.position, a.mode), sort_keys=True))
