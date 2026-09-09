"""Prospective ordinary-link long-reuse contrast."""
import json
import os
from pathlib import Path
import random
import subprocess
import time
import artifact_cost_pilot as pilot

ROOT=pilot.ROOT
OUT=ROOT/'docs/experiments/results/s01-native-amortization-cached-oracle'
TARGET=ROOT/'target/s01-native-amortization-cached-oracle'
CASES=[('chain',32,q) for q in [1024,8192,16384]]+[('payload',64,q) for q in [1024,8192]]


def main():
    assert not OUT.exists(),'Preserve previous receipts'
    OUT.mkdir(parents=True);TARGET.mkdir(parents=True,exist_ok=True);pilot.OUT=OUT
    artifacts={};owned=[];shared={}
    for build,feature in [('ordinary','experiment'),('metered','alloc-meter')]:
        env=os.environ.copy();env['CARGO_TARGET_DIR']=str(TARGET/build)
        pilot.run('build-'+build,['cargo','build','--offline','--release','-p','chr-compiled','--no-default-features','--features',feature,'--lib','--bin','chr-access-emit'],4<<30,env)
        release=TARGET/build/'release';rlib=release/'libchr_compiled.rlib'
        shared.update({str(p.relative_to(ROOT)):pilot.digest(p) for p in (release/'deps').glob('*.rlib')});shared[str(rlib.relative_to(ROOT))]=pilot.digest(rlib)
        for family in ['generic','chain','payload']:
            for rep in range(5 if build=='ordinary' else 1):
                name=f'{build}-{family}-{rep}';source=TARGET/(name+'.rs');binary=TARGET/name
                pilot.run('emit-'+name,[release/'chr-access-emit',family,source],1<<30)
                pilot.run('compile-'+name,['rustc','--edition','2024','-C','opt-level=3','-C','codegen-units=1','-C','lto=off',source,'--extern','chr_compiled='+str(rlib),'-L','dependency='+str(release/'deps'),'-o',binary],4<<30)
                artifacts[name]=dict(source_sha256=pilot.digest(source),binary_sha256=pilot.digest(binary),binary_bytes=binary.stat().st_size)
                owned.extend([source,binary])
            print('compiled',build,family,flush=True)
    cells=[(*c,m) for c in CASES for m in ['planned','native']]
    checked=0
    for block in range(8):
        order=cells.copy();random.Random(7200+block).shuffle(order)
        for f,n,q,m in order:
            binary=TARGET/f'ordinary-{f if m=="native" else "generic"}-0'
            pilot.validate(pilot.run(f'time-{block}-{f}-{q}-{m}',[binary,m,f,n,q],1<<30,pin=True),f,m,n,q)
            checked+=q
        print('timing block',block,'complete',flush=True)
    for f,n,q,m in cells:
        readings=[]
        for rep in [0,1]:
            binary=TARGET/f'metered-{f if m=="native" else "generic"}-0'
            rows=pilot.validate(pilot.run(f'alloc-{rep}-{f}-{q}-{m}',[binary,m,f,n,q],1<<30,pin=True),f,m,n,q,True)
            readings.append([r['memory'] for r in rows]);checked+=q
        assert readings[0]==readings[1],(f,q,m,'allocation replay')
        print('diagnostics',f,q,m,flush=True)
    assert checked==696320
    assert all(pilot.digest(ROOT/p)==h for p,h in shared.items())
    disposals=[]
    for path in owned:
        start=time.perf_counter_ns();path.unlink();ns=time.perf_counter_ns()-start
        assert not path.exists();disposals.append(dict(path=str(path.relative_to(ROOT)),unlink_ns=ns))
    (OUT/'disposal.json').write_text(json.dumps(disposals,indent=2)+'\n')
    (OUT/'summary.json').write_text(json.dumps(dict(parent_commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),rustc=subprocess.check_output(['rustc','-Vv'],text=True),cpu=pilot.CPU,artifacts=artifacts,shared_rlibs=shared,complete_queries=checked,timing_processes=80,diagnostic_processes=20),indent=2)+'\n')
    print('amortization run complete',flush=True)


if __name__=='__main__':main()
