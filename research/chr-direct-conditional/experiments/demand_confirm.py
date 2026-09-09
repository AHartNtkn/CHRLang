#!/usr/bin/env python3
"""Frozen bounded confirmation following S03 exploratory sizing."""
import hashlib
import json
import random
import demand_sizing as pilot

def main():
    sizing=pilot.OUT
    freeze=json.loads((sizing/'freeze.json').read_text())
    for name,digest in freeze['binaries'].items():
        assert hashlib.sha256(pilot.Path(name).read_bytes()).hexdigest()==digest
    for name,digest in freeze['sha256'].items():
        assert hashlib.sha256((pilot.ROOT/name).read_bytes()).hexdigest()==digest
    assert pilot.CPU==freeze['cpu']
    pilot.OUT=pilot.ROOT/'docs/experiments/results/s03-demand-confirmation'
    pilot.OUT.mkdir(parents=True,exist_ok=True)
    (pilot.OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    configs=[(m,f,n,q,True) for m in pilot.MODES for f in pilot.FAMILIES for n in [0,8,32] for q in [1,8]]
    configs += [(m,f,32,8,False) for m in pilot.MODES for f in pilot.FAMILIES]
    assert len(configs)==147
    for block in range(8):
        shuffled=configs.copy();random.Random(7107+block).shuffle(shuffled)
        kind='warmup' if block==0 else f'block{block}'
        for index,config in enumerate(shuffled):pilot.run(kind,index,*config)
        print(f'{kind} complete: 147 validated processes',flush=True)
    for repetition in range(2):
        for index,config in enumerate(configs):pilot.run('allocation',147*repetition+index,*config)
        print(f'allocation repetition {repetition+1} complete',flush=True)
if __name__=='__main__':main()
