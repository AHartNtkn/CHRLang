"""Audit context-algorithm lifecycle timing and exact requested ownership."""
import collections
import gzip
import hashlib
import json
import statistics
import sys
import zipfile
sys.dont_write_bytecode=True
from audit_continuing import ROOT, lifecycle

OUT=ROOT/'docs/experiments/results/s08-context-lifecycle'
FIELDS=['source','mode','enabled','resource','demand','keep']


def read_rows(path):
    with gzip.open(path,'rt') as f:return [json.loads(s) for s in f]


def key(case):return tuple(case[k] for k in FIELDS)


def norm(data):
    root=data['records'][0]['reading']['memory']['live_start']
    rows=[]
    for x in data['records']:
        h=x['reading']['memory'].copy()
        for field in ['live_start','live_end','peak_live']:h[field]-=root
        rows.append(dict(phase=x['phase'],query=x['query'],answer=x['answer'],heap=h))
    assert rows[-1]['heap']['live_end']==0
    return rows


def main():
    f=json.loads((OUT/'freeze.json').read_text())
    sha=lambda b:hashlib.sha256(b).hexdigest()
    assert sha((OUT/'sources.zip').read_bytes())==f['archive_sha256']
    with zipfile.ZipFile(OUT/'sources.zip') as z:
        for p,h in f['sources'].items():assert sha(z.read(p))==h
    for name,count in [('entry',24),('warmups',12)]:
        receipts=json.loads((OUT/(name+'.json')).read_text());assert len(receipts)==count
        for receipt in receipts:
            assert receipt['raw']['exit_code']==0
            assert json.loads(receipt['raw']['stdout'])['validated']
    runs=read_rows(OUT/'runs.jsonl.gz');assert len(runs)==1120
    assert [r['job'] for r in runs]==f['jobs']
    old={}
    for r in read_rows(ROOT/'docs/experiments/results/s08-traversal-lifecycle/runs.jsonl.gz'):
        if r['job']['kind']=='meter':old[key(r['job']['case'])]=norm(json.loads(r['raw']['stdout']))
    cells=collections.defaultdict(dict)
    for r in runs:
        j,c,raw=r['job'],r['job']['case'],r['raw'];assert raw['exit_code']==0
        d=json.loads(raw['stdout']);assert d['validated']
        n=c['demand']
        if c['source']=='continuing':
            lifecycle(raw,n,c['keep'])
            if c['mode'].endswith('-reclaim'):
                assert all((p['removed']>0)==c['resource'] for p in d['snapshots'] if p['stage']=='before_cancel')
        else:
            assert collections.Counter(x['phase'] for x in d['records'])==dict(prepare=1,setup=2,produce=2*n,consumer=2*n,exhaustion=2,producer_drop=2,prepared_drop=1,consumer_drop=1)
        if j['kind']=='rss':
            assert d['snapshots'][0]['stage']=='root' and d['snapshots'][-1]['stage']=='disposed'
            assert all(s['rss_kib']>0 for s in d['snapshots'])
            if c['source']=='flat':assert len(d['snapshots'])==10
        cells[key(c)][j['kind'],j['rep']]=d
    assert len(cells)==160
    summaries=[];differences=[]
    for k,data in sorted(cells.items()):
        assert set(data)=={('time',i) for i in range(5)}|{('meter',i) for i in range(2)}
        a=norm(data['meter',0]);assert a==norm(data['meter',1]),k
        historical=(k[0],k[1],k[1]!='direct',*k[3:])
        if a!=old[historical]:differences.append(k)
        assert a==norm(cells[(k[0],k[1],'lookup',*k[3:])]['meter',0]),k
        totals=[sum(x['reading']['ns'] for x in data['time',i]['records']) for i in range(5)]
        names={x['phase'] for x in a}
        phase_medians={p:statistics.median([sum(x['reading']['ns'] for x in data['time',i]['records'] if x['phase']==p) for i in range(5)]) for p in names}
        first_batch = statistics.median([sum(x['reading']['ns'] for x in data['time', i]['records'] if x['phase']=='produce' and x['answer']==0) for i in range(5)])
        summaries.append(dict(case=dict(zip(FIELDS,k)),median_ns=statistics.median(totals),min_ns=min(totals),max_ns=max(totals),phase_medians=phase_medians,first_production_batch_median_ns=first_batch,
                              requested_bytes=sum(x['heap']['requested_bytes'] for x in a),peak_excess=max(x['heap']['peak_live'] for x in a)))
    assert not differences,('historical allocation differences',differences)
    comparisons=[]
    for k,data in sorted(cells.items()):
        source,mode,enabled,resource,n,keep=k
        if mode=='direct':continue
        for label,control in [('lookup',(source,mode,'lookup',resource,n,keep)),('direct',(source,'direct','lookup',resource,n,keep))]:
            reference=cells[control]
            ratios=[sum(x['reading']['ns'] for x in data['time',i]['records'])/sum(x['reading']['ns'] for x in reference['time',i]['records']) for i in range(5)]
            median=statistics.median(ratios)
            disposition='lower' if median<.9 and max(ratios)<1 else 'higher' if median>1.1 and min(ratios)>1 else 'unresolved'
            comparisons.append(dict(case=dict(zip(FIELDS,k)),control=label,median=median,minimum=min(ratios),maximum=max(ratios),disposition=disposition))
    report=dict(processes=1120,exact_allocation_pairs=160,historical_allocation_reproduced=True,owner_restoration=True,cells=summaries,comparisons=comparisons)
    (OUT/'analysis.json').write_text(json.dumps(report,indent=2)+'\n')
    print('Verified 1,120 runs, 160 exact historical allocation pairs, cross-algorithm ownership and complete disposal')
    for source in ['continuing','flat']:
        for mode in ['dependencies','templates','dependencies-reclaim','templates-reclaim']:
            for control in ['lookup','direct']:
                rs=[r for r in comparisons if r['case']['source']==source and r['case']['mode']==mode and r['control']==control]
                if rs:print(source,mode,control,dict(collections.Counter(r['disposition'] for r in rs)),min(r['median'] for r in rs),max(r['median'] for r in rs))


if __name__=='__main__':main()
