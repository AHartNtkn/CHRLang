"""Audit exact ownership repeats and randomized paired lifecycle sizing."""
import collections,hashlib,itertools,json,random,statistics,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s01-prefix-lifecycle'
MODES=['local-scan','partial','intermediate','scan','indexed','native-indexed','active-indexed','active-native-indexed'];FAMILIES=[f'prefix-{f}-{d}' for f in ['sparse','keyed','kill','miss'] for d in [0,32]]
def modes(f):return [m for m in MODES if not(f.startswith('prefix-kill-') and m.startswith('active-'))]
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def norm(x,base):
    if isinstance(x,list):return [norm(v,base) for v in x]
    if isinstance(x,dict):return {k:(v-base if k in ['live_start','live_end','peak_live'] else norm(v,base)) for k,v in x.items() if k!='ns'}
    return x
f=read(BASE/'freeze.json');assert sha(BASE/'sources.zip')==f['archive_sha256']
with zipfile.ZipFile(BASE/'sources.zip') as z:
    assert set(z.namelist())==set(f['sources'])
    for p,h in f['sources'].items():assert hashlib.sha256(z.read(p)).hexdigest()==h
for b in f['binaries'].values():assert sha(Path(b['path']))==b['sha256']
for p,h in f['orders'].items():assert sha(BASE/(p+'.json'))==h
cells=[[f,w,r,c,m] for f,w,r,c in itertools.product(FAMILIES,[4,8],[1,4],['immediate','all']) for m in modes(f)];assert read(BASE/'cells.json')==cells
meter=list(itertools.product(range(2),range(480)));random.Random(81110).shuffle(meter);assert read(BASE/'meter-order.json')==[list(x) for x in meter]
blocks=list(itertools.product(range(5),FAMILIES,[4,8],[1,4],['immediate','all']));rng=random.Random(81111);rng.shuffle(blocks);timing=[]
for block,(rep,fam,width,reuse,consumer) in enumerate(blocks):
    selected=modes(fam);rng.shuffle(selected)
    for pos,mode in enumerate(selected):timing.append(dict(block=block,position=pos,rep=rep,index=cells.index([fam,width,reuse,consumer,mode])))
assert read(BASE/'time-order.json')==timing
alloc=collections.defaultdict(list);times=collections.defaultdict(list);paired={};phase_times=collections.defaultdict(lambda:collections.defaultdict(list));metadata={}
for kind,jobs in [('meter',meter),('time',timing)]:
    assert len(list((BASE/kind).glob('*.json')))==len(jobs)
    for i,job in enumerate(jobs):
        index=job[1] if kind=='meter' else job['index'];family,width,reuse,consumer,mode=cells[index]
        raw=read(BASE/kind/f'{i}.json');assert raw['exit_code']==0 and not raw['timeout'] and not raw['stderr'];assert raw['command']==[f['binaries'][kind]['path'],mode,family,str(width),str(reuse),consumer]
        r=json.loads(raw['stdout']);assert (r['mode'],r['family'],r['width'],r['reuse'],r['retain'])==(mode,family,width,reuse,consumer=='all')
        expected=['source-build','prepare']+['input','setup','execute','observe','engine-drop','answer-retain' if consumer=='all' else 'answer-drop','input-drop']*reuse+['cancel-input','cancel-setup','cancel-advance','cancel-engine-drop','cancel-input-drop']*2+['prepared-drop','source-drop']+(['retained-drop'] if consumer=='all' else [])
        assert [p['phase'] for p in r['phases']]==expected
        if index in metadata:assert metadata[index]==r['state_bytes']
        else:metadata[index]=r['state_bytes']
        if kind=='meter':
            assert all(p['measurement']['ns']==0 for p in r['phases'])
            ms=[p['measurement']['memory'] for p in r['phases']];base=ms[0]['live_start'];assert ms[-1]['live_end']==base
            assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]))
            for offset in [2+7*reuse,2+7*reuse+5]:assert ms[offset]['live_start']==ms[offset+4]['live_end']
            if consumer=='immediate':
                for q in range(reuse):assert ms[2+7*q+6]['live_end']==ms[1]['live_end']
            alloc[index].append(norm(r,base))
        else:
            assert all('memory' not in p['measurement'] for p in r['phases'])
            ps=collections.Counter()
            for p in r['phases']:ps[p['phase']]+=p['measurement']['ns']
            total=sum(v for p,v in ps.items() if not p.startswith('cancel-'));times[index].append(total);paired[job['block'],mode]=total
            for p,v in ps.items():phase_times[index][p].append(v)
clock=[]
for rep in range(5):
    raw=read(BASE/f'clock-time-{rep}.json');assert raw['exit_code']==0 and not raw['stderr']
    r=json.loads(raw['stdout']);assert r['samples']==10000;clock.append(r['median_ns'])
clock_bound=max(clock)
summary=[]
for i,cell in enumerate(cells):
    assert len(alloc[i])==2 and alloc[i][0]==alloc[i][1];assert len(times[i])==5
    ps=alloc[i][0]['phases'];complete=[p for p in ps if not p['phase'].startswith('cancel-')]
    traffic=collections.Counter()
    for p in ps:traffic[p['phase']]+=p['measurement']['memory']['requested_bytes']
    summary.append(dict(index=i,cell=cell,requested_bytes=sum(p['measurement']['memory']['requested_bytes'] for p in complete),peak_excess=max(p['measurement']['memory']['peak_live'] for p in complete),session_peak_excess=max(p['measurement']['memory']['peak_live'] for p in ps),phase_bytes=dict(traffic),median_ns=statistics.median(times[i]),min_ns=min(times[i]),max_ns=max(times[i]),phase_median_ns={p:statistics.median(v) for p,v in phase_times[i].items()},state_bytes=metadata[i],retained_after_producer_bytes=next(p['measurement']['memory']['live_end'] for p in ps if p['phase']=='source-drop'),clock_floor_ns=100*clock_bound*len(complete),clock_sensitive=min(times[i])<100*clock_bound*len(complete)))
comparisons=[]
for fam,width,reuse,consumer in itertools.product(FAMILIES,[4,8],[1,4],['immediate','all']):
    selected=[b for b,(_,f,w,r,c) in enumerate(blocks) if (f,w,r,c)==(fam,width,reuse,consumer)]
    assert len(selected)==5
    for mode in modes(fam):
        if mode=='intermediate':continue
        ratios=[paired[b,'intermediate']/paired[b,mode] for b in selected]
        comparisons.append(dict(family=fam,width=width,reuse=reuse,consumer=consumer,control=mode,median_ratio=statistics.median(ratios),min_ratio=min(ratios),max_ratio=max(ratios)))
for kind in ['meter','time']:
    prefix=read(BASE/f'prefix-smoke-{kind}.json');assert prefix['exit_code']==0 and '90 independent prefix/retained/cancellation smoke configurations passed' in prefix['stdout']
    smoke=read(BASE/f'smoke-{kind}.json');assert smoke['exit_code']==0 and '80 independent source/retained/cancellation smoke configurations passed' in smoke['stdout']
print(json.dumps(dict(configurations=480,allocation_processes=960,timing_processes=2400,exact_allocation_pairs=480,final_live_restored=True,paired_blocks=320,clock_median_ns=clock,clock_sensitive_cells=sum(x["clock_sensitive"] for x in summary),confirmed_timing=False,summary=summary,comparisons=comparisons),indent=2))
