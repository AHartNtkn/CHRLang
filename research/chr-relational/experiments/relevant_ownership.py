"""Registered allocation/ownership comparison, with separate ordinary replays."""
import hashlib
import itertools
import json
import os
from pathlib import Path
import resource
import subprocess
ROOT=Path.cwd(); BASE=ROOT/'docs/experiments/results/s02-relevant-ownership'; OUT=BASE/'runs'
OUT.mkdir(exist_ok=False)
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
bins={}
for build in ['meter','ordinary']:
    for line in (BASE/f'build-{build}.jsonl').read_text().splitlines():
        r=json.loads(line)
        if r.get('executable') and r['target']['name']=='s02_deduction':
            p=Path(r['executable']);bins[build]={'path':str(p.relative_to(ROOT)),'sha256':digest(p)}
assert set(bins)=={'meter','ordinary'}
paths=subprocess.check_output(['git','ls-files','crates','research','Cargo.toml','Cargo.lock'],text=True).splitlines()
paths=[p for p in paths if p.endswith(('.rs','.toml','.lock'))]
paths+=['research/chr-relational/experiments/relevant_ownership.py','docs/experiments/registrations/S02-relevant-ownership.md']
sources={p:digest(ROOT/p) for p in paths}
(BASE/'freeze.json').write_text(json.dumps({'sources':sources,'binaries':bins},indent=2)+'\n')
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def invoke(build,args,retention,path):
    b=bins[build];assert digest(ROOT/b['path'])==b['sha256']
    r=subprocess.run([str(ROOT/b['path']),*map(str,args)],env=dict(os.environ,DEDUCTION_RETAIN=retention),capture_output=True,text=True,timeout=60,preexec_fn=limits)
    path.write_text(r.stdout+r.stderr)
    assert r.returncode==0,(path,r.returncode,r.stderr)
    return r.stdout
invoke('meter',['meter-check'],'immediate',BASE/'meter-check.log')
def strip(value,semantic=False):
    if isinstance(value,list):return [strip(v,semantic) for v in value]
    if isinstance(value,dict):return {k:strip(v,semantic) for k,v in value.items() if k not in (['ns','first_answer_ns','memory','meter'] if semantic else ['ns','first_answer_ns'])}
    return value
records=[]
modes=['contextual','shared','relevant','persistent-shared','persistent-relevant','scan','lowered']
cells=list(itertools.product(modes,['single','shared','distinct','changed'],[0,16],[1,4],[0,1],['immediate','all']))
configs=[(m,f,d,q,r,h,None) for m,f,d,q,r,h in cells]
configs += [(m,f,16,4,1,h,1) for m,f,h in itertools.product(modes,['single','shared','distinct','changed'],['immediate','all'])]
for index,(mode,family,depth,queries,tokens,hold,cancel) in enumerate(configs):
    args=[mode,family,depth,queries,tokens,0]+([] if cancel is None else [cancel])
    outputs=[]
    for repeat in range(2):
        text=invoke('meter',args,hold,OUT/f'{index}-meter-{repeat}.log')
        result=[json.loads(line) for line in text.splitlines() if line.startswith('{')][-1]
        assert result['event']=='result' and result['meter'] and not result['counters']
        outputs.append(result)
    assert strip(outputs[0])==strip(outputs[1]),index
    if cancel is None:
        text=invoke('ordinary',args,hold,OUT/f'{index}-ordinary.log')
        ordinary=[json.loads(line) for line in text.splitlines() if line.startswith('{')][-1]
        assert not ordinary['meter'] and strip(ordinary,True)==strip(outputs[0],True)
        assert all(s['complete'] for s in outputs[0]['samples'])
    else:
        assert [s['complete'] for s in outputs[0]['samples']]==[False,True,False,True]
    result=outputs[0]
    phases=[result[k] for k in ['source_build','preparation','prepared_drop','consumer_drop']]
    for sample in result['samples']:
        phases += [sample[k] for k in ['input_build','setup','execute_observe','engine_drop','answer_drop','answer_hold'] if sample[k] is not None]
    baseline=phases[0]['memory']['live_start']
    assert result['consumer_drop']['memory']['live_end']==baseline
    records.append(dict(index=index,mode=mode,family=family,depth=depth,queries=queries,tokens=tokens,retention=hold,cancel=cancel,requested_bytes=sum(p['memory']['requested_bytes'] for p in phases),allocation_calls=sum(p['memory']['allocation_calls'] for p in phases),peak_excess=max(p['memory']['peak_live'] for p in phases)-baseline,result=result))
    if (index+1)%50==0:print('validated cells',index+1,flush=True)
    (BASE/'results.json').write_text(json.dumps(records,indent=2)+'\n')
assert len(records)==504
assert all(digest(ROOT/p)==sha for p,sha in sources.items())
(BASE/'audit.json').write_text(json.dumps({'cells':504,'metered_processes':1008,'ordinary_processes':448,'exact_allocation_repeats':True,'restoration':True,'primary_timing':False},indent=2)+'\n')
print('All 1008 metered and 448 ordinary processes passed.',flush=True)
