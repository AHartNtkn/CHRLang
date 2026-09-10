"""Independent endpoint/count audit and descriptive five-sample cost comparison."""
from pathlib import Path
import collections,hashlib,itertools,json,statistics
ROOT=Path(__file__).resolve().parents[3];RAW=ROOT/'docs/experiments/results/s06-projection-cost-pilot'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def expected(cell):
    mode,f,n,queries,endpoint,cancel=cell
    results=[]
    for q in range(queries):
        counts=collections.Counter();visible=[0] if f=='independent' else [n-2,n-1] if f=='star' else list(range(n))
        for values in itertools.product(range(2),repeat=n):
            if f=='star' and values[0]==1 and any(values[1:]):continue
            if q in [1,2] and values[visible[0]]!=q-1:continue
            counts[tuple(values[i] for i in visible)]+=1
        records=list(sorted(counts.items()))
        if endpoint=='expanded':records=[(v,1) for v,w in records for _ in range(w)]
        if cancel:records=records[:cancel]
        h=0;mask=(1<<64)-1
        for v,w in records:
            h=(h*6364136223846793005+len(v))&mask
            for x in v:h=(h*1099511628211+x+1)&mask
            h=(h*1099511628211+w)&mask
        results.append([len(records),h])
    return results
if __name__=='__main__':
    m=json.loads((RAW/'manifest.json').read_text())
    for p,h in m['sha256'].items():assert digest(ROOT/p)==h,p
    control=json.loads((RAW/'controls.json').read_text());assert control['returncode']==0 and not control['stderr']
    assert 'test result: ok. 2 passed; 0 failed; 0 ignored;' in control['stdout']
    schedule=json.loads((RAW/'schedule.json').read_text());assert len(schedule)==1904
    assert len(m['cells'])==272 and len({tuple(c) for c in m['cells']})==272
    samples=collections.defaultdict(lambda:collections.defaultdict(list));seen=set();oracles={}
    for i,(kind,repeat,cell) in enumerate(schedule):
        key=tuple(cell);assert (kind,repeat,key) not in seen;seen.add((kind,repeat,key))
        r=json.loads((RAW/f'run-{i:04}.json').read_text());assert r['returncode']==0 and not r['stderr']
        assert r['command']==[str(ROOT/m[kind]),*map(str,cell)]
        data=[json.loads(s) for s in r['stdout'].splitlines()];summary=data[0];rows=data[1:]
        assert [summary[k] for k in ['mode','family','n','queries','endpoint','cancel']]==cell
        assert summary['metered']==(kind=='meter')
        if key not in oracles:oracles[key]=expected(cell)
        assert summary['observed']==oracles[key],cell
        phases=[('source',0),('prepare',0),('source_dispose',0)]+[(phase,q) for q in range(cell[3]) for phase in ['query_start','first','remaining','query_dispose']]+[('prepare_dispose',0)]
        assert [(r['phase'],r['query']) for r in rows]==phases
        assert summary['total_ns']==sum(r['ns'] for r in rows)
        if kind=='meter':
            assert rows[0]['memory']['live_start']==rows[-1]['memory']['live_end']
            samples[key][kind].append([r['memory'] for r in rows])
        else:
            assert all('memory' not in r for r in rows)
            cold=sum(r['ns'] for r in rows if r['phase'] in ['source','prepare','source_dispose'] or (r['query']==0 and r['phase'] in ['query_start','first']))
            samples[key]['cold_first'].append(cold)
            samples[key]['phases'].append({f"{r['phase']}-{r['query']}":r['ns'] for r in rows})
            samples[key][kind].append(summary['total_ns'])
    assert seen=={(kind,r,tuple(c)) for c in m['cells'] for kind,count in [('primary',5),('meter',2)] for r in range(count)}
    cells=[]
    for cell in m['cells']:
        s=samples[tuple(cell)];assert s['meter'][0]==s['meter'][1],cell
        costs=s['primary'];memory=s['meter'][0]
        cells.append(dict(cell=cell,min_ns=min(costs),median_ns=statistics.median(costs),max_ns=max(costs),cold_first_median_ns=statistics.median(s['cold_first']),requested_bytes=sum(r['requested_bytes'] for r in memory),max_phase_live=max(r['peak_live'] for r in memory)-memory[0]['live_start'],phase_medians={k:statistics.median(p[k] for p in s['phases']) for k in s['phases'][0]}))
    lookup={tuple(c['cell']):c for c in cells};comparisons=[]
    for c in cells:
        mode,*scenario=c['cell']
        if mode not in ['ascending','descending','greedy']:continue
        for control in ['enumerate','separable','structural']:
            other=lookup.get(tuple([control,*scenario]))
            if other is None:continue
            status='gain' if c['max_ns']<0.9*other['min_ns'] else 'loss' if c['min_ns']>1.1*other['max_ns'] else 'unresolved'
            comparisons.append(dict(mode=mode,control=control,scenario=scenario,status=status,median_ratio=c['median_ns']/other['median_ns']))
    result=dict(status='passed',processes=1904,cells=cells,comparisons=comparisons,scope='exploratory five-sample lifecycle separations; no confidence intervals or whole-architecture ranking')
    (RAW/'audit.json').write_text(json.dumps(result,indent=2)+'\n')
    print('Verified 1904 processes, 272 cells, independent full/prefix digests and exact allocation pairs.')
    for mode in ['ascending','descending','greedy']:
        for control in ['enumerate','separable','structural']:
            print(mode,control,dict(collections.Counter(c['status'] for c in comparisons if c['mode']==mode and c['control']==control)))
