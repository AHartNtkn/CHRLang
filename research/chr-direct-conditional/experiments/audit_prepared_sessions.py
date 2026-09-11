"""Receipt validation and descriptive pilot estimates, not confirmation."""
import sys,zipfile,gzip,hashlib,json,statistics
from collections import Counter
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s08-prepared-sessions'
if sys.argv[1:]==['repair']: BASE=ROOT/'docs/experiments/results/s08-prepared-sessions-repair'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    freeze=json.loads((BASE/'freeze.json').read_text())
    assert sha(BASE/'sources.zip')==freeze['archive_sha256']
    assert sha(BASE/'jobs.json')==freeze['jobs_sha256']
    with zipfile.ZipFile(BASE/'sources.zip') as z:
        for p,h in freeze['sources'].items(): assert hashlib.sha256(z.read(p)).hexdigest()==h,p
    jobs=json.loads((BASE/'jobs.json').read_text())
    records=[json.loads(s) for s in gzip.open(BASE/'runs.jsonl.gz','rt')]
    assert len(records)==len(jobs)==3024
    rows=[];memory_repeats={};counts=Counter()
    prep_names={'source','count_prepare','static_prepare','value_prepare','engine_prepare','source_drop'}
    for record,job in zip(records,jobs):
        assert record['job']==job
        raw=record['raw'];assert raw['exit_code']==0
        result=json.loads(raw['stdout'].splitlines()[-1]);family,n,consumer,reuse,config=job['case']
        assert result['meter']==(job['kind']=='meter')
        phases=result['phases'];ends=result['endpoints']
        assert len(ends)==n and len(phases)==7*(1 if reuse else n)+7*n+1
        assert [p['query'] for p in phases if p['phase']=='input']==list(range(n))
        assert sum(p['phase']=='source' for p in phases)==(1 if reuse else n)
        for i,e in enumerate(ends):
            assert e['complete']==(not job['cancel'] or i%2==1)
            if e['complete']: assert e['answers']==8
            assert (e['first_ns'] is not None)==(e['answers']>0)
        first=[];pending=0;index=0
        for p in phases:
            if p['phase'] in prep_names | {'input','count_query','static_query','setup'}: pending+=p['reading']['ns']
            elif p['phase']=='execute_observe':
                e=ends[index];assert e['first_ns'] is None or e['first_ns']<=p['reading']['ns']
                first.append(None if e['first_ns'] is None else pending+e['first_ns']);pending=0;index+=1
        row=dict(job=job,total_ns=sum(p['reading']['ns'] for p in phases),prep_ns=sum(p['reading']['ns'] for p in phases if p['phase'] in prep_names | {'prepared_drop'}),first_lifecycle_ns=first)
        if result['meter']:
            memory=[p['reading']['memory'] for p in phases];root=memory[0]['live_start'];assert memory[-1]['live_end']==root
            key=json.dumps(job['case'])
            if job['stage']=='allocation':
                if key in memory_repeats:assert memory_repeats[key]==memory,key
                memory_repeats[key]=memory
            consumers=[p['reading']['memory']['live_end']-root for p in phases if p['phase']=='consumer']
            row.update(traffic=sum(m['requested_bytes'] for m in memory),peak=max(m['peak_live'] for m in memory)-root,consumer_live=consumers,consumer_only=next(p['reading']['memory']['live_end']-root for p in reversed(phases) if p['phase']=='prepared_drop'))
        rss=result['rss'];assert bool(rss)==job['rss']
        if rss:
            assert len(rss)==n+(1 if reuse else n)+3
            base=rss[0]['kib'];row['rss_delta_kib']=[dict(stage=p['stage'],query=p['query'],kib=p['kib']-base) for p in rss]
        rows.append(row);counts[job['stage']]+=1
    assert len(memory_repeats)==252
    paired=[]
    for family in ['common','independent']:
        for n in [1,16,128]:
            for consumer in ['immediate','window','all']:
                for config in json.loads(json.dumps(__import__('prepared_sessions').CONFIGS)):
                    selected=[r for r in rows if r['job']['stage']=='primary' and r['job']['case'][:3]==[family,n,consumer] and r['job']['case'][4]==config]
                    values={reuse:{r['job']['rep']:r['total_ns'] for r in selected if r['job']['case'][3]==reuse} for reuse in [False,True]}
                    assert all(len(x)==5 for x in values.values())
                    ratios=[values[True][i]/values[False][i] for i in range(5)]
                    paired.append(dict(family=family,queries=n,consumer=consumer,config=config,reuse_over_rebuild_median=statistics.median(ratios),range=[min(ratios),max(ratios)],rebuild_ns=statistics.median(values[False].values()),reuse_ns=statistics.median(values[True].values())))
    out=dict(processes=len(rows),counts=dict(counts),allocation_pairs=len(memory_repeats),paired=paired,rows=rows)
    (BASE/'audit.json').write_text(json.dumps(out,separators=(',',':')))
    print(json.dumps(dict(processes=len(rows),counts=dict(counts),allocation_pairs=len(memory_repeats)),indent=2))
if __name__=='__main__':main()
