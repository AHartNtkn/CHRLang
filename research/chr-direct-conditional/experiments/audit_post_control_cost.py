"""Reconstruct native/access costs, original-control bridges and paired totals."""
import collections,hashlib,itertools,json,random,statistics,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s03-post-control-cost';PARENT=ROOT/'docs/experiments/results/s03-post-ownership'
FAMILIES=['post-input','post-output','post-forward','post-miss','post-duplicate','post-template'];OLD=['birth','birth-miss','birth-miss-template','scan','indexed','sealed'];NEW=['active-scan','active-indexed','native-scan','native-indexed','active-native-scan','active-native-indexed'];MODES=OLD+NEW
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def result(raw):return next(json.loads(l) for l in raw['stdout'].splitlines() if l.startswith('{') and json.loads(l).get('event')=='result')
def norm(x,base):
    if isinstance(x,list):return [norm(v,base) for v in x]
    if isinstance(x,dict):return {k:(v-base if k in ['live_start','live_end','peak_live'] else norm(v,base)) for k,v in x.items() if k!='ns'}
    return x
f=read(BASE/'freeze.json');assert sha(BASE/'sources.zip')==f['archive_sha256'];assert sha(PARENT/'freeze.json')==f['parent_freeze_sha256'];assert sha(PARENT/'sources.zip')==f['parent_archive_sha256']
with zipfile.ZipFile(BASE/'sources.zip') as z:
    assert set(z.namelist())==set(f['sources'])
    for p,h in f['sources'].items():assert hashlib.sha256(z.read(p)).hexdigest()==h
for kind,b in f['binaries'].items():
    assert sha(Path(b['path']))==b['sha256'];build=read(BASE/f'build-{kind}.json');assert build['exit_code']==0
    for l in build['stdout'].splitlines():
        x=json.loads(l)
        if x.get('reason')=='compiler-artifact':
            assert not(set(x.get('features',[]))&{'metrics','kernel-metrics','work-diagnostics','candidate-profile'})
            if kind=='time':assert 'alloc-meter' not in x.get('features',[])
for name,h in f['orders'].items():assert sha(BASE/f'{name}.json')==h
scenarios=list(itertools.product(FAMILIES,[8,32],[False,True],['immediate','window','all']));cells=[[*s,m] for s in scenarios for m in MODES];assert read(BASE/'cells.json')==cells
meter=[dict(index=i,rep=r) for i,c in enumerate(cells) for r in range(1 if c[-1] in OLD else 2)];random.Random(71320).shuffle(meter);assert read(BASE/'meter-order.json')==meter
blocks=list(itertools.product(range(5),range(72)));rng=random.Random(71321);rng.shuffle(blocks);timing=[]
for block,(rep,index) in enumerate(blocks):
    modes=MODES.copy();rng.shuffle(modes)
    for pos,m in enumerate(modes):timing.append(dict(block=block,rep=rep,index=cells.index([*scenarios[index],m]),position=pos))
assert read(BASE/'time-order.json')==timing;warmup=list(range(864));random.Random(71322).shuffle(warmup);assert read(BASE/'warmup-order.json')==warmup
pc={tuple(c):i for i,c in enumerate(read(PARENT/'cells.json'))}
phases=[('source',0),('prepare',0)]+[(p,q) for q in range(4) for p in ['input','setup','execute_observe','engine_drop','consumer']]+[('prepared_drop',0),('consumer_drop',0)]
def validate(path,kind,cell,cancel=False):
    fam,n,rev,c,m=cell;raw=read(path);assert raw['exit_code']==0 and not raw['stderr'] and not raw['timeout'] and raw['first_clock']=='off';assert raw['command']==[f['binaries'][kind]['path'],m,fam,str(n),str(rev).lower(),c,str(cancel).lower()];r=result(raw);assert r['meter']==(kind=='meter');assert [(x['phase'],x['query']) for x in r['phases']]==phases
    assert all(e['first_ns'] is None for e in r['endpoints']);assert [e['complete'] for e in r['endpoints']]==([False,True,False,True] if cancel else [True]*4);assert [e['answers'] for e in r['endpoints']]==([0,1,0,1] if cancel else [1]*4)
    if kind=='meter':
        ms=[x['reading']['memory'] for x in r['phases']];base=ms[0]['live_start'];assert ms[-1]['live_end']==base;assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]));return r,norm(r,base)
    assert all(x['reading']['memory'] is None for x in r['phases']);return r,None
for i,(fam,m) in enumerate(itertools.product(FAMILIES,NEW)):validate(BASE/f'cancel-{i}.json','meter',[fam,32,True,'window',m],True)
clocks=[]
for i in range(5):
    raw=read(BASE/f'clock-{i}.json');assert raw['exit_code']==0 and not raw['stderr'];r=json.loads(raw['stdout']);assert r['samples']==100000;clocks.append(r['median'])
floor=100*max(clocks)*24
alloc=collections.defaultdict(list);ends={};times=collections.defaultdict(list);pt=collections.defaultdict(lambda:collections.defaultdict(list));paired={}
for kind,order,bkind in [('meter',meter,'meter'),('warmup',warmup,'time'),('time',timing,'time')]:
    assert len(list((BASE/kind).glob('*.json')))==len(order)
    for i,j in enumerate(order):
        index=j if kind=='warmup' else j['index'];r,normed=validate(BASE/kind/f'{i}.json',bkind,cells[index]);fam,n,rev,c,m=cells[index]
        if index in ends:assert ends[index]==r['endpoints']
        else:ends[index]=r['endpoints']
        if kind=='meter':
            alloc[index].append(normed)
            if m in OLD:
                for rep in [0,1]:
                    old=result(read(PARENT/'runs'/f'{pc[fam,n,rev,m,c,False]}-meter-{rep}.json'));assert normed==norm(old,old['phases'][0]['reading']['memory']['live_start'])
        if kind=='time':
            p=collections.Counter()
            for x in r['phases']:p[x['phase']]+=x['reading']['ns']
            total=sum(p.values());times[index].append(total);paired[j['block'],m]=total
            for name,v in p.items():pt[index][name].append(v)
summary=[]
for index,cell in enumerate(cells):
    assert len(alloc[index])==(1 if cell[-1] in OLD else 2);assert all(x==alloc[index][0] for x in alloc[index]);assert len(times[index])==5
    ms=[p['reading']['memory'] for p in alloc[index][0]['phases']]
    summary.append(dict(cell=cell,requested_bytes=sum(x['requested_bytes'] for x in ms),peak_excess=max(x['peak_live'] for x in ms),median_ns=statistics.median(times[index]),min_ns=min(times[index]),max_ns=max(times[index]),clock_sensitive=min(times[index])<floor,phase_median_ns={p:statistics.median(v) for p,v in pt[index].items()}))
comparisons=[]
for index,scenario in enumerate(scenarios):
    bs=[b for b,(_,i) in enumerate(blocks) if i==index];assert len(bs)==5
    for demand,control in itertools.product(OLD[:3],MODES[3:]):
        ratios=[paired[b,demand]/paired[b,control] for b in bs];sensitive=any(summary[cells.index([*scenario,m])]['clock_sensitive'] for m in [demand,control]);comparisons.append(dict(scenario=scenario,demand=demand,control=control,median_ratio=statistics.median(ratios),min_ratio=min(ratios),max_ratio=max(ratios),clock_sensitive=sensitive))
print(json.dumps(dict(cells=864,allocation_bridge_cells=432,new_exact_allocation_pairs=432,primary_processes=4320,warmups=864,cancellation_gates=36,paired_blocks=360,comparisons=1944,all_parent_bridges_match=True,final_heap_restored=True,clock_medians_ns=clocks,clock_floor_ns=floor,clock_sensitive_cells=sum(x['clock_sensitive'] for x in summary),confirmed_timing=False,summary=summary,contrasts=comparisons),indent=2))
