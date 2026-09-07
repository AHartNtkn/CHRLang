import unittest
from kernel import Atom, Engine
from theory import axioms, initialize, extract

class ChoiceCorrelation(unittest.TestCase):
    def test_swapped_bindings_keep_only_the_two_explicit_correlated_answers(self):
        signature=(('a',0),('b',0))
        engine=Engine(range(4),3,axioms(signature))
        initialize(engine)
        engine.add(Atom('a',(2,)),3)
        engine.add(Atom('b',(3,)),3)
        for x,y,support in [(0,2,1),(1,3,1),(0,3,2),(1,2,2)]:
            engine.add(Atom('eq',(x,y)),support)
        engine.saturate()
        self.assertEqual(engine.support(Atom('bad',())),0)
        actual=[extract(engine,signature,[0,1],support) for support in [1,2]]
        self.assertEqual(actual,[(('a',()),('b',())),(('b',()),('a',()))])

    def test_equality_only_cycle_is_not_a_positive_constructor_cycle(self):
        engine=Engine(range(3),1,axioms((('f',1),)))
        initialize(engine)
        for x,y in [(0,1),(1,2),(2,0)]:
            engine.add(Atom('eq',(x,y)),1)
        engine.saturate()
        self.assertEqual(engine.support(Atom('bad',())),0)
        self.assertEqual(extract(engine,(('f',1),),[0,1,2],1),(0,0,0))
