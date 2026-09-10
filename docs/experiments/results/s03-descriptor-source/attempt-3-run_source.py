from pathlib import Path
import hashlib,importlib.util,itertools,json,resource,sys
sys.dont_write_bytecode=True
from source import Engine
ROOT=Path(__file__).resolve().parents[2];RAW=ROOT/'docs/experiments/results/s03-descriptor-source/attempt-3'
def load(name,path):
    s=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(s);sys.modules[name]=m;s.loader.exec_module(m);return m
cases=load('cases',ROOT/'research/chr-hvm/source_identity/cases.py');gate=load('identity_gate',ROOT/'research/chr-hvm/source_identity/gate.py')
def ordered_normalize(a):
    out,res=a;vs=sorted({x for x in out if isinstance(x,int)}|{x for _,args in res for x in args if isinstance(x,int)})
    values=[]
    for perm in itertools.permutations(range(len(vs))):
        d=dict(zip(vs,perm));term=lambda x:('v',d[x]) if isinstance(x,int) else ('a',x)
        values.append((tuple(map(term,out)),tuple((n,tuple(map(term,args))) for n,args in res)))
    return min(values)
if __name__=='__main__':
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
    sources=list(cases.cases());c=cases.c;rule=cases.rule;add=cases.add
    sources += [dict(name='ground-disjoint',rules=[rule([],[c('a')],[add('left')]),rule([],[c('b')],[add('right')])],query=[c('a'),c('b')],outputs=[],expected=[[],[c('left'),c('right')]]),dict(name='recursive-order',rules=[rule([],[c('a')],[add('next')]),rule([],[c('next')],[add('left')]),rule([],[c('b')],[add('right')])],query=[c('a'),c('b')],outputs=[],expected=[[],[c('left'),c('right')]])]
    assert len(sources)==79
    paths=[Path(__file__),Path(__file__).with_name('source.py'),ROOT/'research/chr-hvm/source_identity/cases.py',ROOT/'research/chr-hvm/source_identity/gate.py',ROOT/'target/debug/examples/native_identity_reference',ROOT/'docs/experiments/registrations/S03-descriptor-source.md']
    p=RAW/'manifest.json';assert not p.exists();p.write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths},indent=2)+'\n');(RAW/'cases.json').write_text(json.dumps(sources,indent=2)+'\n')
    native={r['case']:r['answer'] for r in json.loads((RAW.parent/'native-controls.json').read_text())}
    refs=[];rows=[]
    try:
        for s in sources:
            reference,receipt=gate.reference(s);assert gate.normalize(reference)==gate.normalize(s['expected']),s['name'];refs.append(dict(case=s['name'],answer=reference,receipt=receipt))
            for ordered,reverse,cancel in itertools.product([False,True],[False,True],[False,True]):
                e=Engine(s,ordered,reverse,cancel);answer=e.run();match=ordered_normalize(answer)==ordered_normalize(native[s['name']])
                assert gate.normalize(answer)==gate.normalize(reference),('canonical mismatch',s['name'],answer,reference)
                row=dict(case=s['name'],ordered=ordered,reverse=reverse,cancel=cancel,matches_ordered_reference=match,answer=answer,descriptors=len(e.descriptors),max_live=e.max_live,snapshots=e.snapshots,cancelled=e.cancelled,trace=e.trace if not match else [])
                rows.append(row)
                if ordered:assert match,row
        result=dict(status='passed',runs=len(rows),rows=rows)
    except Exception as ex:
        (RAW/'failure.json').write_text(json.dumps(dict(error=repr(ex),rows=rows,refs=refs),indent=2)+'\n');raise
    (RAW/'references.json').write_text(json.dumps(refs,indent=2)+'\n');(RAW/'results.json').write_text(json.dumps(result,indent=2)+'\n');print('passed',len(rows),'runs')
