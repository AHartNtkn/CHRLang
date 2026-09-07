import unittest
from net import DATA, Net, Rule, System, compare, data_system, encode, read


def tree(tag, *children):
    return tag, children


def examples():
    atoms = [tree(n) for n, a in DATA.items() if a == 0]
    values = atoms[:]
    for tag, arity in DATA.items():
        if arity == 1:
            values.extend(tree(tag, x) for x in atoms)
        elif arity == 2:
            values.extend(tree(tag, x, y) for x in atoms for y in atoms)
    # Ref IDs are ordinary encoded data. Copying must retain their values.
    values.extend([tree('Ref', tree('S', tree('S', tree('Z')))),
                   tree('App', tree('S', tree('Z')),
                        tree('Cons', tree('Ref', tree('Z')), tree('Nil')))])
    return values


class Services(unittest.TestCase):
    def finish(self, net, newest=False, quantum=1):
        for _ in range(10000):
            net.check()
            if net.advance(quantum, newest) == 'quiescent':
                net.check()
                return
        self.fail('finite data service did not finish')

    def test_generated_comparisons_and_alternative_reduction_orders(self):
        for a in examples():
            for b in examples():
                for newest in (False, True):
                    net, out = compare(a, b)
                    self.finish(net, newest)
                    self.assertEqual(read(net, out), tree('T' if a == b else 'F'))
                    self.assertEqual(net.live, 2)  # output plus Boolean, all garbage charged

    def test_copy_and_erasure(self):
        for value in examples():
            for newest in (False, True):
                net = Net(data_system())
                dup, a, b = net.node('Dup'), net.node('Out'), net.node('Out')
                net.connect((dup, 0), (encode(net, value), 0))
                net.connect((dup, 1), (a, 0))
                net.connect((dup, 2), (b, 0))
                self.finish(net, newest)
                self.assertEqual(read(net, a), value)
                self.assertEqual(read(net, b), value)
                self.assertNotEqual(net.ports[(a, 0)], net.ports[(b, 0)])
                # An entirely erased request has no source meaning; this tests
                # only explicit data reclamation, not CHR failure.
                net = Net(data_system())
                eraser = net.node('Erase')
                net.connect((eraser, 0), (encode(net, value), 0))
                self.finish(net, newest)
                self.assertEqual(net.live, 0)

    def test_finite_yields_and_exact_replay(self):
        value = tree('Z')
        for _ in range(64):
            value = tree('S', value)
        results = []
        for quantum in (1, 3, 16):
            net, out = compare(value, value)
            self.assertEqual(net.advance(0), 'more')
            with self.assertRaisesRegex(ValueError, 'not quiescent'):
                read(net, out)
            self.finish(net, quantum=quantum)
            results.append((read(net, out), net.interactions, net.by_rule))
        self.assertEqual(results[0], results[1])
        self.assertEqual(results[0], results[2])
        self.assertEqual(results[0][1], 130)  # two inspections per unary node

    def test_composition_copy_then_compare(self):
        for value in examples():
            for newest in (False, True):
                net = Net(data_system())
                dup, eq, out = net.node('Dup'), net.node('Eq'), net.node('Out')
                net.connect((dup, 0), (encode(net, value), 0))
                net.connect((dup, 1), (eq, 0))
                net.connect((dup, 2), (eq, 1))
                net.connect((eq, 2), (out, 0))
                self.finish(net, newest)
                self.assertEqual(read(net, out), tree('T'))
                self.assertEqual(net.live, 2)

    def test_preserved_table_lookup_and_first_match(self):
        from net import lookup, lookup_preserving
        zero, one, two = tree('Z'), tree('S', tree('Z')), tree('S', tree('S', tree('Z')))
        tables = [[], [(zero, tree('Ref', one))],
                  [(zero, tree('Ref', one)), (one, tree('App', zero, tree('Nil')))],
                  [(zero, tree('Ref', one)), (zero, tree('Ref', two))]]
        for entries in tables:
            table = tree('Nil')
            for key, value in reversed(entries):
                table = tree('Cons', tree('Pair', key, value), table)
            for key in (zero, one, two):
                expected = next((tree('Some', value) for k, value in entries if k == key), tree('None'))
                for newest in (False, True):
                    for service in (lookup, lookup_preserving):
                        net, kept, result = service(table, key)
                        self.finish(net, newest)
                        self.assertEqual(read(net, kept), table)
                        self.assertEqual(read(net, result), expected)

    def test_preserving_lookup_yields_and_untouched_payload(self):
        from net import lookup_preserving
        from measure import workload
        for position in ('first', 'last', 'absent'):
            table, key, expected = workload(8, 4, position)
            traces = []
            for quantum in (1, 3, 16):
                net, kept, result = lookup_preserving(table, key)
                original_payloads = {n for n, tag in enumerate(net.nodes) if tag == 'Ref'}
                self.finish(net, quantum=quantum)
                self.assertEqual(read(net, kept), table)
                self.assertEqual(read(net, result), expected)
                surviving = sum(net.nodes[n] == 'Ref' for n in original_payloads)
                # Only the selected value is copied for its two outputs.
                self.assertEqual(surviving, 8 if position == 'absent' else 7)
                traces.append((net.interactions, net.by_rule))
            self.assertEqual(traces[0], traces[1])
            self.assertEqual(traces[0], traces[2])

    def test_direct_controls_and_measured_interface(self):
        from measure import direct, materialize, workload, measure
        for v in (0, 1, 4, 16, 64):
            for depth in (0, 8, 64):
                for position in ('first', 'last', 'absent'):
                    table, key, expected = workload(v, depth, position)
                    self.assertEqual(direct(table, key)[0], expected)
                    self.assertEqual(materialize(table), table)
        for mode in ('net', 'preserving', 'borrowed', 'copied'):
            self.assertTrue(measure(1, 0, 'last', mode)['passed'])

    def test_dereference_aliases_preserves_table_and_yields(self):
        from net import dereference
        from measure import nat
        ref = lambda n: tree('Ref', nat(n))
        app = tree('App', nat(0), tree('Cons', ref(3), tree('Nil')))
        table = tree('Nil')
        for key, value in reversed([(0, ref(1)), (1, ref(2)), (2, app)]):
            table = tree('Cons', tree('Pair', nat(key), value), table)
        for operand, expected in [(ref(0), app), (ref(1), app), (ref(2), app),
                                  (ref(3), ref(3)), (app, app)]:
            for newest in (False, True):
                for quantum in (1, 7):
                    net, kept, result = dereference(table, operand)
                    self.finish(net, newest, quantum)
                    self.assertEqual(read(net, kept), table)
                    self.assertEqual(read(net, result), expected)

    def test_cyclic_dereference_cannot_publish_an_answer(self):
        from net import dereference
        from measure import nat
        ref = tree('Ref', nat(0))
        table = tree('Cons', tree('Pair', nat(0), ref), tree('Nil'))
        net, kept, result = dereference(table, ref)
        self.assertEqual(net.advance(300), 'more')
        net.check()
        with self.assertRaisesRegex(ValueError, 'not quiescent'):
            read(net, result)

    def test_occurs_follows_aliases_and_constructor_children(self):
        from net import occurs
        from measure import nat
        ref = lambda n: tree('Ref', nat(n))
        def app(*args):
            children = tree('Nil')
            for arg in reversed(args):
                children = tree('Cons', arg, children)
            return tree('App', nat(0), children)
        tables = [[], [(0, ref(1))], [(0, app(ref(1), ref(2)))],
                  [(0, ref(1)), (1, app(ref(2)))]]
        operands = [ref(0), ref(1), ref(2), ref(3), app(), app(ref(0)),
                    app(app(ref(0)), ref(3)), app(ref(2), ref(2))]
        def oracle(term, target, env):
            if term[0] == 'Ref':
                ident = term[1][0]
                if ident in env:
                    return oracle(env[ident], target, env)
                return ident == target
            children = term[1][1]
            while children[0] == 'Cons':
                child, children = children[1]
                if oracle(child, target, env):
                    return True
            return False
        for entries in tables:
            table = tree('Nil')
            for key, value in reversed(entries):
                table = tree('Cons', tree('Pair', nat(key), value), table)
            env = {nat(k): value for k, value in entries}
            for target in (2, 3):
                for operand in operands:
                    expected = oracle(operand, nat(target), env)
                    for newest in (False, True):
                        net, kept, result = occurs(table, nat(target), operand)
                        self.finish(net, newest, 3)
                        self.assertEqual(read(net, kept), table)
                        self.assertEqual(read(net, result), tree('T' if expected else 'F'))

    def test_transactional_unification_and_occurs_failures(self):
        from net import unification
        from measure import nat
        ref = lambda n: tree('Ref', nat(n))
        def app(tag, *args):
            children = tree('Nil')
            for arg in reversed(args):
                children = tree('Cons', arg, children)
            return tree('App', nat(tag), children)
        a, b = app(0), app(1)
        cases = [([], [], [ref(0), ref(1), ref(2)]),
                 ([], [(ref(0), a)], [a, ref(1), ref(2)]),
                 ([], [(ref(0), ref(1))], [ref(1), ref(1), ref(2)]),
                 ([], [(app(2, ref(0)), app(2, a))], [a, ref(1), ref(2)]),
                 ([], [(ref(0), app(2, ref(0)))], None),
                 ([], [(ref(0), ref(1)), (ref(1), app(2, ref(0)))], None),
                 ([], [(ref(0), a), (ref(0), b)], None),
                 ([], [(app(2, a), app(2))], None),
                 ([], [(app(2), app(2, a))], None),
                 ([], [(a, b)], None),
                 ([], [(a, ref(0))], [a, ref(1), ref(2)]),
                 ([(0, ref(1))], [(ref(1), a)], [a, a, ref(2)]),
                 ([], [(app(2, ref(0), ref(0)), app(2, a, b))], None)]
        def listing(items):
            value = tree('Nil')
            for item in reversed(items):
                value = tree('Cons', item, value)
            return value
        def project(term, env):
            if term[0] == 'Ref' and term[1][0] in env:
                return project(env[term[1][0]], env)
            return tree(term[0], *(project(c, env) for c in term[1]))
        for bindings, equations, expected in cases:
            table = listing([tree('Pair', nat(k), v) for k, v in bindings])
            pending = listing([tree('Pair', x, y) for x, y in equations])
            for newest in (False, True):
                net, original, result = unification(table, pending)
                self.finish(net, newest, 7)
                self.assertEqual(read(net, original), table)
                answer = read(net, result)
                if expected is None:
                    self.assertEqual(answer, tree('None'))
                else:
                    self.assertEqual(answer[0], 'Some')
                    entries, env = answer[1][0], {}
                    while entries[0] == 'Cons':
                        entry, entries = entries[1]
                        key, value = entry[1]
                        self.assertNotIn(key, env)
                        env[key] = value
                    self.assertEqual([project(ref(i), env) for i in range(3)], expected)

    def test_rule_interface_validation(self):
        with self.assertRaisesRegex(ValueError, 'every interface'):
            System({'C': 1, 'Z': 0}, [Rule('C', 'Z', (), ())])
        valid = Rule('C', 'Z', ('Z',), ((0, (0, 0)),))
        with self.assertRaisesRegex(ValueError, 'overlapping'):
            System({'C': 1, 'Z': 0}, [valid, valid])
        with self.assertRaisesRegex(ValueError, 'every interface'):
            System({'C': 1, 'Z': 0}, [Rule('C', 'Z', ('Z',),
                                                ((0, (0, 0)), (0, (0, 0))))])

    def test_internal_interface_wire(self):
        # Both auxiliary ports can already be connected inside the redex.
        # The rewrite must connect the two new Z nodes, not stale agent ports.
        rule = Rule('C', 'D', ('Z', 'Z'), ((0, (0, 0)), (1, (1, 0))))
        net = Net(System({'C': 1, 'D': 1, 'Z': 0}, [rule]))
        c, d = net.node('C'), net.node('D')
        net.connect((c, 0), (d, 0))
        net.connect((c, 1), (d, 1))
        net.check()
        self.assertTrue(net.step())
        net.check()
        self.assertEqual(net.ports[(2, 0)], (3, 0))


if __name__ == '__main__':
    unittest.main()
