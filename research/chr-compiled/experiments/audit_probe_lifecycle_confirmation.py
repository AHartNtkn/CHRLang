"""Audit batched complete lifecycles and exact single-session ownership replay."""
import collections,gzip,hashlib,json,statistics,sys,zipfile
sys.dont_write_bytecode=True
from audit_probe_lifecycle import ROOT,normalized
OUT=ROOT/'docs/experiments/results/s01-probe-lifecycle-confirmation'
def main():
    f=json.loads((OUT/'freeze.json').read_text());sha=lambda b:hashlib.sha256(b).hexdigest()
    assert sha((OUT/'sources.zip').read_bytes())==f['archive_sha256']
    with zipfile.ZipFile(OUT/'sources.zip') as z:
        for p,h in f['sources'].items():assert sha(z.read(p))==h
    with gzip.open(OUT/'runs.jsonl.gz','rt') as g:runs=[json.loads(s) for s in g]
    assert len(runs)==396 and [x['job'] for x in runs]==f['jobs']
    with gzip.open(OUT/'warmups.json.gz','rt') as g:warm=json.load(g)
    assert len(warm)==18 and all(len(x['raw']['sessions'])==50 for x in warm)
    old=json.loads((ROOT/'docs/experiments/results/s01-probe-lifecycle/analysis.json').read_text())
    historical={(r['case']['family'],r['case']['n'],r['case']['policy'],r['case']['feature']):r['heap_phases'] for r in old['cells'] if r['case']['access']=='indexed' and not r['case']['keep'] and not r['case']['cancel']}
    cells=collections.defaultdict(dict)
    for r in runs:
        j=r['job'];ss=r['raw']['sessions'];c=j['case'];assert r['raw']['exit_code']==0 and len(ss)==j['count']
        for s in ss:
            assert s['validated'] and s['feature']==j['feature'] and not s['cancel']
            assert collections.Counter(x['phase'] for x in s['records'])==dict(prepare=1,setup=2,execute=2,observe=2,engine_drop=2,answer_drop=2,prepared_drop=1,retained_drop=1)
            assert all((x['heap'] is None)==(j['kind']=='time') for x in s['records'])
        key=(c['family'],c['n'],c['policy'],j['feature']);assert (j['kind'],j['rep']) not in cells[key];cells[key][j['kind'],j['rep']]=ss
    comparisons=[];summaries=[]
    for key,data in sorted(cells.items()):
        assert set(data)=={('time',i) for i in range(20)}|{('meter',i) for i in range(2)}
        heap=normalized(data['meter',0][0]);assert heap==normalized(data['meter',1][0])==historical[key],key
        totals=[sum(x['ns'] for s in data['time',i] for x in s['records'])/50 for i in range(20)]
        summaries.append(dict(case=key,median_session_ns=statistics.median(totals),minimum_session_ns=min(totals),maximum_session_ns=max(totals)))
        if not key[-1]:continue
        ref=cells[(*key[:-1],False)];reference=[sum(x['ns'] for s in ref['time',i] for x in s['records'])/50 for i in range(20)]
        ratios=[a/b for a,b in zip(totals,reference)];med=statistics.median(ratios)
        comparisons.append(dict(case=key[:-1],median=med,minimum=min(ratios),maximum=max(ratios),ratios=ratios,disposition='lower' if med<.9 and max(ratios)<1 else 'higher' if med>1.1 and min(ratios)>1 else 'unresolved'))
    assert len(cells)==18 and len(comparisons)==9
    (OUT/'analysis.json').write_text(json.dumps(dict(processes=396,timing_sessions=18000,exact_historical_heap_pairs=18,cells=summaries,comparisons=comparisons),indent=2)+'\n')
    print('Verified18000 complete timing sessions and18 exact historical allocation pairs')
    for r in comparisons:print(r['case'],r['median'],r['minimum'],r['maximum'],r['disposition'])
if __name__=='__main__':main()
