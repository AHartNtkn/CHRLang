import unittest
import z3
from heap import Encoding,Heap
from projection import ConcreteHeap
from matching import View

class ProjectionTests(unittest.TestCase):
    def test_static_projection_preserves_aliases_and_forward_bindings(self):
        e=Encoding();h=Heap(e,6,[('s',1),('z',0)])
        x=h.variable();y=h.variable();sy=h.app('s',[y]);z=h.app('z',[])
        h.unify(x,sy,3);h.unify(y,z,3)
        concrete=ConcreteHeap.static(h)
        self.assertIsNotNone(concrete)
        self.assertEqual(concrete.term(x.as_long()),('s',(('z',()),)))
        self.assertEqual(concrete.term(y.as_long()),('z',()))

    def test_model_projection_matches_symbolic_view_for_each_choice(self):
        e=Encoding();h=Heap(e,4,[('s',1),('z',0)])
        x=h.variable();z=h.app('z',[]);sz=h.app('s',[z]);choice=z3.Bool('value_choice')
        h.unify(x,z3.If(choice,sz,z),3)
        self.assertIsNone(ConcreteHeap.static(h))
        view=View(h)
        for bit,expected in [(False,('z',())),(True,('s',(('z',()),)))]:
            e.solver.push();e.solver.add(choice==bit)
            self.assertEqual(e.solver.check(),z3.sat)
            model=e.solver.model();heap=ConcreteHeap(h,model)
            self.assertEqual(heap.term(x.as_long()),expected)
            value=model.eval(view.values[x.as_long()])
            def decode(v):
                i=next(i for i in range(view.sort.num_constructors()) if v.decl()==view.sort.constructor(i))
                return v.arg(0).as_long() if i==0 else (h.signature[i-1][0],tuple(decode(c) for c in v.children()))
            self.assertEqual(decode(value),expected)
            e.solver.pop()
