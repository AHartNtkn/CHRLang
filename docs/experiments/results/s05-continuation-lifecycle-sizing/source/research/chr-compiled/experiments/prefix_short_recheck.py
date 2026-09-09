#!/usr/bin/env python3
import json,hashlib,random
from statistics import median
import prefix_analysis_attribution as parent
pilot=parent.pilot
def main():
    freeze=json.loads((parent.OUT/'freeze.json').read_text())
    for paths in [freeze['original']['binaries'],freeze['after_binaries']]:
        for name,h in paths.items():assert hashlib.sha256(parent.Path(name).read_bytes()).hexdigest()==h
    pilot.OUT=parent.OUT/'short';pilot.OUT.mkdir(exist_ok=True)
    configs=[(f,q,r) for f in parent.sizing.FAMILIES for q in [1,8] for r in [False,True]]
    values={}
    for b in range(8):
        cases=[(v,*c) for v in ['after','sealed'] for c in configs];random.Random(7304+b).shuffle(cases)
        for i,(v,f,q,r) in enumerate(cases):
            pilot.BIN=parent.AFTER if v=='after' else parent.BEFORE
            p=pilot.run(f'{v}-block{b}',i,'lowered' if v=='after' else 'sealed',f,1,q,r)
            values[v,b,f,q,r]=parent.analysis.timing(p)['engine_ns']
        print(f'short block {b} complete',flush=True)
    pilot.BIN=parent.AFTER;memory={}
    for repeat in range(2):
        for i,(f,q,r) in enumerate(configs):
            p=pilot.run('allocation',repeat*16+i,'lowered',f,1,q,r)
            readings=[x['memory'] for x in parent.analysis.phases(p)]
            if (f,q,r) in memory:assert memory[f,q,r][0]==readings
            memory[f,q,r]=(readings,parent.analysis.allocations(p))
    rows=[]
    for f,q,r in configs:
        ratios=[values['after',b,f,q,r]/values['sealed',b,f,q,r] for b in range(1,8)];m=median(ratios)
        rows.append(dict(family=f,queries=q,resource=r,ratios=ratios,median=m,minimum=min(ratios),maximum=max(ratios),status='gain' if m<=.9 and max(ratios)<1 else 'loss' if m>=1.1 and min(ratios)>1 else 'unresolved',allocation=memory[f,q,r][1]))
    (pilot.OUT/'analysis.json').write_text(json.dumps(dict(rows=rows,ordinary_processes=256,allocation_processes=32),indent=2)+'\n')
    for row in rows:print(row['family'],row['queries'],row['resource'],round(row['median'],3),row['status'])
if __name__=='__main__':main()
