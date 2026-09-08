#!/usr/bin/env python3
"""Matched runtime requested-heap matrix; instrumented times are not used."""
from pathlib import Path
import hashlib,itertools,json,resource,subprocess,statistics
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s09-worker-allocation'
BINARY=ROOT/'target/release/examples/worker_allocation'
def limits():resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3))
def main():
    OUT.mkdir(parents=True,exist_ok=True)
    summaries={}
    for (family,depth),queries,quantum,workers in itertools.product(
        [('balanced',64),('balanced',256),('skew',64),('skew',256),('tiny',0)], [1,16],[16,128],[0,1,2,4]):
        key=f'{family}-{depth}-{queries}-{quantum}-{workers}';samples=[]
        for repeat in range(3):
            command=[str(BINARY),str(workers),str(depth),str(queries),str(quantum),family]
            try:r=subprocess.run(command,capture_output=True,text=True,timeout=60,preexec_fn=limits)
            except subprocess.TimeoutExpired as e:
                (OUT/f'{key}-{repeat}.json').write_text(json.dumps(dict(command=command,cutoff=True))+'\n')
                raise AssertionError('allocation cutoff') from e
            (OUT/f'{key}-{repeat}.json').write_text(json.dumps(dict(command=command,exit_code=r.returncode,cutoff=False,stdout=r.stdout,stderr=r.stderr),indent=2)+'\n')
            assert r.returncode==0,(command,r.stderr)
            header,reading=[json.loads(line) for line in r.stdout.splitlines()]
            assert [header[k] for k in ['family','depth','queries','quantum','workers']]==[family,depth,queries,quantum,workers]
            assert reading['live_start']==reading['live_end']
            reading['incremental_peak']=reading['peak_live']-reading['live_start'];samples.append(reading)
        summaries[key]={field:[r[field] for r in samples] for field in ['allocation_calls','requested_bytes','deallocation_calls','incremental_peak']}
        print(key,'passed',flush=True)
    (OUT/'summary.json').write_text(json.dumps(summaries,indent=2)+'\n')
    paths=[ROOT/'research/chr-factors/examples/worker_allocation.rs',Path(__file__).resolve(),
           ROOT/'research/chr-factors/experiments/worker_cases.rs',ROOT/'research/chr-factors/experiments/reusable_regions.rs',
           ROOT/'research/chr-factors/experiments/reusable_workers.rs',ROOT/'research/chr-compiled/experiments/meter.rs',BINARY]
    (OUT/'hashes.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths},indent=2)+'\n')
    for workers in [1,2,4]:
        for field in ['requested_bytes','incremental_peak']:
            ratios=[]
            for key,values in summaries.items():
                if not key.endswith(f'-{workers}'):continue
                base=summaries[key.rsplit('-',1)[0]+'-0'][field]
                ratios.append(statistics.median(values[field])/statistics.median(base))
            print('RATIO',workers,field,min(ratios),max(ratios))
    print('variable traffic cells',sum(len(set(v['requested_bytes']))>1 for v in summaries.values()))
if __name__=='__main__':main()
