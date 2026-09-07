import unittest
from kernel import Atom, Rule, Engine


class GenericRelations(unittest.TestCase):
    def test_join_intersects_contexts_and_repeated_ports_are_equal(self):
        rule = Rule((Atom('left', ('x', 'y')), Atom('right', ('y', 'z'))),
                    (Atom('path', ('x', 'z')),))
        same = Rule((Atom('path', ('x', 'x')),), (Atom('closed', ('x',)),))
        engine = Engine(range(3), 7, (rule, same))
        engine.add(Atom('left', (0, 1)), 3)
        engine.add(Atom('right', (1, 0)), 6)
        engine.add(Atom('right', (2, 0)), 7)
        engine.saturate()
        self.assertEqual(engine.support(Atom('path', (0, 0))), 2)
        self.assertEqual(engine.support(Atom('closed', (0,))), 2)
        self.assertEqual(engine.support(Atom('path', (0, 2))), 0)

    def test_new_support_reactivates_existing_fact_and_arity_is_distinct(self):
        rule = Rule((Atom('p', ('x',)),), (Atom('q', ('x',)),))
        engine = Engine(range(2), 3, (rule,))
        engine.add(Atom('p', (0, 1)), 3)
        engine.add(Atom('p', (0,)), 1)
        engine.saturate()
        self.assertEqual(engine.support(Atom('q', (0,))), 1)
        engine.add(Atom('p', (0,)), 2)
        engine.saturate()
        self.assertEqual(engine.support(Atom('q', (0,))), 3)
        before = dict(engine.facts)
        engine.saturate()
        self.assertEqual(engine.facts, before)

    def test_rule_cannot_allocate_or_introduce_unknown_rule_variables(self):
        with self.assertRaises(ValueError):
            Engine(range(2), 1, (Rule((Atom('p', ('x',)),), (Atom('q', ('y',)),)),))
        engine = Engine(range(2), 1, ())
        with self.assertRaises(ValueError):
            engine.add(Atom('p', (2,)), 1)
        with self.assertRaises(ValueError):
            engine.add(Atom('p', (0,)), 2)


if __name__ == '__main__':
    unittest.main()
