"""Registered clock-boundary sizing; no architecture performance comparisons."""
import hashlib
import json
import os
import random
import resource
import subprocess
import sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s10-clock-calibration'
BUILD=ROOT/'target/s10-clock-calibration'

def main():
    assert {0,1} <= os.sched_getaffinity(0)
    build_commands=[['rustc','-O','--edition=2024',str(Path(__file__).with_name('probe.rs')),'-o',str(BUILD/'rust')],['clang','-O2','-Wall','-Wextra','-Werror',str(Path(__file__).with_name('probe.c')),'-o',str(BUILD/'native')]]
    builds=[]
    for command in build_commands:
        p=subprocess.run(command,capture_output=True,text=True,timeout=60)
        builds.append(dict(command=command,code=p.returncode,stdout=p.stdout,stderr=p.stderr));assert p.returncode==0,p.stderr
    versions={name:subprocess.check_output(command,text=True) for name,command in [('rustc',['rustc','--version']),('clang',['clang','--version']),('python',[sys.executable,'--version'])]}
    (OUT/'build.json').write_text(json.dumps(dict(builds=builds,versions=versions,allowed_cpus=sorted(os.sched_getaffinity(0))),indent=2)+'\n')
    jobs=[(lang,cpu,block) for lang in ['rust','native','python'] for cpu in [0,1] for block in range(7)]
    random.Random(20260910).shuffle(jobs)
    with (OUT/'runs.jsonl').open('w') as log:
        for lang,cpu,block in jobs:
            def bounds():
                os.sched_setaffinity(0,{cpu})
                resource.setrlimit(resource.RLIMIT_AS,(256<<20,256<<20))
                resource.setrlimit(resource.RLIMIT_CPU,(5,5))
            command=([sys.executable,str(Path(__file__).with_name('probe.py'))] if lang=='python' else [str(BUILD/lang)])+[str(block%2)]
            p=subprocess.run(command,capture_output=True,text=True,timeout=10,preexec_fn=bounds)
            assert p.returncode==0 and not p.stderr,(lang,p.returncode,p.stderr)
            data=json.loads(p.stdout)
            assert data['cpu']==cpu
            assert len(data['samples'])==10000 and all(type(x)is int and x>=0 for x in data['samples'])
            assert data['baseline_verified'] and data['baseline_ns']>=0 and data['pairs_ns']>=0
            log.write(json.dumps(dict(language=lang,cpu=cpu,block=block,command=command,result=data))+'\n');log.flush()
    paths=list(Path(__file__).parent.glob('*.*'))+[BUILD/'rust',BUILD/'native',OUT/'runs.jsonl',OUT/'build.json',ROOT/'docs/experiments/registrations/S10-clock-calibration.md']
    (OUT/'validation.json').write_text(json.dumps(dict(processes=42,empty_intervals=420000,hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths if p.is_file()}),indent=2)+'\n')
    print('42 clock processes and 420000 empty intervals recorded; baseline checks pass.')
if __name__=='__main__':main()
