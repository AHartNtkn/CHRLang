"""Audit semantic lifecycle endpoints and exact requested-heap ownership."""
import collections
import hashlib
import json
import zipfile
from audit_continuing import ROOT, rows, read, sha, lifecycle
OUT=ROOT/'docs/experiments/results/s08-traversal-ownership'

def main():
    f=read(OUT/'freeze.json');assert sha(OUT/'sources.zip')==f['archive_sha256']
    with zipfile.ZipFile(OUT/'sources.zip') as z:
        for p,h in f['sources'].items():assert hashlib.sha256(z.read(p)).hexdigest()==h
    for entry in f['binaries']:
        for b in entry['sources'].values():assert sha(ROOT/b['path'])==b['sha256']
    runs=rows(OUT/'runs.jsonl.gz');assert [r['job'] for r in runs]==f['jobs']
    assert len(runs)==336
    assert collections.Counter(r['job']['kind'] for r in runs)==dict(time=112,meter=224)
    allocations={};costs={}
    for r in runs:
        j=r['job'];c=j['case'];raw=r['raw'];assert raw['exit_code']==0,(j,raw['stderr'])
        n=c['demand'];d=json.loads(raw['stdout']);assert d['validated']
        if c['source']=='continuing':
            lifecycle(raw,n,c['keep'])
            if c['mode'].endswith('-reclaim'):
                for s in d['snapshots']:
                    if s['stage']=='before_cancel':assert (s['removed']>0)==c['resource']
        else:
            counts=collections.Counter(x['phase'] for x in d['records'])
            assert counts==dict(prepare=1,setup=2,produce=2*n,consumer=2*n,exhaustion=2,producer_drop=2,prepared_drop=1,consumer_drop=1),counts
        if j['kind']=='meter':
            m=[x['reading']['memory'] for x in d['records']];root=m[0]['live_start']
            assert m[-1]['live_end']==root
            key=tuple(c[k] for k in ['source','mode','enabled','resource','demand','keep'])
            if key in allocations:assert allocations[key]==m,key
            allocations[key]=m
            costs[key]=dict(requested=sum(x['requested_bytes'] for x in m),allocation_calls=sum(x['allocation_calls'] for x in m),peak=max(x['peak_live'] for x in m)-root,retained=m[-1]['live_start']-root)
            if c['keep']=='0':assert costs[key]['retained']==0
    summary=[]
    for key,off in costs.items():
        source,mode,enabled,resource,n,keep=key
        if enabled or mode=='direct':continue
        on=costs[source,mode,True,resource,n,keep]
        assert off['retained']==on['retained']
        direct=costs[source,'direct',False,resource,n,keep]
        summary.append(dict(source=source,mode=mode,resource=resource,demand=n,keep=keep,control=off,shared=on,direct=direct,traffic_ratio=on['requested']/off['requested'],peak_ratio=on['peak']/off['peak']))
    report=dict(processes=len(runs),allocation_cells=len(costs),exact_allocation_repeats=True,owner_restoration=True,rows=summary)
    (OUT/'audit.json').write_text(json.dumps(report,indent=2)+'\n')
    for source in ['continuing','flat']:
        cells=[r for r in summary if r['source']==source]
        print(source,'cells',len(cells),'traffic favorable',sum(r['traffic_ratio']<1 for r in cells),'adverse',sum(r['traffic_ratio']>1 for r in cells),'peak favorable',sum(r['peak_ratio']<1 for r in cells),'adverse',sum(r['peak_ratio']>1 for r in cells))
        for r in cells:
            if r['keep']=='0' and (r['demand']==128 or source=='flat'):print(r)

if __name__=='__main__':main()
