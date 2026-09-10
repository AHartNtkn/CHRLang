"""Cooperative descriptor store and deterministic atom/unknown CHR execution.

No machine concurrency or source choices: those capabilities require their own gate.
"""
from dataclasses import dataclass
from itertools import product

@dataclass
class Ref:
    old: object
    new: object
    descriptor: object

@dataclass
class Descriptor:
    key: tuple
    group: int
    writes: list
    status: str = 'pending'
    installed: int = 0
    cleaned: int = 0

class Engine:
    def __init__(self,source,ordered,reverse,cancel):
        self.rules=source['rules'];self.outputs=source['outputs'];self.ordered=ordered;self.reverse=reverse;self.cancel=cancel
        for r in self.rules:
            assert r['kept'] or r['removed'],'empty head'
            if any(a[0] not in ['eq','add'] for a in r['body']):raise ValueError('unsupported body form')
        self.cells={('fact',i):c for i,c in enumerate(source['query'])};self.oid=len(source['query']);self.birth={i:i for i in range(self.oid)};self.next_birth=self.oid
        variables=[x for c in source['query'] for x in c[1] if isinstance(x,int)]+self.outputs
        self.fresh=max(variables,default=-1)+1;self.epoch=0;self.descriptors=[];self.trace=[];self.cancelled=False;self.snapshots=0;self.max_live=0
        groups=[{i} for i in range(len(self.rules))]
        for i,a in enumerate(self.rules):
            ah={c[0] for c in a['kept']+a['removed']};aw={c[0] for c in a['removed']}|{x[1][0] for x in a['body'] if x[0]=='add'}
            for j,b in enumerate(self.rules[:i]):
                bh={c[0] for c in b['kept']+b['removed']};bw={c[0] for c in b['removed']}|{x[1][0] for x in b['body'] if x[0]=='add'}
                if aw&bh or bw&ah or any(x[0]=='eq' for x in a['body']+b['body']):
                    ga=next(g for g in groups if i in g);gb=next(g for g in groups if j in g)
                    if ga is not gb:ga.update(gb);groups.remove(gb)
        self.groups=groups
    def logical(self):
        # Catalogue and status changes increment epoch. Cooperative reads cannot
        # overlap a machine write; native synchronization remains unimplemented.
        before=self.epoch;result={}
        for k,v in self.cells.items():
            if isinstance(v,Ref):v=v.new if v.descriptor.status=='committed' else v.old
            if v is not None:result[k]=v
        assert before==self.epoch;return result
    @staticmethod
    def resolve(x,state):
        seen=set()
        while isinstance(x,int) and ('eq',x) in state:
            assert x not in seen;seen.add(x);x=state[('eq',x)]
        return x
    def matches(self,state):
        facts=sorted(((k[1],v) for k,v in state.items() if k[0]=='fact'),key=lambda item:self.birth[item[0]])
        for ri,r in enumerate(self.rules):
            heads=r['kept']+r['removed'];candidates=[[item for item in facts if item[1][0]==h[0] and len(item[1][1])==len(h[1])] for h in heads]
            for chosen in product(*candidates):
                ids=tuple(x[0] for x in chosen)
                if len(set(ids))!=len(ids) or (not r['removed'] and ('hist',(ri,ids)) in state):continue
                sub={};valid=True
                for h,(_,fact) in zip(heads,chosen):
                    for pat,actual in zip(h[1],fact[1]):
                        actual=self.resolve(actual,state)
                        if isinstance(pat,int):
                            if pat in sub and self.resolve(sub[pat],state)!=actual:valid=False
                            else:sub[pat]=actual
                        elif pat!=actual:valid=False
                if valid:yield (ri,ids),sub
    def prepare(self,key,sub,state,group):
        ri,ids=key;r=self.rules[ri];after=dict(state)
        def inst(x):
            if not isinstance(x,int):return x
            if x not in sub:sub[x]=self.fresh;self.fresh+=1
            return sub[x]
        for oid in ids[len(r['kept']):]:after.pop(('fact',oid))
        if not r['removed']:after[('hist',key)]=True
        for a in r['body']:
            if a[0]=='add':
                after[('fact',self.oid)]=[a[1][0],[inst(x) for x in a[1][1]]];self.oid+=1
            else:
                x=self.resolve(inst(a[1]),after);y=self.resolve(inst(a[2]),after)
                if x!=y:
                    if isinstance(x,int):after[('eq',x)]=y
                    elif isinstance(y,int):after[('eq',y)]=x
                    else:raise ValueError('failed equality outside successful-source entry')
        writes=[(k,after.get(k)) for k in sorted(set(state)|set(after),key=repr) if state.get(k)!=after.get(k)]
        d=Descriptor(key,group,writes);d.new_ids=sorted(k[1] for k in after if k[0]=='fact' and k not in state);self.descriptors.append(d);self.epoch+=1;return d
    def answer(self):
        s=self.logical();return [[self.resolve(x,s) for x in self.outputs],[[c[0],[self.resolve(x,s) for x in c[1]]] for _,c in sorted(((k[1],v) for k,v in s.items() if k[0]=='fact'),key=lambda item:self.birth[item[0]])]]
    def run(self):
        for step in range(10000):
            state=self.logical();live=[d for d in self.descriptors if d.cleaned<len(d.writes) or d.status=='pending'];busy={d.group for d in live}
            candidates=list(self.matches(state));first=candidates[0][0] if candidates else None
            for gi,g in enumerate(self.groups):
                if gi not in busy:
                    candidate=next(((key,sub) for key,sub in candidates if key[0] in g),None)
                    if candidate:self.prepare(*candidate,state,gi)
            live=[d for d in self.descriptors if d.cleaned<len(d.writes) or d.status=='pending'];self.max_live=max(self.max_live,len(live))
            if not live:return self.answer()
            assert len(self.descriptors)<=1000 and sum(k[0]=='fact' for k in state)<=1000
            advanced=False
            for d in reversed(live) if self.reverse else live:
                before=self.logical();label=None
                if d.status=='pending':
                    if self.cancel and not self.cancelled and d.installed>0:
                        d.status='aborted';self.epoch+=1;self.cancelled=True;label='abort'
                    elif d.installed<len(d.writes):
                        k,value=d.writes[d.installed];old=self.cells.get(k)
                        assert not isinstance(old,Ref),'overlapping descriptor writes'
                        self.cells[k]=Ref(old,value,d);d.installed+=1;label='install'
                    elif not self.ordered or d.key==next(iter(self.matches(before)),(None,None))[0]:
                        for oid in d.new_ids:self.birth[oid]=self.next_birth;self.next_birth+=1
                        d.status='committed';self.epoch+=1;label='commit'
                elif d.cleaned<d.installed:
                    k,value=d.writes[d.cleaned];ref=self.cells[k];assert isinstance(ref,Ref) and ref.descriptor is d
                    value=ref.new if d.status=='committed' else ref.old
                    if value is None:self.cells.pop(k)
                    else:self.cells[k]=value
                    d.cleaned+=1;label='cleanup'
                elif d.status=='aborted':d.cleaned=len(d.writes);label='release'
                if label:
                    advanced=True;self.trace.append([step,d.key,label]);after=self.logical();self.snapshots+=1
                    if label!='commit':assert before==after,(label,'partial visibility')
            assert advanced,('stalled descriptors',self.trace[-20:])
        raise RuntimeError('service cutoff')
