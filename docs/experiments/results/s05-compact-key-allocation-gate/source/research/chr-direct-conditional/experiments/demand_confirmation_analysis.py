#!/usr/bin/env python3
"""Registered descriptive paired analysis; no workload weights or CIs."""
import json
from pathlib import Path
from statistics import median
import demand_sizing_analysis as base
OUT=base.ROOT/'docs/experiments/results/s03-demand-confirmation'
def main():
    cells={};allocation={}
    warmups=list(OUT.glob('warmup-*.json'));assert len(warmups)==147
    for path in warmups:base.load(path)
    for block in range(1,8):
        paths=list(OUT.glob(f'block{block}-*.json'));assert len(paths)==147
        for path in paths:
            key,p,wall=base.load(path);assert not p['meter'] and all(s['complete'] for s in p['samples'])
            cell=cells.setdefault(key,{})
            assert block not in cell
            cell[block]=dict(base.timing(p),wall_seconds=wall)
    assert len(cells)==147 and all(len(v)==7 for v in cells.values())
    for path in sorted(OUT.glob('allocation-*.json')):
        key,p,_=base.load(path);assert p['meter'] and all(s['complete'] for s in p['samples'])
        readings=[phase['memory'] for phase in base.phases(p)]
        if key in allocation:
            assert readings==allocation[key]['readings'],key
            allocation[key]['repetitions']+=1
        else:allocation[key]=dict(base.allocations(p),readings=readings,repetitions=1)
    assert len(allocation)==147 and all(v['repetitions']==2 for v in allocation.values())
    rows=[]
    for key,samples in sorted(cells.items()):
        mode,family,n,q,resource=key
        row=dict(mode=mode,family=family,depth=n,queries=q,resource=resource)
        for endpoint in ['engine_ns','including_source_input_ns','execution_observation_ns','preparation_ns','first_answer_ns','wall_seconds']:
            values=[samples[b][endpoint] for b in range(1,8)]
            row[endpoint]=dict(median=median(values),minimum=min(values),maximum=max(values))
        row['allocation']={k:v for k,v in allocation[key].items() if k not in ('readings','repetitions')}
        rows.append(row)
    pairs=[]
    configurations=sorted({key[1:] for key in cells})
    for numerator,denominator in [('demand','current'),('demand','graph'),('demand','conditional'),('demand','scan'),('demand','indexed'),('lowered','demand')]:
        for config in configurations:
            ratios=[cells[(numerator,*config)][b]['engine_ns']/cells[(denominator,*config)][b]['engine_ns'] for b in range(1,8)]
            med=median(ratios)
            status='gain' if med<=.90 and max(ratios)<1 else 'loss' if med>=1.10 and min(ratios)>1 else 'unresolved'
            family,n,q,resource=config
            pairs.append(dict(numerator=numerator,denominator=denominator,family=family,depth=n,queries=q,resource=resource,ratios=ratios,median=med,minimum=min(ratios),maximum=max(ratios),status=status))
    result=dict(ordinary_warmups=147,ordinary_measured=1029,allocation_processes=294,allocation_replays_exact=True,interpretation='seven paired ratios, 10 percent practical criterion and consistent direction; descriptive, not confidence intervals',rows=rows,pairs=pairs)
    (OUT/'analysis.json').write_text(json.dumps(result,indent=2)+'\n')
    for family in ['plain','opaque','discriminate']:
        print(family,'depth32 queries8 resource=true')
        for row in rows:
            if row['family']==family and row['depth']==32 and row['queries']==8 and row['resource']:
                print(row['mode'],round(row['engine_ns']['median']/1e6,3),'ms',row['allocation']['requested_bytes'],'bytes')
        for pair in pairs:
            if pair['family']==family and pair['depth']==32 and pair['queries']==8 and pair['resource']:
                print(pair['numerator']+'/'+pair['denominator'],pair['status'],round(pair['median'],3),(round(pair['minimum'],3),round(pair['maximum'],3)))
    print('lowering cold zero-depth',[(p['family'],p['status'],round(p['median'],3)) for p in pairs if p['numerator']=='lowered' and p['depth']==0 and p['queries']==1])
if __name__=='__main__':main()
