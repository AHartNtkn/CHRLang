"""Validate coverage ablation, parent reproduction and paired lifecycle estimates."""
import collections
import hashlib
import json
import statistics
import zipfile
from audit_continuing import ROOT, COST, rows, read, lifecycle, sha

OUT=ROOT/'docs/experiments/results/s08-coverage-ablation'

def memory(d):
    root=d['records'][0]['reading']['memory']['live_start']
    return [dict((k,v-root if k in ['live_start','live_end','peak_live'] else v) for k,v in x['reading']['memory'].items()) for x in d['records']]

def main():
    f=read(OUT/'freeze.json')
    assert sha(OUT/'sources.zip')==f['archive_sha256']
    with zipfile.ZipFile(OUT/'sources.zip') as z:
        for p,h in f['sources'].items():assert hashlib.sha256(z.read(p)).hexdigest()==h
    for v in f['binaries'].values():
        for b in v.values():assert sha(ROOT/b['path'])==b['sha256']
    diag=rows(OUT/'diagnostic.jsonl.gz');assert [x['job'] for x in diag]==f['diagnostic']
    stages={}
    for x in diag:
        j=x['job'];raw=x['raw'];assert raw['exit_code']==0
        key=(j['variant'],j['resource'])
        d=[json.loads(line) for line in raw['stdout'].splitlines()]
        if key in stages:assert stages[key]==d
        stages[key]=d
    runs=rows(OUT/'runs.jsonl.gz');assert [x['job'] for x in runs]==f['jobs']
    assert collections.Counter(x['job']['stage'] for x in runs)==dict(warmup=12,primary=84,allocation=24)
    cells={};alloc={}
    for x in runs:
        j=x['job'];v,r,n=j['case'];d=lifecycle(x['raw'],n,'all')
        cells[j['stage'],j['rep'],v,r,n]=d
        if j['stage']=='allocation':
            key=(v,r,n);m=memory(d)
            if key in alloc:assert alloc[key]==m
            alloc[key]=m
    parents={}
    for x in rows(COST/'runs.jsonl.gz'):
        j=x['job'];r,n,keep,pack,cover=j['case']
        if j['stage']=='allocation' and j['rep']==0 and keep=='all' and not pack:
            parents['both' if cover else 'off',r,n]=json.loads(x['raw']['stdout'])
    offsets=[]
    for key,d in parents.items():
        assert memory(d)==alloc[key],key
        current=cells['allocation',0,*key]
        # CLI argument strings survive outside the lifecycle; executable paths differ.
        oldroot=d['records'][0]['reading']['memory']['live_start']
        newroot=current['records'][0]['reading']['memory']['live_start']
        parentraw=next(x['raw'] for x in rows(COST/'runs.jsonl.gz') if x['job']['stage']=='allocation' and x['job']['rep']==0 and x['job']['case']==[key[1],key[2],'all',False,key[0]=='both'])
        newpath=f['binaries'][key[0]]['meter']['path']
        assert newroot-oldroot==len(newpath)-len(parentraw['command'][0])
        offsets.append(dict(variant=key[0],resource=key[1],demand=key[2],root_offset=newroot-oldroot,allocation_records_relative_to_root_exact=True))
    results=[]
    for r in [False,True]:
        for n in [32,128]:
            times={v:[sum(x['reading']['ns'] for x in cells['primary',rep,v,r,n]['records']) for rep in range(7)] for v in ['off','equality','both']}
            ratios={}
            for denominator in ['off','both']:
                values=[a/b for a,b in zip(times['equality'],times[denominator])]
                ratios[denominator]=dict(median=statistics.median(values),range=[min(values),max(values)],saved_ms=statistics.median([(a-b)/1e6 for a,b in zip(times[denominator],times['equality'])]))
            result=dict(resource=r,demand=n,median_ms={v:statistics.median(t)/1e6 for v,t in times.items()},equality_ratios=ratios,requested_bytes={v:sum(x['requested_bytes'] for x in alloc[v,r,n]) for v in times},stages={v:next(x for x in stages[v,r] if x['answers']==n) for v in times})
            results.append(result)
    report=dict(processes=len(runs),diagnostic_processes=len(diag),allocation_repeats_exact=True,parent_reproduction=offsets,rows=results)
    (OUT/'audit.json').write_text(json.dumps(report,indent=2)+'\n')
    for x in results:print({k:v for k,v in x.items() if k!='stages'})

if __name__=='__main__':main()
