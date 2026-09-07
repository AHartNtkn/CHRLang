import unittest
from kernel import Circuit

class KernelTests(unittest.TestCase):
    def test_symbolic_pairs_match_independent_unifier(self):
        nodes=[('var',()),('var',()),('z',()),('s',(0,)),('s',(2,)),('a',(0,1)),('k',()),('a',(1,0)),('s',(1,)),('a',(0,0)),('a',(2,4))]
        circuit=Circuit(nodes,12)
        # Independent nested-tree Robinson procedure; no slot-vector algorithm.
        def tree(i):
            name,args=nodes[i]
            return i if name=='var' else (name,tuple(tree(c) for c in args))
        def unify(a,b):
            subst={}
            def resolve(x):
                if isinstance(x,int):return resolve(subst[x]) if x in subst else x
                return (x[0],tuple(resolve(c) for c in x[1]))
            def occurs(v,x):return x==v if isinstance(x,int) else any(occurs(v,c) for c in x[1])
            pairs=[(tree(a),tree(b))]
            while pairs:
                x,y=map(resolve,pairs.pop())
                if x==y:continue
                if isinstance(x,int):
                    if occurs(x,y):return None
                    subst[x]=y
                elif isinstance(y,int):
                    if occurs(y,x):return None
                    subst[y]=x
                elif x[0]!=y[0] or len(x[1])!=len(y[1]):return None
                else:pairs.extend(reversed(list(zip(x[1],y[1]))))
            return [resolve(i) for i in [0,1]]
        for a in range(len(nodes)):
            for b in range(len(nodes)):
                with self.subTest(a=a,b=b):
                    result=circuit.solve(a,b)
                    expected=unify(a,b)
                    self.assertEqual(result['status'],'failed' if expected is None else 'done')
                    if expected is not None:self.assertEqual(result['variables'],expected)
    def test_cutoff_is_not_failure_or_answer(self):
        c=Circuit([('var',()),('z',())],0)
        self.assertEqual(c.solve(0,1)['status'],'cutoff')

if __name__=='__main__':unittest.main()
