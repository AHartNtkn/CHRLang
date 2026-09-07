"""Prospective cost grid with hand-derived finite answers."""
import json
from pathlib import Path


def nest(depth, term):
    for _ in range(depth):
        term = ['f', [term]]
    return term


def cases():
    source = Path(__file__).resolve().parents[2] / 'docs/experiments/results/E06-unification-reference.jsonl'
    selected = {f'{e}-{a}-{b}' for e in range(3) for a, b in [(2, 3), (9, 8), (10, 2), (5, 6)]}
    for line in source.read_text().splitlines():
        row = json.loads(line)
        if row['id'] in selected:
            yield dict(id='reference-' + row['id'], initial=row['initial'],
                       equations=[row['equation']], selected=list(range(3)), expected=row['answers'])
    a, b = ['a', []], ['b', []]
    for n in (0, 1, 4, 16):
        for d in (0, 4, 16):
            left, right = nest(d, a), nest(d, b)
            padding = [[i, right] for i in range(1, n + 1)]
            chain = [[i, i + 1] for i in range(n)]
            for kind in ('fresh', 'late-clash', 'occurs', 'chain', 'chain-occurs', 'identity', 'decompose'):
                initial, chosen = padding, list(range(n + 1))
                if kind == 'fresh':
                    equations, expected = [[0, left]], [[left] + [right] * n]
                elif kind == 'late-clash':
                    equations, expected = [[0, left], [0, right]], []
                elif kind == 'occurs':
                    equations, expected = [[0, nest(d + 1, 0)]], []
                elif kind == 'chain':
                    initial = chain
                    equations, expected = [[0, left]], [[left] * (n + 1)]
                elif kind == 'chain-occurs':
                    initial = chain
                    equations, expected = [[n, nest(d + 1, 0)]], []
                elif kind == 'identity':
                    equations, expected = [[0, 0]], [[0] + [right] * n]
                else:
                    initial = [[i, right] for i in range(2, n + 2)]
                    chosen = list(range(n + 2))
                    equations = [[['p', [nest(d, 0), nest(d, 1)]], ['p', [left, right]]]]
                    expected = [[a, b] + [right] * n]
                yield dict(id=f'{kind}-n{n}-d{d}', initial=initial, equations=equations,
                           selected=chosen, expected=expected)
