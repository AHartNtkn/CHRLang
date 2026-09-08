#!/usr/bin/env python3
"""Execute exactly the prospective R02 integrated cost pilot and retain all outcomes."""
import hashlib
import json
import os
from pathlib import Path
import random
import resource
import subprocess
import time

ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/R02-integrated-pilot'
BINS=Path('/tmp/chr-r02-bins')

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def main():
    freeze=json.loads((OUT/'freeze.json').read_text())
    for name,sha in freeze['sha256'].items():
        assert digest(ROOT/name)==sha, f'changed source {name}'
    for mode,sha in freeze['binaries'].items():
        assert digest(BINS/mode)==sha, f'changed binary {mode}'
    workloads=[(f,n,d) for f in ['independent','fanout','repair'] for n in [8,64] for d in [1,8]]+ [('build',n,1) for n in [8,64]]
    jobs=[(mode,rep,f,n,d,v) for mode,reps in [('time',5),('work',1),('memory',1)] for rep in range(reps) for f,n,d in workloads
          for v in ['integrated','generic-indexed','generated-indexed','generated-scan','generated-global-scan']]
    random.Random(20260910).shuffle(jobs)
    cpu=min(os.sched_getaffinity(0))
    (OUT/'order.json').write_text(json.dumps({'seed':20260910,'cpu':cpu,'jobs':jobs},indent=2)+'\n')
    def limits():
        os.sched_setaffinity(0,{cpu})
        resource.setrlimit(resource.RLIMIT_AS,(2*1024**3,2*1024**3))
        resource.setrlimit(resource.RLIMIT_CPU,(20,20))
        resource.setrlimit(resource.RLIMIT_CORE,(0,0))
    started=time.monotonic()
    with (OUT/'raw.jsonl').open('x') as out:
        for i,(mode,rep,f,n,d,v) in enumerate(jobs):
            if time.monotonic()-started>900:
                raise RuntimeError('whole-batch 15-minute bound reached')
            cmd=[str(BINS/mode),mode,f,str(n),str(d),'4',v]
            record={'index':i,'mode':mode,'rep':rep,'command':cmd}
            before=time.monotonic_ns()
            try:
                p=subprocess.run(cmd,capture_output=True,text=True,timeout=30,preexec_fn=limits)
                record.update(returncode=p.returncode,stdout=p.stdout,stderr=p.stderr,process_ns=time.monotonic_ns()-before)
                if p.returncode!=0:
                    raise RuntimeError(f'child exited {p.returncode}')
                record['report']=json.loads(record.pop('stdout'))
                r=record['report']
                assert all(r[k]==(mode=='work') for k in ['metrics','compiled_metrics','kernel_metrics'])
                assert r['allocator_meter']==(mode=='memory')
                if mode=='memory' and r['completed']==4:
                    live=[s['memory'][4]['live_end'] for s in r['samples']]
                    assert len(set(live))==1,f'query retained bytes change: {live}'
            except subprocess.TimeoutExpired as e:
                record.update(timeout=True,stdout=(e.stdout or b'').decode(),stderr=(e.stderr or b'').decode())
            except Exception as e:
                record['validation_error']=str(e)
                out.write(json.dumps(record)+'\n');out.flush()
                raise
            out.write(json.dumps(record)+'\n');out.flush()
            if (i+1)%70==0:
                print(f'{i+1}/{len(jobs)} processes recorded',flush=True)

if __name__=='__main__':
    main()
