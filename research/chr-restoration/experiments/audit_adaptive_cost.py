"""Audit exact matrix coverage, ownership and exploratory cost separations."""
from pathlib import Path
import collections,gzip,hashlib,json,statistics,sys
ROOT=Path(__file__).resolve().parents[3];RAW=ROOT/'docs/experiments/results/s04-adaptive-cost-pilot'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def read(path,kind,cell,m):
    r=json.loads(gzip.decompress(path.read_bytes()));assert r['returncode']==0 and not r['stderr'],path
    assert r['command']==[str(ROOT/m['binaries'][kind]),*map(str,cell)]
    data=[json.loads(s) for s in r['stdout'].splitlines()];head=data[0]
    mode,family,depth,reuse,keep,cancel=cell
    assert head['counts']==[1 if cancel else 16]*reuse
    assert head['retained']==(sum(head['counts']) if keep=='all' else 0)
    assert head['metered']==(kind=='meter')
    if mode in ['eager','scheduled']:assert head['engine_bytes']==(88 if mode=='eager' else 120)
    rows=data[2:] if kind=='meter' else data[1:]
    phases=[('source',0),('prepare',0),('source_dispose',0)]+[(phase,q) for q in range(reuse) for phase in ['input','setup','first','remaining','engine_dispose','input_dispose']]+[('prepared_dispose',reuse),('consumer_dispose',reuse)]
    assert [(x['phase'],x['query']) for x in rows]==phases
    assert head['total_ns']==sum(x['ns'] for x in rows)
    if kind=='meter':
        initial=rows[0]['memory']['live_start'];assert rows[-1]['memory']['live_end']==initial
        assert data[1]['unreleased_bytes']==0 and data[1]['consumer_bytes']==rows[-2]['memory']['live_end']-initial
        if keep=='0':
            assert data[1]['consumer_bytes']==0
            assert all(x['memory']['live_end']==rows[2]['memory']['live_end'] for x in rows if x['phase']=='input_dispose')
    else:assert all('memory' not in x for x in rows)
    return head,rows
if __name__=='__main__':
    mode=sys.argv[1];assert mode in ['qualify','pilot'];m=json.loads((RAW/'manifest.json').read_text())
    for p,h in m['sha256'].items():assert digest(ROOT/p)==h,p
    assert len(m['cells'])==384 and len({tuple(c) for c in m['cells']})==384
    for kind in ['primary','meter']:
        for i,cell in enumerate(m['cells']):read(RAW/f'qualify-{kind}-{i:03}.json.gz',kind,cell,m)
    if mode=='qualify':print('Verified 768 qualified endpoints and metered owner conservation.');sys.exit(0)
    jobs=json.loads((RAW/'schedule.json').read_text());assert len(jobs)==2688
    samples=collections.defaultdict(lambda:collections.defaultdict(list));seen=set()
    for i,(kind,rep,cell) in enumerate(jobs):
        key=tuple(cell);assert (kind,rep,key) not in seen;seen.add((kind,rep,key))
        head,rows=read(RAW/f'run-{i:04}.json.gz',kind,cell,m)
        if kind=='meter':samples[key]['memory'].append([x['memory'] for x in rows])
        else:
            samples[key]['total'].append(head['total_ns'])
            samples[key]['phases'].append({f"{x['phase']}-{x['query']}":x['ns'] for x in rows})
            samples[key]['cold_first'].append(sum(x['ns'] for x in rows if x['phase'] in ['source','prepare','source_dispose'] or (x['query']==0 and x['phase'] in ['input','setup','first'])))
    assert seen=={(kind,r,tuple(c)) for c in m['cells'] for kind,count in [('primary',5),('meter',2)] for r in range(count)}
    cells=[]
    for cell in m['cells']:
        s=samples[tuple(cell)];assert s['memory'][0]==s['memory'][1],cell
        costs=s['total'];memory=s['memory'][0];base=memory[0]['live_start']
        cells.append(dict(cell=cell,samples_ns=costs,min_ns=min(costs),median_ns=statistics.median(costs),max_ns=max(costs),cold_first_median_ns=statistics.median(s['cold_first']),requested_bytes=sum(r['requested_bytes'] for r in memory),peak_live_bytes=max(r['peak_live'] for r in memory)-base,consumer_bytes=memory[-2]['live_end']-base,phase_medians={k:statistics.median(x[k] for x in s['phases']) for k in s['phases'][0]}))
    lookup={tuple(c['cell']):c for c in cells};comparisons=[]
    for cell in cells:
        candidate,*scenario=cell['cell']
        if candidate not in ['scheduled','backoff1','backoff8']:continue
        controls=['eager'] if candidate=='scheduled' else ['copy','reunion','eager','fixed1','fixed8']
        for control in controls:
            other=lookup[tuple([control,*scenario])]
            status='gain' if cell['max_ns']<.9*other['min_ns'] else 'loss' if cell['min_ns']>1.1*other['max_ns'] else 'unresolved'
            comparisons.append(dict(candidate=candidate,control=control,scenario=scenario,status=status,median_ratio=cell['median_ns']/other['median_ns']))
    (RAW/'audit.json').write_text(json.dumps(dict(status='passed',qualification_processes=768,pilot_processes=2688,cells=cells,comparisons=comparisons,scope='exploratory warmed lifecycle ranges, not confidence intervals or architecture ranking'),indent=2)+'\n')
    print('Verified 2688 pilot processes, 384 cells and exact allocation pairs.')
    for candidate in ['scheduled','backoff1','backoff8']:
        for control in (['eager'] if candidate=='scheduled' else ['copy','reunion','eager','fixed1','fixed8']):
            print(candidate,control,dict(collections.Counter(c['status'] for c in comparisons if c['candidate']==candidate and c['control']==control)))
