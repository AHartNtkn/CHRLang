"""Finite active-pair graph interpreter; data services, not a CHR engine."""
from collections import deque
from dataclasses import dataclass


@dataclass(frozen=True)
class Rule:
    left: str
    right: str
    nodes: tuple
    # An integer names a free interface port. (node, port) names a RHS port.
    wires: tuple


class System:
    def __init__(self, arities, rules):
        self.arities = dict(arities)
        self.rules = {}
        for rule in rules:
            key = frozenset((rule.left, rule.right))
            if key in self.rules:
                raise ValueError('overlapping active-pair rules')
            # Symmetric self-pairs require an additional symmetry check. This
            # controller/data compiler never emits them.
            if rule.left == rule.right:
                raise ValueError('self-pair rule outside this compiler discipline')
            expected = set(range(arities[rule.left] + arities[rule.right]))
            expected.update((n, p) for n, tag in enumerate(rule.nodes)
                            for p in range(arities[tag] + 1))
            used = [p for wire in rule.wires for p in wire]
            if any(len(wire) != 2 for wire in rule.wires):
                raise ValueError('wire must have two endpoints')
            if len(set(used)) != len(used) or set(used) != expected:
                raise ValueError('rule must use every interface and RHS port once')
            self.rules[key] = rule


class Net:
    def __init__(self, system):
        self.system = system
        self.nodes = []
        self.ports = {}
        self.ready = deque()
        self.interactions = 0
        self.by_rule = {}
        self.created = 0
        self.live = 0
        self.peak_live = 0

    def node(self, tag):
        self.system.arities[tag]
        n = len(self.nodes)
        self.nodes.append(tag)
        self.created += 1
        self.live += 1
        self.peak_live = max(self.peak_live, self.live)
        return n

    def connect(self, a, b):
        assert a != b and a not in self.ports and b not in self.ports
        for n, p in (a, b):
            assert self.nodes[n] is not None
            assert 0 <= p <= self.system.arities[self.nodes[n]]
        self.ports[a] = b
        self.ports[b] = a
        if a[1] == b[1] == 0:
            key = frozenset((self.nodes[a[0]], self.nodes[b[0]]))
            if key in self.system.rules:
                self.ready.append((a[0], b[0]))

    def step(self, newest=False):
        if not self.ready:
            return False
        a, b = self.ready.pop() if newest else self.ready.popleft()
        assert self.ports[(a, 0)] == (b, 0)
        rule = self.system.rules[frozenset((self.nodes[a], self.nodes[b]))]
        if self.nodes[a] != rule.left:
            a, b = b, a
        boundary = [(n, p) for n in (a, b)
                    for p in range(1, self.system.arities[self.nodes[n]] + 1)]
        index = {p: i for i, p in enumerate(boundary)}
        # Glue the RHS interface to the old interface. Internal auxiliary
        # wires can connect two boundary ports; follow these paths explicitly.
        adjacency = {i: [] for i in range(len(boundary))}

        def edge(x, y):
            adjacency.setdefault(x, []).append(y)
            adjacency.setdefault(y, []).append(x)

        for i, p in enumerate(boundary):
            q = self.ports[p]
            if q in index:
                if i < index[q]:
                    edge(i, index[q])
            else:
                edge(i, ('port', q))
        created = [self.node(tag) for tag in rule.nodes]

        def actual(p):
            return p if isinstance(p, int) else ('port', (created[p[0]], p[1]))

        for x, y in rule.wires:
            edge(actual(x), actual(y))
        for n in (a, b):
            for p in range(self.system.arities[self.nodes[n]] + 1):
                endpoint = (n, p)
                other = self.ports.pop(endpoint, None)
                if other is not None:
                    self.ports.pop(other, None)
            self.nodes[n] = None
            self.live -= 1
        seen = set()
        for start in adjacency:
            if isinstance(start, int) or start in seen:
                continue
            assert len(adjacency[start]) == 1
            previous, current = start, adjacency[start][0]
            while isinstance(current, int):
                assert len(adjacency[current]) == 2
                x, y = adjacency[current]
                previous, current = current, y if x == previous else x
            seen.update((start, current))
            self.connect(start[1], current[1])
        self.interactions += 1
        name = rule.left + '/' + rule.right
        self.by_rule[name] = self.by_rule.get(name, 0) + 1
        return True

    def advance(self, quantum, newest=False):
        if quantum < 0:
            raise ValueError('negative quantum')
        for _ in range(quantum):
            if not self.step(newest):
                break
        return 'more' if self.ready else 'quiescent'

    def check(self):
        expected = {(n, p) for n, tag in enumerate(self.nodes) if tag is not None
                    for p in range(self.system.arities[tag] + 1)}
        assert set(self.ports) == expected
        assert all(self.ports[q] == p for p, q in self.ports.items())
        assert self.live == sum(tag is not None for tag in self.nodes)
        # Every reducible active pair has exactly one queue entry.
        actual = {tuple(sorted((a[0], b[0]))) for a, b in self.ports.items()
                  if a[1] == b[1] == 0 and frozenset((self.nodes[a[0]], self.nodes[b[0]]))
                  in self.system.rules}
        queued = [tuple(sorted(pair)) for pair in self.ready]
        assert len(queued) == len(set(queued)) and set(queued) == actual


DATA = {'Z': 0, 'S': 1, 'Nil': 0, 'Cons': 2, 'Ref': 1, 'App': 2,
        'T': 0, 'F': 0, 'Pair': 2, 'Some': 1, 'None': 0}


def data_system():
    arities = dict(DATA, Eq=2, And=2, Dup=2, Erase=0, Out=0, Look=2, Entry=3, Decide=4, Keep=2, KeepEntry=3, KeepDecide=5, Restore=3, Unpack=2)
    arities.update({'Match' + tag: arity + 1 for tag, arity in DATA.items()})
    rules = []
    for tag, arity in DATA.items():
        # Eq(right, result) >< tag(children): inspect the second root next.
        rules.append(Rule('Eq', tag, ('Match' + tag,), tuple(
            [(0, (0, 0)), (1, (0, arity + 1))] +
            [(2 + i, (0, i + 1)) for i in range(arity)])))
        for other, other_arity in DATA.items():
            nodes, wires = [], []

            def new(kind):
                nodes.append(kind)
                return len(nodes) - 1

            if other != tag:
                f = new('F')
                wires.append((arity, (f, 0)))
                for i in list(range(arity)) + list(range(arity + 1, arity + 1 + other_arity)):
                    e = new('Erase')
                    wires.append((i, (e, 0)))
            elif arity == 0:
                t = new('T')
                wires.append((0, (t, 0)))
            elif arity == 1:
                e = new('Eq')
                wires.extend([(0, (e, 0)), (2, (e, 1)), (1, (e, 2))])
            else:
                assert arity == 2
                left, right, conjunction = new('Eq'), new('Eq'), new('And')
                wires.extend([(0, (left, 0)), (3, (left, 1)),
                              (1, (right, 0)), (4, (right, 1)),
                              ((left, 2), (conjunction, 0)),
                              ((right, 2), (conjunction, 1)), (2, (conjunction, 2))])
            rules.append(Rule('Match' + tag, other, tuple(nodes), tuple(wires)))
        # Dup may copy immutable encoded data only, never a controller.
        nodes = (tag, tag) + ('Dup',) * arity
        wires = [(0, (0, 0)), (1, (1, 0))]
        for i in range(arity):
            wires.extend([(2 + i, (2 + i, 0)),
                          ((0, i + 1), (2 + i, 1)),
                          ((1, i + 1), (2 + i, 2))])
        rules.append(Rule('Dup', tag, nodes, tuple(wires)))
        rules.append(Rule('Erase', tag, ('Erase',) * arity,
                          tuple((i, (i, 0)) for i in range(arity))))
    rules.extend([Rule('And', 'T', (), ((0, 1),)),
                  Rule('And', 'F', ('Erase', 'F'), ((0, (0, 0)), (1, (1, 0))))])
    rules.extend([
        # Look(key, result) inspects the association list, consuming its copy.
        Rule('Look', 'Nil', ('Erase', 'None'), ((0, (0, 0)), (1, (1, 0)))),
        Rule('Look', 'Cons', ('Entry',),
             ((2, (0, 0)), (0, (0, 1)), (3, (0, 2)), (1, (0, 3)))),
        # Entry(key, tail, result) inspects Pair(id, value). Keep one key
        # copy for a possible next lookup while Eq consumes the other.
        Rule('Entry', 'Pair', ('Dup', 'Eq', 'Decide'),
             ((0, (0, 0)), ((0, 1), (1, 1)), ((0, 2), (2, 1)),
              (3, (1, 0)), ((1, 2), (2, 0)), (4, (2, 2)),
              (1, (2, 3)), (2, (2, 4)))),
        Rule('Decide', 'T', ('Some', 'Erase', 'Erase'),
             ((1, (0, 1)), (3, (0, 0)), (0, (1, 0)), (2, (2, 0)))),
        Rule('Decide', 'F', ('Look', 'Erase'),
             ((2, (0, 0)), (0, (0, 1)), (3, (0, 2)), (1, (1, 0)))),
    ])
    rules.extend([
        Rule('Keep', 'Nil', ('Erase', 'Pair', 'Nil', 'None'),
             ((0, (0, 0)), (1, (1, 0)), ((1, 1), (2, 0)), ((1, 2), (3, 0)))),
        Rule('Keep', 'Cons', ('KeepEntry',),
             ((2, (0, 0)), (0, (0, 1)), (3, (0, 2)), (1, (0, 3)))),
        Rule('KeepEntry', 'Pair', ('Dup', 'Dup', 'Eq', 'KeepDecide'),
             ((0, (0, 0)), (3, (1, 0)), ((0, 1), (2, 1)),
              ((0, 2), (3, 4)), ((1, 1), (2, 0)), ((1, 2), (3, 1)),
              ((2, 2), (3, 0)), (4, (3, 2)), (1, (3, 3)), (2, (3, 5)))),
        Rule('KeepDecide', 'T', ('Dup', 'Pair', 'Cons', 'Pair', 'Some', 'Erase'),
             ((1, (0, 0)), (0, (3, 1)), (2, (2, 2)), ((2, 1), (3, 0)),
              ((0, 1), (3, 2)), ((0, 2), (4, 1)), ((1, 1), (2, 0)),
              ((1, 2), (4, 0)), (4, (1, 0)), (3, (5, 0)))),
        Rule('KeepDecide', 'F', ('Keep', 'Restore'),
             ((2, (0, 0)), (3, (0, 1)), ((0, 2), (1, 0)),
              (0, (1, 1)), (1, (1, 2)), (4, (1, 3)))),
        Rule('Restore', 'Pair', ('Pair', 'Cons', 'Pair'),
             ((2, (0, 0)), ((0, 1), (1, 0)), ((0, 2), 4),
              ((1, 1), (2, 0)), ((1, 2), 3), ((2, 1), 0), ((2, 2), 1))),
        Rule('Unpack', 'Pair', (), ((0, 2), (1, 3))),
    ])
    return System(arities, rules)


def encode(net, tree):
    tag, children = tree
    assert tag in DATA and len(children) == DATA[tag]
    n = net.node(tag)
    for i, child in enumerate(children, 1):
        net.connect((n, i), (encode(net, child), 0))
    return n


def read(net, output):
    if net.ready:
        raise ValueError('service not quiescent')
    if any(tag is not None and tag not in DATA and tag != 'Out' for tag in net.nodes):
        raise ValueError('stuck service controller')

    def visit(port):
        n, p = net.ports[port]
        tag = net.nodes[n]
        if p != 0 or tag not in DATA:
            raise ValueError('stuck or non-data result')
        return tag, tuple(visit((n, i)) for i in range(1, DATA[tag] + 1))

    return visit((output, 0))


def compare(a, b):
    net = Net(data_system())
    out, eq = net.node('Out'), net.node('Eq')
    net.connect((out, 0), (eq, 2))
    net.connect((eq, 0), (encode(net, a), 0))
    net.connect((eq, 1), (encode(net, b), 0))
    return net, out


def lookup(table, key):
    net = Net(data_system())
    kept, result = net.node('Out'), net.node('Out')
    dup, look = net.node('Dup'), net.node('Look')
    net.connect((dup, 0), (encode(net, table), 0))
    net.connect((dup, 1), (kept, 0))
    net.connect((dup, 2), (look, 0))
    net.connect((look, 1), (encode(net, key), 0))
    net.connect((look, 2), (result, 0))
    return net, kept, result


def lookup_preserving(table, key):
    net = Net(data_system())
    kept, result = net.node('Out'), net.node('Out')
    keep, unpack = net.node('Keep'), net.node('Unpack')
    net.connect((keep, 0), (encode(net, table), 0))
    net.connect((keep, 1), (encode(net, key), 0))
    net.connect((keep, 2), (unpack, 0))
    net.connect((unpack, 1), (kept, 0))
    net.connect((unpack, 2), (result, 0))
    return net, kept, result
