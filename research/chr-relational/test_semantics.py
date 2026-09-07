import unittest
from kernel import Atom, Rule, Engine
from theory import axioms, initialize, extract


def world(nodes=8, contexts=1, extra=(), signature=(('f', 1), ('g', 1), ('a', 0))):
    engine = Engine(range(nodes), contexts, axioms(signature) + tuple(extra))
    initialize(engine)
    return engine


class ConstructorRelations(unittest.TestCase):
    def test_injectivity_congruence_and_descriptor_retention(self):
        e = world()
        facts = [Atom('f', (0, 1)), Atom('f', (2, 3)), Atom('eq', (0, 2)),
                 Atom('g', (4, 1)), Atom('g', (5, 3))]
        for f in facts:
            e.add(f, 1)
        e.saturate()
        self.assertEqual(e.support(Atom('eq', (1, 3))), 1)
        self.assertEqual(e.support(Atom('eq', (4, 5))), 1)
        self.assertTrue(all(e.support(f) == 1 for f in facts))
        self.assertEqual(extract(e, (('f', 1), ('g', 1), ('a', 0)), [1, 3, 0], 1),
                         (0, 0, ('f', (0,))))

    def test_clash_and_indirect_cycle_fail_only_their_contexts(self):
        e = world(contexts=7)
        for fact, support in [(Atom('f', (0, 1)), 7), (Atom('g', (2, 3)), 1),
                              (Atom('eq', (0, 2)), 1), (Atom('f', (4, 5)), 2),
                              (Atom('eq', (1, 4)), 2), (Atom('eq', (5, 0)), 2)]:
            e.add(fact, support)
        e.saturate()
        self.assertEqual(e.support(Atom('bad', ())), 3)
        self.assertIsNone(extract(e, (('f', 1), ('g', 1), ('a', 0)), [0], 1))
        self.assertEqual(extract(e, (('f', 1), ('g', 1), ('a', 0)), [0], 4),
                         (('f', (0,)),))

    def test_union_graph_cycle_has_finite_separate_projections(self):
        e = world(nodes=2, contexts=3)
        e.add(Atom('f', (0, 1)), 1)
        e.add(Atom('f', (1, 0)), 2)
        e.saturate()
        self.assertEqual(e.support(Atom('bad', ())), 0)
        self.assertEqual(extract(e, (('f', 1),), [0, 1], 1), (('f', (0,)), 0))
        self.assertEqual(extract(e, (('f', 1),), [0, 1], 2), (0, ('f', (0,))))

    def test_ordinary_rules_introduce_and_read_relations_gradually(self):
        extra = (
            Rule((Atom('request', ('r', 'x')),), (Atom('f', ('r', 'x')),)),
            Rule((Atom('connect', ('x', 'y')),), (Atom('eq', ('x', 'y')),)),
            Rule((Atom('watch', ('r',)), Atom('eq', ('r', 's')), Atom('f', ('s', 'x'))),
                 (Atom('seen', ('x',)),)),
        )
        e = world(contexts=3, extra=extra)
        e.add(Atom('watch', (0,)), 3)
        e.saturate()
        self.assertEqual(e.support(Atom('seen', (1,))), 0)
        self.assertFalse(any(a.predicate == 'f' for a in e.facts))
        e.add(Atom('request', (2, 1)), 3)
        e.saturate()
        self.assertEqual(e.support(Atom('seen', (1,))), 0)
        e.add(Atom('connect', (0, 2)), 1)
        e.saturate()
        self.assertEqual(e.support(Atom('seen', (1,))), 1)
        e.add(Atom('connect', (0, 2)), 2)
        e.saturate()
        self.assertEqual(e.support(Atom('seen', (1,))), 3)

    def test_same_name_different_arity_and_repeated_holes(self):
        signature = (('f', 0), ('f', 1), ('pair', 2), ('a', 0), ('b', 0))
        e = world(contexts=3, signature=signature)
        for fact, support in [(Atom('f', (0,)), 1), (Atom('f', (0, 1)), 1),
                              (Atom('pair', (2, 3, 3)), 2), (Atom('pair', (4, 5, 6)), 2),
                              (Atom('a', (5,)), 2), (Atom('b', (6,)), 2),
                              (Atom('eq', (2, 4)), 2)]:
            e.add(fact, support)
        e.saturate()
        self.assertEqual(e.support(Atom('bad', ())), 3)


if __name__ == '__main__':
    unittest.main()
