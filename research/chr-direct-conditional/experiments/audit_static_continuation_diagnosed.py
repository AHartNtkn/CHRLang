"""Audit source-control equivalence, full ownership, and71 registered intervals."""
import collections,gzip,hashlib,json,math,statistics,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s10-static-control';PARENT=ROOT/'docs/experiments/results/s10-post-continuation-cost'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def result(raw):return next(json.loads(l) for l in raw['stdout'].splitlines() if l.startswith('{') and json.loads(l).get('event')=='result')
def memory(r):
    base=r['phases'][0]['reading']['memory']['live_start'];last=base;rows=[]
    for p in r['phases']:
        m=p['reading']['memory'];assert m['live_start']==last;last=m['live_end'];rows.append({k:v-base if k in ['live_start','live_end','peak_live'] else v for k,v in m.items()})
    assert last==base;return rows

def main():
    f=json.loads((BASE/'freeze.json').read_text());assert sha(BASE/'sources.zip')==f['archive_sha256'];assert sha(BASE/'jobs.json')==f['jobs_sha256'];assert sha(PARENT/'freeze.json')==f['parent_sha256'];assert sha(PARENT/'runs.jsonl.gz')==f['parent_runs_sha256']
    with zipfile.ZipFile(BASE/'sources.zip') as z:
        for p,h in f['sources'].items():assert sha(ROOT/p)==h and hashlib.sha256(z.read(p)).hexdigest()==h,p
    for b in f['binaries'].values():assert sha(Path(b['path']))==b['sha256']
    parent={}
    with gzip.open(PARENT/'runs.jsonl.gz','rt') as inp:
        for line in inp:
            r=json.loads(line);j=r['job']
            if j['stage']=='allocation' and j['counted'] and j['rep']==0:parent[(*j['case'],j['mode'])]=memory(result(r['raw']))
    jobs=json.loads((BASE/'jobs.json').read_text());times=collections.defaultdict(dict);alloc=collections.defaultdict(list);ends={};counts=collections.Counter();n=0
    phases=[(p,0) for p in ['source','count_prepare','static_prepare','value_prepare','engine_prepare','source_drop']]+[(p,i) for i in range(4) for p in ['input','count_query','static_query','setup','execute_observe','engine_drop','consumer']]+[('prepared_drop',0),('consumer_drop',0)]
    clock=json.loads((BASE/'clock-check.json').read_text());clock=next(json.loads(l) for l in clock['stdout'].splitlines() if l.startswith('{'));floor=clock['median']*3600
    with gzip.open(BASE/'runs.jsonl.gz','rt') as inp:
        for n,line in enumerate(inp,1):
            r=json.loads(line);j=r['job'];assert j==jobs[n-1];assert r['raw']['exit_code']==0;value=result(r['raw']);counts[j['stage']]+=1
            assert value['meter']==(j['kind']=='meter');assert [(p['phase'],p['query']) for p in value['phases']]==phases
            for i,e in enumerate(value['endpoints']):assert e['complete']==(not j['cancel'] or i%2==1);assert e['first_ns'] is None
            key=(*j['case'],j['mode'],j['initialize']);ek=(*key,j['cancel']);assert ends.setdefault(ek,value['endpoints'])==value['endpoints']
            if j['stage']=='primary':times[key][j['rep']]=sum(p['reading']['ns'] for p in value['phases'])
            if j['kind']=='meter':
                m=memory(value)
                if j['stage']=='allocation':alloc[key].append(m)
    assert n==16100 and len(alloc)==2400 and len(times)==100
    bridges=0;differences=[];summary=[]
    for key,rows in sorted(alloc.items()):
        assert len(rows)==2 and rows[0]==rows[1],key
        if not key[-1] or key[-2].startswith('birth'):
            before=parent[key[:-1]]
            if rows[0]==before:bridges+=1
            else:
                assert key[-2]=='birth-miss-template'
                changes=[]
                for phase,a,b in zip(phases,before,rows[0]):
                    changed={k:(a[k],b[k]) for k in a if a[k]!=b[k]}
                    if not changed:continue
                    assert phase[0]=='execute_observe'
                    assert set(changed)=={'allocation_calls','requested_bytes','deallocation_calls'}
                    calls=b['allocation_calls']-a['allocation_calls'];assert b['deallocation_calls']-a['deallocation_calls']==calls
                    assert b['requested_bytes']-a['requested_bytes']==192*calls
                    changes.append(dict(phase=phase,changes=changed))
                differences.append(dict(key=key,changes=changes))
        ts=times.get(key,{});summary.append(dict(key=key,bytes=sum(p['requested_bytes'] for p in rows[0]),peak=max(p['peak_live'] for p in rows[0]),median_ns=statistics.median(ts.values()) if ts else None,min_ns=min(ts.values()) if ts else None,max_ns=max(ts.values()) if ts else None))
    assert bridges+len(differences)==1344
    comparisons=[]
    for key in sorted(times):
        case=key[:-2];mode,init=key[-2:]
        if init and not mode.startswith('birth'):comparisons.append((key,(*case,mode,False),'initialization on/off'))
        if mode.startswith('birth') and case[1]==3:
            for control in ['native-scan','active-native-scan','sealed-scan']:comparisons.append((key,(*case,control,True),'demand/initialized control'))
    assert len(comparisons)==71;intervals=[]
    for left,right,comparison in comparisons:
        assert set(times[left])==set(times[right])==set(range(64));ratios=[times[left][i]/times[right][i] for i in range(64)];ss=sorted(ratios);lo,hi=ss[18],ss[45]
        limited=any(t<floor for t in list(times[left].values())+list(times[right].values()))
        verdict='signal-limited' if limited else 'gain' if hi<.9 else 'loss' if lo>1.1 else 'within-10%' if lo>=.9 and hi<=1.1 else 'unresolved magnitude'
        intervals.append(dict(left=left,right=right,comparison=comparison,median_ratio=statistics.median(ratios),interval=[lo,hi],verdict=verdict,first_half=statistics.median(ratios[:32]),last_half=statistics.median(ratios[32:]),ratios=ratios))
    out=dict(processes=n,stages=dict(counts),allocation_pairs=2400,parent_bridges=bridges,parent_address_order_differences=differences,clock_floor_ns=floor,family_error_bound=142*sum(math.comb(64,i) for i in range(19))/2**64,counts=dict(collections.Counter(r['verdict'] for r in intervals)),summary=summary,intervals=intervals)
    (BASE/'diagnosed-audit.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k not in ['summary','intervals','parent_address_order_differences']}))
if __name__=='__main__':main()
