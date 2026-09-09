#!/usr/bin/env python3
"""Registered paired attribution of immutable obligation record reads."""
import hashlib
import json
import random
import demand_sizing as pilot

ORIGINAL=pilot.BIN
REPAIRED=pilot.Path('/tmp/chr-demand-borrow-26e59f5')

def main():
    original=json.loads((pilot.OUT/'freeze.json').read_text())
    for name,digest in original['binaries'].items():
        assert hashlib.sha256(pilot.Path(name).read_bytes()).hexdigest()==digest
    assert pilot.CPU==original['cpu']
    pilot.OUT=pilot.ROOT/'docs/experiments/results/s03-demand-borrow'
    pilot.OUT.mkdir(parents=True,exist_ok=True)
    freeze=dict(original_control=original,cpu=pilot.CPU,
        repaired_source=hashlib.sha256((pilot.ROOT/'research/chr-direct-choice/src/demand.rs').read_bytes()).hexdigest(),
        repaired_binaries={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in [REPAIRED/'ordinary',REPAIRED/'meter']})
    (pilot.OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    configs=[(f,r) for f in pilot.FAMILIES for r in [False,True]]
    for block in range(8):
        cases=[(variant,f,r) for variant in ['before','after'] for f,r in configs]
        random.Random(7108+block).shuffle(cases)
        for i,(variant,f,r) in enumerate(cases):
            pilot.BIN=ORIGINAL if variant=='before' else REPAIRED
            pilot.run(f'{variant}-block{block}',i,'demand',f,32,8,r)
        print(f'paired block {block} complete',flush=True)
    pilot.BIN=REPAIRED
    for repeat in range(2):
        for i,(f,r) in enumerate(configs):pilot.run('allocation',repeat*6+i,'demand',f,32,8,r)
    print('12 allocation processes complete',flush=True)

if __name__=='__main__':main()
