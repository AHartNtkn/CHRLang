#!/usr/bin/env python3
"""Registered before/after eligibility attribution and sealed-control recheck."""
import json,hashlib,random
from pathlib import Path
from statistics import median
from collections import Counter
import prefix_sizing as sizing
pilot=sizing.pilot
analysis=sizing.analysis
BEFORE=pilot.BIN
AFTER=Path('/tmp/chr-prefix-after-17444e43')
OUT=pilot.ROOT/'docs/experiments/results/s06-prefix-analysis'
def main():
    old=json.loads((pilot.OUT/'freeze.json').read_text())
    for name,h in old['binaries'].items():assert hashlib.sha256(Path(name).read_bytes()).hexdigest()==h
    for name,h in old['sha256'].items():
        if name!='research/chr-compiled/src/pure_prefix.rs':assert hashlib.sha256((pilot.ROOT/name).read_bytes()).hexdigest()==h,name
    assert pilot.CPU==old['cpu'];pilot.OUT=OUT
    (OUT/'freeze.json').write_text(json.dumps(dict(original=old,after_source=hashlib.sha256((pilot.ROOT/'research/chr-compiled/src/pure_prefix.rs').read_bytes()).hexdigest(),after_binaries={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in [AFTER/'ordinary',AFTER/'meter']}),indent=2)+'\n')
    configs=[(f,q,r) for f in sizing.FAMILIES for q in [1,8] for r in [False,True]]
    values={}
    for block in range(8):
        cases=[(v,*c) for v in ['before','after','sealed'] for c in configs]
        random.Random(7303+block).shuffle(cases)
        for i,(v,f,q,r) in enumerate(cases):
            pilot.BIN=AFTER if v=='after' else BEFORE
            payload=pilot.run(f'{v}-block{block}',i,'sealed' if v=='sealed' else 'lowered',f,32,q,r)
            values[v,block,f,q,r]=analysis.timing(payload)
        print(f'block {block}: 48 validated processes',flush=True)
    pilot.BIN=AFTER;memory={};counts=Counter()
    for repeat in range(2):
        for i,(f,q,r) in enumerate(configs):
            payload=pilot.run('allocation',repeat*16+i,'lowered',f,32,q,r)
            readings=[p['memory'] for p in analysis.phases(payload)];key=(f,q,r);counts[key]+=1
            if key in memory:assert memory[key][0]==readings
            memory[key]=(readings,analysis.allocations(payload))
    assert set(counts.values())=={2}
    rows=[]
    for f,q,r in configs:
        row=dict(family=f,queries=q,resource=r,allocation=memory[f,q,r][1])
        for control in ['before','sealed']:
            ratios=[values['after',b,f,q,r]['engine_ns']/values[control,b,f,q,r]['engine_ns'] for b in range(1,8)]
            m=median(ratios)
            row[control]=dict(median=m,minimum=min(ratios),maximum=max(ratios),ratios=ratios,status='gain' if m<=.9 and max(ratios)<1 else 'loss' if m>=1.1 and min(ratios)>1 else 'unresolved')
        for v in ['before','after','sealed']:
            row[v+'_ms']=median(values[v,b,f,q,r]['engine_ns'] for b in range(1,8))/1e6
            row[v+'_preparation_us']=median(values[v,b,f,q,r]['preparation_ns'] for b in range(1,8))/1e3
        rows.append(row)
    (OUT/'analysis.json').write_text(json.dumps(dict(ordinary_processes=384,allocation_processes=32,rows=rows),indent=2)+'\n')
    for r in rows:
        if r['resource']:print(r)
if __name__=='__main__':main()
