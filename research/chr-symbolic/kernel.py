"""Finite immutable constructor arena, symbolic operands, deterministic unification.

Service cutoff is explicit. This is an E11 correctness gate, not a CHR machine.
"""
import z3


def select(values, index):
    result=values[-1]
    for i in reversed(range(len(values)-1)):
        result=z3.If(index==i,values[i],result)
    return result


class Circuit:
    def __init__(self,nodes,steps):
        if not nodes or steps<0:raise ValueError('nonempty arena and nonnegative bound required')
        for i,(name,args) in enumerate(nodes):
            if name=='var' and args:raise ValueError('variable has children')
            if any(c<0 or c>=i for c in args):raise ValueError('constructor arena must be acyclic and topological')
        self.nodes=nodes
        self.solver=z3.Solver()
        self.serial=0
        n=len(nodes)
        self.left=z3.Int('left');self.right=z3.Int('right')
        self.solver.add(self.left>=0,self.left<n,self.right>=0,self.right<n)
        sub=list(map(z3.IntVal,range(n)))
        # At most maxarity-1 extra pairs per decomposition. Capacity covers every
        # registered microstep, independently of shape/sharing.
        arity=max(len(args) for _,args in nodes)
        capacity=1+steps*max(arity-1,0)
        xs=[self.left]+[z3.IntVal(0)]*(capacity-1)
        ys=[self.right]+[z3.IntVal(0)]*(capacity-1)
        length=z3.IntVal(1);failed=z3.BoolVal(False)
        tags={key:i for i,key in enumerate(sorted(set((name,len(args)) for name,args in nodes)))}
        labels=[z3.IntVal(tags[(name,len(args))]) for name,args in nodes]
        sizes=[z3.IntVal(len(args)) for _,args in nodes]
        variables=[z3.BoolVal(name=='var') for name,_ in nodes]
        def deref(x):
            for _ in range(n):x=select(sub,x)
            return self.bind(x)
        def occurs(v,x):
            reachable=[z3.IntVal(i)==v for i in range(n)]
            # Before each proposed binding the dereference/constructor graph is
            # acyclic. Any path is shorter than n nodes.
            for _ in range(n):
                reachable=[self.bind(z3.Or(z3.IntVal(i)==v,z3.If(sub[i]!=i,select(reachable,sub[i]),z3.Or(*[reachable[c] for c in args])))) for i,(_,args) in enumerate(nodes)]
            return select(reachable,x)
        for _ in range(steps):
            active=z3.And(z3.Not(failed),length>0)
            a=deref(select(xs,length-1));b=deref(select(ys,length-1))
            same=a==b;va=select(variables,a);vb=select(variables,b)
            bind_a=z3.And(z3.Not(same),va)
            bind_b=z3.And(z3.Not(same),z3.Not(va),vb)
            cycle=z3.Or(z3.And(bind_a,occurs(a,b)),z3.And(bind_b,occurs(b,a)))
            constructor=z3.And(z3.Not(same),z3.Not(va),z3.Not(vb))
            clash=z3.And(constructor,select(labels,a)!=select(labels,b))
            bad=z3.Or(cycle,clash)
            success=z3.And(active,z3.Not(bad))
            target=z3.If(bind_a,a,b);value=z3.If(bind_a,b,a)
            sub=[self.bind(z3.If(z3.And(success,z3.Or(bind_a,bind_b),target==i),value,old)) for i,old in enumerate(sub)]
            push=z3.If(constructor,select(sizes,a),0)
            new_x=[];new_y=[]
            for slot in range(capacity):
                x=xs[slot];y=ys[slot]
                # Reverse push gives the leftmost constructor child first.
                for child in range(arity):
                    at=z3.And(success,constructor,child<push,slot==length-1+push-1-child)
                    ac=select([z3.IntVal(args[child] if child<len(args) else 0) for _,args in nodes],a)
                    bc=select([z3.IntVal(args[child] if child<len(args) else 0) for _,args in nodes],b)
                    x=z3.If(at,ac,x);y=z3.If(at,bc,y)
                new_x.append(self.bind(x));new_y.append(self.bind(y))
            xs,ys=new_x,new_y
            length=self.bind(z3.If(success,length-1+push,length))
            failed=self.bind(z3.Or(failed,z3.And(active,bad)))
        self.sub=sub;self.failed=failed;self.length=length

    def bind(self,expression):
        expression=z3.simplify(expression)
        if z3.is_true(expression) or z3.is_false(expression) or z3.is_int_value(expression):return expression
        self.serial+=1
        value=z3.Const(f'v{self.serial}',expression.sort())
        self.solver.add(value==expression)
        return value

    def solve(self,left,right):
        if not 0<=left<len(self.nodes) or not 0<=right<len(self.nodes):raise ValueError('operand outside arena')
        self.solver.push();self.solver.add(self.left==left,self.right==right)
        status=self.solver.check()
        if status!=z3.sat:
            self.solver.pop()
            raise RuntimeError(f'deterministic circuit has no model: {status}')
        model=self.solver.model()
        failure=z3.is_true(model.eval(self.failed))
        done=model.eval(self.length).as_long()==0
        sub=[model.eval(x).as_long() for x in self.sub]
        def decode(i):
            if sub[i]!=i:return decode(sub[i])
            name,args=self.nodes[i]
            return i if name=='var' else (name,tuple(decode(c) for c in args))
        result={'status':'failed' if failure else 'done' if done else 'cutoff','variables':[decode(i) for i,(name,_) in enumerate(self.nodes) if name=='var']}
        self.solver.pop()
        return result
