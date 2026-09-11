"""Reconstruct frozen blocks, phase endpoints and simultaneous median intervals."""
import collections,hashlib,itertools,json,math,random,statistics,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s01-prefix-confirmation';PARENT=ROOT/'docs/experiments/results/s01-prefix-lifecycle'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def classify(lo,hi,sensitive):
    if sensitive:return 'instrumentation-sensitive'
    if hi<.9:return 'faster'
    if lo>1.1:return 'slower'
    if lo>=.9 and hi<=1.1:return 'within-10-percent'
    return 'unresolved'
def main():
    f=read(BASE/'freeze.json');p=read(PARENT/'freeze.json')
    assert sha(PARENT/'freeze.json')==f['parent_freeze_sha256'];assert sha(PARENT/'sources.zip')==f['parent_archive_sha256']==p['archive_sha256']
    with zipfile.ZipFile(PARENT/'sources.zip') as z:
        assert set(z.namelist())==set(p['sources'])
        for path,h in p['sources'].items():assert hashlib.sha256(z.read(path)).hexdigest()==h
    for path,h in f['sources'].items():assert hashlib.sha256(f['source_text'][path].encode()).hexdigest()==h
    assert f['binary']==p['binaries']['time'];assert sha(Path(f['binary']['path']))==f['binary']['sha256']
    for name,h in f['orders'].items():assert sha(BASE/f'{name}.json')==h
    scenarios=[list(x) for x in itertools.product([f'prefix-{f}-{d}' for f in ['sparse','keyed','kill','miss'] for d in [0,32]],[4,8],[1,4],['immediate','all'])];assert read(BASE/'scenarios.json')==scenarios
    blocks=list(itertools.product(range(64),range(64)));rng=random.Random(81210);rng.shuffle(blocks);jobs=[]
    for block,(rep,index) in enumerate(blocks):
        modes=['intermediate','local-scan','native-indexed' if scenarios[index][0].startswith('prefix-kill-') else 'active-native-indexed'];rng.shuffle(modes)
        for pos,mode in enumerate(modes):jobs.append(dict(block=block,rep=rep,index=index,position=pos,mode=mode))
    assert jobs==read(BASE/'jobs.json');assert len(list((BASE/'runs').glob('*.json')))==12288
    def raw(path,args):
        r=read(path);assert r['exit_code']==0 and not r['stderr'] and not r['timeout'];assert r['command']==[f['binary']['path'],*map(str,args)];return r['stdout']
    assert '80 independent source/retained/cancellation smoke configurations passed' in raw(BASE/'smoke.json',[])
    assert '90 independent prefix/retained/cancellation smoke configurations passed' in raw(BASE/'prefix-smoke.json',['prefix-check'])
    clocks=[]
    for rep in range(5):
        r=json.loads(raw(BASE/f'clock-{rep}.json',['clock-check']));assert r['samples']==10000;clocks.append(r['median_ns'])
    totals={};samples=collections.defaultdict(list);phases=collections.defaultdict(lambda:collections.defaultdict(list));floors={}
    for i,j in enumerate(jobs):
        fam,w,q,c=scenarios[j['index']];mode=j['mode'];r=json.loads(raw(BASE/'runs'/f'{i}.json',[mode,fam,w,q,c]));assert [r['family'],r['width'],r['reuse'],'all' if r['retain'] else 'immediate']==[fam,w,q,c];assert r['mode']==mode
        names=['source-build','prepare']+['input','setup','execute','observe','engine-drop','answer-retain' if c=='all' else 'answer-drop','input-drop']*q+['cancel-input','cancel-setup','cancel-advance','cancel-engine-drop','cancel-input-drop']*2+['prepared-drop','source-drop']+(['retained-drop'] if c=='all' else [])
        assert [x['phase'] for x in r['phases']]==names;assert all(set(x['measurement'])=={'ns'} and x['measurement']['ns']>=0 for x in r['phases'])
        ps=collections.Counter()
        for x in r['phases']:
            if not x['phase'].startswith('cancel-'):ps[x['phase']]+=x['measurement']['ns']
        total=sum(ps.values());assert total>0
        totals[j['block'],mode]=total;samples[j['index'],mode].append(total)
        for name,t in ps.items():phases[j['index'],mode][name].append(t)
        floors[j['index']]=100*max(clocks)*sum(not n.startswith('cancel-') for n in names)
    n=64;k=max(k for k in range(1,33) if sum(math.comb(n,i) for i in range(k))*5120<=2**n);assert k==18
    comparisons=[];summaries=[]
    for index,scenario in enumerate(scenarios):
        chosen=[b for b,(_,i) in enumerate(blocks) if i==index];assert len(chosen)==64
        controls=['local-scan','native-indexed' if scenario[0].startswith('prefix-kill-') else 'active-native-indexed']
        for mode in ['intermediate',*controls]:
            vals=samples[index,mode];assert len(vals)==64
            summaries.append(dict(scenario=scenario,mode=mode,median_ns=statistics.median(vals),min_ns=min(vals),max_ns=max(vals),clock_floor_ns=floors[index],clock_sensitive=min(vals)<floors[index],phase_median_ns={p:statistics.median(v) for p,v in phases[index,mode].items()}))
        for mode in controls:
            vals=sorted(totals[b,'intermediate']/totals[b,mode] for b in chosen);lo,hi=vals[k-1],vals[n-k]
            sensitive=any(min(samples[index,m])<floors[index] for m in ['intermediate',mode])
            comparisons.append(dict(scenario=scenario,control=mode,median_ratio=statistics.median(vals),min_ratio=vals[0],max_ratio=vals[-1],interval_low=lo,interval_high=hi,status=classify(lo,hi,sensitive)))
    print(json.dumps(dict(processes=12288,scenarios=64,comparisons=128,repetitions=64,clock_medians_ns=clocks,median_interval_ranks=[k,n-k+1],family_error_bound=.05,conditional_sampling_assumptions=True,status_counts=dict(collections.Counter(c['status'] for c in comparisons)),summary=summaries,contrasts=comparisons),indent=2))
if __name__=='__main__':main()
