"""Each transition changes one physical slot, one status, or one observer field."""
from collections import deque

def transitions(s,checked,cancellation):
    status,refs,values,pos,before,reads=s
    # 0 pending, 1 committed, 2 aborted. Four initially zero-valued fields.
    if status==0:
        for field in range(4):
            bit=1<<field
            if not refs&bit:yield f'install {field}',(status,refs|bit,values,pos,before,reads)
        if refs==15:yield 'commit descriptor',(1,refs,values,pos,before,reads)
        if cancellation:yield 'abort descriptor',(2,refs,values,pos,before,reads)
    else:
        for field in range(4):
            bit=1<<field
            if refs&bit:
                new_values=values|bit if status==1 else values
                yield f'materialize {field}',(status,refs&~bit,new_values,pos,before,reads)
    if pos==-1:
        yield 'begin scan',(status,refs,values,0,status,reads)
    elif pos<4:
        bit=1<<pos
        value=int(status==1) if refs&bit else int(bool(values&bit))
        yield f'read {pos}={value}',(status,refs,values,pos+1,before,reads+(value,))
    elif pos==4:
        accepted=not checked or before==status
        yield 'publish scan' if accepted else 'discard scan',(status,refs,values,5 if accepted else 6,before,reads)

def explore_publication(checked,cancellation):
    start=(0,0,0,-1,-1,());q=deque([start]);parents={start:None};bad={};published=set();discard=0;edges=0
    while q:
        s=q.popleft();ts=list(transitions(s,checked,cancellation));edges+=len(ts)
        for action,t in ts:
            if action=='publish scan':
                published.add(t[-1])
                if t[-1] not in [(0,)*4,(1,)*4] and t[-1] not in bad:
                    trace=[action];p=s
                    while parents[p] is not None:p,a=parents[p];trace.append(a)
                    bad[t[-1]]=list(reversed(trace))
            if action=='discard scan':discard+=1
            if t not in parents:
                parents[t]=(s,action);q.append(t);assert len(parents)<=200000,'state cutoff'
    return dict(states=len(parents),edges=edges,published=sorted(published),discard_edges=discard,mixed=[dict(snapshot=k,trace=v) for k,v in sorted(bad.items())])
