#!/usr/bin/env python3
"""Describe exploratory S03 sizing without inferential rankings."""
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s03-demand-sizing'
PHASES=('input_build','setup','execute_observe','engine_drop','answer_drop')
def load(path):
    record=json.loads(path.read_text())
    assert record.get('returncode')==0 and not record.get('timeout')
    payload=[json.loads(line) for line in record['stdout'].splitlines() if line.startswith('{')][-1]
    assert payload['event']=='result' and not payload['counters']
    key=(payload['mode'],payload['family'],payload['samples'][0]['depth'],len(payload['samples']),payload['resource'])
    return key,payload,record['wall_seconds']
def phases(payload):
    yield payload['source_build']
    yield payload['preparation']
    for sample in payload['samples']:
        for name in PHASES:yield sample[name]
    yield payload['prepared_drop']
def timing(payload):
    samples=payload['samples']
    execution=sum(s['execute_observe']['ns'] for s in samples)
    engine=payload['preparation']['ns']+payload['prepared_drop']['ns']+sum(s[p]['ns'] for s in samples for p in PHASES if p!='input_build')
    return dict(engine_ns=engine,including_source_input_ns=sum(p['ns'] for p in phases(payload)),execution_observation_ns=execution,preparation_ns=payload['preparation']['ns'],first_answer_ns=samples[0]['first_answer_ns'])
def allocations(payload):
    readings=[p['memory'] for p in phases(payload)]
    assert all(r is not None for r in readings)
    return dict(requested_bytes=sum(r['requested_bytes'] for r in readings),allocation_calls=sum(r['allocation_calls'] for r in readings),peak_above_host_baseline=max(r['peak_live'] for r in readings)-readings[0]['live_start'])
def main():
    ordinary={};allocation={}
    for path in sorted(OUT.glob('ordinary-*.json')):
        key,payload,wall=load(path);assert key not in ordinary
        assert all(s['complete'] for s in payload['samples']) and not payload['meter']
        ordinary[key]=dict(timing(payload),wall_seconds=wall)
    assert len(ordinary)==147
    for path in sorted(OUT.glob('allocation-*.json')):
        key,payload,wall=load(path);assert payload['meter'] and all(s['complete'] for s in payload['samples'])
        memories=[p['memory'] for p in phases(payload)]
        if key in allocation:
            assert memories==allocation[key]['phase_readings'],f'Allocation replay disagreement: {key}'
            allocation[key]['repetitions']+=1
        else:allocation[key]=dict(allocations(payload),phase_readings=memories,repetitions=1)
    assert len(allocation)==42 and all(v['repetitions']==2 for v in allocation.values())
    for kind in ['cancel','cancel-meter']:
        paths=list(OUT.glob(kind+'-*.json'))
        if kind=='cancel':paths=[p for p in paths if not p.name.startswith('cancel-meter')]
        assert len(paths)==7
        for p in paths:
            _,r,_=load(p);assert [s['complete'] for s in r['samples']]==[False,True]
    rows=[]
    for key,values in sorted(ordinary.items()):
        mode,family,n,q,resource=key
        row=dict(mode=mode,family=family,depth=n,queries=q,resource=resource,**values)
        if key in allocation:row['allocation']={k:v for k,v in allocation[key].items() if k!='phase_readings'}
        rows.append(row)
    result=dict(status='exploratory single timing sample per cell, not confirmation',ordinary_processes=147,allocation_processes=84,cancellation_processes=14,allocation_replays_exact=True,rows=rows)
    (OUT/'analysis.json').write_text(json.dumps(result,indent=2)+'\n')
    for family in ['plain','opaque','discriminate']:
        print(family)
        for row in rows:
            if row['family']==family and row['depth']==32 and row['queries']==8 and row['resource']:
                print(row['mode'],round(row['engine_ns']/1e6,3),'ms',row['allocation']['requested_bytes'],'bytes',row['allocation']['peak_above_host_baseline'],'peak growth')
    print('max host wall seconds',max(r['wall_seconds'] for r in rows))
if __name__=='__main__':main()
