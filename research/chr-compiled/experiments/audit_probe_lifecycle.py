"""Audit complete costs, exact heap repeats and paired native lifecycle ratios."""
import collections,gzip,hashlib,json,statistics,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s01-probe-lifecycle'
FIELDS=['feature','family','n','policy','access','keep','cancel']
def key(c):return tuple(c[k] for k in FIELDS)
def normalized(d):
    root=d['records'][0]['heap']['live_start'];result=[];last=root
    for r in d['records']:
        h=r['heap'].copy();assert h['live_start']==last;last=h['live_end']
        assert h['peak_live']>=max(h['live_start'],h['live_end'])
        for k in ['live_start','live_end','peak_live']:h[k]-=root
        result.append(dict(phase=r['phase'],q=r['q'],heap=h))
    assert last==root
    return result

def main():
    f=json.loads((OUT/'freeze.json').read_text());sha=lambda b:hashlib.sha256(b).hexdigest()
    assert sha((OUT/'sources.zip').read_bytes())==f['archive_sha256']
    with zipfile.ZipFile(OUT/'sources.zip') as z:
        for p,h in f['sources'].items():assert sha(z.read(p))==h
    for name,count in [('entry',32),('warmups',16)]:
        rr=json.loads((OUT/(name+'.json')).read_text());assert len(rr)==count
        for x in rr:assert x['raw']['exit_code']==0 and json.loads(x['raw']['stdout'])['validated']
    with gzip.open(OUT/'runs.jsonl.gz','rt') as g:runs=[json.loads(line) for line in g]
    assert len(runs)==1344 and [x['job'] for x in runs]==f['jobs']
    cells=collections.defaultdict(dict)
    for r in runs:
        j=r['job'];c=j['case'];assert r['raw']['exit_code']==0;d=json.loads(r['raw']['stdout']);assert d['validated'] and d['feature']==c['feature'] and d['cancel']==c['cancel']
        counts=collections.Counter(x['phase'] for x in d['records']);assert counts==dict(prepare=1,setup=2,execute=2,observe=1 if c['cancel'] else 2,engine_drop=2,answer_drop=2,prepared_drop=1,retained_drop=1)
        for phase in ['setup','execute','engine_drop','answer_drop']:assert [x['q'] for x in d['records'] if x['phase']==phase]==[0,1]
        assert [x['q'] for x in d['records'] if x['phase']=='observe']==([1] if c['cancel'] else [0,1])
        assert all((x['heap'] is None)==(j['kind']=='time') for x in d['records'])
        k=key(c);assert (j['kind'],j['rep']) not in cells[k];cells[k][j['kind'],j['rep']]=d
    assert len(cells)==192
    summaries=[];comparisons=[]
    def total(d):return sum(x['ns'] for x in d['records'])
    for k,data in sorted(cells.items()):
        assert set(data)=={('time',i) for i in range(5)}|{('meter',i) for i in range(2)}
        heap=normalized(data['meter',0]);assert heap==normalized(data['meter',1]),k
        totals=[total(data['time',i]) for i in range(5)]
        phases={p:statistics.median(sum(x['ns'] for x in data['time',i]['records'] if x['phase']==p) for i in range(5)) for p in {x['phase'] for x in heap}}
        summaries.append(dict(case=dict(zip(FIELDS,k)),median_ns=statistics.median(totals),minimum_ns=min(totals),maximum_ns=max(totals),phase_medians=phases,requested_bytes=sum(x['heap']['requested_bytes'] for x in heap),peak_excess=max(x['heap']['peak_live'] for x in heap),heap_phases=heap))
        if not k[0]:continue
        for access in ['indexed','scan']:
            control=list(k);control[0]=False;control[4]=access;ref=cells[tuple(control)]
            ratios=[total(data['time',i])/total(ref['time',i]) for i in range(5)];med=statistics.median(ratios)
            comparisons.append(dict(case=dict(zip(FIELDS,k)),control=access,median=med,minimum=min(ratios),maximum=max(ratios),disposition='lower' if med<.9 and max(ratios)<1 else 'higher' if med>1.1 and min(ratios)>1 else 'unresolved'))
    report=dict(processes=1344,cells=summaries,comparisons=comparisons,exact_heap_pairs=192,owner_restoration=True)
    (OUT/'analysis.json').write_text(json.dumps(report,indent=2)+'\n')
    print('Verified1344 samples,192 exact heap pairs and complete ownership restoration')
    for family in ['selective','neutral','duplicate','broad']:
        for policy in ['global','active']:
            rs=[r for r in comparisons if r['case']['family']==family and r['case']['policy']==policy and r['control']=='indexed']
            print(family,policy,dict(collections.Counter(x['disposition'] for x in rs)),min(x['median'] for x in rs),max(x['median'] for x in rs))
if __name__=='__main__':main()
