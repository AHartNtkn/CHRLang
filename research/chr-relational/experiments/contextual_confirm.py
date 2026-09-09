#!/usr/bin/env python3
"""Registered five-path confirmation, retaining every raw process."""
from pathlib import Path
import sys,json,hashlib,random,subprocess,platform
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'research/chr-direct-conditional/experiments'))
import demand_sizing as pilot
pilot.BIN=Path('/tmp/chr-context-confirm-8f91a09b')
pilot.OUT=ROOT/'docs/experiments/results/s02-contextual-confirmation'
MODES=['contextual','relational','scan','indexed','lowered']
FAMILIES=['shared0','shared3','local0','local3']
def main():
    sources=['research/chr-relational/examples/s02_contextual.rs','research/chr-relational/examples/support/contextual_source.rs','research/chr-relational/src/contextual.rs','research/chr-relational/src/contextual_execute.rs','research/chr-relational/src/execute.rs','research/chr-relational/src/store.rs','research/chr-relational/src/lib.rs','research/chr-direct-conditional/experiments/demand_sizing.py','research/chr-relational/experiments/contextual_confirm.py','docs/experiments/registrations/S02-contextual-lifecycle-confirmation.md','Cargo.lock']
    freeze=dict(base_commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),cpu=pilot.CPU,host=platform.platform(),rustc=subprocess.check_output(['rustc','--version'],text=True).strip(),sha256={p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in sources},binaries={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in [pilot.BIN/'ordinary',pilot.BIN/'meter']})
    (pilot.OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    check=subprocess.run([str(pilot.BIN/'meter'),'meter-check'],capture_output=True,text=True,timeout=60,preexec_fn=pilot.limits)
    (pilot.OUT/'meter-check.log').write_text(check.stdout+check.stderr);assert check.returncode==0
    for i,m in enumerate(MODES):
        pilot.run('cancel',i,m,'local3',8,2,True,1)
        pilot.run('cancel-meter',i,m,'local3',8,2,True,1)
    configs=[(m,f,n,q,r) for m in MODES for f in FAMILIES for n in [0,8,32] for q in [1,4] for r in [False,True]]
    assert len(configs)==240
    for block in range(8):
        cells=configs.copy();random.Random(7202+block).shuffle(cells)
        for i,c in enumerate(cells):pilot.run(f'block{block}',i,*c)
        print(f'block {block} complete: 240 processes',flush=True)
    for repeat in range(2):
        for i,c in enumerate(configs):pilot.run('allocation',repeat*240+i,*c)
        print(f'allocation repeat {repeat+1} complete',flush=True)
if __name__=='__main__':main()
