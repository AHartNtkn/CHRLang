#!/usr/bin/env python3
"""Prospectively registered competitor recheck after causal repair."""
import json
import hashlib
import random
from statistics import median
import demand_sizing as pilot
from demand_borrow import ORIGINAL,REPAIRED
from demand_sizing_analysis import load,timing

def main():
    parent=pilot.ROOT/'docs/experiments/results/s03-demand-borrow'
    freeze=json.loads((parent/'freeze.json').read_text())
    for paths in [freeze['original_control']['binaries'],freeze['repaired_binaries']]:
        for name,digest in paths.items():assert hashlib.sha256(pilot.Path(name).read_bytes()).hexdigest()==digest
    assert pilot.CPU==freeze['cpu']
    pilot.OUT=parent/'recheck';pilot.OUT.mkdir(exist_ok=True)
    configs=[(m,f,r) for m in ['demand','scan','indexed','graph','lowered'] for f in pilot.FAMILIES for r in [False,True]]
    values={}
    for block in range(8):
        shuffled=configs.copy();random.Random(7109+block).shuffle(shuffled)
        for i,(m,f,r) in enumerate(shuffled):
            pilot.BIN=REPAIRED if m=='demand' else ORIGINAL
            p=pilot.run(f'block{block}',i,m,f,32,8,r)
            values[block,m,f,r]=timing(p)['engine_ns']
        print(f'recheck block {block} complete',flush=True)
    rows=[]
    for m,f,r in configs:
        if m=='demand':continue
        ratios=[values[b,'demand',f,r]/values[b,m,f,r] for b in range(1,8)]
        row=dict(control=m,family=f,resource=r,median_ratio=median(ratios),ratio_range=[min(ratios),max(ratios)],
            demand_ms=median(values[b,'demand',f,r] for b in range(1,8))/1e6,
            control_ms=median(values[b,m,f,r] for b in range(1,8))/1e6,
            disposition='gain' if median(ratios)<=.9 and max(ratios)<1 else 'loss' if median(ratios)>=1.1 and min(ratios)>1 else 'unresolved')
        rows.append(row)
    (pilot.OUT/'analysis.json').write_text(json.dumps(dict(rows=rows,processes=240),indent=2)+'\n')
    for row in rows:
        if row['resource']:print(row)
if __name__=='__main__':main()
