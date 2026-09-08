#!/usr/bin/env python3
"""Prospectively registered wall/CPU pilot over identical inline/worker service."""
from pathlib import Path
import hashlib,itertools,json,os,random,resource,subprocess
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s09-worker-timing'
BINARY=ROOT/'target/release/examples/worker_cost'
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def limits():resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3))
def key(cell):return '-'.join(map(str,cell))
def main():
    OUT.mkdir(parents=True,exist_ok=True)
    source={}
    for directory in ['crates/chr-syntax','research/chr-factors','research/chr-persistent','research/chr-observe','research/chr-cases']:
        for p in (ROOT/directory).rglob('*'):
            if p.is_file() and (p.suffix in ['.rs','.py'] or p.name=='Cargo.toml'):
                source[str(p.relative_to(ROOT))]=sha(p)
    for name in ['Cargo.toml','Cargo.lock']:source[name]=sha(ROOT/name)
    freeze=dict(source=source,binary=sha(BINARY),rustc=subprocess.check_output(['rustc','-Vv'],text=True))
    path=OUT/'freeze.json'
    if path.exists():assert json.loads(path.read_text())==freeze,'freeze mismatch'
    else:path.write_text(json.dumps(freeze,indent=2)+'\n')
    hardware=dict(affinity=sorted(os.sched_getaffinity(0)),cpu_count=os.cpu_count())
    for path in ['/proc/self/cgroup','/sys/fs/cgroup/cpu.max']:
        try:hardware[path]=Path(path).read_text()
        except OSError:hardware[path]=None
    (OUT/'hardware.json').write_text(json.dumps(hardware,indent=2)+'\n')
    sources=[('balanced',64),('balanced',256),('skew',64),('skew',256),('tiny',0)]
    cells=[(family,depth,queries,quantum,workers) for (family,depth),queries,quantum,workers in
           itertools.product(sources,[1,16],[16,128],[0,1,2,4])]
    rng=random.Random(20260908)
    for block in range(-1,7):
        order=cells.copy();rng.shuffle(order)
        for cell in order:
            output=OUT/f'{block}-{key(cell)}.json'
            if output.exists():
                receipt=json.loads(output.read_text())
                assert receipt['exit_code']==0 and not receipt['cutoff']
                continue
            family,depth,queries,quantum,workers=cell
            command=[str(BINARY),str(workers),str(depth),str(queries),str(quantum),family]
            before=resource.getrusage(resource.RUSAGE_CHILDREN)
            try:r=subprocess.run(command,capture_output=True,text=True,timeout=60,preexec_fn=limits)
            except subprocess.TimeoutExpired as error:
                output.write_text(json.dumps(dict(command=command,exit_code=None,cutoff=True))+'\n')
                raise AssertionError('pilot cutoff; investigate') from error
            after=resource.getrusage(resource.RUSAGE_CHILDREN)
            receipt=dict(command=command,block=block,cell=cell,cutoff=False,exit_code=r.returncode,
                         stdout=r.stdout,stderr=r.stderr,
                         process_cpu_seconds=after.ru_utime-before.ru_utime+after.ru_stime-before.ru_stime)
            output.write_text(json.dumps(receipt,indent=2)+'\n')
            assert r.returncode==0,receipt
            rows=[json.loads(line) for line in r.stdout.splitlines()]
            header,qs=rows[0],rows[1:]
            assert len(qs)==queries and [q['query'] for q in qs]==list(range(queries))
            assert header['lifecycle_ns']==header['prepare_ns']+header['shutdown_ns']+header['runtime_drop_ns']+sum(
                q['setup_ns']+q['execute_observe_ns']+q['close_ns']+q['dispose_ns'] for q in qs)
            assert all(0<=q['first_observation_ns']<=q['execute_observe_ns'] for q in qs)
        print('block',block,'complete',flush=True)
if __name__=='__main__':main()
