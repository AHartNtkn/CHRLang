"""Independent phase, endpoint, frozen-feature and paired timing audit."""
import collections,hashlib,itertools,json,random,statistics,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s03-post-timing';PARENT=ROOT/'docs/experiments/results/s03-post-ownership'
FAMILIES=['post-input','post-output','post-forward','post-miss','post-duplicate','post-template'];MODES=['birth','birth-miss','birth-miss-template','scan','indexed','sealed']
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def result(raw):return next(json.loads(l) for l in raw['stdout'].splitlines() if l.startswith('{') and json.loads(l).get('event')=='result')
f=read(BASE/'freeze.json');p=read(PARENT/'freeze.json');assert sha(PARENT/'freeze.json')==f['parent_freeze_sha256'];assert sha(PARENT/'sources.zip')==f['parent_archive_sha256']==p['source_archive_sha256'];assert sha(PARENT/'build-ordinary.jsonl')==f['parent_build_sha256']
with zipfile.ZipFile(PARENT/'sources.zip') as z:
    assert set(z.namelist())==set(p['sources'])
    for path,h in p['sources'].items():assert hashlib.sha256(z.read(path)).hexdigest()==h
for path,h in f['sources'].items():assert hashlib.sha256(f['source_text'][path].encode()).hexdigest()==h
assert f['binary']==p['binaries']['ordinary'];assert sha(Path(f['binary']['path']))==f['binary']['sha256']
for line in (PARENT/'build-ordinary.jsonl').read_text().splitlines():
    x=json.loads(line)
    if x.get('reason')=='compiler-artifact':assert not(set(x.get('features',[])) & {'metrics','kernel-metrics','work-diagnostics','candidate-profile','alloc-meter'})
for name,h in f['orders'].items():assert sha(BASE/f'{name}.json')==h
scenarios=list(itertools.product(FAMILIES,[8,32],[False,True],['immediate','window','all']));cells=[[*s,m] for s in scenarios for m in MODES];assert read(BASE/'cells.json')==cells
blocks=list(itertools.product(range(5),range(72)));rng=random.Random(71310);rng.shuffle(blocks);jobs=[]
for block,(rep,index) in enumerate(blocks):
    modes=MODES.copy();rng.shuffle(modes)
    for pos,mode in enumerate(modes):jobs.append(dict(block=block,rep=rep,index=cells.index([*scenarios[index],mode]),position=pos))
assert jobs==read(BASE/'jobs.json');warmup=list(range(432));random.Random(71311).shuffle(warmup);assert read(BASE/'warmup-order.json')==warmup
parent_cells=read(PARENT/'cells.json');parent_by_key={tuple(c):i for i,c in enumerate(parent_cells)}
expected_phases=[('source',0),('prepare',0)]+[(name,q) for q in range(4) for name in ['input','setup','execute_observe','engine_drop','consumer']]+[('prepared_drop',0),('consumer_drop',0)]
def validate(path,cell,cancel=False):
    fam,size,rev,consumer,mode=cell;raw=read(path);assert raw['exit_code']==0 and not raw['stderr'] and not raw['timeout'] and raw['first_clock']=='off';assert raw['command']==[f['binary']['path'],mode,fam,str(size),str(rev).lower(),consumer,str(cancel).lower()]
    r=result(raw);assert not r['meter'];assert all(e['first_ns'] is None for e in r['endpoints']);assert [(x['phase'],x['query']) for x in r['phases']]==expected_phases;assert all(x['reading']['memory'] is None and x['reading']['ns']>=0 for x in r['phases'])
    index=parent_by_key[fam,size,rev,mode,consumer,cancel];prior=read(PARENT/'runs'/f'{index}-ordinary-0.json');assert prior['exit_code']==0;assert r['endpoints']==result(prior)['endpoints'];assert [e['complete'] for e in r['endpoints']]==([False,True,False,True] if cancel else [True]*4)
    return r,index
for i,(fam,mode) in enumerate(itertools.product(FAMILIES,MODES)):validate(BASE/f'cancel-{i}.json',[fam,8,False,'window',mode],True)
clocks=[]
for i in range(5):
    raw=read(BASE/f'clock-{i}.json');assert raw['exit_code']==0 and not raw['stderr'];r=json.loads(raw['stdout']);assert r['samples']==100000;clocks.append(r['median'])
clock_floor=100*max(clocks)*24
samples=collections.defaultdict(list);phase_samples=collections.defaultdict(lambda:collections.defaultdict(list));paired={};parent_indices={}
for kind,order in [('warmup',warmup),('runs',jobs)]:
    assert len(list((BASE/kind).glob('*.json')))==len(order)
    for i,j in enumerate(order):
        index=j if kind=='warmup' else j['index'];r,pi=validate(BASE/kind/f'{i}.json',cells[index]);parent_indices[index]=pi
        if kind=='warmup':continue
        ps=collections.Counter()
        for x in r['phases']:ps[x['phase']]+=x['reading']['ns']
        total=sum(ps.values());assert total>0;samples[index].append(total);paired[j['block'],cells[index][-1]]=total
        for name,t in ps.items():phase_samples[index][name].append(t)
summary=[]
for index,cell in enumerate(cells):
    assert len(samples[index])==5;raw=read(PARENT/'runs'/f'{parent_indices[index]}-meter-0.json');assert raw['exit_code']==0
    old=result(raw);ms=[x['reading']['memory'] for x in old['phases']];base=ms[0]['live_start'];assert ms[-1]['live_end']==base
    summary.append(dict(cell=cell,median_ns=statistics.median(samples[index]),min_ns=min(samples[index]),max_ns=max(samples[index]),clock_sensitive=min(samples[index])<clock_floor,phase_median_ns={name:statistics.median(v) for name,v in phase_samples[index].items()},parent_requested_bytes=sum(m['requested_bytes'] for m in ms),parent_peak_excess=max(m['peak_live'] for m in ms)-base))
contrasts=[]
for index,scenario in enumerate(scenarios):
    bs=[b for b,(_,i) in enumerate(blocks) if i==index];assert len(bs)==5
    for demand,control in itertools.product(MODES[:3],MODES[3:]):
        ratios=[paired[b,demand]/paired[b,control] for b in bs]
        sensitive=any(summary[cells.index([*scenario,m])]['clock_sensitive'] for m in [demand,control])
        contrasts.append(dict(scenario=scenario,demand=demand,control=control,median_ratio=statistics.median(ratios),min_ratio=min(ratios),max_ratio=max(ratios),clock_sensitive=sensitive))
print(json.dumps(dict(primary_processes=2160,warmups=432,cancellation_gates=36,cells=432,paired_blocks=360,contrasts=648,exact_parent_endpoints=True,clock_medians_ns=clocks,clock_floor_ns=clock_floor,clock_sensitive_cells=sum(x['clock_sensitive'] for x in summary),allocation_source='Frozen matched post-ownership matrix, not newly measured',confirmed_timing=False,summary=summary,comparisons=contrasts),indent=2))
