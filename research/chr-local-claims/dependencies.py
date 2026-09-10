"""Static potential-dependency components; source inference is not implemented."""
from collections import deque
from model import initial,enabled,successors,terminal,observation

def components(case,bindings):
    apps=case[2];groups=[{i} for i in range(len(apps))]
    for i,a in enumerate(apps):
        for j,b in enumerate(apps[:i]):
            conflict=bool(set(a.take)&set(b.take+b.keep) or set(b.take)&set(a.take+a.keep))
            conflict |= bindings and ((a.binds and b.needs_binding) or (b.binds and a.needs_binding))
            if conflict:
                ga=next(g for g in groups if i in g);gb=next(g for g in groups if j in g)
                if ga is not gb:ga.update(gb);groups.remove(gb)
    return groups

def explore_components(case,cancel,bindings):
    groups=components(case,bindings);start=initial(case);parents={start:None};q=deque([start]);out={};edges=0;parallel=0;deadlocks=[]
    while q:
        s=q.popleft();selected=set()
        for group in groups:
            ready=[i for i in sorted(group) if enabled(case,s,i)]
            selected.update(ready[:1])
        parallel+=len(selected)>1
        ts=[]
        for action,t in successors(case,s,cancel,False):
            if action.startswith(('acquire','commit')) and int(action.split()[1].split(':')[0]) not in selected:continue
            ts.append((action,t))
        if terminal(case,s,cancel):
            trace=[];t=s
            while parents[t] is not None:t,action=parents[t];trace.append(action)
            out.setdefault(observation(s),list(reversed(trace)))
            assert not ts
        elif not ts:
            trace=[];t=s
            while parents[t] is not None:t,action=parents[t];trace.append(action)
            deadlocks.append(dict(state=s,trace=list(reversed(trace))))
        edges+=len(ts)
        for action,t in ts:
            if t not in parents:
                parents[t]=(s,action);q.append(t)
                assert len(parents)<=200000,'state cutoff'
    return out,len(parents),edges,parallel,[sorted(g) for g in groups],deadlocks
