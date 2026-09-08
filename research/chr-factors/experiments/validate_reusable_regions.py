#!/usr/bin/env python3
"""Complete-product correspondence and deliberate faults; no performance claims."""
from pathlib import Path
import hashlib
import json
import os
import signal
import subprocess

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s09-regional-source'
TARGET = ROOT / 'research/chr-factors/experiments/reusable_regions.rs'


def run(name, command, fault=False):
    process = subprocess.Popen(command, cwd=ROOT, text=True, stdout=subprocess.PIPE,
                               stderr=subprocess.PIPE, start_new_session=True)
    cutoff = False
    try:
        stdout, stderr = process.communicate(timeout=60)
    except subprocess.TimeoutExpired:
        cutoff = True
        os.killpg(process.pid, signal.SIGKILL)
        stdout, stderr = process.communicate()
    receipt = dict(command=command, exit_code=process.returncode, cutoff=cutoff,
                   stdout=stdout, stderr=stderr)
    (OUT / (name + '.json')).write_text(json.dumps(receipt, indent=2) + '\n')
    assert not cutoff, (name, 'timeout is not a result')
    if fault:
        assert process.returncode != 0 and 'panicked at' in stdout, (name, receipt)
    else:
        assert process.returncode == 0, (name, receipt)
    print(name, process.returncode, flush=True)


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    original = TARGET.read_text()
    base = ['cargo', 'test', '-p', 'chr-factors', '--release', '--test', 'reusable_regions']
    executable = ['cargo', 'run', '-p', 'chr-factors', '--no-default-features',
                  '--release', '--example', 'reusable_regions_gate']
    run('release', base)
    run('counter-free', base + ['--no-default-features'])
    run('ordinary-executable', executable)
    run('package', ['cargo', 'test', '-p', 'chr-factors'])
    run('clippy', ['cargo', 'clippy', '-p', 'chr-factors', '--no-default-features',
                  '--test', 'reusable_regions', '--test', 'reusable_workers',
                  '--example', 'reusable_regions_gate', '--', '-D', 'warnings'])
    for i in range(4):
        run(f'repeat-{i}', base + ['--no-default-features'])
    variants = {
        'fault-cross-factor-alias': (
            'let answer = &factor.answers[job.cursor[region]];',
            'next = 0; // injected accidental cross-factor alias\n'
            '            let answer = &factor.answers[job.cursor[region]];'),
        'fault-missing-products': (
            'if sizes.iter().all(|n| *n > 0) {',
            'if false && sizes.iter().all(|n| *n > 0) {'),
        'fault-incomplete-product': (
            'if job.next() {', 'if false && job.next() {'),
    }
    try:
        for name, (old, new) in variants.items():
            assert original.count(old) == 1, name
            TARGET.write_text(original.replace(old, new))
            (OUT / (name + '.patch.txt')).write_text('Replace exactly:\n' + old + '\nWith:\n' + new + '\n')
            run(name, base, fault=True)
    finally:
        TARGET.write_text(original)
    run('restored', base)
    run('restored-executable', executable)
    paths = [TARGET, ROOT / 'research/chr-factors/experiments/reusable_workers.rs',
             ROOT / 'research/chr-factors/src/partition.rs',
             ROOT / 'research/chr-factors/tests/reusable_regions.rs',
             ROOT / 'research/chr-factors/examples/reusable_regions_gate.rs',
             Path(__file__).resolve()]
    (OUT / 'source-hashes.json').write_text(json.dumps({str(p.relative_to(ROOT)):
        hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}, indent=2) + '\n')


if __name__ == '__main__':
    main()
