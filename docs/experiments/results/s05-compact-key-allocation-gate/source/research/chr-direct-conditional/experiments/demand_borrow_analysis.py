#!/usr/bin/env python3
"""Analyze registered obligation-copy intervention, excluding warmup."""
import json
from statistics import median
from demand_sizing_analysis import ROOT,load,timing,allocations,phases
OUT=ROOT/'docs/experiments/results/s03-demand-borrow'

def main():
    timings={};memories={};original={}
    for path in sorted(OUT.glob('*-block*.json')):
        key,payload,_=load(path)
        assert not payload['meter'] and all(s['complete'] for s in payload['samples'])
        variant,block=path.name.split('-')[:2]
        block=int(block.removeprefix('block'))
        timings[variant,block,key]=timing(payload)['engine_ns']
    assert len(timings)==96
    for path in sorted(OUT.glob('allocation-*.json')):
        key,payload,_=load(path)
        readings=[p['memory'] for p in phases(payload)]
        if key in memories:assert readings==memories[key][0]
        else:memories[key]=(readings,allocations(payload))
    assert len(memories)==6
    for path in (ROOT/'docs/experiments/results/s03-demand-confirmation').glob('allocation-*.json'):
        key,payload,_=load(path)
        if key in memories:original[key]=allocations(payload)
    rows=[]
    for key in sorted(memories):
        ratios=[timings['after',b,key]/timings['before',b,key] for b in range(1,8)]
        row=dict(family=key[1],resource=key[4],median_ratio=median(ratios),ratio_range=[min(ratios),max(ratios)],
            before_ms=median(timings['before',b,key] for b in range(1,8))/1e6,
            after_ms=median(timings['after',b,key] for b in range(1,8))/1e6,
            before_memory=original[key],after_memory=memories[key][1],
            practical_gain=median(ratios)<=.9 and max(ratios)<1)
        rows.append(row)
        print(row)
    (OUT/'analysis.json').write_text(json.dumps(dict(rows=rows,timing_processes=96,allocation_processes=12),indent=2)+'\n')
if __name__=='__main__':main()
