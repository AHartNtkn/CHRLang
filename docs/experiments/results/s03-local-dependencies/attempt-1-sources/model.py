"""Finite claim protocol. Atomic commit visibility is an explicit model assumption."""
from dataclasses import dataclass
from collections import deque

@dataclass(frozen=True)
class App:
    take: tuple
    keep: tuple = ()
    needs_binding: bool = False
    binds: bool = False

CASES = {
    'disjoint': (3, False, (App((0,)), App((1,)))),
    'contended': (1, False, (App((0,)), App((0,)))),
    'overlap': (3, False, (App((0,1)), App((1,2)))),
    'kept-reader': (2, False, (App((1,), (0,)), App((0,)))),
    'late-equality': (2, False, (App((0,), needs_binding=True), App((1,), binds=True), App((0,)))),
    'initial-equality': (2, True, (App((0,), needs_binding=True), App((1,), binds=True), App((0,)))),
}

def initial(case):
    count,binding,apps=case
    return ((1,)*count,binding,(0,)*len(apps),(-1,)*count)

def enabled(case,state,i):
    alive,binding,status,locks=state;a=case[2][i]
    return status[i]==0 and (binding or not a.needs_binding) and all(alive[r] for r in a.take+a.keep)

def observation(state):
    alive,binding,status,_=state
    return alive,binding,tuple(i for i,s in enumerate(status) if s==1),tuple(i for i,s in enumerate(status) if s==2)

def successors(case,state,cancel,ordered):
    alive,binding,status,locks=state;apps=case[2]
    eligible=[i for i in range(len(apps)) if enabled(case,state,i)]
    selected=eligible[:1] if ordered else eligible
    for i,a in enumerate(apps):
        if status[i]!=0:continue
        if i==cancel:
            ns=list(status);ns[i]=2
            yield f'cancel {i}',(alive,binding,tuple(ns),tuple(-1 if owner==i else owner for owner in locks))
        if i not in selected:continue
        required=sorted(set(a.take+a.keep))
        missing=[r for r in required if locks[r]!=i]
        if missing:
            r=missing[0]
            if locks[r]==-1:
                ll=list(locks);ll[r]=i
                yield f'acquire {i}:{r}',(alive,binding,status,tuple(ll))
        else:
            aa=list(alive)
            for r in a.take:aa[r]=0
            ns=list(status);ns[i]=1
            yield f'commit {i}',(tuple(aa),binding or a.binds,tuple(ns),tuple(-1 if owner==i else owner for owner in locks))
    # A contender that lost a consumed head must release any partial reservation.
    for i,a in enumerate(apps):
        if status[i]==0 and any(not alive[r] for r in a.take+a.keep) and i in locks:
            yield f'release-invalid {i}',(alive,binding,status,tuple(-1 if x==i else x for x in locks))

def terminal(case,state,cancel):
    return not any(enabled(case,state,i) for i in range(len(case[2]))) and not any(x!=-1 for x in state[3]) and (cancel is None or state[2][cancel]!=0)

def explore(case,cancel,ordered):
    start=initial(case);parents={start:None};queue=deque([start]);edges={};outcomes=set();partial=0
    while queue:
        s=queue.popleft();trans=list(successors(case,s,cancel,ordered));edges[s]=[t for _,t in trans]
        alive,binding,status,locks=s
        assert all(owner==-1 or status[owner]==0 for owner in locks)
        assert all(alive[r] or owner==-1 for r,owner in enumerate(locks))
        if terminal(case,s,cancel):outcomes.add(observation(s));assert not trans
        else:assert trans,('deadlock',s)
        for action,t in trans:
            assert all(not x or y for x,y in zip(t[0],alive)),('resurrection',s,t)
            if action.startswith('cancel') and cancel in locks:partial+=1
            if t not in parents:
                parents[t]=(s,action);queue.append(t)
                assert len(parents)<=200000,'state cutoff'
    # Kahn's algorithm proves the explored graph has no cycle; with no deadlocks,
    # every maximal execution terminates under this finite, no-retry action model.
    incoming={s:0 for s in edges}
    for targets in edges.values():
        for t in targets:incoming[t]+=1
    q=deque(s for s,n in incoming.items() if n==0);visited=0
    while q:
        s=q.popleft();visited+=1
        for t in edges[s]:
            incoming[t]-=1
            if incoming[t]==0:q.append(t)
    assert visited==len(edges),'cycle'
    traces={}
    for s in parents:
        if terminal(case,s,cancel):
            trace=[];t=s
            while parents[t] is not None:
                t,action=parents[t];trace.append(action)
            traces.setdefault(observation(s),list(reversed(trace)))
    return outcomes,len(parents),sum(map(len,edges.values())),partial,traces
