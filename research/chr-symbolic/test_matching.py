import unittest
import z3
from heap import Encoding,Heap
from matching import View

class MatchingTests(unittest.TestCase):
    def test_repeated_head_variables_use_equality_without_binding(self):
        e=Encoding();h=Heap(e,5,[('p',2),('z',0)])
        x=h.variable();y=h.variable();root=h.app('p',[x,y])
        before=list(h.sub)
        ok,_=View(h).match(('p',(0,0)),root)
        e.solver.push();e.solver.add(ok)
        self.assertEqual(e.solver.check(),z3.unsat)
        e.solver.pop()
        self.assertEqual([v.sexpr() for v in before],[v.sexpr() for v in h.sub])
        h.unify(x,y,3)
        ok,_=View(h).match(('p',(0,0)),root)
        e.solver.add(z3.Not(ok));self.assertEqual(e.solver.check(),z3.unsat)

    def test_constructor_head_does_not_instantiate_free_variable(self):
        e=Encoding();h=Heap(e,2,[('s',1)])
        x=h.variable()
        ok,_=View(h).match(('s',(0,)),x)
        e.solver.add(ok);self.assertEqual(e.solver.check(),z3.unsat)

    def test_symbolic_child_and_pure_guard(self):
        e=Encoding();h=Heap(e,5,[('p',1),('z',0),('s',1)])
        choice=z3.Bool('choice')
        x=h.variable();z=h.app('z',[]);s=h.app('s',[z]);root=h.app('p',[z3.If(choice,s,z)])
        view=View(h);ok,bindings=view.match(('p',(0,)),root)
        guard=view.guard(0,('z',()),bindings)
        for bit in [False,True]:
            e.solver.push();e.solver.add(choice==bit,ok,guard if bit else z3.Not(guard))
            self.assertEqual(e.solver.check(),z3.unsat);e.solver.pop()
        self.assertEqual(h.variables.as_long(),1)

    def test_rule_selection_respects_order_identity_and_history(self):
        from machine import Rule
        from matching import applications
        e=Encoding();h=Heap(e,3,[('p',0)])
        p=h.app('p',[])
        rules=[Rule((('p',()),),(),('true',)),Rule((),(('p',()),('p',())),('true',))]
        seen=z3.Bool('already_fired')
        choices=applications(h,rules,[p,p],[z3.BoolVal(True),z3.BoolVal(False)],{(0,(0,)):seen})
        for bit in [False,True]:
            e.solver.push();e.solver.add(seen==bit)
            self.assertEqual(e.solver.check(),z3.sat)
            model=e.solver.model()
            selected=[(a.rule,a.occurrences) for a in choices if z3.is_true(model.eval(a.selected))]
            self.assertEqual(selected,[] if bit else [(0,(0,))])
            e.solver.pop()
