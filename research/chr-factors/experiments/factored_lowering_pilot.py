#!/usr/bin/env python3
"""Registered existing-lowering control, fresh paired wall/CPU and heap runs."""
from pathlib import Path
import hashlib,itertools,json,os,random,resource,subprocess
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s09-factored-lowering'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3))
def main():
    OUT.mkdir(parents=True,exist_ok=True)
    source={}
    for d in ['crates/chr-syntax','research/chr-factors','research/chr-compiled','research/chr-persistent','research/chr-observe','research/chr-cases']:
        for p in (ROOT/d).rglob('*'):
            if p.is_file() and (p.suffix in ['.rs','.py'] or p.name=='Cargo.toml'):source[str(p.relative_to(ROOT))]=sha(p)
    binaries={name:sha(ROOT/'target/release/examples'/name) for name in ['worker_cost','worker_allocation','worker_lowered_cost','worker_lowered_allocation']}
    freeze=dict(source=source,binaries=binaries,rustc=subprocess.check_output(['rustc','-Vv'],text=True),affinity=sorted(os.sched_getaffinity(0)))
    f=OUT/'freeze.json'
    if f.exists():assert json.loads(f.read_text())==freeze
    else:f.write_text(json.dumps(freeze,indent=2)+'\n')
    cells=[(family,depth,queries,mode) for (family,depth),queries,mode in itertools.product(
        [('balanced',64),('balanced',256),('skew',64),('skew',256),('tiny',0)],[1,16],['inline','workers4','factored','unpartitioned'])]
    rng=random.Random(20260910)
    for allocation,blocks in [(False,range(-1,7)),(True,range(3))]:
        for block in blocks:
            order=cells.copy();rng.shuffle(order)
            for cell in order:
                family,depth,queries,mode=cell
                name=('heap' if allocation else 'time')+'-'+str(block)+'-'+'-'.join(map(str,cell))
                path=OUT/(name+'.json')
                if path.exists():
                    r=json.loads(path.read_text());assert r['exit_code']==0 and not r['cutoff'];continue
                if mode in ['inline','workers4','factored']:
                    binary='worker_allocation' if allocation else 'worker_cost'
                    args=[{'inline':'0','workers4':'4','factored':'contracted'}[mode],str(depth),str(queries),'128',family]
                else:
                    binary='worker_lowered_allocation' if allocation else 'worker_lowered_cost'
                    args=['contracted',str(depth),str(queries),family]
                command=[str(ROOT/'target/release/examples'/binary),*args]
                before=resource.getrusage(resource.RUSAGE_CHILDREN)
                try:r=subprocess.run(command,capture_output=True,text=True,timeout=60,preexec_fn=limits)
                except subprocess.TimeoutExpired as error:
                    path.write_text(json.dumps(dict(command=command,cutoff=True,exit_code=None))+'\n');raise AssertionError('control cutoff') from error
                after=resource.getrusage(resource.RUSAGE_CHILDREN)
                record=dict(command=command,allocation=allocation,block=block,cell=cell,exit_code=r.returncode,cutoff=False,stdout=r.stdout,stderr=r.stderr,
                            process_cpu_seconds=after.ru_utime-before.ru_utime+after.ru_stime-before.ru_stime)
                path.write_text(json.dumps(record,indent=2)+'\n')
                assert r.returncode==0,record
            print('heap' if allocation else 'time',block,'complete',flush=True)
if __name__=='__main__':main()
