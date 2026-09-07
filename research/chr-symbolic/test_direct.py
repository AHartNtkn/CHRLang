import unittest
from machine import Rule
from direct import Direct,Arena,Cutoff
from machine import unify,resolve

class DirectTests(unittest.TestCase):
    def test_arena_pairs_against_independent_tree_unifier(self):
        nodes=[0,1,('z',()),('s',(0,)),('s',(2,)),('a',(0,1)),('k',()),('a',(1,0)),('s',(1,)),('a',(0,0)),('a',(2,4))]
        def tree(h):
            t=nodes[h]
            return t if isinstance(t,int) else (t[0],tuple(tree(c) for c in t[1]))
        for a in range(len(nodes)):
            for b in range(len(nodes)):
                arena=Arena(len(nodes));arena.nodes=list(nodes);arena.variables=2
                subst={};expected=unify(tree(a),tree(b),subst)
                self.assertEqual(arena.unify(a,b,12),expected)
                if expected:self.assertEqual([arena.value(i) for i in (0,1)],[resolve(i,subst) for i in (0,1)])
        arena=Arena(2);arena.nodes=[0,('z',())];arena.variables=1
        with self.assertRaises(Cutoff):arena.unify(0,1,0)

    def run_case(self,rules,constraints,outputs,**overrides):
        bounds=dict(transitions=8,nodes=8,occurrences=3,pending=3,service=4)
        bounds.update(overrides)
        return Direct(rules,constraints,outputs,**bounds)

    def test_choice_keeps_aliases_and_failed_branch_out(self):
        rule=Rule((),(('p',(0,)),),('or',('eq',0,('f',(0,))),('eq',0,('a',()))))
        run=self.run_case([rule],[('p',(0,))],[0])
        self.assertEqual(run.answers(),[{'outputs':[('a',())],'residual':[]}])
        self.assertEqual(run.models,1)
        self.assertEqual(run.boundary_status(),dict(resource_cutoff=False,transition_cutoff=False))

    def test_same_node_and_equal_distinct_nodes_have_different_service_cost(self):
        for equation,cutoff in [(('eq',0,0),False),(('eq',('f',(0,)),('f',(0,))),True)]:
            run=self.run_case([Rule((),(('p',(0,)),),equation)],[('p',(0,))],[0],service=1)
            self.assertEqual(run.boundary_status()['resource_cutoff'],cutoff)
            self.assertEqual(len(run.answers()),int(not cutoff))

    def test_all_resource_dimensions_and_transition_endpoint(self):
        rule=Rule((),(('p',(0,)),),('and',('true',),('true',)))
        for bound,value in [('nodes',1),('occurrences',1),('pending',1)]:
            constraints=[('p',(0,)),('p',(0,))] if bound=='occurrences' else [('p',(0,))]
            run=self.run_case([rule],constraints,[0],**{bound:value})
            self.assertTrue(run.boundary_status()['resource_cutoff'])
        run=self.run_case([],[],[],transitions=0)
        self.assertEqual(run.answers(),[])
        self.assertTrue(run.boundary_status()['transition_cutoff'])
        self.assertEqual(self.run_case([],[],[],transitions=1).answers(),[{'outputs':[],'residual':[]}])

    def test_history_and_distinct_occurrences(self):
        rule=Rule((('p',(0,)),('p',(0,))),(),('true',))
        run=self.run_case([rule],[('p',(0,)),('p',(0,))],[0],transitions=10)
        self.assertEqual(run.models,1)
        self.assertEqual(run.answers(),[{'outputs':[0],'residual':[('p',(0,)),('p',(0,))]}])
