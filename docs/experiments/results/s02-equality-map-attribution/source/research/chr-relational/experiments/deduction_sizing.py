#!/usr/bin/env python3
"""Registered exploratory cost sizing, separate ordinary and meter binaries."""
import hashlib
import itertools
import json
import random
import shutil
import subprocess
import sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'research/chr-direct-conditional/experiments'))
import derivation_sizing as runner
OUT = ROOT / 'docs/experiments/results/s02-deduction-sizing'
BIN = Path('/tmp/chr-deduction-lifecycle-e51a0be6')
MODES = ['contextual', 'shared', 'relational', 'scan', 'indexed', 'sealed', 'lowered']
FAMILIES = ['single', 'shared', 'distinct', 'changed']


def main():
    OUT.mkdir(exist_ok=False)
    runner.OUT, runner.BIN = OUT, BIN
    sources = ['research/chr-relational/examples/s02_deduction.rs', 'research/chr-relational/examples/support/deduction_source.rs',
               'research/chr-relational/experiments/deduction_sizing.py', 'research/chr-relational/src/contextual.rs',
               'research/chr-relational/src/contextual_execute.rs', 'research/chr-relational/src/execute.rs', 'research/chr-relational/src/store.rs',
               'research/chr-relational/Cargo.toml', 'research/chr-direct-conditional/experiments/derivation_sizing.py',
               'research/chr-direct-conditional/tests/runtime_support/mod.rs', 'research/chr-compiled/experiments/meter.rs',
               'docs/experiments/registrations/S02-deduction-lifecycle-sizing.md', 'Cargo.lock']
    freeze = {'commit': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
              'rustc': subprocess.check_output(['rustc', '-Vv'], text=True), 'cpu': runner.CPU, 'sources': {}, 'binaries': {}}
    for p in sources:
        data = (ROOT / p).read_bytes()
        freeze['sources'][p] = hashlib.sha256(data).hexdigest()
        target = OUT / 'source' / p
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(data)
    for name in ['ordinary', 'meter']:
        p = BIN / name
        freeze['binaries'][str(p)] = hashlib.sha256(p.read_bytes()).hexdigest()
    (OUT / 'freeze.json').write_text(json.dumps(freeze, indent=2) + '\n')
    check = subprocess.run([str(BIN / 'meter'), 'meter-check'], capture_output=True, text=True, timeout=60, preexec_fn=runner.limits)
    (OUT / 'meter-check.log').write_text(check.stdout + check.stderr)
    assert check.returncode == 0
    configs = [(m, f, n, q, r, o) for m, f, n, q, r, o in itertools.product(MODES, FAMILIES, [0, 8, 32], [1, 4], [False, True], [False, True])]
    assert len(configs) == 672
    random.Random(7211).shuffle(configs)
    for i, config in enumerate(configs):
        runner.run('ordinary', i, config)
        if (i + 1) % 112 == 0:
            print(f'ordinary {i + 1}/672', flush=True)
    configs = [(m, f, n, q, r, o) for m, f, r, o in itertools.product(MODES, FAMILIES, [False, True], [False, True]) for n, q in [(0, 1), (32, 4)]]
    previous = []
    for rep in range(2):
        for i, config in enumerate(configs):
            record = runner.allocation_records(runner.run('allocation', rep * len(configs) + i, config))
            if rep == 0:
                previous.append(record)
            else:
                assert record == previous[i], config
        print(f'allocation replay {rep + 1}/2', flush=True)
    for i, (mode, cancel) in enumerate(itertools.product(MODES, [0, 1])):
        for kind in ['cancel', 'cancel-meter']:
            runner.run(kind, i, (mode, 'shared', 8, 2, True, False), cancel)
    print('Completed 672 exploratory timings, 448 allocation runs, 28 cancellation runs.', flush=True)


if __name__ == '__main__':
    main()
