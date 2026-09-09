#!/usr/bin/env python3
"""Registered confirmation of complete source transformation costs."""
import json,hashlib,random
from pathlib import Path
import prefix_sizing as sizing
pilot=sizing.pilot
OUT=pilot.ROOT/'docs/experiments/results/s06-prefix-confirmation'
def main():
    freeze=json.loads((pilot.OUT/'freeze.json').read_text())
    for name,h in freeze['sha256'].items():assert hashlib.sha256((pilot.ROOT/name).read_bytes()).hexdigest()==h,name
    for name,h in freeze['binaries'].items():assert hashlib.sha256(Path(name).read_bytes()).hexdigest()==h,name
    assert pilot.CPU==freeze['cpu'];OUT.mkdir(exist_ok=True);pilot.OUT=OUT
    (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    configs=[(m,f,n,q,r) for m in sizing.MODES for f in sizing.FAMILIES for n in [1,8,32] for q in [1,8] for r in [False,True]]
    assert len(configs)==192
    for block in range(8):
        cells=configs.copy();random.Random(7302+block).shuffle(cells)
        for i,c in enumerate(cells):pilot.run(f'block{block}',i,*c)
        print(f'block {block}: 192 validated processes',flush=True)
    for repeat in range(2):
        for i,c in enumerate(configs):pilot.run('allocation',repeat*192+i,*c)
        print(f'allocation {repeat+1}: 192 validated processes',flush=True)
if __name__=='__main__':main()
