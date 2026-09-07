import unittest
from machine import Rule
from symbolic import Bounded

class SymbolicTests(unittest.TestCase):
    def test_explicit_choice_has_two_quiescent_answers(self):
        rule=Rule((),(('p',(0,)),),('or',('eq',0,('a',())),('eq',0,('b',()))))
        run=Bounded([rule],[('p',(0,))],[0],transitions=7,nodes=5,occurrences=2,pending=3,service=2)
        answers=run.answers()
        self.assertEqual({repr(a['outputs']) for a in answers},{repr([('a',())]),repr([('b',())])})
        self.assertTrue(all(a['residual']==[] for a in answers))

    def test_history_prevents_refiring_and_keeps_residual(self):
        rule=Rule((('p',(0,)),),(),('post',('q',(0,))))
        run=Bounded([rule],[('p',(0,))],[0],transitions=6,nodes=4,occurrences=3,pending=2,service=1)
        answers=run.answers()
        self.assertEqual(len(answers),1)
        self.assertEqual(answers[0],{'outputs':[0],'residual':[('p',(0,)),('q',(0,))]})

    def test_distinct_equal_occurrences_are_consumed_together(self):
        z=('z',())
        rule=Rule((),(('p',(0,)),('p',(0,))),('post',('q',(0,))))
        run=Bounded([rule],[('p',(z,)),('p',(z,))],[],transitions=6,nodes=6,occurrences=3,pending=3,service=1)
        self.assertEqual(run.answers(),[{'outputs':[],'residual':[('q',(z,))]}])

    def test_binding_then_post_preserves_nonground_residual(self):
        rule=Rule((),(('p',(0,1)),),('and',('eq',0,('z',())),('post',('no_c',(1,)))))
        run=Bounded([rule],[('p',(0,1))],[0,1],transitions=8,nodes=6,occurrences=2,pending=3,service=2)
        self.assertEqual(run.answers(),[{'outputs':[('z',()),1],'residual':[('no_c',(1,))]}])
