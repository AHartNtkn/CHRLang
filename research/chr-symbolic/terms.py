"""Direct term expressions with private allocation IDs and algorithmic unification.

Source holes remain explicit identities. SMT equality does not choose their values.
"""
import z3
from heap import select

_SORTS={}


def term_sort(signature):
    key=tuple(signature)
    if key not in _SORTS:
        name=f'CHRDirect{len(_SORTS)}'
        d=z3.Datatype(name)
        d.declare(name+'_hole',(name+'_hn',z3.IntSort()),(name+'_hv',z3.IntSort()))
        for i,(_,arity) in enumerate(signature):
            d.declare(name+f'_c{i}',(name+f'_n{i}',z3.IntSort()),*[(name+f'_a{i}_{k}',d) for k in range(arity)])
        _SORTS[key]=d.create()
    return _SORTS[key]


class TermStore:
    def __init__(self,encoding,capacity,signature):
        if capacity<1:raise ValueError('positive capacity required')
        self.e=encoding;self.n=capacity
        self.signature=list(dict.fromkeys(signature))
        self.arity=max([a for _,a in self.signature],default=0)
        self.sort=term_sort(self.signature);self.hole=self.sort.constructor(0)
        self.invalid=self.hole(-1,-1)
        self.sub=z3.K(z3.IntSort(),self.invalid)
        self.count=z3.IntVal(0);self.variables=z3.IntVal(0)
        self.failed=z3.BoolVal(False);self.cutoff=z3.BoolVal(False)

    def active(self,guard=True):return z3.And(guard,z3.Not(self.failed),z3.Not(self.cutoff))

    def allocate(self,value,guard,fresh=False):
        active=self.active(guard)
        if self.e.impossible(active):return self.invalid
        fits=self.count<self.n;write=z3.And(active,fits)
        if fresh:
            self.sub=z3.simplify(z3.If(write,z3.Store(self.sub,self.variables,value),self.sub))
            self.variables=self.e.bind(z3.If(write,self.variables+1,self.variables))
        self.count=self.e.bind(z3.If(write,self.count+1,self.count))
        self.cutoff=self.e.bind(z3.Or(self.cutoff,z3.And(active,z3.Not(fits))))
        return z3.simplify(z3.If(write,value,self.invalid))

    def variable(self,guard=True):return self.allocate(self.hole(self.count,self.variables),guard,True)

    def app(self,name,args,guard=True):
        index=self.signature.index((name,len(args)))+1
        self.e.solver.add(z3.Implies(self.active(guard),z3.And(*[z3.And(self.node(t)>=0,self.node(t)<self.count) for t in args])))
        return self.allocate(self.sort.constructor(index)(self.count,*args),guard)

    def tag(self,t):
        value=z3.IntVal(1)
        for i in range(len(self.signature)):
            value=z3.If(self.sort.recognizer(i+1)(t),i+2,value)
        return z3.simplify(value)

    def node(self,t):
        value=self.sort.accessor(0,0)(t)
        for i in range(len(self.signature)):
            value=z3.If(self.sort.recognizer(i+1)(t),self.sort.accessor(i+1,0)(t),value)
        return z3.simplify(value)

    def size(self,t):
        value=z3.IntVal(0)
        for i,(_,arity) in enumerate(self.signature):
            value=z3.If(self.sort.recognizer(i+1)(t),arity,value)
        return z3.simplify(value)

    def child(self,t,k):
        value=self.invalid
        for i,(_,arity) in enumerate(self.signature):
            if k<arity:value=z3.If(self.sort.recognizer(i+1)(t),self.sort.accessor(i+1,k+1)(t),value)
        return z3.simplify(value)

    def deref(self,t):
        for _ in range(self.n):
            value=z3.simplify(z3.If(self.sort.recognizer(0)(t),z3.Select(self.sub,self.sort.accessor(0,1)(t)),t))
            if self.e.impossible(t!=value):break
            t=value
        return t

    def occurs(self,variable,root,guard):
        """Visit each immutable node at most once; charge at most 1+A*N pops."""
        capacity=1+self.n*max(self.arity-1,0)
        stack=[root]+[self.invalid]*(capacity-1);length=z3.IntVal(1)
        seen=[z3.IntVal(-1)]*self.n;used=z3.IntVal(0);found=z3.BoolVal(False)
        for _ in range(1+self.arity*self.n):
            active=z3.And(guard,z3.Not(found),length>0)
            if self.e.impossible(active):break
            value=self.deref(select(stack,length-1));node=self.node(value)
            duplicate=z3.Or(*[z3.And(i<used,node==v) for i,v in enumerate(seen)])
            visit=z3.And(active,z3.Not(duplicate))
            found=self.e.bind(z3.Or(found,z3.And(visit,node==self.node(variable))))
            seen=[self.e.bind(z3.If(z3.And(visit,used==i),node,v)) for i,v in enumerate(seen)]
            used=self.e.bind(z3.If(visit,used+1,used))
            push=z3.If(z3.Not(duplicate),self.size(value),0)
            stack=self.push(stack,length,value,push,active)
            length=self.e.bind(z3.If(active,length-1+push,length))
        if not self.e.impossible(z3.And(guard,z3.Not(found),length>0)):
            raise ValueError("occurs traversal did not establish completion within its derived bound")
        return found

    def push(self,stack,length,value,push,guard):
        result=[]
        for i,old in enumerate(stack):
            for k in range(self.arity):
                old=z3.If(z3.And(guard,k<push,i==length-1+push-1-k),self.child(value,k),old)
            result.append(z3.simplify(old))
        return result

    def unify(self,left,right,steps,guard=True):
        if steps<0:raise ValueError('negative service bound')
        entry=self.active(guard)
        if self.e.impossible(entry):return
        self.e.solver.add(z3.Implies(entry,z3.And(self.node(left)>=0,self.node(left)<self.count,self.node(right)>=0,self.node(right)<self.count)))
        capacity=1+steps*max(self.arity-1,0)
        xs=[left]+[self.invalid]*(capacity-1);ys=[right]+[self.invalid]*(capacity-1)
        length=z3.IntVal(1)
        for _ in range(steps):
            active=z3.And(entry,z3.Not(self.failed),length>0)
            if self.e.impossible(active):break
            a=self.deref(select(xs,length-1));b=self.deref(select(ys,length-1))
            same=self.node(a)==self.node(b);va=self.sort.recognizer(0)(a);vb=self.sort.recognizer(0)(b)
            ba=z3.And(z3.Not(same),va);bb=z3.And(z3.Not(same),z3.Not(va),vb)
            constructor=z3.And(z3.Not(same),z3.Not(va),z3.Not(vb))
            ca=z3.BoolVal(False) if self.e.impossible(z3.And(active,ba)) else self.occurs(a,b,z3.And(active,ba))
            cb=z3.BoolVal(False) if self.e.impossible(z3.And(active,bb)) else self.occurs(b,a,z3.And(active,bb))
            bad=z3.Or(ca,cb,z3.And(constructor,self.tag(a)!=self.tag(b)))
            success=z3.And(active,z3.Not(bad))
            target=z3.If(ba,a,b);value=z3.If(ba,b,a)
            self.sub=z3.simplify(z3.If(z3.And(success,z3.Or(ba,bb)),z3.Store(self.sub,self.sort.accessor(0,1)(target),value),self.sub))
            push=z3.If(constructor,self.size(a),0)
            xs=self.push(xs,length,a,push,success);ys=self.push(ys,length,b,push,success)
            length=self.e.bind(z3.If(success,length-1+push,length))
            self.failed=self.e.bind(z3.Or(self.failed,z3.And(active,bad)))
        self.cutoff=self.e.bind(z3.Or(self.cutoff,z3.And(entry,z3.Not(self.failed),length>0)))

    def decoder(self,model):
        memo={};visiting=set()
        def term(t):
            t=model.eval(t);node=model.eval(self.node(t)).as_long()
            if node in memo:return memo[node]
            if node in visiting:raise ValueError('cyclic decoded term')
            visiting.add(node)
            tag=model.eval(self.tag(t)).as_long()
            if tag==1:
                var=model.eval(self.sort.accessor(0,1)(t)).as_long()
                target=model.eval(z3.Select(self.sub,var))
                value=var if model.eval(self.node(target)).as_long()==node else term(target)
            else:value=(self.signature[tag-2][0],tuple(term(self.child(t,k)) for k in range(self.signature[tag-2][1])))
            visiting.remove(node);memo[node]=value
            return value
        return term

    def decode(self,model):
        term=self.decoder(model)
        status='failed' if z3.is_true(model.eval(self.failed)) else 'cutoff' if z3.is_true(model.eval(self.cutoff)) else 'done'
        return {'status':status,'variables':[term(z3.Select(self.sub,i)) for i in range(model.eval(self.variables).as_long())]}
