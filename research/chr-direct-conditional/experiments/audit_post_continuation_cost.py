"""Audit complete phase ownership, outcomes, exact allocation replay and paired costs."""
import collections,gzip,hashlib,json,statistics,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s10-post-continuation-cost'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    freeze=json.loads((BASE/'freeze.json').read_text());assert sha(BASE/'sources.zip')==freeze['archive_sha256'];assert sha(BASE/'jobs.json')==freeze['jobs_sha256']
    with zipfile.ZipFile(BASE/'sources.zip') as z:
        for p,h in freeze['sources'].items():assert sha(ROOT/p)==h and hashlib.sha256(z.read(p)).hexdigest()==h,p
    for b in freeze['binaries'].values():assert sha(Path(b['path']))==b['sha256']
    jobs=json.loads((BASE/'jobs.json').read_text());assert len(jobs)==26880
    phases=[(p,0) for p in ['source','count_prepare','static_prepare','value_prepare','engine_prepare','source_drop']]+[(p,i) for i in range(4) for p in ['input','count_query','static_query','setup','execute_observe','engine_drop','consumer']]+[('prepared_drop',0),('consumer_drop',0)]
    clock=json.loads((BASE/'clock-check.json').read_text());clock=next(json.loads(l) for l in clock['stdout'].splitlines() if l.startswith('{'));floor=clock['median']*36*100
    times=collections.defaultdict(dict);alloc=collections.defaultdict(list);endpoints={};counts=collections.Counter()
    with gzip.open(BASE/'runs.jsonl.gz','rt') as f:
        n=0
        for n,line in enumerate(f,1):
            row=json.loads(line);j=row['job'];assert j==jobs[n-1];raw=row['raw'];assert raw['exit_code']==0 and not raw['timeout']
            result=next(json.loads(l) for l in raw['stdout'].splitlines() if l.startswith('{') and json.loads(l).get('event')=='result');counts[j['stage']]+=1
            assert result['meter']==(j['kind']=='meter');assert [(r['phase'],r['query']) for r in result['phases']]==phases
            for i,e in enumerate(result['endpoints']):assert e['complete']==(not j['cancel'] or i%2==1);assert e['first_ns'] is None
            key=(*j['case'],j['mode'],j['counted']);endkey=(*key,j['cancel'])
            assert endpoints.setdefault(endkey,result['endpoints'])==result['endpoints']
            if j['stage']=='primary':times[key][j['rep']]=sum(r['reading']['ns'] for r in result['phases'])
            if j['kind']=='meter':
                base=result['phases'][0]['reading']['memory']['live_start'];last=base;norm=[];traffic=0;peak=base
                for p in result['phases']:
                    m=p['reading']['memory'];assert m['live_start']==last,(j,p);last=m['live_end'];traffic+=m['requested_bytes'];peak=max(peak,m['peak_live']);norm.append({k:v-base if k in ['live_start','live_end','peak_live'] else v for k,v in m.items()})
                assert last==base
                if j['stage']=='allocation':alloc[key].append(dict(phases=norm,bytes=traffic,peak=peak-base))
    assert n==26880 and len(times)==len(alloc)==2688
    for rows in alloc.values():assert len(rows)==2 and rows[0]==rows[1]
    summaries=[];contrasts=[]
    for key,ts in sorted(times.items()):
        assert set(ts)==set(range(5));summaries.append(dict(key=key,median_ns=statistics.median(ts.values()),min_ns=min(ts.values()),max_ns=max(ts.values()),sensitive=min(ts.values())<floor,bytes=alloc[key][0]['bytes'],peak=alloc[key][0]['peak']))
        if key[-1]:
            other=(*key[:-1],False);ratios=[ts[i]/times[other][i] for i in range(5)];contrasts.append(dict(key=key,comparison='counted/original',median_ratio=statistics.median(ratios),ratios=ratios,sensitive=min(list(ts.values())+list(times[other].values()))<floor))
        if key[-2]!='native-indexed':
            other=(*key[:-2],'native-indexed',key[-1]);ratios=[ts[i]/times[other][i] for i in range(5)];contrasts.append(dict(key=key,comparison='mode/generated-indexed',median_ratio=statistics.median(ratios),ratios=ratios,sensitive=min(list(ts.values())+list(times[other].values()))<floor))
    out=dict(processes=n,stages=dict(counts),allocation_pairs=len(alloc),clock_floor_ns=floor,summary=summaries,contrasts=contrasts)
    (BASE/'audit.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k not in ['summary','contrasts']}))
if __name__=='__main__':main()
