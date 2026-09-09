#!/usr/bin/env python3
"""Registered paired equality-map attribution and ordered-control calibration."""
import hashlib
import itertools
import json
import random
import subprocess
from pathlib import Path
import deduction_sizing as sizing
runner = sizing.runner
ROOT = sizing.ROOT
OUT = ROOT / 'docs/experiments/results/s02-equality-map-attribution'
BIN = Path('/tmp/chr-equality-map-d648f15a')
OLD = Path('/tmp/chr-deduction-lifecycle-e51a0be6')
MODES = ['contextual', 'shared', 'persistent', 'persistent-shared']
CONFIGS = list(itertools.product(sizing.FAMILIES, [0, 32], [1, 4], [False, True], [False, True]))
CALIBRATION = [(m, f, 32, 4, True, False) for m, f in itertools.product(['contextual', 'shared'], sizing.FAMILIES)]


def main():
    OUT.mkdir(exist_ok=False)
    paths = ['research/chr-relational/src/contextual.rs', 'research/chr-relational/src/contextual_execute.rs',
             'research/chr-relational/tests/contextual.rs', 'research/chr-relational/tests/contextual_source.rs',
             'research/chr-relational/examples/s02_deduction.rs', 'research/chr-relational/examples/support/deduction_source.rs',
             'research/chr-relational/Cargo.toml', 'research/chr-persistent/src/lib.rs', 'research/chr-persistent/src/map.rs',
             'research/chr-persistent/Cargo.toml', 'Cargo.lock',
             'research/chr-relational/experiments/equality_map_attribution.py', 'research/chr-relational/experiments/equality_map_analysis.py',
             'research/chr-relational/experiments/deduction_sizing.py', 'research/chr-direct-conditional/experiments/derivation_sizing.py',
             'research/chr-direct-conditional/experiments/shared_template_analysis.py',
             'docs/experiments/registrations/S02-equality-map-attribution.md']
    freeze = {'commit': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
              'rustc': subprocess.check_output(['rustc', '-Vv'], text=True), 'cpu': runner.CPU,
              'configs': CONFIGS, 'modes': MODES, 'calibration': CALIBRATION, 'sources': {}, 'binaries': {}}
    for p in paths:
        data = (ROOT / p).read_bytes()
        freeze['sources'][p] = hashlib.sha256(data).hexdigest()
        target = OUT / 'source' / p
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(data)
    old_freeze = json.loads((ROOT / 'docs/experiments/results/s02-deduction-sizing/freeze.json').read_text())
    for directory in [OLD, BIN]:
        for name in ['ordinary', 'meter']:
            p = directory / name
            digest = hashlib.sha256(p.read_bytes()).hexdigest()
            if directory == OLD:
                assert old_freeze['binaries'][str(p)] == digest
            freeze['binaries'][str(p)] = digest
        check = subprocess.run([str(directory / 'meter'), 'meter-check'], capture_output=True, text=True, timeout=60, preexec_fn=runner.limits)
        (OUT / ('old-meter.log' if directory == OLD else 'meter.log')).write_text(check.stdout + check.stderr)
        assert check.returncode == 0
    (OUT / 'freeze.json').write_text(json.dumps(freeze, indent=2) + '\n')
    runner.OUT, runner.BIN = OUT, BIN
    rng = random.Random(7221)
    for repetition in range(7):
        order = list(range(len(CONFIGS)))
        rng.shuffle(order)
        for i in order:
            modes = list(enumerate(MODES))
            rng.shuffle(modes)
            for j, mode in modes:
                runner.run('ordinary', (repetition * len(CONFIGS) + i) * len(MODES) + j, (mode, *CONFIGS[i]))
        print(f'timing {repetition + 1}/7', flush=True)
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
    for i, (mode, cancel) in enumerate(itertools.product(MODES, [0, 1])):
        for kind in ['cancel', 'cancel-meter']:
            runner.run(kind, i, (mode, 'shared', 8, 2, True, False), cancel)
    for version in ['before', 'after']:
        (OUT / version).mkdir()
    for repetition in range(7):
        for i, config in enumerate(CALIBRATION):
            order = ['before', 'after']
            rng.shuffle(order)
            for version in order:
                runner.OUT = OUT / version
                runner.BIN = OLD if version == 'before' else BIN
                runner.run('ordinary', repetition * len(CALIBRATION) + i, config)
    for version in ['before', 'after']:
        runner.OUT = OUT / version
        runner.BIN = OLD if version == 'before' else BIN
        previous = []
        for repetition in range(2):
            for i, config in enumerate(CALIBRATION):
                value = runner.allocation_records(runner.run('allocation', repetition * len(CALIBRATION) + i, config))
                if repetition == 0:
                    previous.append(value)
                else:
                    assert value == previous[i]
    print('Completed 1792 main timings, 512 allocations, 16 cancellations, 112 calibration timings and 32 calibration allocations.', flush=True)


if __name__ == '__main__':
    main()
