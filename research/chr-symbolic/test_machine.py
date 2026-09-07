import unittest
from machine import Rule, Search, Branch, verify_witness
from copy import deepcopy

Z=('z',())
def S(x):return ('s',(x,))
def C(n,*args):return (n,tuple(args))
def post(n,*args):return ('post',C(n,*args))
def eq(a,b):return ('eq',a,b)

class MachineTests(unittest.TestCase):
    def test_relational_addition_and_symbolic_residual(self):
        rule=Rule((),(C('add',0,1,2),),('or',('and',eq(0,Z),eq(1,2)),('and',eq(0,S(3)),eq(2,S(4)),post('add',3,1,4))))
        run=Search([rule],[C('add',0,1,S(S(Z))),C('no_c',3)],[0,1,3])
        answers=run.advance(200)
        self.assertTrue(run.exhausted)
        self.assertEqual(len(answers),3)
        self.assertEqual({repr(a['outputs'][:2]) for a in answers},{repr([Z,S(S(Z))]),repr([S(Z),S(Z)]),repr([S(S(Z)),Z])})
        for a in answers:self.assertEqual(a['residual'],[C('no_c',a['outputs'][2])])

    def test_history_and_distinct_occurrences(self):
        propagate=Rule((C('p',0),),(),post('q',0))
        run=Search([propagate],[C('p',Z),C('p',Z)],[])
        answer=run.advance(40)[0]
        self.assertEqual(answer['residual'].count(C('q',Z)),2)
        self.assertTrue(run.exhausted)
        join=Rule((),(C('p',0),C('p',0)),post('joined'))
        run=Search([join],[C('p',Z)],[])
        self.assertEqual(run.advance(20)[0]['residual'],[C('p',Z)])

    def test_committed_order_and_cutoff(self):
        first=Rule((),(C('p'),),post('a'))
        second=Rule((),(C('p'),),post('b'))
        run=Search([first,second],[C('p')],[])
        self.assertEqual(run.advance(20)[0]['residual'],[C('a')])
        run=Search([Rule((),(C('p'),),post('p'))],[C('p')],[])
        self.assertEqual(run.advance(20),[])
        self.assertFalse(run.exhausted)


class WitnessTests(unittest.TestCase):
    def test_checks_history_and_quiescence_at_each_transition(self):
        rules=[Rule((C('p',0),),(),post('q',0))]
        initial=[C('p',Z)]
        branch=Branch(initial,[])
        states=[branch.snapshot()];actions=[]
        while True:
            outcome=branch.step(rules)
            actions.append(branch.trace[-1]);states.append(branch.snapshot())
            if outcome[0]=='answer':break
        self.assertEqual(verify_witness(rules,initial,[],states,actions),outcome[1])
        malformed=deepcopy(states)
        malformed[2]['history']=[]
        with self.assertRaises(ValueError):verify_witness(rules,initial,[],malformed,actions)
        with self.assertRaises(ValueError):verify_witness(rules,initial,[],states[:-1],actions[:-1])

    def test_explicit_choice_and_committed_action_are_checked(self):
        rules=[Rule((),(C('p'),),('or',post('left'),post('right')))]
        branch=Branch([C('p')],[]);states=[branch.snapshot()];actions=[]
        while True:
            outcome=branch.step(rules)
            action=branch.trace[-1]
            if outcome[0]=='split':
                action=('or',True);branch.pending.appendleft(outcome[2])
            actions.append(action);states.append(branch.snapshot())
            if outcome[0]=='answer':break
        self.assertEqual(verify_witness(rules,[C('p')],[],states,actions)['residual'],[C('right')])
        wrong=list(actions);wrong[2]=('or',False)
        with self.assertRaises(ValueError):verify_witness(rules,[C('p')],[],states,wrong)
