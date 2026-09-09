#!/usr/bin/env python3
"""Prospectively registered isolated lifecycle sizing; stops on any invalid receipt."""
import hashlib
import itertools
import json
import os
from pathlib import Path
import random
import resource
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
NAME = sys.argv[1] if len(sys.argv) == 2 else 's02-local-lifecycle-sizing'
assert NAME in ['s02-local-lifecycle-sizing', 's02-local-validation-attribution']
OUT = ROOT / 'docs/experiments/results' / NAME
BIN = ROOT / 'target' / NAME
MODES = ['endpoint', 'filtered', 'local-indexed', 'scan', 'indexed', 'special-scan', 'special-indexed']
FAMILIES = ['chain', 'flat', 'shared', 'lowyield']
FEATURES = {'time': [], 'meter': ['alloc-meter'], 'work': ['local-work', 'compiled-work']}
CPU = min(os.sched_getaffinity(0))

def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

def limits():
    resource.setrlimit(resource.RLIMIT_AS, (1024**3, 1024**3))
    os.sched_setaffinity(0, {CPU})

def invoke(binary, cell):
    mode, family, n, reuse, stop = cell
    cmd = [str(binary), mode, family, str(n), str(reuse), stop]
    p = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True, timeout=60, preexec_fn=limits)
    if p.returncode:
        raise RuntimeError(json.dumps({'command': cmd, 'code': p.returncode, 'stderr': p.stderr}))
    receipt = json.loads(p.stdout)
    assert receipt['family'] == family and receipt['size'] == n and receipt['reuse'] == reuse and receipt['stop'] == stop
    assert len(receipt['phases']) == reuse * 5 + 2
    if receipt['meter']:
        assert receipt['restored'] and not receipt['metrics']
    if receipt['metrics'] and stop == 'complete':
        applications = n if family in ['chain', 'flat'] else n + 1 if family == 'shared' else 1
        assert len(receipt['work']) == reuse
        for w in receipt['work']:
            assert w['applications'] == applications, (cell, w)
    return cmd, receipt

def main():
    assert not (OUT / 'runs.jsonl').exists(), 'Preserve existing matrix; choose a new registration/output for another run.'
    OUT.mkdir(parents=True, exist_ok=True)
    BIN.mkdir(parents=True, exist_ok=True)
    binaries = {}
    builds = {}
    for kind, features in FEATURES.items():
        cmd = ['cargo', 'test', '-p', 'chr-relational', '--release', '--test', 'local_lifecycle', '--no-run', '--message-format=json']
        if features:
            cmd += ['--features', ','.join(features)]
        p = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True, check=True)
        (OUT / f'build-{kind}.log').write_text(p.stderr.rstrip() + '\n')
        artifacts = [json.loads(s) for s in p.stdout.splitlines() if s.startswith('{')]
        paths = [a['executable'] for a in artifacts if a.get('reason') == 'compiler-artifact' and a.get('executable') and a['target']['name'] == 'local_lifecycle']
        assert len(paths) == 1
        dest = BIN / kind
        shutil.copy2(paths[0], dest)
        binaries[kind] = dest
        builds[kind] = {'command': cmd, 'binary': str(dest.relative_to(ROOT)), 'sha256': sha(dest)}
        smoke = subprocess.run([str(dest)], cwd=ROOT, capture_output=True, text=True, check=True, timeout=60, preexec_fn=limits)
        assert '84 independent source comparisons pass' in smoke.stdout
        (OUT / f'smoke-{kind}.log').write_text(smoke.stdout)
        # Complete phase/restoration checks for each control before the matrix.
        for mode in MODES:
            invoke(dest, (mode, 'shared', 4, 1, 'complete'))
    paths = subprocess.check_output(['rg', '--files', 'research/chr-relational', 'research/chr-compiled', 'research/chr-persistent', 'research/chr-observe', 'research/chr-direct-conditional/tests/runtime_support', 'crates/chr-syntax'], cwd=ROOT, text=True).splitlines()
    paths += ['Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', 'docs/experiments/registrations/S02-local-lifecycle-sizing.md', 'docs/experiments/registrations/S02-local-validation-attribution.md']
    source_hashes = {p: sha(ROOT / p) for p in sorted(set(paths)) if (ROOT / p).is_file()}
    order = []
    rng = random.Random(7204)
    cells = list(itertools.product(MODES, FAMILIES, [4, 32, 128], [1, 4], ['complete']))
    for kind, repetitions in [('time', 5), ('meter', 2), ('work', 2)]:
        for rep in range(repetitions):
            block = cells.copy()
            rng.shuffle(block)
            order += [(kind, rep, cell) for cell in block]
    for rep in range(2):
        block = list(itertools.product(MODES, FAMILIES, [32], [1], ['setup', 'cancel']))
        rng.shuffle(block)
        order += [('meter', rep, cell) for cell in block]
    assert len(order) == 1624
    manifest = {'base_head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(), 'builds': builds, 'source_hashes': source_hashes, 'cpu': CPU, 'timeout_seconds': 60, 'address_space_bytes': 1024**3, 'toolchain': subprocess.check_output(['rustc', '--version'], text=True).strip(), 'order': order}
    (OUT / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
    with (OUT / 'runs.jsonl').open('x') as log:
        for i, (kind, rep, cell) in enumerate(order):
            cmd, receipt = invoke(binaries[kind], cell)
            assert receipt['metrics'] == (kind == 'work') and receipt['meter'] == (kind == 'meter')
            log.write(json.dumps({'index': i, 'kind': kind, 'rep': rep, 'cell': cell, 'command': cmd, 'receipt': receipt}) + '\n')
            log.flush()
            if (i + 1) % 56 == 0:
                print(f'{i+1}/{len(order)} valid processes', flush=True)
    for p, digest in source_hashes.items():
        assert sha(ROOT / p) == digest, p
    for kind, p in binaries.items():
        assert sha(p) == builds[kind]['sha256']
    print('All registered processes validated; source and binary freezes unchanged.', flush=True)

if __name__ == '__main__':
    main()
