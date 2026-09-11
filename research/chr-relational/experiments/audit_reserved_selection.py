"""Reconstruct complete lifecycle comparisons and independent source work."""
import collections,gzip,hashlib,json,statistics,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s02-reserved-selection'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def norm(x,base):
    if isinstance(x,list):return [norm(v,base) for v in x]
    if isinstance(x,dict):return {k:(v-base if k in ['live_start','live_end','peak_live'] else norm(v,base)) for k,v in x.items() if k not in ['ns','cpu_ns']}
    return x
def main():
    f=read(BASE/'freeze.json');assert sha(BASE/'sources.zip')==f['archive_sha256'];assert sha(BASE/'jobs.json')==f['jobs_sha256']
    with zipfile.ZipFile(BASE/'sources.zip') as z:
        for p,h in f['sources'].items():
            assert hashlib.sha256(z.read(p)).hexdigest()==h
            assert sha(ROOT/p)==h,p
    for b in f['binaries'].values():assert sha(Path(b['path']))==b['sha256']
    clocks=[read(BASE/f'clock-{i}.json') for i in range(3)]
    assert all(c['exit_code']==0 for c in clocks)
    floor=max(json.loads(c['stdout'])['median_ns'] for c in clocks)
    jobs=read(BASE/'jobs.json');groups=collections.defaultdict(list);ends={};count=0
    phases=[('source',0),('prepare',0)]+[(p,i) for i in range(4) for p in ['input','encode','setup','execute','observe','engine_drop','input_drop','consumer']]+[('prepared_drop',0),('source_drop',0),('consumer_drop',0)]
    with gzip.open(BASE/'runs.jsonl.gz','rt') as stream:
        for i,line in enumerate(stream):
            raw=json.loads(line);assert raw['index']==i;count+=1;j=jobs[i]
            family,n,mode,retention,stop=j['cell'];kind=j['kind'];cell=tuple(j['cell'])
            assert raw['command']==[f['binaries'][kind]['path'],'selection',mode,family,str(n),retention,stop]
            assert raw['exit_code']==0 and not raw['timeout'] and not raw['stderr'],i
            events=[json.loads(x) for x in raw['stdout'].splitlines()];assert [x['event'] for x in events]==['start','result'];r=events[1]
            assert r['meter']==(kind=='meter') and r['work_enabled']==(kind=='work') and r['chr']==mode.startswith(('chr-','reserved-'))
            assert [(p['phase'],p['query']) for p in r['phases']]==phases
            assert [e['cancelled'] for e in r['endpoints']]==([False]*4 if stop=='complete' else [True,False,True,False])
            assert all(e['exhausted'] and e['observed'] for e in r['endpoints'] if not e['cancelled'])
            assert all(not e['observed'] for e in r['endpoints'] if e['cancelled'])
            if cell in ends:assert ends[cell]==r['endpoints']
            ends[cell]=r['endpoints']
            if kind=='meter':
                ms=[p['reading']['heap'] for p in r['phases']];base=ms[0]['live_start'];assert ms[-1]['live_end']==base
                assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]))
                groups[(kind,cell)].append(norm(r,base));assert r['work']==[]
            elif kind=='work':
                assert len(r['work'])==4;groups[(kind,cell)].append(r['work'])
            else:
                assert r['work']==[] and all('heap' not in p['reading'] for p in r['phases'])
                if not j['warmup']:
                    totals=collections.Counter()
                    for p in r['phases']:totals[p['phase']]+=p['reading']['ns']
                    total=sum(totals.values())
                    groups[(kind,cell)].append(dict(rep=j['rep'],total_ns=total,phases=dict(totals),sensitive=total<100*floor*len(phases)))
    assert count==len(jobs)==2112
    summaries=[];works={}
    for (kind,cell),values in sorted(groups.items()):
        if kind=='meter':
            assert len(values)==2 and values[0]==values[1],cell
            phases0=values[0]['phases'];ms=[p['reading']['heap'] for p in phases0]
            summaries.append(dict(kind=kind,cell=cell,requested_bytes=sum(m['requested_bytes'] for m in ms),peak_excess=max(m['peak_live'] for m in ms),phases=phases0))
        elif kind=='work':
            assert len(values)==2 and values[0]==values[1],cell
            works[cell]=values[0];summaries.append(dict(kind=kind,cell=cell,work=values[0]))
        else:
            assert len(values)==(5 if cell[-1]=='complete' else 1),cell
            summaries.append(dict(kind=kind,cell=cell,median_ns=statistics.median(v['total_ns'] for v in values),sensitive=any(v['sensitive'] for v in values),samples=values))
    traces=[]
    for rep in range(2):
        raw=read(BASE/f'trace-{rep}.json');assert raw['exit_code']==0 and not raw['stderr']
        rows=[json.loads(x) for x in raw['stdout'].splitlines()];assert len(rows)==32
        for row in rows:
            assert sum(row['rules'].values())==row['applications']
            prefix='reserved-' if row['reserved'] else 'chr-'
            for suffix in ['scan','indexed','sealed']:
                w=works[(row['family'],row['size'],prefix+suffix,'all','complete')]
                for i in ([0,2] if row['value']=='a' else [1,3]):assert w[i]['applications']==row['applications']
        traces.append(rows)
    assert traces[0]==traces[1]
    meter=read(BASE/'meter-check.json');assert meter['exit_code']==0
    ordinary={tuple(s['cell']):s for s in summaries if s['kind']=='ordinary' and s['cell'][-1]=='complete'}
    comparisons=[]
    for cell,left in ordinary.items():
        family,n,mode,retention,stop=cell
        if not mode.startswith('reserved-'):continue
        for control in ['chr-'+mode.removeprefix('reserved-'),'local','local-filtered','scan','indexed','sealed']:
            right=ordinary[(family,n,control,retention,stop)]
            a={s['rep']:s['total_ns'] for s in left['samples']};b={s['rep']:s['total_ns'] for s in right['samples']}
            ratios=[a[r]/b[r] for r in sorted(a)]
            comparisons.append(dict(cell=cell,control=control,median_paired_ratio=statistics.median(ratios),ratios=ratios,sensitive=left['sensitive'] or right['sensitive']))
    result=dict(processes=count,allocation_pairs=352,work_pairs=88,primary_cells=176,clock_median_ns=floor,summary=summaries,traces=traces[0],comparisons=comparisons)
    (BASE/'audit.json').write_text(json.dumps(result,indent=2))
    print(json.dumps({k:v for k,v in result.items() if k not in ['summary','traces','comparisons']}))
if __name__=='__main__':main()
