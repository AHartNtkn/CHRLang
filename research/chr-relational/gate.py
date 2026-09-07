"""Registered 432-recipe semantic gate; no timing/performance comparisons."""
import json
from itertools import product
from time import monotonic
from kernel import Atom, Engine
from theory import axioms, initialize, extract
from oracle import solve

SIGNATURE = (('a', 0), ('b', 0), ('f', 1), ('g', 1), ('pair', 2))
TERMS = (0, 1, 2, ('a', ()), ('b', ()), ('f', (0,)), ('f', (1,)),
         ('f', (('a', ()),)), ('g', (0,)), ('pair', (0, 0)),
         ('pair', (0, 1)), ('f', (('f', (0,)),)))
ENVIRONMENTS = ((), ((0, 1),), ((0, ('f', (1,))),))
RECIPES = tuple(product(range(12), range(12), range(3)))


def run_batch(offset):
    deadline = monotonic() + 30
    recipes = RECIPES[offset:offset+4]
    next_node = 3
    seeds = []
    expected = []
    for bit_index, (left, right, environment) in enumerate(recipes):
        support = 1 << bit_index
        def encode(term):
            nonlocal next_node
            if isinstance(term, int):
                return term
            children = tuple(encode(x) for x in term[1])
            node = next_node
            next_node += 1
            seeds.append((Atom(term[0], (node,)+children), support))
            return node
        equations = ((TERMS[left], TERMS[right]),) + ENVIRONMENTS[environment]
        for a, b in equations:
            seeds.append((Atom('eq', (encode(a), encode(b))), support))
        expected.append(solve(equations, (0, 1, 2)))
    e = Engine(range(next_node), (1 << len(recipes))-1, axioms(SIGNATURE))
    initialize(e)
    for fact, support in seeds:
        e.add(fact, support)
    e.saturate(max_rounds=1000, deadline=deadline)
    actual = [extract(e, SIGNATURE, (0, 1, 2), 1 << i) for i in range(len(recipes))]
    if actual != expected:
        raise AssertionError((offset, recipes, expected, actual))
    if any(e.support(fact) & support != support for fact, support in seeds):
        raise AssertionError('descriptor/equation evidence not retained')
    return {'offset': offset, 'recipes': recipes, 'expected': expected, 'actual': actual,
            'universe': next_node, 'bad_support': e.support(Atom('bad', ())),
            'facts': len(e.facts), 'stats': dict(e.stats)}


if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument('--batch', type=int, choices=range(0, len(RECIPES), 4))
    args = parser.parse_args()
    offsets = range(0, len(RECIPES), 4) if args.batch is None else [args.batch]
    for offset in offsets:
        print(json.dumps(run_batch(offset), sort_keys=True), flush=True)
