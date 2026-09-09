#!/usr/bin/env python3
"""Validate integrated subscriptions and reject consequential source-effect faults."""
from pathlib import Path
import subprocess
import json
import hashlib
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s01-low-yield-gate'
TARGET=ROOT/'research/chr-compiled/experiments/subscription_runtime.rs'
original=TARGET.read_text()
def run(name,command,fault=False):
    p=subprocess.run(command,cwd=ROOT,text=True,capture_output=True,timeout=120)
    (OUT/(name+'.json')).write_text(json.dumps(dict(command=command,exit_code=p.returncode,stdout=p.stdout,stderr=p.stderr),indent=2)+'\n')
    if fault: assert p.returncode!=0 and 'panicked at' in p.stdout,(name,p.stdout,p.stderr)
    else: assert p.returncode==0,(name,p.stdout,p.stderr)
    print(name,p.returncode,flush=True)
base=['cargo','test','-p','chr-compiled','--release','--test','subscription_source_gate','--test','subscription_join_gate']
OUT.mkdir(exist_ok=True)
run('release-metrics',base)
run('release-counter-free',base+['--no-default-features'])
run('clippy',['cargo','clippy','-p','chr-compiled','--no-default-features','--test','subscription_source_gate','--test','subscription_join_gate','--','-D','warnings'])
variants={
    'fault-consumption':('self.remove_row(2, r);','let _ = r;'),
    'fault-demand-binding':('(before != after).then_some((id, after))','(false && before != after).then_some((id, after))'),
    'fault-row-binding':('(before != after).then_some((id, before, after))','(false && before != after).then_some((id, before, after))'),
    'fault-consuming-order':('candidates.sort_unstable_by_key(|[l, m, r, d]| [*l, *m, *d, *r]);','candidates.sort_unstable_by_key(|[l, m, r, d]| [*d, *l, *m, *r]);'),
}
try:
    for name,(old,new) in variants.items():
        assert original.count(old)==1,(name,original.count(old))
        TARGET.write_text(original.replace(old,new))
        (OUT/(name+'.patch.txt')).write_text('Replace exactly:\n'+old+'\nWith:\n'+new+'\n')
        run(name,['cargo','test','-p','chr-compiled','--release','--test','subscription_source_gate'],True)
finally: TARGET.write_text(original)
run('release-restored',base)
paths=[ROOT/'research/chr-compiled/experiments/subscription_low_yield.rs',TARGET,ROOT/'research/chr-compiled/experiments/subscription_source.rs',ROOT/'research/chr-compiled/experiments/subscription_join.rs',ROOT/'research/chr-compiled/tests/subscription_source_gate.rs',ROOT/'research/chr-compiled/tests/subscription_join_gate.rs']
(OUT/'source-hashes.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths},indent=2)+'\n')
