"""Keep CPU diagnostics distinct from the registered primary wall series."""
import collections,gzip,hashlib,json,statistics,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s01-probe-clock-diagnosis'
def main():
    f=json.loads((OUT/'freeze.json').read_text());sha=lambda b:hashlib.sha256(b).hexdigest()
    assert sha((OUT/'sources.zip').read_bytes())==f['archive_sha256']
    with zipfile.ZipFile(OUT/'sources.zip') as z:
        for p,h in f['sources'].items():assert sha(z.read(p))==h
    with gzip.open(OUT/'runs.jsonl.gz','rt') as g:runs=[json.loads(s) for s in g]
    assert len(runs)==360 and [x['job'] for x in runs]==f['jobs']
    cells=collections.defaultdict(dict);overheads=[]
    for r in runs:
        j=r['job'];c=j['case'];ss=r['raw']['sessions'];assert r['raw']['exit_code']==0 and len(ss)==50
        for s in ss:
            assert s['validated'] and not s['cancel'] and s['feature']==j['feature']
            assert collections.Counter(x['phase'] for x in s['records'])==dict(prepare=1,setup=2,execute=2,observe=2,engine_drop=2,answer_drop=2,prepared_drop=1,retained_drop=1)
            assert all(x['heap'] is None and x['cpu_ns']>=0 for x in s['records'])
        totals={k:sum(x[k] for s in ss for x in s['records'])/50 for k in ['ns','cpu_ns']}
        key=(c['family'],c['n'],c['policy'],j['feature']);assert j['rep'] not in cells[key];cells[key][j['rep']]=totals
        overheads.append(dict(case=key,rep=j['rep'],wall_cpu=totals['ns']/totals['cpu_ns'],**totals))
    comparisons=[]
    for key,data in sorted(cells.items()):
        assert set(data)==set(range(20))
        if not key[-1]:continue
        ref=cells[(*key[:-1],False)]
        for metric in ['ns','cpu_ns']:
            ratios=[data[i][metric]/ref[i][metric] for i in range(20)];med=statistics.median(ratios)
            comparisons.append(dict(case=key[:-1],metric=metric,median=med,minimum=min(ratios),maximum=max(ratios),disposition='lower' if med<.9 and max(ratios)<1 else 'higher' if med>1.1 and min(ratios)>1 else 'unresolved'))
    assert len(cells)==18
    report=dict(processes=360,sessions=18000,comparisons=comparisons,wall_cpu=overheads)
    (OUT/'analysis.json').write_text(json.dumps(report,indent=2)+'\n')
    print('Verified18000 complete dual-clock sessions')
    ratios=[x['wall_cpu'] for x in overheads];print('wall/CPU median/min/max',statistics.median(ratios),min(ratios),max(ratios),'above1.2',sum(x>1.2 for x in ratios))
    for x in comparisons:
        if x['metric']=='cpu_ns':print(x)
if __name__=='__main__':main()
