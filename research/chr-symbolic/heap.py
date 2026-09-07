"""Bounded symbolic heap: guarded allocation and finite-tree unification."""
import z3


def select(values,index):
    result=values[-1]
    for i in reversed(range(len(values)-1)):
        result=z3.If(index==i,values[i],result)
    return result


class Encoding:
    def __init__(self):
        self.solver=z3.Solver()
        self.serial=0

    def bind(self,expression):
        if isinstance(expression,bool):return z3.BoolVal(expression)
        if isinstance(expression,int):return z3.IntVal(expression)
        expression=z3.simplify(expression)
        if z3.is_true(expression) or z3.is_false(expression) or z3.is_int_value(expression):return expression
        self.serial+=1
        value=z3.Const(f'v{self.serial}',expression.sort())
        self.solver.add(value==expression)
        return value


class Heap:
    def __init__(self,encoding,capacity,signature):
        if capacity<1:raise ValueError('positive heap capacity required')
        self.e=encoding;self.n=capacity
        self.signature=list(dict.fromkeys(signature))
        if any(arity<0 for _,arity in self.signature):raise ValueError('negative arity')
        self.arity=max([a for _,a in self.signature],default=0)
        self.tags=[z3.IntVal(0)]*capacity # 0 unused, 1 variable, constructor index+2
        self.children=[[z3.IntVal(0)]*capacity for _ in range(self.arity)]
        self.ids=[z3.IntVal(0)]*capacity
        self.sub=list(map(z3.IntVal,range(capacity)))
        self.count=z3.IntVal(0);self.variables=z3.IntVal(0)
        self.failed=z3.BoolVal(False);self.cutoff=z3.BoolVal(False)

    def active(self,guard=True):return z3.And(guard,z3.Not(self.failed),z3.Not(self.cutoff))

    def allocate(self,tag,args,guard):
        active=self.active(guard)
        fits=self.count<self.n
        valid=z3.And(*[z3.And(c>=0,c<self.count) for c in args])
        self.e.solver.add(z3.Implies(active,valid))
        write=z3.And(active,fits)
        handle=self.count
        self.tags=[self.e.bind(z3.If(z3.And(write,handle==i),tag,old)) for i,old in enumerate(self.tags)]
        self.ids=[self.e.bind(z3.If(z3.And(write,handle==i),self.variables,old)) for i,old in enumerate(self.ids)]
        for child in range(self.arity):
            value=args[child] if child<len(args) else 0
            self.children[child]=[self.e.bind(z3.If(z3.And(write,handle==i),value,old)) for i,old in enumerate(self.children[child])]
        self.variables=self.e.bind(z3.If(z3.And(write,tag==1),self.variables+1,self.variables))
        self.count=self.e.bind(z3.If(write,self.count+1,self.count))
        self.cutoff=self.e.bind(z3.Or(self.cutoff,z3.And(active,z3.Not(fits))))
        return self.e.bind(z3.If(write,handle,0))

    def variable(self,guard=True):return self.allocate(1,[],guard)

    def app(self,name,args,guard=True):
        try:tag=self.signature.index((name,len(args)))+2
        except ValueError:raise ValueError('constructor outside declared signature') from None
        return self.allocate(tag,args,guard)

    def deref(self,handle):
        for _ in range(self.n):handle=select(self.sub,handle)
        return self.e.bind(handle)

    def sizes(self):
        return [select([z3.IntVal(0),z3.IntVal(0)]+[z3.IntVal(a) for _,a in self.signature],tag) for tag in self.tags]

    def occurs(self,variable,handle):
        sizes=self.sizes()
        reachable=[z3.IntVal(i)==variable for i in range(self.n)]
        for _ in range(self.n):
            reachable=[self.e.bind(z3.Or(z3.IntVal(i)==variable,z3.If(self.sub[i]!=i,select(reachable,self.sub[i]),z3.Or(*[z3.And(k<sizes[i],select(reachable,child[i])) for k,child in enumerate(self.children)])))) for i in range(self.n)]
        return select(reachable,handle)

    def unify(self,left,right,steps,guard=True):
        if steps<0:raise ValueError('negative service bound')
        entry=self.active(guard)
        self.e.solver.add(z3.Implies(entry,z3.And(left>=0,left<self.count,right>=0,right<self.count)))
        capacity=1+steps*max(self.arity-1,0)
        xs=[left]+[z3.IntVal(0)]*(capacity-1);ys=[right]+[z3.IntVal(0)]*(capacity-1)
        length=z3.IntVal(1)
        for _ in range(steps):
            active=z3.And(entry,z3.Not(self.failed),length>0)
            a=self.deref(select(xs,length-1));b=self.deref(select(ys,length-1))
            ta=select(self.tags,a);tb=select(self.tags,b)
            same=a==b;va=ta==1;vb=tb==1
            bind_a=z3.And(z3.Not(same),va)
            bind_b=z3.And(z3.Not(same),z3.Not(va),vb)
            constructor=z3.And(z3.Not(same),z3.Not(va),z3.Not(vb))
            bad=z3.Or(z3.And(bind_a,self.occurs(a,b)),z3.And(bind_b,self.occurs(b,a)),z3.And(constructor,ta!=tb))
            success=z3.And(active,z3.Not(bad))
            target=z3.If(bind_a,a,b);value=z3.If(bind_a,b,a)
            self.sub=[self.e.bind(z3.If(z3.And(success,z3.Or(bind_a,bind_b),target==i),value,old)) for i,old in enumerate(self.sub)]
            push=z3.If(constructor,select(self.sizes(),a),0)
            nx=[];ny=[]
            for slot in range(capacity):
                x=xs[slot];y=ys[slot]
                for child in range(self.arity):
                    at=z3.And(success,constructor,child<push,slot==length-1+push-1-child)
                    x=z3.If(at,select(self.children[child],a),x)
                    y=z3.If(at,select(self.children[child],b),y)
                nx.append(self.e.bind(x));ny.append(self.e.bind(y))
            xs,ys=nx,ny
            length=self.e.bind(z3.If(success,length-1+push,length))
            self.failed=self.e.bind(z3.Or(self.failed,z3.And(active,bad)))
        self.cutoff=self.e.bind(z3.Or(self.cutoff,z3.And(entry,z3.Not(self.failed),length>0)))

    def decode(self,model):
        integer=lambda x:model.eval(x).as_long()
        count=integer(self.count)
        tags=[integer(t) for t in self.tags];sub=[integer(t) for t in self.sub]
        def term(i):
            if sub[i]!=i:return term(sub[i])
            if tags[i]==1:return integer(self.ids[i])
            name,arity=self.signature[tags[i]-2]
            return (name,tuple(term(integer(self.children[k][i])) for k in range(arity)))
        status='failed' if z3.is_true(model.eval(self.failed)) else 'cutoff' if z3.is_true(model.eval(self.cutoff)) else 'done'
        return {'status':status,'variables':[term(i) for i in range(count) if tags[i]==1]}
