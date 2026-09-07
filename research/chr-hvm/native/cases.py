"""Finite source-choice probes with explicit intended answer multisets."""
from itertools import product


def atom(name):
    return '#' + name


def pair(a, b):
    return f'#Pair{{{a},{b}}}'


def sequence(values):
    result = '#Nil'
    for value in reversed(values):
        result = f'#Cons{{{value},{result}}}'
    return result


MAKE = '''
@make = λ{
  0: λb. #Nil;
  λn. λb. !N&(1000)=n; !B&(1001)=b;
    #Cons{&(B₀+N₀){#A,#B},@make(N₁-1,B₁)}
}
'''


def cases():
    aa, ab, ba, bb = [pair(a, b) for a, b in product(['#A', '#B'], repeat=2)]
    entries = [
        ('copy-distinct-label', '!X&(1000)=&(1){#A,#B};#Pair{X₀,X₁}', [aa, bb], [aa, bb], [1], [1000]),
        ('copy-same-label', '!X&(1)=&(1){#A,#B};#Pair{X₀,X₁}', [aa, bb], [ab], [1], [1]),
        ('independent-births', '#Pair{&(1){#A,#B},&(2){#A,#B}}', [aa, ab, ba, bb], [aa, ab, ba, bb], [1, 2], []),
        ('reused-birth-label', '#Pair{&(1){#A,#B},&(1){#A,#B}}', [aa, ab, ba, bb], [aa, bb], [1, 1], []),
        ('wrapped-birth-label', '#Pair{&(1){#A,#B},&(16777217){#A,#B}}', [aa, ab, ba, bb], [aa, bb], [1, 16777217], []),
        ('wrapped-copy-label', '!X&(16777217)=&(1){#A,#B};#Pair{X₀,X₁}', [aa, bb], [ab], [1], [16777217]),
        ('duplicate-answer', '&(1){#A,#A}', ['#A', '#A'], ['#A', '#A'], [1], []),
        ('lambda-copy', '!F&(1000)=λx.#Wrap{x};#Pair{F₀(#A),F₁(#B)}',
         [pair('#Wrap{#A}', '#Wrap{#B}')], [pair('#Wrap{#A}', '#Wrap{#B}')], [], [1000]),
        ('off-output-failure', '!unused = &{};#A', [], ['#A'], [], []),
        ('joined-failure', '#Join{#A,&{}}', [], [], [], []),
        ('guarded-branch', '!X&(1000)=&(1){#A,#B};@guard(X₀,#Pair{X₁,#K})',
         [pair('#A', '#K')], [pair('#A', '#K')], [1], [1000]),
        ('sequential-reuse', '(λx.#Pair{x,&(1){#A,#B}})(&(1){#A,#B})',
         [aa, ab, ba, bb], [aa, bb], [1, 1], []),
        ('sequential-fresh', '(λx.#Pair{x,&(2){#A,#B}})(&(1){#A,#B})',
         [aa, ab, ba, bb], [aa, ab, ba, bb], [1, 2], []),
    ]
    for name, expression, expected, prediction, births, copies in entries:
        yield dict(id=name, expression=expression, source_expected=expected,
                   prediction=prediction, birth_labels=births, copy_labels=copies,
                   limit=32, timeout=False, graph_trace=True)
    for depth in (1, 2, 3):
        values = [sequence(v) for v in product(['#A', '#B'], repeat=depth)]
        yield dict(id=f'recursive-{depth}', expression=f'@make({depth},0)', source_expected=values,
                   prediction=values, birth_labels=list(range(1, depth + 1)), copy_labels=[1000,1001],
                   limit=32, timeout=False, graph_trace=True)
    values = [sequence(v) for v in product(['#A', '#B'], repeat=2)]
    cross = [pair(a, b) for a, b in product(values, repeat=2)]
    for name, second_base, prediction in [('two-calls-reuse', 0, [pair(a, a) for a in values]),
                                          ('two-calls-fresh', 2, cross)]:
        yield dict(id=name, expression=f'#Pair{{@make(2,0),@make(2,{second_base})}}',
                   source_expected=cross, prediction=prediction, birth_labels=[1,2,second_base+1,second_base+2],
                   copy_labels=[1000,1001], limit=32, timeout=False, graph_trace=True)
    yield dict(id='divergent-first', expression='&(1){@spin(#Loop),#Done{#A}}',
               source_expected=['#Done{#A}'], prediction=None, birth_labels=[1], copy_labels=[],
               limit=1, timeout=True, graph_trace=False)
    yield dict(id='finite-first', expression='&(1){#Done{#A},@spin(#Loop)}',
               source_expected=['#Done{#A}'], prediction=['#Done{#A}'], birth_labels=[1], copy_labels=[],
               limit=1, timeout=False, graph_trace=False)
    yield dict(id='data-continuations', expression='&(1){#More{#Loop},#Done{#A}}',
               source_expected=['#More{#Loop}', '#Done{#A}'], prediction=['#More{#Loop}', '#Done{#A}'],
               birth_labels=[1], copy_labels=[], limit=32, timeout=False, graph_trace=True)


def program(case):
    return MAKE + '\n@guard = λ{#A:λx.x;#B:λx.&{}}\n@spin = λx.@spin(x)\n@main = ' + case['expression'] + '\n'
