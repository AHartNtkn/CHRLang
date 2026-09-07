"""Registered fresh-process matrix. Run timing and memory batches separately."""
import argparse
from dataclasses import asdict
import json
from pathlib import Path
import random
import subprocess
import sys
import time
from probes import workloads


def cases(path):
    result = []
    for c in workloads():
        c = dict(c)
        c['rules'] = [asdict(r) for r in c['rules']]
        result.append(c)
    for line in Path(path).read_text().splitlines():
        c = json.loads(line)
        if c['id'].startswith('app-') and c['exhausted']:
            c['prefix'] = False
            result.append(c)
    assert len(result) == 25
    return result


def configs():
    yield 'fifo', False, 'reverse'
    for policy in ('round', 'async'):
        yield policy, False, 'reverse'
        for mode in ('reverse', 'forward', 'identity'):
            yield policy, True, mode


def order_jobs(jobs, traced):
    jobs = list(jobs)
    random.Random(9092 if traced else 9091).shuffle(jobs)
    if not traced:
        jobs.sort(key=lambda job: job[-1] != 0)
    return jobs


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('fixtures')
    parser.add_argument('batch', choices=('time', 'memory'))
    args = parser.parse_args()
    traced = args.batch == 'memory'
    jobs = [(c, policy, grouping, mode, repeat) for c in cases(args.fixtures)
            for policy, grouping, mode in configs()
            for repeat in (range(2) if traced else range(6))]
    jobs = order_jobs(jobs, traced)
    for c, policy, grouping, mode, repeat in jobs:
        request = dict(case_json=json.dumps(c), policy=policy, grouping=grouping, mode=mode, traced=traced)
        start = time.perf_counter_ns()
        try:
            child = subprocess.run([sys.executable, str(Path(__file__).with_name('measure.py'))],
                                   input=json.dumps(request), text=True, capture_output=True, timeout=30)
            elapsed = time.perf_counter_ns()-start
            if child.returncode:
                row = dict(status='error', stderr=child.stderr, returncode=child.returncode)
            else:
                row = json.loads(child.stdout)
        except subprocess.TimeoutExpired:
            row = dict(status='timeout')
            elapsed = time.perf_counter_ns()-start
        row.update(id=c['id'], policy=policy, grouping=grouping, mode=mode, traced=traced,
                   repeat=repeat, warmup=not traced and repeat == 0, process_ns=elapsed)
        print(json.dumps(row, sort_keys=True), flush=True)
        if row['status'] != 'pass':
            raise SystemExit('affected batch stopped for investigation')
