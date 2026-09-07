"""Ordinary rules presenting finite free constructors and value equality."""
from kernel import Atom, Rule


def axioms(signature):
    signature = tuple(signature)
    if len(set(signature)) != len(signature) or any(n in {'node', 'eq', 'reach', 'bad'} or k < 0 for n, k in signature):
        raise ValueError('invalid constructor signature')
    def r(premises, conclusions):
        return Rule(tuple(Atom(n, tuple(p)) for n, p in premises),
                    tuple(Atom(n, tuple(p)) for n, p in conclusions))
    rules = [r([('node', ('x',))], [('eq', ('x', 'x'))]),
             r([('eq', ('x', 'y'))], [('eq', ('y', 'x'))]),
             r([('eq', ('x', 'y')), ('eq', ('y', 'z'))], [('eq', ('x', 'z'))]),
             r([('eq', ('x', 'y')), ('reach', ('y', 'z'))], [('reach', ('x', 'z'))]),
             r([('reach', ('x', 'y')), ('eq', ('y', 'z'))], [('reach', ('x', 'z'))]),
             r([('reach', ('x', 'y')), ('reach', ('y', 'z'))], [('reach', ('x', 'z'))]),
             r([('reach', ('x', 'x'))], [('bad', ())])]
    for name, arity in signature:
        xs = tuple('x'+str(i) for i in range(arity))
        ys = tuple('y'+str(i) for i in range(arity))
        desc = [(name, ('r',)+xs), (name, ('s',)+ys)]
        if arity:
            rules.append(r(desc+[('eq', ('r', 's'))], [('eq', (x, y)) for x, y in zip(xs, ys)]))
        rules.append(r(desc+[('eq', (x, y)) for x, y in zip(xs, ys)], [('eq', ('r', 's'))]))
        for x in xs:
            rules.append(r([(name, ('r',)+xs)], [('reach', ('r', x))]))
    for i, (name, arity) in enumerate(signature):
        for other, count in signature[i+1:]:
            rules.append(r([(name, ('r',)+tuple('x'+str(j) for j in range(arity))),
                            (other, ('s',)+tuple('y'+str(j) for j in range(count))),
                            ('eq', ('r', 's'))], [('bad', ())]))
    for name, arity in signature:
        rules.extend(value_transport(name, arity + 1))
    return tuple(rules)


def value_transport(predicate, arity, value_columns=None):
    """Explicit schema: occurrence identity columns must never be transported."""
    columns = range(arity) if value_columns is None else tuple(value_columns)
    rules = []
    ports = tuple('x'+str(i) for i in range(arity))
    for column in columns:
        if column not in range(arity):
            raise ValueError('invalid value column')
        output = list(ports)
        output[column] = 'replacement'
        rules.append(Rule((Atom(predicate, ports), Atom('eq', (ports[column], 'replacement'))),
                          (Atom(predicate, tuple(output)),)))
    return tuple(rules)


def initialize(engine):
    for node in sorted(engine.universe):
        engine.add(Atom('node', (node,)), engine.context_mask)


def extract(engine, signature, roots, context_bit):
    if not engine.complete:
        raise ValueError('answers require completed closure')
    if context_bit <= 0 or context_bit & (context_bit-1) or context_bit & ~engine.context_mask:
        raise ValueError('one explicit context required')
    if any(root not in engine.universe for root in roots):
        raise ValueError('unknown output root')
    if engine.support(Atom('bad', ())) & context_bit:
        return None
    representative = {x: min(y for y in engine.universe
                              if engine.support(Atom('eq', (x, y))) & context_bit)
                      for x in engine.universe}
    descriptors = {}
    for fact, support in engine.facts.items():
        if support & context_bit and (fact.predicate, len(fact.ports)-1) in signature:
            root, *children = fact.ports
            descriptor = (fact.predicate, tuple(representative[x] for x in children))
            key = representative[root]
            if key in descriptors and descriptors[key] != descriptor:
                raise AssertionError('inconsistent descriptors survived closure')
            descriptors[key] = descriptor
    holes = {}
    active = set()
    def visit(node):
        node = representative[node]
        if node in active:
            raise AssertionError('cycle survived closure')
        if node not in descriptors:
            if node not in holes:
                holes[node] = len(holes)
            return holes[node]
        active.add(node)
        name, children = descriptors[node]
        result = (name, tuple(visit(x) for x in children))
        active.remove(node)
        return result
    return tuple(visit(root) for root in roots)
