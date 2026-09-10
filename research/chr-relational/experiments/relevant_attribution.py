"""Check diagnostic attribution against the frozen lifecycle allocation comparison."""
import hashlib
import json
import os
from pathlib import Path
import resource
import subprocess
ROOT=Path.cwd();BASE=ROOT/'docs/experiments/results/s02-relevant-attribution';OUT=BASE/'runs';OUT.mkdir(exist_ok=False)
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
exe=None
for line in (BASE/'build.jsonl').read_text().splitlines():
    r=json.loads(line)
    if r.get('executable') and r['target']['name']=='s02_deduction':exe=Path(r['executable'])
assert exe
paths=subprocess.check_output(['git','ls-files','crates','research','Cargo.toml','Cargo.lock'],text=True).splitlines()
paths=[p for p in paths if p.endswith(('.rs','.toml','.lock'))]
paths+=['research/chr-relational/src/deduction_profile.rs','research/chr-relational/examples/support/deduction_profile.rs','research/chr-relational/experiments/relevant_attribution.py','docs/experiments/registrations/S02-relevant-attribution.md']
sources={p:digest(ROOT/p) for p in paths};binary_hash=digest(exe)
(BASE/'freeze.json').write_text(json.dumps({'sources':sources,'binary':{'path':str(exe.relative_to(ROOT)),'sha256':binary_hash}},indent=2)+'\n')
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def invoke(args,retention,path):
    assert digest(exe)==binary_hash
    r=subprocess.run([str(exe),*map(str,args)],env=dict(os.environ,DEDUCTION_RETAIN=retention),capture_output=True,text=True,timeout=60,preexec_fn=limits)
    path.write_text(r.stdout+r.stderr);assert r.returncode==0,(path,r.returncode,r.stderr)
    return r.stdout
invoke(['meter-check'],'immediate',BASE/'meter-check.log')
def normalize(r):
    baseline=r['source_build']['memory']['live_start']
    def visit(v):
        if isinstance(v,list):return [visit(x) for x in v]
        if isinstance(v,dict):return {k:(x-baseline if k in ['live_start','live_end','peak_live'] else visit(x)) for k,x in v.items() if k not in ['ns','first_answer_ns','attribution']}
        return v
    return visit(r)
old=json.loads((ROOT/'docs/experiments/results/s02-relevant-ownership/results.json').read_text())
selected=[r for r in old if r['cancel'] is None and r['tokens']==1 and r['mode'] in ['contextual','shared','relevant','persistent-shared','persistent-relevant']]
assert len(selected)==160
records=[]
for cell in selected:
    paired=[]
    for repeat in [0,1]:
        text=invoke([cell['mode'],cell['family'],cell['depth'],cell['queries'],1,0],cell['retention'],OUT/f"{cell['index']}-{repeat}.log")
        result=[json.loads(x) for x in text.splitlines() if x.startswith('{')][-1]
        assert normalize(result)==normalize(cell['result']),('instrumentation changed allocation',cell['index'])
        assert result['attribution'] is not None
        for field in ['requested_bytes','allocation_calls','deallocation_calls']:
            eq=sum(x[field] for x in result['attribution'])
            total=sum(s['execute_observe']['memory'][field] for s in result['samples'])
            assert eq<=total,(cell['index'],field,eq,total)
        paired.append(result['attribution'])
    assert paired[0]==paired[1],cell['index']
    record={k:cell[k] for k in ['index','mode','family','depth','queries','tokens','retention','requested_bytes','peak_excess']}
    record['phases']=paired[0];record['execution_bytes']=sum(s['execute_observe']['memory']['requested_bytes'] for s in cell['result']['samples'])
    records.append(record)
    (BASE/'results.json').write_text(json.dumps(records,indent=2)+'\n')
    if len(records)%25==0:print('verified cells',len(records),flush=True)
assert all(digest(ROOT/p)==sha for p,sha in sources.items())
(BASE/'audit.json').write_text(json.dumps({'cells':160,'processes':320,'normalized_allocation_matches_prior':True,'attribution_repeats':True,'nested_totals_within_execution':True,'primary_timing':False},indent=2)+'\n')
print('320 diagnostic processes reproduce prior allocations and repeated attribution.',flush=True)
