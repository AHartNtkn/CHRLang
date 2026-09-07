"""Pure matching for direct terms; private node IDs are not source equality."""
import z3
from matching import tree_sort


class TermView:
    def __init__(self,store):
        self.h=store;self.sort=tree_sort(store.signature)
        self.hole=self.sort.constructor(0);self.cache={}

    def project(self,t,guard=True,depth=None):
        if depth is None:depth=self.h.n
        t=self.h.deref(t);guard=z3.simplify(z3.BoolVal(guard) if isinstance(guard,bool) else guard)
        key=(t.get_id(),guard.get_id(),depth)
        if key in self.cache:return self.cache[key][2]
        value=self.hole(self.h.sort.accessor(0,1)(t))
        for i,(_,arity) in enumerate(self.h.signature):
            recognized=self.h.sort.recognizer(i+1)(t)
            context=z3.And(guard,recognized)
            if self.h.e.impossible(context):continue
            if arity and depth==0:raise ValueError('term projection exceeded finite node bound')
            args=[self.project(self.h.child(t,k),context,depth-1) for k in range(arity)]
            value=z3.If(recognized,self.sort.constructor(i+1)(*args),value)
        value=z3.simplify(value)
        self.cache[key]=(t,guard,value)
        return value

    def match(self,pattern,value,bindings=None):
        bindings={} if bindings is None else dict(bindings)
        def visit(p,t):
            t=self.h.deref(t)
            if isinstance(p,int):
                if p in bindings:return self.project(bindings[p])==self.project(t)
                bindings[p]=t;return z3.BoolVal(True)
            name,args=p
            if (name,len(args)) not in self.h.signature:return z3.BoolVal(False)
            tag=self.h.signature.index((name,len(args)))+2
            return z3.And(self.h.tag(t)==tag,*[visit(child,self.h.child(t,k)) for k,child in enumerate(args)])
        return self.h.e.bind(visit(pattern,value)),bindings

    def guard(self,left,right,bindings):
        local={v:self.project(t) for v,t in bindings.items()};fresh=0
        def instantiate(t):
            nonlocal fresh
            if isinstance(t,int):
                if t not in local:
                    local[t]=self.hole(self.h.variables+fresh);fresh+=1
                return local[t]
            name,args=t;index=self.h.signature.index((name,len(args)))+1
            return self.sort.constructor(index)(*[instantiate(c) for c in args])
        return self.h.e.bind(instantiate(left)==instantiate(right))
