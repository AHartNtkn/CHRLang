"""Concrete views of literal or model-evaluated heap fields."""
import z3


class ConcreteHeap:
    def __init__(self,heap,model=None):
        self.signature=heap.signature
        def integer(v):
            if model is not None:v=model.eval(v)
            if not z3.is_int_value(v):raise ValueError('heap field is not concrete')
            return v.as_long()
        self.tags=list(map(integer,heap.tags));self.ids=list(map(integer,heap.ids))
        self.sub=list(map(integer,heap.sub))
        self.children=[list(map(integer,row)) for row in heap.children]
        self.memo={};self.visiting=set()

    @classmethod
    def static(cls,heap):
        fields=[*heap.tags,*heap.ids,*heap.sub,*[v for row in heap.children for v in row]]
        if not all(z3.is_int_value(v) for v in fields):return None
        return cls(heap)

    def term(self,index):
        if not 0<=index<len(self.tags):raise ValueError('heap handle outside arena')
        if index in self.memo:return self.memo[index]
        if index in self.visiting:raise ValueError('cyclic decoded heap')
        self.visiting.add(index)
        tag=self.tags[index]
        if tag==0:value=None
        elif self.sub[index]!=index:value=self.term(self.sub[index])
        elif tag==1:value=self.ids[index]
        else:
            name,arity=self.signature[tag-2]
            children=tuple(self.term(self.children[k][index]) for k in range(arity))
            if any(c is None for c in children):raise ValueError('constructor references unused slot')
            value=(name,children)
        self.visiting.remove(index);self.memo[index]=value
        return value
