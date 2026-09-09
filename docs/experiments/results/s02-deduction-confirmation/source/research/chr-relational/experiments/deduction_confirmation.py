#!/usr/bin/env python3
"""Frozen cross-engine confirmation following equality-map attribution."""
import hashlib
import itertools
import json
import random
import shutil
import subprocess
from pathlib import Path
import deduction_sizing as sizing
runner = sizing.runner

ROOT = runner.ROOT
OUT = ROOT / 'docs/experiments/results/s02-deduction-confirmation'
BIN = Path('/tmp/chr-equality-map-d648f15a')
MODES = ['contextual', 'shared', 'persistent', 'persistent-shared', 'relational', 'scan', 'indexed', 'sealed', 'lowered']
CONFIGS = list(itertools.product(sizing.FAMILIES, [0, 32], [1, 4], [False, True], [False, True]))


def main():
    OUT.mkdir(exist_ok=False)
    runner.OUT, runner.BIN = OUT, BIN
    old = ROOT / 'docs/experiments/results/s02-equality-map-attribution/freeze.json'
    old_freeze = json.loads(old.read_text())
    paths = list(old_freeze['sources']) + ['research/chr-relational/experiments/deduction_confirmation.py',
            'research/chr-relational/experiments/deduction_confirmation_analysis.py',
            'docs/experiments/registrations/S02-deduction-confirmation.md']
    # All Rust source inputs recorded for the repaired binary must still agree.
    for p, digest in old_freeze['sources'].items():
        if p.endswith('.rs') or p.endswith('.toml') or p == 'Cargo.lock':
            assert hashlib.sha256((ROOT / p).read_bytes()).hexdigest() == digest, p
    freeze = {'commit': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
              'rustc': subprocess.check_output(['rustc', '-Vv'], text=True), 'cpu': runner.CPU,
              'sources': {}, 'binaries': {}, 'configs': CONFIGS, 'modes': MODES,
              'preceding_freeze_sha256': hashlib.sha256(old.read_bytes()).hexdigest()}
    for p in paths:
        path = ROOT / p
        freeze['sources'][p] = hashlib.sha256(path.read_bytes()).hexdigest()
        target = OUT / 'source' / p
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(path, target)
    for name in ['ordinary', 'meter']:
        path = BIN / name
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        assert old_freeze['binaries'][str(path)] == digest
        freeze['binaries'][str(path)] = digest
    (OUT / 'freeze.json').write_text(json.dumps(freeze, indent=2) + '\n')
    check = subprocess.run([str(BIN / 'meter'), 'meter-check'], capture_output=True, text=True, timeout=60, preexec_fn=runner.limits)
    (OUT / 'meter-check.log').write_text(check.stdout + check.stderr)
    assert check.returncode == 0
    rng = random.Random(7231)
    for repetition in range(7):
        order = list(range(len(CONFIGS)))
        rng.shuffle(order)
        for i in order:
            modes = list(enumerate(MODES))
            rng.shuffle(modes)
            for j, mode in modes:
                runner.run('ordinary', (repetition * len(CONFIGS) + i) * len(MODES) + j, (mode, *CONFIGS[i]))
        print(f'ordinary repetition {repetition + 1}/7', flush=True)
    previous = []
    for repetition in range(2):
        for i, config in enumerate(CONFIGS):
            for j, mode in enumerate(MODES):
                index = i * len(MODES) + j
                value = runner.allocation_records(runner.run('allocation', repetition * len(CONFIGS) * len(MODES) + index, (mode, *config)))
                if repetition == 0:
                    previous.append(value)
                else:
                    assert value == previous[index], (mode, config)
        print(f'allocation repetition {repetition + 1}/2', flush=True)
    for i, (mode, cancel) in enumerate(itertools.product(MODES, [0, 1])):
        for kind in ['cancel', 'cancel-meter']:
            runner.run(kind, i, (mode, 'shared', 8, 2, True, False), cancel)
    print('Completed 4032 timings, 1152 exact allocation runs and 36 cancellation runs.', flush=True)


if __name__ == '__main__':
    main()
