import unittest
from kernel import Atom, Rule, Engine
from theory import axioms, initialize, extract, value_transport


class IntegratedValueRelations(unittest.TestCase):
    def test_equality_enables_constant_premise_and_repeated_value_join(self):
        rules = (Rule((Atom('P', (1,)),), (Atom('eq', (2, 3)),)),
                 Rule((Atom('Pair', ('x', 'x')),), (Atom('hit', ('x',)),)))
        e = Engine(range(5), 3, axioms(()) + value_transport('P', 1)
                   + value_transport('Pair', 2) + rules)
        initialize(e)
        for fact, support in [(Atom('P', (0,)), 3), (Atom('Pair', (0, 1)), 3),
                              (Atom('eq', (0, 1)), 1)]:
            e.add(fact, support)
        e.saturate()
        self.assertEqual(e.support(Atom('eq', (2, 3))), 1)
        self.assertEqual(e.support(Atom('hit', (0,))), 1)
        self.assertEqual(e.support(Atom('hit', (1,))), 1)
        self.assertEqual(e.support(Atom('bad', ())), 0)
        self.assertEqual(extract(e, (), [0, 1], 2), (0, 1))

    def test_constructor_premises_follow_equal_root_and_child_values(self):
        rules = (Rule((Atom('f', (0, 3)),), (Atom('hit', (0,)),)),)
        e = Engine(range(4), 3, axioms((('f', 1),)) + rules)
        initialize(e)
        e.add(Atom('f', (1, 2)), 3)
        e.add(Atom('eq', (0, 1)), 1)
        e.add(Atom('eq', (2, 3)), 1)
        e.saturate()
        self.assertEqual(e.support(Atom('hit', (0,))), 1)
        self.assertEqual(e.support(Atom('f', (1, 2))), 3)

    def test_identity_column_is_not_substituted(self):
        e = Engine(range(3), 1, axioms(()) + value_transport('occurrence', 2, (1,)))
        initialize(e)
        e.add(Atom('occurrence', (0, 0)), 1)
        e.add(Atom('eq', (0, 1)), 1)
        e.saturate()
        self.assertEqual(e.support(Atom('occurrence', (0, 1))), 1)
        self.assertEqual(e.support(Atom('occurrence', (1, 1))), 0)

    def test_constant_congruence_and_readout_requires_complete_closure(self):
        e = Engine(range(3), 1, axioms((('a', 0),)))
        initialize(e)
        e.add(Atom('a', (0,)), 1)
        e.add(Atom('a', (1,)), 1)
        with self.assertRaises(ValueError):
            extract(e, (('a', 0),), [0], 1)
        e.saturate()
        self.assertEqual(e.support(Atom('eq', (0, 1))), 1)
        self.assertEqual(e.support(Atom('bad', ())), 0)
        e.add(Atom('eq', (1, 2)), 1)
        with self.assertRaises(ValueError):
            extract(e, (('a', 0),), [0], 1)
