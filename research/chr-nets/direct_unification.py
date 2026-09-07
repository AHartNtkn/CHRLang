"""Direct immutable-term controls with map, table and encoded-table lookups."""
from check_unification import TAGS, encoded, items, number
from measure import nat


def freeze(t):
    return t if isinstance(t, int) else (t[0], tuple(freeze(c) for c in t[1]))


def thaw(t):
    return t if isinstance(t, int) else [t[0], [thaw(c) for c in t[1]]]


def decoded(t):
    if t[0] == 'Ref':
        return number(t[1][0])
    assert t[0] == 'App'
    tag, children = t[1]
    return [TAGS[number(tag)], [decoded(c) for c in items(children)]]


def extract_tables(original, result, selected):
    if result is None:
        return original, None, []
    env = dict(result)
    def project(t, ancestors):
        if isinstance(t, int):
            if t not in env:
                return t
            assert t not in ancestors, 'cyclic result'
            return project(env[t], ancestors | {t})
        return [t[0], [project(c, ancestors) for c in t[1]]]
    return original, result, [project(v, set()) for v in selected]


class Direct:
    def __init__(self, initial, equations, mode):
        assert mode in ('map', 'table', 'encoded')
        self.mode = mode
        self.stats = dict(pairs=0, dereferences=0, entries=0, id_nodes=0, occurs=0, child_links=0, bindings=0)
        prepare = encoded if mode == 'encoded' else freeze
        key = nat if mode == 'encoded' else lambda x: x
        self.original = [(key(k), prepare(v)) for k, v in initial]
        self.order = [k for k, _ in self.original]
        self.env = dict(self.original) if mode == 'map' else list(self.original)
        self.pending = [(prepare(a), prepare(b)) for a, b in reversed(equations)]

    def isvar(self, term):
        return term[0] == 'Ref' if self.mode == 'encoded' else isinstance(term, int)

    def var(self, term):
        return term[1][0] if self.mode == 'encoded' else term

    def same_id(self, a, b):
        if self.mode != 'encoded':
            self.stats['id_nodes'] += 1
            return a == b
        while True:
            self.stats['id_nodes'] += 1
            if a[0] != b[0]:
                return False
            if a[0] == 'Z':
                return True
            assert a[0] == 'S'
            a, b = a[1][0], b[1][0]

    def lookup(self, key):
        if self.mode == 'map':
            self.stats['entries'] += 1
            return self.env.get(key)
        for k, value in self.env:
            self.stats['entries'] += 1
            if self.same_id(k, key):
                return value
        return None

    def deref(self, term):
        while self.isvar(term):
            self.stats['dereferences'] += 1
            value = self.lookup(self.var(term))
            if value is None:
                break
            term = value
        return term

    def fields(self, term):
        if self.mode != 'encoded':
            self.stats['child_links'] += len(term[1])
            return term[0], term[1]
        tag, pending = term[1]
        children = []
        while pending[0] == 'Cons':
            self.stats['child_links'] += 1
            head, pending = pending[1]
            children.append(head)
        assert pending[0] == 'Nil'
        return tag, children

    def contains(self, key, term):
        pending = [term]
        while pending:
            self.stats['occurs'] += 1
            term = self.deref(pending.pop())
            if self.isvar(term):
                if self.same_id(key, self.var(term)):
                    return True
            else:
                # Match the net's first-child-first occurs traversal.
                pending.extend(reversed(self.fields(term)[1]))
        return False

    def solve(self):
        while self.pending:
            self.stats['pairs'] += 1
            a, b = self.pending.pop()
            a, b = self.deref(a), self.deref(b)
            if self.isvar(a) or self.isvar(b):
                if not self.isvar(a):
                    a, b = b, a
                key = self.var(a)
                if self.isvar(b):
                    if self.same_id(key, self.var(b)):
                        continue
                elif self.contains(key, b):
                    return False
                if self.mode == 'map':
                    self.env[key] = b
                else:
                    self.env.insert(0, (key, b))
                self.order.insert(0, key)
                self.stats['bindings'] += 1
            else:
                tag_a, children_a = self.fields(a)
                tag_b, children_b = self.fields(b)
                same = self.same_id(tag_a, tag_b) if self.mode == 'encoded' else tag_a == tag_b
                if not same or len(children_a) != len(children_b):
                    return False
                # Net Zip accumulates equations in reverse child order.
                self.pending.extend(zip(children_a, children_b))
        return True

    def extract(self, success, selected):
        entries = [(k, self.env[k]) for k in self.order] if self.mode == 'map' else self.env
        def export(source):
            if self.mode == 'encoded':
                return [[number(k), decoded(v)] for k, v in source]
            return [[k, thaw(v)] for k, v in source]
        return extract_tables(export(self.original), export(entries) if success else None, selected)
