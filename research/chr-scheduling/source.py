"""E09 resumable CHR source control, independent of the reference evaluator.

Finite immutable trees; explicit OR; fixed rule/occurrence order. Action quanta
bound instrumented work, not host allocation latency. Tuple/dict finalization,
permutation advancement and hashing are host operations, not real-time claims.
This version uses direct unification, not the interaction-net service.
"""
from collections import Counter
from dataclasses import dataclass, replace
from itertools import permutations


@dataclass(frozen=True)
class Rule:
    kept: tuple
    removed: tuple
    body: tuple
    guards: tuple = ()


@dataclass(frozen=True)
class State:
    pending: tuple
    outputs: tuple
    next_var: int
    store: tuple = ()
    sub: tuple = ()
    history: frozenset = frozenset()
    next_occurrence: int = 0


def map_term(term, leaf, label):
    """Iterative traversal; leaf may return a bound tree needing traversal."""
    work = [('visit', term)]
    values = []
    while work:
        kind, value = work.pop()
        yield label
        if kind == 'assemble':
            name, count = value
            children = tuple(values[-count:]) if count else ()
            if count:
                del values[-count:]
            values.append((name, children))
        elif isinstance(value, int):
            result, revisit = leaf(value)
            if revisit:
                work.append(('visit', result))
            else:
                values.append(result)
        else:
            work.append(('assemble', (value[0], len(value[1]))))
            for child in reversed(value[1]):
                yield label
                work.append(('visit', child))
    return values[0]


def resolve(term, sub):
    return (yield from map_term(term, lambda v: (sub[v], True) if v in sub else (v, False), 'resolve'))


def equal(a, b):
    todo = [(a, b)]
    while todo:
        a, b = todo.pop()
        yield 'compare'
        if isinstance(a, int) or isinstance(b, int):
            if not isinstance(a, int) or not isinstance(b, int) or a != b:
                return False
        elif a[0] != b[0] or len(a[1]) != len(b[1]):
            return False
        else:
            for pair in zip(a[1], b[1]):
                yield 'compare'
                todo.append(pair)
    return True


class Rename:
    def __init__(self, start=0, bindings=None):
        self.next = start
        self.bindings = {} if bindings is None else bindings

    def leaf(self, v):
        if v not in self.bindings:
            self.bindings[v] = self.next
            self.next += 1
        return self.bindings[v], False

    def term(self, t):
        return (yield from map_term(t, self.leaf, 'rename'))

    def goal(self, goal):
        todo = [('visit', goal)]
        values = []
        while todo:
            action, g = todo.pop()
            yield 'rename'
            if action == 'assemble':
                kind, count = g
                children = tuple(values[-count:]) if count else ()
                if count:
                    del values[-count:]
                values.append((kind, *children))
            elif g[0] in ('and', 'or'):
                todo.append(('assemble', (g[0], len(g)-1)))
                for c in reversed(g[1:]):
                    yield 'rename'
                    todo.append(('visit', c))
            elif g[0] == 'post':
                values.append(('post', (yield from self.term(g[1]))))
            elif g[0] == 'eq':
                a = yield from self.term(g[1])
                b = yield from self.term(g[2])
                values.append(('eq', a, b))
            elif g[0] in ('true', 'fail'):
                values.append(g)
            else:
                raise ValueError(f'unknown goal {g[0]}')
        return values[0]


def drain(generator):
    while True:
        try:
            next(generator)
        except StopIteration as result:
            return result.value


def initial(constraints, outputs):
    # Admission is outside source-step measurements; report separately in runs.
    rename = Rename()
    pending = tuple(('post', drain(rename.term(c))) for c in constraints)
    outputs = tuple(drain(rename.term(t)) for t in outputs)
    return State(pending, outputs, rename.next)


def unify(a, b, sub):
    todo = [(a, b)]
    while todo:
        a, b = todo.pop()
        yield 'equation'
        a = yield from resolve(a, sub)
        b = yield from resolve(b, sub)
        if (yield from equal(a, b)):
            continue
        if isinstance(a, int) or isinstance(b, int):
            v, t = (a, b) if isinstance(a, int) else (b, a)
            pending = [t]
            while pending:
                item = pending.pop()
                yield 'occurs'
                if isinstance(item, int):
                    if item == v:
                        return False
                else:
                    for child in item[1]:
                        yield 'occurs'
                        pending.append(child)
            sub[v] = t
        elif a[0] != b[0] or len(a[1]) != len(b[1]):
            return False
        else:
            for pair in reversed(tuple(zip(a[1], b[1]))):
                yield 'equation'
                todo.append(pair)
    return True


def match(pattern, value, bindings):
    todo = [(pattern, value)]
    while todo:
        p, v = todo.pop()
        yield 'match'
        if isinstance(p, int):
            if p in bindings:
                if not (yield from equal(bindings[p], v)):
                    return False
            else:
                bindings[p] = v
        elif isinstance(v, int) or p[0] != v[0] or len(p[1]) != len(v[1]):
            return False
        else:
            for pair in reversed(tuple(zip(p[1], v[1]))):
                yield 'match'
                todo.append(pair)
    return True


class StepJob:
    def __init__(self, state, rules):
        self.state, self.rules = state, rules
        self.done = False
        self.counts = Counter()
        self.action = None
        self._runner = self._run()

    def advance(self, quantum):
        if quantum <= 0:
            raise ValueError('positive quantum required')
        used = 0
        while not self.done and used < quantum:
            try:
                label = next(self._runner)
            except StopIteration as result:
                self.result = result.value
                self.done = True
                label = 'publish'
            self.counts[label] += 1
            used += 1
        return used

    def observe(self):
        if not self.done:
            raise ValueError('unfinished source step')
        return self.result

    def _run(self):
        s = self.state
        sub = {}
        for v, t in s.sub:
            yield 'copy'
            sub[v] = t
        if s.pending:
            g, *rest = s.pending
            for _ in rest:
                yield 'copy'
            s = replace(s, pending=tuple(rest))
            kind = g[0]
            self.action = (kind,)
            yield 'dispatch'
            if kind == 'post':
                for _ in s.store:
                    yield 'copy'
                s = replace(s, store=s.store + ((s.next_occurrence, g[1]),), next_occurrence=s.next_occurrence+1)
            elif kind == 'eq':
                if not (yield from unify(g[1], g[2], sub)):
                    return ('failed',)
                for _ in sub:
                    yield 'copy'
                s = replace(s, sub=tuple(sub.items()))
            elif kind == 'and':
                for _ in g[1:]:
                    yield 'copy'
                s = replace(s, pending=g[1:] + s.pending)
            elif kind == 'or':
                if len(g) != 3:
                    raise ValueError('binary OR required')
                return ('split', replace(s, pending=(g[1],) + s.pending), replace(s, pending=(g[2],) + s.pending))
            elif kind == 'fail':
                return ('failed',)
            elif kind != 'true':
                raise ValueError(f'unknown goal {kind}')
            return ('continue', s)
        for index, rule in enumerate(self.rules):
            yield 'rule'
            heads = rule.kept + rule.removed
            if not heads:
                raise ValueError('empty rule head')
            for selected in permutations(s.store, len(heads)):
                yield 'tuple'
                ids = tuple(o[0] for o in selected)
                token = (index, ids)
                if token in s.history:
                    continue
                bindings = {}
                matches = True
                for h, (_, t) in zip(heads, selected):
                    value = yield from resolve(t, sub)
                    if not (yield from match(h, value, bindings)):
                        matches = False
                        break
                if not matches:
                    continue
                rename = Rename(s.next_var, bindings)
                guarded = True
                for a, b in rule.guards:
                    a = yield from rename.term(a)
                    b = yield from rename.term(b)
                    a = yield from resolve(a, sub)
                    b = yield from resolve(b, sub)
                    if not (yield from equal(a, b)):
                        guarded = False
                        break
                if not guarded:
                    continue
                body = yield from rename.goal(rule.body)
                removed = set(ids[len(rule.kept):])
                store = []
                for o in s.store:
                    yield 'copy'
                    if o[0] not in removed:
                        store.append(o)
                for _ in s.history:
                    yield 'copy'
                self.action = ('apply', index, ids)
                return ('continue', replace(s, store=tuple(store), history=s.history | {token}, next_var=rename.next, pending=(body,)))
        self.action = ('answer',)
        outputs, residual = [], []
        for t in s.outputs:
            outputs.append((yield from resolve(t, sub)))
        for _, t in s.store:
            residual.append((yield from resolve(t, sub)))
        return ('answer', {'outputs': outputs, 'residual': residual})
