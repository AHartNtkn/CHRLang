#!/usr/bin/env python3
"""Prospectively registered paired compiler-representation attribution."""
import hashlib
import itertools
import json
import random
import shutil
import subprocess
from pathlib import Path
import derivation_sizing as runner

ROOT = runner.ROOT
OUT = ROOT / 'docs/experiments/results/s03-shared-template-bounded'
BINS = {'before': Path('/tmp/chr-derivation-lifecycle-b5d235e3'),
        'after': Path('/tmp/chr-shared-template-bounded-83deb0d3')}
FAMILIES = ['single', 'repeat', 'distinct', 'grow']
CONFIGS = [('templates', f, n, q, r, o)
           for f, q, r, o in itertools.product(FAMILIES, [1, 8], [False, True], [False, True])
           for n in [0, 12 if f == 'grow' else 32]]
CONFIGS += [('dependencies', f, 12 if f == 'grow' else 32, 8, True, False) for f in FAMILIES]


def run(version, kind, index, config, cancel=None):
    runner.OUT = OUT / version
    runner.BIN = BINS[version]
    return runner.run(kind, index, config, cancel)


def main():
    OUT.mkdir(exist_ok=False)
    for version in BINS:
        (OUT / version).mkdir()
    sources = ['research/chr-direct-choice/src/demand.rs', 'research/chr-direct-choice/src/demand/templates.rs',
               'research/chr-direct-conditional/experiments/derivation_cost.rs',
               'research/chr-direct-conditional/experiments/derivation_source.rs',
               'research/chr-direct-conditional/experiments/derivation_sizing.py',
               'research/chr-direct-conditional/experiments/shared_template_attribution.py',
               'research/chr-direct-conditional/experiments/shared_template_analysis.py',
               'docs/experiments/registrations/S03-shared-template-attribution.md', 'Cargo.lock']
    freeze = {'base_commit': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
              'rustc': subprocess.check_output(['rustc', '-Vv'], text=True), 'cpu': runner.CPU,
              'sources': {}, 'binaries': {}, 'configs': CONFIGS}
    for source in sources:
        path = ROOT / source
        freeze['sources'][source] = hashlib.sha256(path.read_bytes()).hexdigest()
        target = OUT / 'source' / source
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(path, target)
    for version, directory in BINS.items():
        for allocator in ['ordinary', 'meter']:
            path = directory / allocator
            freeze['binaries'][str(path)] = hashlib.sha256(path.read_bytes()).hexdigest()
        check = subprocess.run([str(directory / 'meter'), 'meter-check'], capture_output=True, text=True, timeout=60, preexec_fn=runner.limits)
        (OUT / version / 'meter-check.log').write_text(check.stdout + check.stderr)
        assert check.returncode == 0
    (OUT / 'freeze.json').write_text(json.dumps(freeze, indent=2) + '\n')
    rng = random.Random(7121)
    for repetition in range(7):
        order = list(range(len(CONFIGS)))
        rng.shuffle(order)
        for i in order:
            versions = list(BINS)
            rng.shuffle(versions)
            for version in versions:
                run(version, 'ordinary', repetition * len(CONFIGS) + i, CONFIGS[i])
        print(f'timing repetition {repetition + 1}/7', flush=True)
    for version in BINS:
        first = []
        for repetition in range(2):
            for i, config in enumerate(CONFIGS):
                records = runner.allocation_records(run(version, 'allocation', repetition * len(CONFIGS) + i, config))
                if repetition == 0:
                    first.append(records)
                else:
                    assert records == first[i], (version, config)
        print(f'{version} exact allocation replay passed', flush=True)
        for cancel in [0, 1]:
            for kind in ['cancel', 'cancel-meter']:
                run(version, kind, cancel, ('templates', 'choice', 8, 2, True, False), cancel)
    print('Completed 952 paired timings, 272 allocation runs, eight cancellation runs.', flush=True)


if __name__ == '__main__':
    main()
