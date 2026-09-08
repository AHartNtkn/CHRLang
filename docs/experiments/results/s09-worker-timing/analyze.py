#!/usr/bin/env python3
"""Recompute the prospectively specified paired-block summaries."""
from pathlib import Path
import json,statistics,hashlib
ROOT=Path(__file__).resolve().parents[4]
OUT=Path(__file__).resolve().parent

def classify(ratios):
    median=statistics.median(ratios)
    if median<=0.90 and max(ratios)<1:return 'benefit'
    if median>=1.10 and min(ratios)>1:return 'loss'
    return 'unresolved'

def main():
    freeze=json.loads((OUT/'freeze.json').read_text())
    for name,h in freeze['source'].items():
        assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==h,name
    data={};warmups=0
    for path in OUT.glob('*.json'):
        r=json.loads(path.read_text())
        if 'block' not in r:continue
        assert r['exit_code']==0 and not r['cutoff'],path
        rows=[json.loads(line) for line in r['stdout'].splitlines()]
        h,qs=rows[0],rows[1:]
        assert tuple(h[k] for k in ['family','depth','queries','quantum','workers'])==tuple(r['cell']),path
        assert r['process_cpu_seconds']>=0
        assert all(0<=q['first_observation_ns']<=q['execute_observe_ns'] for q in qs)
        assert len(qs)==h['queries']
        assert h['lifecycle_ns']==h['prepare_ns']+h['shutdown_ns']+h['runtime_drop_ns']+sum(
            q['setup_ns']+q['execute_observe_ns']+q['close_ns']+q['dispose_ns'] for q in qs)
        if r['block']<0:warmups+=1;continue
        fields=dict(lifecycle=h['lifecycle_ns'],cpu=r['process_cpu_seconds'],first=qs[0]['first_observation_ns'],
                    prepare=h['prepare_ns'],setup=sum(q['setup_ns'] for q in qs),
                    execute_observe=sum(q['execute_observe_ns'] for q in qs),close=sum(q['close_ns'] for q in qs),
                    dispose=sum(q['dispose_ns'] for q in qs),shutdown=h['shutdown_ns']+h['runtime_drop_ns'])
        if len(qs)>1:
            fields['warm_query']=statistics.mean(q['setup_ns']+q['execute_observe_ns']+q['close_ns']+q['dispose_ns'] for q in qs[1:])
        data[(tuple(r['cell']),r['block'])]=fields
    assert len(data)==560 and warmups==80,(len(data),warmups)
    summaries=[]
    for cell in sorted({k[0] for k in data}):
        if cell[-1]==0:continue
        row=dict(cell=cell)
        for control in [0,1]:
            baseline=cell[:-1]+(control,)
            stats={}
            for field in data[(cell,0)]:
                values=[data[(cell,b)][field] for b in range(7)]
                ratios=[data[(cell,b)][field]/data[(baseline,b)][field] for b in range(7)]
                stats[field]=dict(median=statistics.median(values),min=min(values),max=max(values),
                                  ratios=ratios,ratio_median=statistics.median(ratios),
                                  classification=classify(ratios))
            row[f'versus_{control}']=stats
        summaries.append(row)
    (OUT/'summary.json').write_text(json.dumps(summaries,indent=2)+'\n')
    for workers in [1,2,4]:
        rs=[r for r in summaries if r['cell'][-1]==workers]
        for endpoint in ['lifecycle','cpu','first']:
            print(workers,endpoint,{c:sum(r['versus_0'][endpoint]['classification']==c for r in rs) for c in ['benefit','loss','unresolved']})
    for r in summaries:
        if r['cell'][-1]==4:
            print(r['cell'],[(x,round(r['versus_0'][x]['ratio_median'],3),r['versus_0'][x]['classification']) for x in ['lifecycle','cpu']])
if __name__=='__main__':main()
