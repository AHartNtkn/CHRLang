"""Registered E15 three-query sessions; separate timing and allocation batches."""
import argparse
import json
from pathlib import Path
import random
import subprocess
import sys
import time
from run_measurements import cases


def configurations(fixtures):
    extra = {'lag-64','duplicates-6','opaque-64','recursive-64','app-sk-identity'}
    for case in cases(fixtures):
        policies = [('async', False)]
        if case['id'] in extra:
            policies += [('fifo', False), ('round', False), ('round', True), ('async', True)]
        for service in ('direct','scan','count'):
            for policy, grouping in policies:
                yield case, service, policy, grouping


def ordered_jobs(configs, traced):
    jobs = [(c, s, p, g, r) for c,s,p,g in configs for r in range(2 if traced else 6)]
    random.Random(15102 if traced else 15101).shuffle(jobs)
    if not traced:
        jobs.sort(key=lambda j: j[-1] != 0)
    return jobs


def signature(query):
    return {k:v for k,v in query.items() if k != 'timings_ns'}


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('fixtures')
    parser.add_argument('batch', choices=('time','memory'))
    args = parser.parse_args()
    traced = args.batch == 'memory'
    configs = list(configurations(args.fixtures))
    assert len(configs) == 135
    seen = {}
    for case, service, policy, grouping, repeat in ordered_jobs(configs, traced):
        request = dict(case_json=json.dumps(case), service=service, policy=policy,
                       grouping=grouping, traced=traced, queries=3)
        start = time.perf_counter_ns()
        try:
            child = subprocess.run([sys.executable, str(Path(__file__).with_name('measure_net.py'))],
                                   input=json.dumps(request), text=True, capture_output=True, timeout=1800)
            row = json.loads(child.stdout) if child.returncode == 0 else dict(
                status='error', stderr=child.stderr, returncode=child.returncode)
        except subprocess.TimeoutExpired:
            row = dict(status='timeout')
        row.update(id=case['id'], service=service, policy=policy, grouping=grouping,
                   repeat=repeat, warmup=not traced and repeat == 0, traced=traced,
                   process_ns=time.perf_counter_ns()-start)
        if row['status'] == 'pass':
            key = (case['id'], service, policy, grouping)
            signatures = [signature(q) for q in row['queries']]
            baseline = seen.setdefault(key, signatures[0])
            if any(q != baseline for q in signatures) or len(set(row['outputs'])) != 1:
                row.update(status='error', detail='session/replica semantic or action mismatch')
        print(json.dumps(row, sort_keys=True), flush=True)
        if row['status'] != 'pass':
            raise SystemExit('affected batch stopped for investigation')
