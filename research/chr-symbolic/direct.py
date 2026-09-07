"""Bounded scalar control with immutable arena nodes and copied branch metadata.

Independent of symbolic services; private node identity preserves pair-step cost.
No traces, solver objects, or completed branches are retained by this runner.
"""
from collections import deque
from copy import copy
from itertools import permutations
from check_cases import equivalent


class Cutoff(Exception):
    pass


class Arena:
    def __init__(self,capacity):
        self.capacity=capacity
        self.nodes=[]
        self.sub={}
        self.variables=0

    def fork(self):
        other=copy(self)
        other.nodes=list(self.nodes)
        other.sub=dict(self.sub)
        return other

    def allocate(self,node):
        if len(self.nodes)>=self.capacity:raise Cutoff()
        handle=len(self.nodes);self.nodes.append(node)
        return handle

    def term(self,t,bindings):
        if isinstance(t,int):
            if t not in bindings:
                bindings[t]=self.allocate(self.variables)
                self.variables+=1
            return bindings[t]
        return self.allocate((t[0],tuple(self.term(c,bindings) for c in t[1])))

    def goal(self,g,bindings):
        if g[0]=='post':return ('post',self.term(g[1],bindings))
        if g[0]=='eq':return ('eq',self.term(g[1],bindings),self.term(g[2],bindings))
        if g[0] in ('and','or'):return (g[0],*(self.goal(c,bindings) for c in g[1:]))
        return g

    def deref(self,h):
        while h in self.sub:h=self.sub[h]
        return h

    def value(self,h):
        t=self.nodes[self.deref(h)]
        return t if isinstance(t,int) else (t[0],tuple(self.value(c) for c in t[1]))

    def occurs(self,v,h):
        stack=[h];seen=set()
        while stack:
            h=self.deref(stack.pop())
            if h==v:return True
            if h in seen:continue
            seen.add(h);t=self.nodes[h]
            if not isinstance(t,int):stack.extend(t[1])
        return False

    def unify(self,a,b,budget):
        stack=[(a,b)]
        for _ in range(budget):
            if not stack:return True
            a,b=(self.deref(h) for h in stack.pop())
            if a==b:continue
            left,right=self.nodes[a],self.nodes[b]
            if isinstance(left,int):
                if self.occurs(a,b):return False
                self.sub[a]=b
            elif isinstance(right,int):
                if self.occurs(b,a):return False
                self.sub[b]=a
            elif left[0]!=right[0] or len(left[1])!=len(right[1]):return False
            else:stack.extend(reversed(list(zip(left[1],right[1]))))
        if stack:raise Cutoff()
        return True

    def match(self,p,h,bindings):
        h=self.deref(h)
        if isinstance(p,int):
            if p in bindings:return self.value(bindings[p])==self.value(h)
            bindings[p]=h;return True
        t=self.nodes[h]
        return not isinstance(t,int) and p[0]==t[0] and len(p[1])==len(t[1]) and all(self.match(a,b,bindings) for a,b in zip(p[1],t[1]))

    def guards(self,guards,bindings):
        local={v:self.value(h) for v,h in bindings.items()}
        fresh=self.variables
        def term(t):
            nonlocal fresh
            if isinstance(t,int):
                if t not in local:local[t]=fresh;fresh+=1
                return local[t]
            return (t[0],tuple(term(c) for c in t[1]))
        return all(term(a)==term(b) for a,b in guards)


class State:
    def __init__(self,constraints,outputs,nodes):
        self.arena=Arena(nodes);bindings={}
        self.queue=deque(('post',self.arena.term(c,bindings)) for c in constraints)
        self.outputs=[self.arena.term(t,bindings) for t in outputs]
        self.store=[];self.history=set();self.next_occ=0;self.depth=0

    def fork(self):
        other=copy(self);other.arena=self.arena.fork()
        other.queue=deque(self.queue);other.store=list(self.store)
        other.history=set(self.history)
        return other


class Direct:
    def __init__(self,rules,constraints,outputs,*,transitions,nodes,occurrences,pending,service):
        if min(transitions,service)<0 or min(nodes,occurrences,pending)<1:raise ValueError('invalid bounds')
        if any(not r.kept and not r.removed for r in rules):raise ValueError('empty head')
        self.result=[];self.models=0;self.steps=0;self.resource=False;self.transition=False
        try:
            state=State(constraints,outputs,nodes)
            if len(state.queue)>pending:raise Cutoff()
        except Cutoff:
            self.resource=True;return
        frontier=deque([state])
        while frontier:
            state=frontier.popleft()
            if state.depth>=transitions:self.transition=True;continue
            self.steps+=1;state.depth+=1
            arena=state.arena
            try:
                if state.queue:
                    goal=state.queue.popleft();kind=goal[0]
                    if kind=='post':
                        if state.next_occ>=occurrences:raise Cutoff()
                        state.store.append((state.next_occ,goal[1]));state.next_occ+=1
                    elif kind=='eq':
                        if not arena.unify(goal[1],goal[2],service):continue
                    elif kind=='fail':continue
                    elif kind=='and':state.queue.extendleft(reversed(goal[1:]))
                    elif kind=='or':
                        sibling=state.fork()
                        state.queue.appendleft(goal[1]);sibling.queue.appendleft(goal[2])
                        frontier.append(sibling)
                    elif kind!='true':raise ValueError('unknown goal')
                    if len(state.queue)>pending:raise Cutoff()
                    frontier.append(state);continue
                chosen=None
                for index,rule in enumerate(rules):
                    for selected in permutations(state.store,len(rule.kept+rule.removed)):
                        ids=tuple(i for i,_ in selected);bindings={}
                        if (index,ids) in state.history:continue
                        if not all(arena.match(p,h,bindings) for p,(_,h) in zip(rule.kept+rule.removed,selected)):continue
                        if arena.guards(rule.guards,bindings):chosen=(index,rule,ids,bindings);break
                    if chosen is not None:break
                if chosen is None:
                    self.models+=1
                    answer={'outputs':[arena.value(h) for h in state.outputs],
                            'residual':[arena.value(h) for _,h in state.store]}
                    if not any(equivalent(answer,a) for a in self.result):self.result.append(answer)
                    continue
                index,rule,ids,bindings=chosen
                # Commit guard-only variables without allocating guard constructors.
                def fresh(t):
                    if isinstance(t,int):arena.term(t,bindings)
                    else:
                        for c in t[1]:fresh(c)
                for a,b in rule.guards:fresh(a);fresh(b)
                body=arena.goal(rule.body,bindings)
                state.queue.append(body)
                removed=set(ids[len(rule.kept):])
                state.store=[o for o in state.store if o[0] not in removed]
                state.history.add((index,ids));frontier.append(state)
            except Cutoff:self.resource=True

    def answers(self):return self.result

    def boundary_status(self):
        return {'resource_cutoff':self.resource,'transition_cutoff':self.transition}
