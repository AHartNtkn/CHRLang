#!/usr/bin/env python3
"""Frozen cross-engine confirmation following shared-term attribution."""
import hashlib
import itertools
import json
import random
import shutil
import subprocess
from pathlib import Path
import derivation_sizing as runner

ROOT = runner.ROOT
OUT = ROOT / 'docs/experiments/results/s03-derivation-confirmation'
BIN = Path('/tmp/chr-shared-template-bounded-83deb0d3')
MODES = runner.MODES
CONFIGS = [(f, n, q, r, o) for f, q, r, o in itertools.product(runner.FAMILIES, [1, 8], [False, True], [False, True])
           for n in [0, 12 if f == 'grow' else 32]]


def main():
    OUT.mkdir(exist_ok=False)
    runner.OUT, runner.BIN = OUT, BIN
    old = ROOT / 'docs/experiments/results/s03-shared-template-bounded/freeze.json'
    old_freeze = json.loads(old.read_text())
    paths = list(old_freeze['sources']) + ['research/chr-direct-conditional/experiments/derivation_confirmation.py',
            'research/chr-direct-conditional/experiments/derivation_confirmation_analysis.py',
            'docs/experiments/registrations/S03-derivation-confirmation.md']
    # All Rust source inputs recorded for the repaired binary must still agree.
    for p, digest in old_freeze['sources'].items():
        if p.endswith('.rs') or p == 'Cargo.lock':
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
    rng = random.Random(7131)
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
            runner.run(kind, i, (mode, 'choice', 8, 2, True, False), cancel)
    print('Completed 3360 timings, 960 exact allocation runs and 24 cancellation runs.', flush=True)


if __name__ == '__main__':
    main()
