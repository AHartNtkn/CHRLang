#!/usr/bin/env python3
"""Run only the prospectively registered R01 anchor pilot; retain every outcome."""
import hashlib
import itertools
import json
import os
from pathlib import Path
import random
import resource
import subprocess
import time

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/R01-anchor-pilot'
BINS = {'before': Path('/tmp/chr-r01-bins'), 'after': Path('/tmp/chr-r01-anchor-bins')}


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    manifest = json.loads((OUT / 'freeze.json').read_text())
    for name, expected in manifest['sha256'].items():
        assert digest(ROOT / name) == expected, f'changed frozen source: {name}'
    for variant, binaries in manifest['binaries'].items():
        for mode, expected in binaries.items():
            assert digest(BINS[variant] / mode) == expected, f'changed binary: {variant}/{mode}'
    cells = list(itertools.product(['build', 'chain', 'delayed', 'collision', 'repair'],
                                   [8, 64], ['generic', 'generated'],
                                   ['global', 'active'], ['scan', 'indexed']))
    jobs = [(variant, mode, rep, *cell) for variant in ['before', 'after']
            for mode, repeats in [('time', 5), ('work', 1), ('memory', 1)]
            for rep in range(repeats) for cell in cells]
    random.Random(20260909).shuffle(jobs)
    cpu = min(os.sched_getaffinity(0))
    (OUT / 'order.json').write_text(json.dumps({'seed': 20260909, 'cpu': cpu, 'jobs': jobs}, indent=2) + '\n')

    def limits():
        os.sched_setaffinity(0, {cpu})
        resource.setrlimit(resource.RLIMIT_AS, (2 * 1024**3, 2 * 1024**3))
        resource.setrlimit(resource.RLIMIT_CPU, (20, 20))
        resource.setrlimit(resource.RLIMIT_CORE, (0, 0))

    started = time.monotonic()
    with (OUT / 'raw.jsonl').open('x') as out:
        for index, (variant, mode, rep, family, size, execution, policy, access) in enumerate(jobs):
            if time.monotonic() - started > 900:
                raise RuntimeError('15-minute batch bound reached; unrun jobs remain registered')
            cmd = [str(BINS[variant] / mode), mode, family, str(size), '4', execution, policy, access]
            before = time.monotonic_ns()
            record = {'index': index, 'variant': variant, 'mode': mode, 'rep': rep, 'command': cmd}
            try:
                result = subprocess.run(cmd, capture_output=True, text=True, timeout=30, preexec_fn=limits)
                record.update(returncode=result.returncode, stderr=result.stderr, stdout=result.stdout,
                              process_ns=time.monotonic_ns() - before)
                if result.returncode != 0:
                    raise RuntimeError(f'child failed with exit {result.returncode}; inspect preserved stdout/stderr')
                if result.returncode == 0:
                    report = json.loads(record.pop('stdout'))
                    record['report'] = report
                    assert report['engine_metrics'] == (mode == 'work')
                    assert report['kernel_metrics'] == (mode == 'work')
                    assert report['allocator_meter'] == (mode == 'memory')
                    if mode == 'memory' and report['completed'] == 4:
                        live = [s['memory'][4]['live_end'] for s in report['samples']]
                        assert len(set(live)) == 1, f'query retention grows: {live}'
            except subprocess.TimeoutExpired as error:
                record.update(timeout=True, stdout=(error.stdout or b'').decode(),
                              stderr=(error.stderr or b'').decode())
            except Exception as error:
                record['validation_error'] = str(error)
                out.write(json.dumps(record) + '\n')
                out.flush()
                raise
            out.write(json.dumps(record) + '\n')
            out.flush()
            if index % 80 == 79:
                print(f'{index + 1}/{len(jobs)} processes recorded', flush=True)


if __name__ == '__main__':
    main()
