#!/usr/bin/env python3
"""Prospective paired practical criteria; no weighted rankings."""
from pathlib import Path
import sys,json,hashlib
from statistics import median
from collections import Counter
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'research/chr-direct-conditional/experiments'))
from demand_sizing_analysis import load,phases,timing,allocations
OUT=ROOT/'docs/experiments/results/s02-contextual-confirmation'
def main():
    freeze=json.loads((OUT/'freeze.json').read_text())
    for name,h in freeze['sha256'].items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==h,name
    for name,h in freeze['binaries'].items():assert hashlib.sha256(Path(name).read_bytes()).hexdigest()==h,name
    times={};memory={};counts=Counter()
    for block in range(8):
        files=list(OUT.glob(f'block{block}-*.json'));assert len(files)==240
        for p in files:
            k,r,w=load(p);assert not r['meter'] and all(s['complete'] for s in r['samples'])
            if block:times.setdefault(k,{})[block]=dict(timing(r),wall_seconds=w)
    assert len(times)==240 and all(len(x)==7 for x in times.values())
    for p in OUT.glob('allocation-*.json'):
        k,r,_=load(p);assert r['meter'] and all(s['complete'] for s in r['samples']);counts[k]+=1
        readings=[x['memory'] for x in phases(r)]
        if k in memory:assert memory[k][0]==readings,k
        memory[k]=(readings,allocations(r))
    assert len(counts)==240 and set(counts.values())=={2}
    for kind in ['cancel','cancel-meter']:
        files=[p for p in OUT.glob(kind+'-*.json') if kind!='cancel' or not p.name.startswith('cancel-meter')];assert len(files)==5
        for p in files:
            _,r,_=load(p);assert [s['complete'] for s in r['samples']]==[False,True]
    rows=[];pairs=[]
    for k,values in sorted(times.items()):
        row=dict(mode=k[0],family=k[1],depth=k[2],queries=k[3],resource=k[4],allocation=memory[k][1])
        for endpoint in next(iter(values.values())):
            ns=[values[b][endpoint] for b in range(1,8)];row[endpoint]=dict(median=median(ns),minimum=min(ns),maximum=max(ns))
        rows.append(row)
    configs=sorted({k[1:] for k in times})
    for a,b in [('contextual','relational'),('contextual','scan'),('contextual','indexed'),('lowered','contextual')]:
        for c in configs:
            ratios=[times[(a,*c)][i]['engine_ns']/times[(b,*c)][i]['engine_ns'] for i in range(1,8)]
            m=median(ratios);status='gain' if m<=.9 and max(ratios)<1 else 'loss' if m>=1.1 and min(ratios)>1 else 'unresolved'
            pairs.append(dict(numerator=a,denominator=b,family=c[0],depth=c[1],queries=c[2],resource=c[3],ratios=ratios,median=m,minimum=min(ratios),maximum=max(ratios),status=status))
    (OUT/'analysis.json').write_text(json.dumps(dict(ordinary_measured=1680,warmups=240,allocation=480,cancellation=10,rows=rows,pairs=pairs,allocation_replays_exact=True),indent=2)+'\n')
    for row in rows:
        if row['depth']==32 and row['queries']==4 and row['resource']:print(row['family'],row['mode'],round(row['engine_ns']['median']/1e6,3),'ms',row['allocation'])
    for a,b in [('contextual','relational'),('contextual','scan'),('contextual','indexed'),('lowered','contextual')]:
        selected=[p for p in pairs if p['numerator']==a and p['denominator']==b]
        print(a,b,dict(Counter(p['status'] for p in selected)))
        for p in selected:
            if p['depth']==32 and p['queries']==4 and p['resource']:print(p['family'],round(p['median'],3),round(p['minimum'],3),round(p['maximum'],3),p['status'])
if __name__=='__main__':main()
