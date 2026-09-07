"""Read-only finite-tree views for symbolic head matching and equality guards.

Datatype expressions are determined by the heap. Free source variables are Hole
constructors with explicit IDs, not unconstrained solver-selected ground terms.
"""
import z3
from heap import select

_SORTS={}


def tree_sort(signature):
    key=tuple(signature)
    if key not in _SORTS:
        name=f'CHRTree{len(_SORTS)}'
        declaration=z3.Datatype(name)
        declaration.declare(f'{name}_hole',(f'{name}_id',z3.IntSort()))
        for i,(_,arity) in enumerate(signature):
            declaration.declare(f'{name}_c{i}',*[(f'{name}_c{i}_{j}',declaration) for j in range(arity)])
        _SORTS[key]=declaration.create()
    return _SORTS[key]


class View:
    def __init__(self,heap):
        self.h=heap;self.sort=tree_sort(heap.signature)
        self.hole=self.sort.constructor(0)
        self.next_var=heap.variables
        values=[self.hole(i) for i in heap.ids]
        # A valid allocated heap has no cycle after combining constructor and
        # substitution edges. n rounds cover every path from an allocated root.
        for _ in range(heap.n):
            updated=[]
            for i in range(heap.n):
                value=self.hole(heap.ids[i])
                for tag,(_,arity) in enumerate(heap.signature):
                    app=self.sort.constructor(tag+1)(*[select(values,heap.children[k][i]) for k in range(arity)])
                    value=z3.If(heap.tags[i]==tag+2,app,value)
                updated.append(heap.e.bind(z3.If(heap.sub[i]!=i,select(values,heap.sub[i]),value)))
            values=updated
        self.values=values

    def match(self,pattern,handle,bindings=None):
        bindings={} if bindings is None else dict(bindings)
        def visit(p,value):
            value=self.h.deref(value)
            if isinstance(p,int):
                if p in bindings:return select(self.values,bindings[p])==select(self.values,value)
                bindings[p]=value
                return z3.BoolVal(True)
            name,args=p
            if (name,len(args)) not in self.h.signature:return z3.BoolVal(False)
            tag=self.h.signature.index((name,len(args)))+2
            return z3.And(select(self.h.tags,value)==tag,*[visit(child,select(self.h.children[k],value)) for k,child in enumerate(args)])
        return self.h.e.bind(visit(pattern,handle)),bindings

    def guard(self,left,right,bindings):
        local={v:select(self.values,handle) for v,handle in bindings.items()}
        fresh=0
        def instantiate(t):
            nonlocal fresh
            if isinstance(t,int):
                if t not in local:
                    local[t]=self.hole(self.next_var+fresh);fresh+=1
                return local[t]
            name,args=t
            if (name,len(args)) not in self.h.signature:raise ValueError('guard constructor outside signature')
            index=self.h.signature.index((name,len(args)))+1
            return self.sort.constructor(index)(*[instantiate(c) for c in args])
        return self.h.e.bind(instantiate(left)==instantiate(right))


from dataclasses import dataclass
from itertools import permutations


@dataclass
class Application:
    rule: int
    occurrences: tuple
    bindings: dict
    selected: object


def applications(heap,rules,roots,alive,history):
    """One-hot committed selection over fixed occurrence slots.

    Slot indices are monotone occurrence identities. Inactive slots cannot match;
    permutations exclude using the same occurrence twice. This emits factored
    enabling predicates, not a table of concrete reachable stores.
    """
    if len(roots)!=len(alive):raise ValueError('occurrence vectors differ')
    view=View(heap)
    found=z3.BoolVal(False)
    result=[]
    for ri,rule in enumerate(rules):
        heads=rule.kept+rule.removed
        if not heads:raise ValueError('empty head')
        for ids in permutations(range(len(roots)),len(heads)):
            bindings={}
            enabled=[alive[i] for i in ids]
            enabled.append(z3.Not(history.get((ri,ids),z3.BoolVal(False))))
            for head,oi in zip(heads,ids):
                ok,bindings=view.match(head,roots[oi],bindings)
                enabled.append(ok)
            enabled.extend(view.guard(a,b,bindings) for a,b in rule.guards)
            ready=heap.e.bind(z3.And(*enabled))
            selected=heap.e.bind(z3.And(z3.Not(found),ready))
            result.append(Application(ri,ids,bindings,selected))
            found=heap.e.bind(z3.Or(found,ready))
    return result
