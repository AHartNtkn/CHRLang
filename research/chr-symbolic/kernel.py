"""Fixed-arena harness for the E11 symbolic heap equation service."""
import z3
from heap import Encoding,Heap


class Circuit:
    def __init__(self,nodes,steps,equations=1):
        if not nodes or steps<0 or equations<1:raise ValueError('invalid circuit bounds')
        for i,(name,args) in enumerate(nodes):
            if name=='var' and args:raise ValueError('variable has children')
            if any(c<0 or c>=i for c in args):raise ValueError('constructor arena must be topological')
        self.e=Encoding();self.solver=self.e.solver
        self.heap=Heap(self.e,len(nodes),[(name,len(args)) for name,args in nodes if name!='var'])
        for name,args in nodes:
            if name=='var':self.heap.variable()
            else:self.heap.app(name,list(args))
        self.operands=[(z3.Int(f'left{i}'),z3.Int(f'right{i}')) for i in range(equations)]
        for left,right in self.operands:
            self.solver.add(left>=0,left<len(nodes),right>=0,right<len(nodes))
            self.heap.unify(left,right,steps)

    def solve(self,equations):
        if len(equations)!=len(self.operands):raise ValueError('equation count differs from circuit')
        if any(not 0<=i<self.heap.n for pair in equations for i in pair):raise ValueError('operand outside arena')
        self.solver.push()
        for (left,right),(a,b) in zip(self.operands,equations):self.solver.add(left==a,right==b)
        status=self.solver.check()
        if status!=z3.sat:
            self.solver.pop()
            raise RuntimeError(f'deterministic circuit has no model: {status}')
        result=self.heap.decode(self.solver.model())
        self.solver.pop()
        return result
