#!/usr/bin/env python3
"""Independent product/progress gate for the compiled regional service."""
from pathlib import Path
import hashlib,json,os,signal,subprocess
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s09-factored-lowering-gate'
TARGET=ROOT/'research/chr-factors/experiments/contracted_region.rs'
def run(name,cmd,fault=False):
    p=subprocess.Popen(cmd,cwd=ROOT,text=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
    cutoff=False
    try:stdout,stderr=p.communicate(timeout=60)
    except subprocess.TimeoutExpired:
        cutoff=True;os.killpg(p.pid,signal.SIGKILL);stdout,stderr=p.communicate()
    (OUT/(name+'.json')).write_text(json.dumps(dict(command=cmd,exit_code=p.returncode,cutoff=cutoff,stdout=stdout,stderr=stderr),indent=2)+'\n')
    assert not cutoff,name
    if fault:assert p.returncode!=0 and 'panicked at' in stdout,(name,stdout,stderr)
    else:assert p.returncode==0,(name,stdout,stderr)
    print(name,p.returncode,flush=True)
def main():
    OUT.mkdir(parents=True,exist_ok=True)
    base=['cargo','test','-p','chr-factors','--release','--no-default-features','--features','worker-lowering','--test','reusable_regions']
    run('counter-free',base)
    run('metrics',base+['--features','metrics'])
    run('default-package',['cargo','test','-p','chr-factors'])
    executable=['cargo','run','-p','chr-factors','--no-default-features','--features','worker-lowering','--release','--example','reusable_regions_gate']
    run('ordinary-executable',executable)
    run('clippy',['cargo','clippy','-p','chr-factors','--no-default-features','--features','worker-lowering','--test','reusable_regions','--example','reusable_regions_gate','--','-D','warnings'])
    original=TARGET.read_text()
    variants={
        'raw-multiplicity':('.checked_add(1)', '.checked_add(2)'),
        'premature-exhaustion':('exhausted: self.engine.pending_branches() == 0','exhausted: true'),
        'missing-observation':('if self.seen.insert(answer.clone()) {','if false && self.seen.insert(answer.clone()) {'),
    }
    try:
        for name,(old,new) in variants.items():
            assert original.count(old)==1,(name,original.count(old))
            TARGET.write_text(original.replace(old,new))
            (OUT/(name+'.patch.txt')).write_text('Replace exactly:\n'+old+'\nWith:\n'+new+'\n')
            run(name,base,True)
    finally:TARGET.write_text(original)
    run('restored',base)
    run('restored-executable',executable)
    paths=[TARGET,Path(__file__).resolve(),ROOT/'research/chr-factors/experiments/reusable_regions.rs',ROOT/'research/chr-factors/tests/reusable_regions.rs',ROOT/'research/chr-factors/examples/reusable_regions_gate.rs']
    (OUT/'hashes.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths},indent=2)+'\n')
if __name__=='__main__':main()
