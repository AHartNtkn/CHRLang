#!/usr/bin/env python3
"""One observation per prospective sizing cell; not a timing comparison."""
from pathlib import Path
import hashlib,json,resource,subprocess
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s09-native-sizing'
BINARY=ROOT/'target/release/examples/worker_cost'
def limits():resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3))
def main():
    OUT.mkdir(parents=True,exist_ok=True)
    for depth in [16,64,256]:
        for workers in [0,1,2,4]:
            command=[str(BINARY),str(workers),str(depth),'4','16','balanced']
            before=resource.getrusage(resource.RUSAGE_CHILDREN)
            try:r=subprocess.run(command,capture_output=True,text=True,timeout=60,preexec_fn=limits)
            except subprocess.TimeoutExpired as e:
                (OUT/f'{depth}-{workers}.json').write_text(json.dumps(dict(command=command,cutoff=True))+'\n')
                raise AssertionError('sizing cutoff; investigate before confirmation') from e
            after=resource.getrusage(resource.RUSAGE_CHILDREN)
            receipt=dict(command=command,cutoff=False,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr,
                         process_cpu_seconds=(after.ru_utime-before.ru_utime)+(after.ru_stime-before.ru_stime))
            (OUT/f'{depth}-{workers}.json').write_text(json.dumps(receipt,indent=2)+'\n')
            assert r.returncode==0,receipt
            header=json.loads(r.stdout.splitlines()[0])
            print(depth,workers,round(header['lifecycle_ns']/1e6,3),'ms; CPU',round(receipt['process_cpu_seconds'],5),flush=True)
    paths=[ROOT/'research/chr-factors/examples/worker_cost.rs',ROOT/'research/chr-factors/experiments/worker_cases.rs',
           ROOT/'research/chr-factors/experiments/reusable_regions.rs',ROOT/'research/chr-factors/experiments/reusable_workers.rs',
           Path(__file__).resolve(),BINARY]
    (OUT/'hashes.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()
                                            for p in paths},indent=2)+'\n')
if __name__=='__main__':main()
