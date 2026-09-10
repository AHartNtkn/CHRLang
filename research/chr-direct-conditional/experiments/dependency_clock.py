"""Prospective clock/endpoint qualification and explicitly exploratory sizing."""
import hashlib,itertools,json,os,random,resource,subprocess
from pathlib import Path
ROOT=Path.cwd();BASE=ROOT/'docs/experiments/results/s03-dependency-clock';OUT=BASE/'runs';OUT.mkdir(exist_ok=False)
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def exe(kind):
    rows=[json.loads(x) for x in (BASE/f'build-{kind}.jsonl').read_text().splitlines()]
    for name in ['chr_direct_choice','chr_compiled','chr_observe','chr_persistent']:
        assert next(r for r in rows if r.get('target',{}).get('name')==name)['features']==[]
    return Path(next(r['executable'] for r in rows if r.get('executable') and r['target']['name']=='dependency_ownership'))
binaries={kind:exe(kind) for kind in ['ordinary','meter']}
paths=subprocess.check_output(['git','ls-files','crates','research','Cargo.toml','Cargo.lock'],text=True).splitlines()
paths=[p for p in paths if p.endswith(('.rs','.toml','.lock'))]
paths+=['research/chr-direct-conditional/experiments/dependency_clock.py','docs/experiments/registrations/S03-dependency-clock.md']
freeze={'sources':{p:digest(ROOT/p) for p in paths},'binaries':{k:{'path':str(v.relative_to(ROOT)),'sha256':digest(v)} for k,v in binaries.items()},'affinity':sorted(os.sched_getaffinity(0))}
(BASE/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def call(kind,args,clock,path):
    assert digest(binaries[kind])==freeze['binaries'][kind]['sha256']
    r=subprocess.run([str(binaries[kind]),*map(str,args)],env=dict(os.environ,DEPENDENCY_FIRST_CLOCK=clock),text=True,capture_output=True,preexec_fn=limits,timeout=60)
    path.write_text(r.stdout+r.stderr);assert r.returncode==0,(path,r.returncode,r.stderr)
    rows=[json.loads(x) for x in r.stdout.splitlines() if x.startswith('{')]
    return rows[-1] if rows else None
call('meter',['meter-check'],'off',BASE/'meter-check.log')
clocks=[call('ordinary',['clock-check'],'off',OUT/f'clock-{i}.log') for i in range(5)]
(BASE/'clocks.json').write_text(json.dumps(clocks,indent=2)+'\n')
def normalize(value,baseline=0):
    if isinstance(value,list):return [normalize(x,baseline) for x in value]
    if isinstance(value,dict):return {k:(v-baseline if k in ['live_start','live_end','peak_live'] else normalize(v,baseline)) for k,v in value.items() if k not in ['ns','first_ns']}
    return value
old=json.loads((ROOT/'docs/experiments/results/s03-dependency-ownership/results.json').read_text())
selected=[r for r in old if not r['reverse'] and r['retention']=='immediate' and not r['cancel']]
assert len(selected)==80
records=[]
def args(r):return [r['mode'],r['family'],r['size'],str(r['reverse']).lower(),r['retention'],str(r['cancel']).lower()]
def check(r,clock):
    for i,e in enumerate(r['endpoints']):
        assert (e['first_ns'] is not None)==(clock=='on' and e['answers']>0)
        duration=next(p['reading']['ns'] for p in r['phases'] if p['phase']=='execute_observe' and p['query']==i)
        assert e['first_ns'] is None or 0<=e['first_ns']<=duration
for cell in selected:
    results=[]
    for clock in ['off','on']:
        for kind,repeat in [('meter',0),('meter',1),('ordinary',0)]:
            r=call(kind,args(cell),clock,OUT/f"qual-{cell['index']}-{clock}-{kind}-{repeat}.log");check(r,clock)
            assert normalize(r['endpoints'])==cell['result']['endpoints']
            if kind=='meter':
                prior=cell['result'];baseline=r['phases'][0]['reading']['memory']['live_start'];before=prior['phases'][0]['reading']['memory']['live_start']
                assert normalize(r,baseline)==normalize(prior,before),cell['index']
            results.append({'clock':clock,'kind':kind,'repeat':repeat,'result':r})
    records.append({'index':cell['index'],'args':args(cell),'runs':results})
    (BASE/'qualification.json').write_text(json.dumps(records,indent=2)+'\n')
    if len(records)%10==0:print('qualified cells',len(records),flush=True)
extra=[r for r in old if r['family'] in ['known-hit','delayed-miss'] and r['size']==8 and not r['reverse'] and r['retention'] in ['window','all'] and r['cancel']]
assert len(extra)==32
for cell in extra:
    r=call('ordinary',args(cell),'on',OUT/f"cancel-{cell['index']}.log");check(r,'on');assert normalize(r['endpoints'])==cell['result']['endpoints']
rng=random.Random(73041);sizing=[]
cells=[r for r in selected if r['mode'] in ['current','current-miss','scan','indexed']];assert len(cells)==40
for repeat in range(5):
    ordered=cells.copy();rng.shuffle(ordered)
    for cell in ordered:
        r=call('ordinary',args(cell),'on',OUT/f"sizing-{cell['index']}-{repeat}.log");check(r,'on');assert normalize(r['endpoints'])==cell['result']['endpoints']
        sizing.append({'index':cell['index'],'args':args(cell),'repeat':repeat,'result':r})
        (BASE/'sizing.json').write_text(json.dumps(sizing,indent=2)+'\n')
    print('sizing repetition',repeat+1,flush=True)
assert all(digest(ROOT/p)==sha for p,sha in freeze['sources'].items())
(BASE/'audit.json').write_text(json.dumps({'qualification_cells':80,'qualification_processes':480,'cancellation_processes':32,'sizing_processes':200,'clock_processes':5,'allocator_self_checks':1,'allocation_matches_prior':True,'first_observation_valid':True,'counter_free_engines':True,'comparative_confirmation':False},indent=2)+'\n')
print('Qualification and exploratory sizing complete.',flush=True)
