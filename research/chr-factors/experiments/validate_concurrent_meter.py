#!/usr/bin/env python3
"""Prospective concurrent-region allocation diagnostic; no timing ranking."""
from pathlib import Path
import hashlib
import json
import resource
import subprocess
ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT/'docs/experiments/results/s09-concurrent-meter'
BINARY = ROOT/'target/release/examples/concurrent_meter_gate'

def limits():
    resource.setrlimit(resource.RLIMIT_AS, (1024**3, 1024**3))

def main():
    OUT.mkdir(parents=True, exist_ok=True)
    summaries = {}
    for workers in [0,1,2,4]:
        for quantum in [1,16]:
            for window in ['phased','whole']:
                key = f'{workers}-{quantum}-{window}'
                grouped = {}
                for repeat in range(5):
                    command = [str(BINARY),str(workers),str(quantum),window]
                    try:
                        r = subprocess.run(command,capture_output=True,text=True,timeout=60,preexec_fn=limits)
                    except subprocess.TimeoutExpired as e:
                        (OUT/f'{key}-{repeat}.json').write_text(json.dumps(dict(command=command,cutoff=True))+'\n')
                        raise AssertionError('diagnostic timeout') from e
                    receipt = dict(command=command,exit_code=r.returncode,cutoff=False,stdout=r.stdout,stderr=r.stderr)
                    (OUT/f'{key}-{repeat}.json').write_text(json.dumps(receipt,indent=2)+'\n')
                    assert r.returncode == 0, receipt
                    rows = [json.loads(line) for line in r.stdout.splitlines()]
                    assert sum('total' in row for row in rows)==4
                    for row in rows:
                        if 'total' in row:
                            reading=row['total']; group='total'
                            assert reading['live_start']==reading['live_end'],row
                        elif 'reading' in row:
                            reading=row['reading']; group=f"{row['query']}-{row['phase']}"
                        else: continue
                        target=grouped.setdefault(group,[])
                        target.append(reading)
                summaries[key]={group:{field:sorted({r[field] for r in rs}) for field in
                    ['allocation_calls','requested_bytes','deallocation_calls']} for group,rs in grouped.items()}
                print(key,'passed',flush=True)
    (OUT/'traffic-values.json').write_text(json.dumps(summaries,indent=2)+'\n')
    paths=[ROOT/'research/chr-factors/examples/concurrent_meter_gate.rs',
           ROOT/'research/chr-factors/experiments/worker_cases.rs',Path(__file__).resolve(),
           ROOT/'research/chr-compiled/experiments/meter.rs',
           ROOT/'research/chr-factors/experiments/reusable_regions.rs',
           ROOT/'research/chr-factors/experiments/reusable_workers.rs',BINARY]
    (OUT/'hashes.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()
                                            for p in paths},indent=2)+'\n')

if __name__=='__main__': main()
