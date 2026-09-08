#!/usr/bin/env python3
"""Bounded lifecycle correctness and deliberate-fault checks, not timing evidence."""
from pathlib import Path
import subprocess,os,signal,json,hashlib
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s09-worker-lifecycle'
TARGET=ROOT/'research/chr-factors/experiments/reusable_workers.rs'
original=TARGET.read_text()
def run(name,cmd,fault=False):
    p=subprocess.Popen(cmd,cwd=ROOT,text=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
    cutoff=False
    try:stdout,stderr=p.communicate(timeout=60)
    except subprocess.TimeoutExpired:
        cutoff=True;os.killpg(p.pid,signal.SIGKILL);stdout,stderr=p.communicate()
    r=dict(command=cmd,exit_code=p.returncode,cutoff=cutoff,stdout=stdout,stderr=stderr)
    (OUT/(name+'.json')).write_text(json.dumps(r,indent=2)+'\n')
    assert not cutoff,(name,'lifecycle timeout')
    if fault:assert p.returncode!=0 and 'panicked at' in stdout,(name,stdout,stderr)
    else:assert p.returncode==0,(name,stdout,stderr)
    print(name,p.returncode,flush=True)
base=['cargo','test','-p','chr-factors','--release','--test','reusable_workers']
run('release',base)
run('counter-free',base+['--no-default-features'])
run('ordinary-executable',['cargo','run','-p','chr-factors','--no-default-features','--release','--example','reusable_workers_gate'])
run('clippy',['cargo','clippy','-p','chr-factors','--no-default-features','--test','reusable_workers','--example','reusable_workers_gate','--','-D','warnings'])
for i in range(8):run(f'repeat-{i}',base+['--no-default-features'])
variants={
    'fault-query-retention':('searches.clear();','// injected retained searches'),
    'fault-reply-generation':('epoch != self.generation ||',''),
    'fault-pool-identity':('!Arc::ptr_eq(&id.owner, &self.owner)\n            || ','')
}
try:
    for name,(old,new) in variants.items():
        assert original.count(old)==1,(name,original.count(old))
        TARGET.write_text(original.replace(old,new))
        (OUT/(name+'.patch.txt')).write_text('Replace exactly:\n'+old+'\nWith:\n'+new+'\n')
        run(name,base,True)
finally:TARGET.write_text(original)
run('restored',base)
run('restored-executable',['cargo','run','-p','chr-factors','--no-default-features','--release','--example','reusable_workers_gate'])
paths=[TARGET,ROOT/'research/chr-factors/tests/reusable_workers.rs',ROOT/'research/chr-factors/examples/reusable_workers_gate.rs']
(OUT/'source-hashes.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths},indent=2)+'\n')
