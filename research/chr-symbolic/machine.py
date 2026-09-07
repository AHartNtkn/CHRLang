"""Independent tree-based CHR transitions for E11 witness checking.

This copying runner is a semantic control; its costs are not yet a registered
performance baseline. Terms are variable integers or (constructor, children).
"""
from collections import deque
from copy import deepcopy
from dataclasses import dataclass
from itertools import permutations


@dataclass(frozen=True)
class Rule:
    kept: tuple
    removed: tuple
    body: tuple
    guards: tuple = ()


def resolve(term, subst):
    if isinstance(term,int):
        return resolve(subst[term],subst) if term in subst else term
    return (term[0],tuple(resolve(c,subst) for c in term[1]))


def unify(left,right,subst):
    def occurs(v,x):
        return v==x if isinstance(x,int) else any(occurs(v,c) for c in x[1])
    work=[(left,right)]
    while work:
        a,b=(resolve(t,subst) for t in work.pop())
        if a==b:continue
        if isinstance(a,int):
            if occurs(a,b):return False
            subst[a]=b
        elif isinstance(b,int):
            if occurs(b,a):return False
            subst[b]=a
        elif a[0]!=b[0] or len(a[1])!=len(b[1]):return False
        else:work.extend(reversed(list(zip(a[1],b[1]))))
    return True


def match(pattern,value,bindings):
    if isinstance(pattern,int):
        if pattern in bindings:return bindings[pattern]==value
        bindings[pattern]=value
        return True
    return not isinstance(value,int) and pattern[0]==value[0] and len(pattern[1])==len(value[1]) and all(match(a,b,bindings) for a,b in zip(pattern[1],value[1]))


class Renaming:
    def __init__(self,start=0,bindings=None):
        self.next=start
        self.bindings={} if bindings is None else dict(bindings)

    def term(self,t):
        if isinstance(t,int):
            if t not in self.bindings:
                self.bindings[t]=self.next
                self.next+=1
            return self.bindings[t]
        return (t[0],tuple(self.term(c) for c in t[1]))

    def goal(self,g):
        kind=g[0]
        if kind=='post':return (kind,self.term(g[1]))
        if kind=='eq':return (kind,self.term(g[1]),self.term(g[2]))
        if kind in ('and','or'):return (kind,*(self.goal(c) for c in g[1:]))
        if kind in ('true','fail'):return g
        raise ValueError(f'unknown goal {kind}')


class Branch:
    def __init__(self,constraints,outputs):
        rename=Renaming()
        self.pending=deque(('post',rename.term(c)) for c in constraints)
        self.outputs=[rename.term(t) for t in outputs]
        self.next_var=rename.next
        self.next_occurrence=0
        self.store=[]
        self.sub={}
        self.history=set()
        self.trace=[]

    def snapshot(self):
        return deepcopy({
            'pending': list(self.pending), 'store': self.store,
            'substitution': self.sub, 'history': sorted(self.history),
            'next_var': self.next_var, 'next_occurrence': self.next_occurrence,
            'outputs': self.outputs,
        })

    def step(self,rules):
        if self.pending:
            goal=self.pending.popleft()
            kind=goal[0]
            self.trace.append((kind,))
            if kind=='post':
                self.store.append((self.next_occurrence,goal[1]))
                self.next_occurrence+=1
            elif kind=='eq':
                if not unify(goal[1],goal[2],self.sub):return ('failed',)
            elif kind=='and':self.pending.extendleft(reversed(goal[1:]))
            elif kind=='or':return ('split',goal[1],goal[2])
            elif kind=='fail':return ('failed',)
            elif kind!='true':raise ValueError(f'unknown goal {kind}')
            return ('continue',)
        for index,rule in enumerate(rules):
            heads=rule.kept+rule.removed
            for selected in permutations(self.store,len(heads)):
                ids=tuple(o[0] for o in selected)
                token=(index,ids)
                if token in self.history:continue
                bindings={}
                if not all(match(h,resolve(o[1],self.sub),bindings) for h,o in zip(heads,selected)):continue
                rename=Renaming(self.next_var,bindings)
                if not all(resolve(rename.term(a),self.sub)==resolve(rename.term(b),self.sub) for a,b in rule.guards):continue
                body=rename.goal(rule.body)
                removed=set(ids[len(rule.kept):])
                self.store=[o for o in self.store if o[0] not in removed]
                self.history.add(token)
                self.next_var=rename.next
                self.pending.append(body)
                self.trace.append(('apply',index,ids))
                return ('continue',)
        self.trace.append(('answer',))
        return ('answer',{'outputs':[resolve(t,self.sub) for t in self.outputs],'residual':[resolve(c,self.sub) for _,c in self.store]})


class Search:
    def __init__(self,rules,constraints,outputs):
        if any(not r.kept and not r.removed for r in rules):raise ValueError('empty rule head')
        self.rules=rules
        self.frontier=deque([Branch(constraints,outputs)])
        self.steps=0
        self.completed=[]
        self.failed=[]

    @property
    def exhausted(self):return not self.frontier

    def advance(self,budget):
        answers=[]
        for _ in range(budget):
            if not self.frontier:break
            branch=self.frontier.popleft()
            self.steps+=1
            outcome=branch.step(self.rules)
            if outcome[0]=='continue':self.frontier.append(branch)
            elif outcome[0]=='failed':self.failed.append(branch)
            elif outcome[0]=='answer':
                answers.append(outcome[1]);self.completed.append(branch)
            else:
                sibling=deepcopy(branch)
                branch.pending.appendleft(outcome[1]);branch.trace[-1]=('or',False)
                sibling.pending.appendleft(outcome[2]);sibling.trace[-1]=('or',True)
                self.frontier.extend([branch,sibling])
        return answers


def verify_witness(rules,constraints,outputs,states,actions):
    """Replay every decoded state under the committed policy; require an answer.

    Explicit OR action bits select one child. Every other action must equal the
    independently selected action. A bound-ending active state is not an answer.
    """
    if len(states)!=len(actions)+1:raise ValueError('witness state/action lengths')
    branch=Branch(constraints,outputs)
    if states[0]!=branch.snapshot():raise ValueError('initial state differs')
    answer=None
    for index,action in enumerate(actions):
        if answer is not None:raise ValueError('transition after endpoint')
        outcome=branch.step(rules)
        if outcome[0]=='split':
            if action not in [('or',False),('or',True)]:raise ValueError('missing explicit choice')
            branch.pending.appendleft(outcome[2] if action[1] else outcome[1])
        elif branch.trace[-1]!=action:raise ValueError('action violates committed transition')
        if outcome[0]=='failed':raise ValueError('failed witness has no answer')
        if outcome[0]=='answer':answer=outcome[1]
        if states[index+1]!=branch.snapshot():raise ValueError(f'state differs after transition {index}')
    if answer is None:raise ValueError('witness endpoint is not quiescent')
    return answer
