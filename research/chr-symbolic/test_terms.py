import unittest
import z3
from heap import Encoding
from terms import TermStore

class TermStoreTests(unittest.TestCase):
    def test_freshness_aliases_and_occurs_failure(self):
        e=Encoding();s=TermStore(e,6,[('z',0),('s',1)])
        x=s.variable();y=s.variable();sy=s.app('s',[y]);z=s.app('z',[])
        s.unify(x,sy,4);s.unify(y,z,4)
        self.assertEqual(e.solver.check(),z3.sat)
        self.assertEqual(s.decode(e.solver.model()),{'status':'done','variables':[('s',(('z',()),)),('z',())]})
        e=Encoding();s=TermStore(e,3,[('s',1)])
        x=s.variable();sx=s.app('s',[x]);s.unify(x,sx,2)
        self.assertEqual(e.solver.check(),z3.sat)
        self.assertEqual(s.decode(e.solver.model())['status'],'failed')

    def test_symbolic_choices_keep_alternatives_and_explicit_identity(self):
        e=Encoding();s=TermStore(e,5,[('z',0),('s',1)])
        x=s.variable();z=s.app('z',[]);sz=s.app('s',[z]);choice=z3.Bool('term_choice')
        s.unify(x,z3.If(choice,sz,z),3)
        for bit,value in [(False,('z',())),(True,('s',(('z',()),)))]:
            e.solver.push();e.solver.add(choice==bit)
            self.assertEqual(e.solver.check(),z3.sat)
            self.assertEqual(s.decode(e.solver.model()),{'status':'done','variables':[value]})
            e.solver.pop()

    def test_service_and_allocation_cutoffs(self):
        e=Encoding();s=TermStore(e,2,[('z',0)])
        x=s.variable();z=s.app('z',[]);s.unify(x,z,0)
        self.assertEqual(e.solver.check(),z3.sat)
        self.assertEqual(s.decode(e.solver.model())['status'],'cutoff')
        e=Encoding();s=TermStore(e,1,[('z',0)])
        s.variable();s.app('z',[])
        self.assertEqual(e.solver.check(),z3.sat)
        self.assertEqual(s.decode(e.solver.model())['status'],'cutoff')

    def test_equal_constructors_keep_allocation_identity_for_pair_bounds(self):
        for steps,status in [(1,'cutoff'),(2,'done')]:
            e=Encoding();s=TermStore(e,4,[('z',0),('s',1)])
            z=s.app('z',[]);a=s.app('s',[z]);b=s.app('s',[z])
            s.unify(a,b,steps)
            self.assertEqual(e.solver.check(),z3.sat)
            self.assertEqual(s.decode(e.solver.model())['status'],status)

    def test_projection_does_not_confuse_a_union_cycle_with_a_branch_cycle(self):
        from term_matching import TermView
        e=Encoding();s=TermStore(e,4,[('f',1)])
        x=s.variable();y=s.variable();fx=s.app('f',[x]);fy=s.app('f',[y]);choice=z3.Bool('cycle_choice')
        s.unify(x,fy,3,guard=choice);s.unify(y,fx,3,guard=z3.Not(choice))
        view=TermView(s)
        e.solver.add(view.project(x)==view.project(y))
        self.assertEqual(e.solver.check(),z3.unsat)
