"""Compare net MGUs against separately exported reference-interpreter answers."""
import argparse
import json
from pathlib import Path
from measure import nat
from net import read, unification

TAGS = ['a', 'b', 'f', 'p']


def listing(items):
    result = ('Nil', ())
    for item in reversed(items):
        result = ('Cons', (item, result))
    return result


def encoded(term):
    if isinstance(term, int):
        return 'Ref', (nat(term),)
    return 'App', (nat(TAGS.index(term[0])), listing([encoded(t) for t in term[1]]))


def number(term):
    n = 0
    while term[0] == 'S':
        n += 1
        term = term[1][0]
    assert term == ('Z', ())
    return n


def items(term):
    out = []
    while term[0] == 'Cons':
        head, term = term[1]
        out.append(head)
    assert term == ('Nil', ())
    return out


def canonical(terms):
    holes = {}
    def visit(t):
        if isinstance(t, int):
            return holes.setdefault(t, len(holes))
        return [t[0], [visit(c) for c in t[1]]]
    return [visit(t) for t in terms]


def observation(option):
    if option == ('None', ()):
        return []
    assert option[0] == 'Some'
    env = {}
    for pair in items(option[1][0]):
        assert pair[0] == 'Pair'
        key, value = pair[1]
        key = number(key)
        assert key not in env
        env[key] = value
    def project(t, ancestors):
        if t[0] == 'Ref':
            key = number(t[1][0])
            if key not in env:
                return key
            assert key not in ancestors, 'cyclic binding escaped'
            return project(env[key], ancestors | {key})
        assert t[0] == 'App'
        tag, children = t[1]
        return [TAGS[number(tag)], [project(c, ancestors) for c in items(children)]]
    return [canonical([project(('Ref', (nat(i),)), set()) for i in range(3)])]


def check(source, output):
    passed = True
    with output.open('w') as out:
        for line in source.read_text().splitlines():
            case = json.loads(line)
            table = listing([('Pair', (nat(k), encoded(v))) for k, v in case['initial']])
            pending = listing([('Pair', tuple(encoded(t) for t in case['equation']))])
            expected = [canonical(answer) for answer in case['answers']]
            for newest in (False, True):
                net, original, result = unification(table, pending)
                net.check()
                status = 'more'
                for _ in range(3125):
                    status = net.advance(32, newest)
                    net.check()
                    if status == 'quiescent':
                        break
                observed = observation(read(net, result)) if status == 'quiescent' else None
                ok = status == 'quiescent' and read(net, original) == table and observed == expected
                passed &= ok
                row = dict(id=case['id'], newest=newest, status=status, passed=ok,
                           observed=observed, interactions=net.interactions,
                           created=net.created, peak_live=net.peak_live, live=net.live)
                out.write(json.dumps(row, sort_keys=True) + '\n')
    return passed


if __name__ == '__main__':
    p = argparse.ArgumentParser()
    p.add_argument('reference', type=Path)
    p.add_argument('output', type=Path)
    a = p.parse_args()
    if not check(a.reference, a.output):
        raise SystemExit('net/reference unification mismatch; inspect results')
