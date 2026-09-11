"""Post-pilot attribution bound; no new timing verdicts or discarded runs."""
import collections,json,statistics
from pathlib import Path
RAW=Path(__file__).resolve().parents[3]/'docs/experiments/results/s06-connected-cost-pilot'
def diagnose():
    manifest=json.loads((RAW/'manifest.json').read_text());phases={}
    for kind,k,i in json.loads((RAW/'schedule.json').read_text()):
        if kind!='primary':continue
        lines=[json.loads(x) for x in json.loads((RAW/f'{kind}-{k:02}-{i:03}.json').read_text())['stdout'].splitlines()]
        row=collections.Counter()
        for x in lines[1:]:row[x['phase']]+=x['ns']
        phases[tuple(manifest['cells'][i]),k]=row
    out=[]
    for cell in map(tuple,manifest['cells']):
        if cell[0]!='projection':continue
        other=('enumerate',*cell[1:]);free=[];shares=[];service=[]
        for k in range(10):
            a,b=phases[cell,k],phases[other,k]
            free.append((sum(a.values())-a['prepare'])/sum(b.values()))
            shares.append(a['prepare']/sum(a.values()))
            service.append((a['setup']+a['first']+a['remaining'])/(b['setup']+b['first']+b['remaining']))
        out.append(dict(scenario=list(cell[1:]),free_prepare_median=statistics.median(free),prepare_share_median=statistics.median(shares),service_ratio_median=statistics.median(service)))
    return out
if __name__=='__main__':
    assert diagnose()==json.loads((RAW/'diagnosis.json').read_text())
    print('All 192 attribution bounds reproduce from the 3840 primary sessions.')
