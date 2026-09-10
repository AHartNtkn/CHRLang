"""Complete expectations from chain length and independent choice assignments."""
import itertools

def case(n, noise, choices, late, failure, base):
    root = base
    selected = [root] + [base + 64 + j for j in range(choices - 1)]
    rules = [
        dict(kept=[['ready', []]], removed=[['pick', [0]]],
             body=[['or', [['eq', 0, 'a']], [['fail']] if failure else [['eq', 0, 'b']]]]),
        dict(kept=[['watch', [0]], ['cursor', [0]]], removed=[],
             body=[['add', ['seen', [0]]]]),
        dict(kept=[], removed=[['cursor', [0]], ['edge', [0, 1]]],
             body=[['eq', 0, 1], ['add', ['cursor', [1]]]]),
        dict(kept=[], removed=[['cursor', [0]], ['finish', [0]]],
             body=[['add', ['ready', []]]] if late else []),
    ]
    distractors = [['edge', [f'z{i}', f'z{i}']] for i in range(noise)]
    query = [['pick', [v]] for v in selected]
    if not late:
        query += [['ready', []]]
    query += [['watch', [root]], ['cursor', [root]]]
    query += distractors + [['edge', [base + i, base + i + 1]] for i in range(n)]
    query += [['finish', [base + n]]]
    expected = []
    for assignment in itertools.product(['a'] if failure else ['a', 'b'], repeat=choices):
        expected.append([list(assignment), [['ready', []], ['watch', [assignment[0]]]] +
                         [['seen', [assignment[0]]] for _ in range(n + 1)] + distractors])
    return dict(name=f'n{n}-noise{noise}-k{choices}-late{int(late)}-fail{int(failure)}-id{base}',
                rules=rules, query=query, outputs=selected, expected=expected, ongoing=False,
                parameters=dict(chain=n, noise=noise, choices=choices, late=late, failure=failure, base=base))

def cases():
    for args in itertools.product([0, 2, 8], [0, 8], [1, 3], [False, True], [False, True], [10, 1000]):
        yield case(*args)
