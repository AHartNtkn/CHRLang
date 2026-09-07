import unittest
import z3
from heap import Encoding, Heap

class HeapTests(unittest.TestCase):
    def test_symbolic_allocation_keeps_branch_local_bindings(self):
        e=Encoding();h=Heap(e,5,[('z',0),('s',1)])
        choice=z3.Bool('choice')
        x=h.variable();z=h.app('z',[])
        sx=h.app('s',[x],guard=choice)
        value=z3.If(choice,sx,z)
        h.unify(x,value,4)
        for selected in [False,True]:
            e.solver.push();e.solver.add(choice==selected)
            self.assertEqual(e.solver.check(),z3.sat)
            result=h.decode(e.solver.model())
            self.assertEqual(result['status'],'failed' if selected else 'done')
            if not selected:self.assertEqual(result['variables'],[('z',())])
            e.solver.pop()

    def test_allocations_and_equations_compose(self):
        e=Encoding();h=Heap(e,6,[('z',0),('s',1)])
        x=h.variable();y=h.variable();sy=h.app('s',[y])
        h.unify(x,sy,4)
        z=h.app('z',[]);h.unify(y,z,4)
        self.assertEqual(e.solver.check(),z3.sat)
        self.assertEqual(h.decode(e.solver.model())['variables'],[('s',(('z',()),)),('z',())])

    def test_allocation_limit_is_cutoff(self):
        e=Encoding();h=Heap(e,1,[('z',0)])
        h.variable();h.app('z',[])
        self.assertEqual(e.solver.check(),z3.sat)
        self.assertEqual(h.decode(e.solver.model())['status'],'cutoff')

    def test_fresh_variable_identity_follows_actual_allocations(self):
        e=Encoding();h=Heap(e,4,[])
        choice=z3.Bool('allocate_extra')
        x=h.variable();h.variable(guard=choice);last=h.variable()
        h.unify(last,x,3)
        for selected,expected in [(False,[0,0]),(True,[0,1,0])]:
            e.solver.push();e.solver.add(choice==selected)
            self.assertEqual(e.solver.check(),z3.sat)
            self.assertEqual(h.decode(e.solver.model()),{'status':'done','variables':expected})
            e.solver.pop()

    def test_inactive_equation_does_not_bind_or_consume_service(self):
        e=Encoding();h=Heap(e,2,[('z',0)])
        x=h.variable();z=h.app('z',[])
        h.unify(x,z,0,guard=False)
        self.assertEqual(e.solver.check(),z3.sat)
        self.assertEqual(h.decode(e.solver.model()),{'status':'done','variables':[0]})
