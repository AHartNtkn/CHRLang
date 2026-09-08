#!/usr/bin/env python3
"""Execute the prospectively registered R06 restricted-publication comparison."""
import hashlib
import itertools
import json
import os
import pathlib
import random
import resource
import subprocess
import time

ROOT = pathlib.Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/r06-restricted-publication'

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def main():
    OUT.mkdir(exist_ok=False)
    cpu = min(os.sched_getaffinity(0))
    meta = {'affinity_cpu': cpu, 'uname': list(os.uname()),
            'rustc': subprocess.check_output(['rustc', '-Vv'], text=True),
            'base_commit': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()}
    files = [ROOT / 'Cargo.lock', ROOT / 'Cargo.toml',
             ROOT / 'docs/experiments/registrations/R06-restricted-publication.md']
    for base in ['research/chr-direct-conditional', 'research/chr-compiled',
                 'research/chr-persistent', 'research/chr-observe', 'crates/chr-syntax']:
        files.extend(p for p in (ROOT / base).rglob('*') if p.suffix in ['.rs', '.toml', '.py'])
    meta['sources'] = {str(p.relative_to(ROOT)): digest(p) for p in sorted(set(files))}
    prior = json.loads((ROOT / 'docs/experiments/results/r06-streaming-lifetime/metadata.json').read_text())
    baseline = {}
    meta['baseline'] = {'manifest': 'docs/experiments/results/r06-streaming-lifetime/metadata.json',
                        'source_commit': prior['base_commit'], 'builds': {}}
    for mode, build in prior['builds'].items():
        command = build['command']
        binary = pathlib.Path(command[command.index('--target-dir') + 1]) / 'release/chr-stream-cost'
        assert digest(binary) == build['binary_sha256'], ('baseline binary mismatch', mode)
        baseline[mode] = str(binary)
        meta['baseline']['builds'][mode] = {'binary': str(binary), 'binary_sha256': build['binary_sha256']}
    binaries = {}
    for mode, features in [('primary', ['--no-default-features', '--features', 'experiment']),
                           ('allocation', ['--no-default-features', '--features', 'alloc-meter']),
                           ('work', ['--features', 'experiment'])]:
        target = ROOT / 'target' / ('r06-restricted-' + mode)
        command = ['cargo', 'build', '--release', '-p', 'chr-direct-conditional',
                   '--bin', 'chr-stream-cost', '--target-dir', str(target), *features]
        with (OUT / (mode + '-build.log')).open('w') as log:
            subprocess.run(command, cwd=ROOT, stdout=log, stderr=subprocess.STDOUT, check=True)
        binaries[mode] = str(target / 'release/chr-stream-cost')
        meta.setdefault('builds', {})[mode] = {'command': command,
                                               'binary_sha256': digest(pathlib.Path(binaries[mode]))}
    (OUT / 'metadata.json').write_text(json.dumps(meta, indent=2) + '\n')
    def limits():
        os.sched_setaffinity(0, {cpu})
        resource.setrlimit(resource.RLIMIT_AS, (1024**3, 1024**3))
    for label, binary in [('current', binaries['allocation']), ('baseline', baseline['allocation'])]:
        check = subprocess.run([binary, 'meter-check'], capture_output=True, text=True, timeout=30, preexec_fn=limits)
        (OUT / (label + '-meter-check.log')).write_text(check.stdout + check.stderr)
        check.check_returncode()
    cells = list(itertools.product(['baseline', 'conditional', 'specialized'], ['ground', 'aliases'],
                                   ['unbounded', 'finite'], [16, 64, 128], ['drop', 'retain']))
    jobs = []
    rng = random.Random(40041)
    for mode, reps in [('warmup', 1), ('primary', 5), ('allocation', 1), ('work', 1)]:
        batch = [(mode, rep, cell) for rep in range(reps) for cell in cells]
        rng.shuffle(batch)
        jobs.extend(batch)
    start = time.monotonic()
    with (OUT / 'runs.jsonl').open('x') as log:
        for i, (mode, rep, cell) in enumerate(jobs):
            if time.monotonic() - start > 1800:
                (OUT / 'total-bound.json').write_text(json.dumps({'completed': i, 'planned': len(jobs)}))
                break
            build_mode = 'primary' if mode == 'warmup' else mode
            command = [baseline[build_mode] if cell[0] == 'baseline' else binaries[build_mode],
                       'conditional' if cell[0] == 'baseline' else cell[0], *map(str, cell[1:])]
            before = time.monotonic()
            row = {'mode': mode, 'rep': rep, 'cell': cell, 'command': command}
            try:
                result = subprocess.run(command, capture_output=True, text=True,
                                        timeout=30, preexec_fn=limits)
                row.update(exit=result.returncode, stderr=result.stderr, stdout=result.stdout)
                if result.returncode == 0:
                    try:
                        row['result'] = json.loads(result.stdout)
                    except json.JSONDecodeError as error:
                        row['decode_error'] = str(error)
            except subprocess.TimeoutExpired as error:
                row.update(timeout=True, stdout=str(error.stdout), stderr=str(error.stderr))
            row['wall_s'] = time.monotonic() - before
            log.write(json.dumps(row) + '\n')
            log.flush()
            if i % 16 == 15:
                print(f'{i + 1}/{len(jobs)} processes recorded', flush=True)
    for name, expected in meta['sources'].items():
        assert digest(ROOT / name) == expected, ('source changed', name)
    for mode, binary in binaries.items():
        assert digest(pathlib.Path(binary)) == meta['builds'][mode]['binary_sha256']
    for mode, binary in baseline.items():
        assert digest(pathlib.Path(binary)) == meta['baseline']['builds'][mode]['binary_sha256']
    (OUT / 'source-check.log').write_text('All frozen source and binary hashes unchanged after runs.\n')

if __name__ == '__main__':
    main()
