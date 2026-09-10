"""Registered finite dependency ownership matrix; no primary timing analysis."""
import hashlib
import itertools
import json
from pathlib import Path
import resource
import subprocess
ROOT=Path.cwd(); BASE=ROOT/'docs/experiments/results/s03-dependency-ownership'
OUT=BASE/'runs';OUT.mkdir(exist_ok=False)
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def executable(kind):
    rows=[json.loads(line) for line in (BASE/f'build-{kind}.jsonl').read_text().splitlines()]
    return Path(next(r['executable'] for r in rows if r.get('executable') and r['target']['name']=='dependency_ownership'))
binaries={kind:executable(kind) for kind in ['meter','ordinary']}
paths=subprocess.check_output(['git','ls-files','crates','research','Cargo.toml','Cargo.lock'],text=True).splitlines()
paths=[p for p in paths if p.endswith(('.rs','.toml','.lock'))]
paths+=['research/chr-direct-conditional/examples/dependency_ownership.rs','research/chr-direct-conditional/tests/runtime_support/dependency_source.rs','research/chr-direct-conditional/experiments/dependency_ownership.py','docs/experiments/registrations/S03-dependency-ownership.md']
freeze={'sources':{p:digest(ROOT/p) for p in paths},'binaries':{k:{'path':str(p.relative_to(ROOT)),'sha256':digest(p)} for k,p in binaries.items()}}
(BASE/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def invoke(kind,args,path):
    assert digest(binaries[kind])==freeze['binaries'][kind]['sha256']
    r=subprocess.run([str(binaries[kind]),*map(str,args)],capture_output=True,text=True,timeout=60,preexec_fn=limits)
    path.write_text(r.stdout+r.stderr)
    assert r.returncode==0,(path,r.returncode,r.stderr)
    return r.stdout
invoke('meter',['meter-check'],BASE/'meter-check.log')
def normalized(value):
    if isinstance(value,list):return [normalized(x) for x in value]
    if isinstance(value,dict):return {k:normalized(v) for k,v in value.items() if k!='ns'}
    return value
modes=['current','current-miss','birth','birth-miss','dependencies','dependencies-miss','scan','indexed']
cells=list(itertools.product(['plain','known-hit','known-miss','delayed-hit','delayed-miss'],[8,128],[False,True],modes,['immediate','window','all'],[False,True]))
assert len(cells)==960
records=[]
for i,(family,size,reverse,mode,retention,cancel) in enumerate(cells):
    args=[mode,family,size,str(reverse).lower(),retention,str(cancel).lower()]
    results=[]
    for kind,repeat in [('meter',0),('meter',1),('ordinary',0)]:
        stdout=invoke(kind,args,OUT/f'{i}-{kind}-{repeat}.log')
        rows=[json.loads(line) for line in stdout.splitlines() if line.startswith('{')]
        r=next(row for row in rows if row.get('event')=='result');results.append(r)
        assert len(r['endpoints'])==4
        assert all(e['complete'] for j,e in enumerate(r['endpoints']) if not cancel or j%2==1)
    assert normalized(results[0])==normalized(results[1]),i
    assert results[0]['endpoints']==results[2]['endpoints'],i
    readings=[row['reading']['memory'] for row in results[0]['phases']]
    baseline=readings[0]['live_start'];assert readings[-1]['live_end']==baseline
    records.append({'index':i,'family':family,'size':size,'reverse':reverse,'mode':mode,'retention':retention,'cancel':cancel,
                    'requested_bytes':sum(r['requested_bytes'] for r in readings),'peak_excess':max(r['peak_live'] for r in readings)-baseline,'result':results[0]})
    (BASE/'results.json').write_text(json.dumps(records,indent=2)+'\n')
    if (i+1)%40==0:print('qualified cells',i+1,flush=True)
assert all(digest(ROOT/p)==sha for p,sha in freeze['sources'].items())
(BASE/'audit.json').write_text(json.dumps({'cells':960,'meter_processes':1920,'ordinary_processes':960,'exact_diagnostic_repeats':True,'ordinary_endpoints_match':True,'final_live_restored':True,'primary_timing':False},indent=2)+'\n')
print('All 2880 lifecycle processes qualified.',flush=True)
