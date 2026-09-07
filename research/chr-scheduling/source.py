"""E09 resumable CHR source control, independent of the reference evaluator.

Finite immutable trees; explicit OR; fixed rule/occurrence order. Action quanta
bound instrumented work, not host allocation latency. Tuple/dict finalization,
permutation advancement and hashing are host operations, not real-time claims.
This version uses direct unification, not the interaction-net service.
"""
from collections import Counter
from dataclasses import dataclass, replace
from itertools import permutations, product


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


def prefix_guards(guards, bindings, sub):
    for a, b in guards:
        pending = [a, b]
        ready = True
        while pending:
            term = pending.pop()
            yield 'guard_dependency'
            if isinstance(term, int):
                if term not in bindings:
                    ready = False
                    break
            else:
                for child in term[1]:
                    yield 'guard_dependency'
                    pending.append(child)
        if not ready:
            continue
        yield 'guard_early'
        # All rule variables are mapped: this renaming allocates no fresh IDs.
        rename = Rename(0, bindings)
        a = yield from rename.term(a)
        b = yield from rename.term(b)
        a = yield from resolve(a, sub)
        b = yield from resolve(b, sub)
        if not (yield from equal(a, b)):
            return False
    return True


def prefix_matches(heads, pools, sub, guards=()):
    """Yield charged actions or complete (occurrences, pattern bindings)."""
    stack = [(0, 0, (), {}, frozenset())]
    while stack:
        depth, cursor, selected, bindings, used = stack.pop()
        yield 'prefix_visit'
        if depth == len(heads):
            yield selected, bindings
            continue
        if cursor == len(pools[depth]):
            continue
        stack.append((depth, cursor+1, selected, bindings, used))
        occurrence = pools[depth][cursor]
        yield 'prefix_candidate'
        if occurrence[0] in used:
            continue
        child = {}
        for k, v in bindings.items():
            yield 'prefix_copy'
            child[k] = v
        value = yield from resolve(occurrence[1], sub)
        if not (yield from match(heads[depth], value, child)):
            continue
        if guards and not (yield from prefix_guards(guards, child, sub)):
            continue
        for _ in selected:
            yield 'prefix_copy'
        for _ in used:
            yield 'prefix_copy'
        stack.append((depth+1, 0, selected+(occurrence,), child, used | {occurrence[0]}))


class StepJob:
    def __init__(self, state, rules, selector="scan", equality=None):
        if selector not in ("scan", "predicate", "prefix", "guard-prefix"):
            raise ValueError("unknown selector")
        self.selector = selector
        self.equality = equality
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
                if self.equality is None:
                    if not (yield from unify(g[1], g[2], sub)):
                        return ('failed',)
                else:
                    for _ in sub:
                        yield 'copy'
                    result = yield from self.equality.solve(tuple(sub.items()), ((g[1], g[2]),))
                    if result is None:
                        return ('failed',)
                    sub = {}
                    for key, value in result:
                        yield 'copy'
                        sub[key] = value
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
            if self.selector in ('predicate', 'prefix', 'guard-prefix'):
                pools = []
                for head in heads:
                    pool = []
                    for occurrence in s.store:
                        yield 'index_visit'
                        root = occurrence[1]
                        while isinstance(root, int) and root in sub:
                            yield 'index_deref'
                            root = sub[root]
                        if isinstance(head, int) or (not isinstance(root, int) and
                                head[0] == root[0] and len(head[1]) == len(root[1])):
                            pool.append(occurrence)
                    pools.append(pool)
                selections = (prefix_matches(heads, pools, sub, rule.guards if self.selector == 'guard-prefix' else ())
                              if self.selector in ('prefix', 'guard-prefix')
                              else ((item, None) for item in product(*pools)))
            else:
                selections = ((item, None) for item in permutations(s.store, len(heads)))
            for entry in selections:
                if isinstance(entry, str):
                    yield entry
                    continue
                selected, bindings = entry
                yield 'tuple'
                if self.selector == 'predicate':
                    seen = set()
                    for occurrence in selected:
                        yield 'index_distinct'
                        if occurrence[0] in seen:
                            break
                        seen.add(occurrence[0])
                    if len(seen) != len(heads):
                        continue
                ids = tuple(o[0] for o in selected)
                token = (index, ids)
                if token in s.history:
                    continue
                if bindings is None:
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
