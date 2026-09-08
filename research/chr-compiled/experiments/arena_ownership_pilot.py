#!/usr/bin/env python3
"""Execute the prospectively registered T052 arena ownership lifecycle comparison."""
import argparse
import hashlib
import json
import os
import pathlib
import random
import resource
import subprocess
import time

ROOT = pathlib.Path(__file__).resolve().parents[3]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', type=pathlib.Path, required=True)
    parser.add_argument('--registration', type=pathlib.Path, required=True)
    parser.add_argument('--seed', type=int, required=True)
    args = parser.parse_args()
    OUT = args.output.resolve()
    registration = args.registration.resolve()
    registration.relative_to(ROOT)
    if not registration.is_file():
        parser.error('registration must exist before running')
    OUT.mkdir(exist_ok=False)
    cpu = min(os.sched_getaffinity(0))
    meta = {'affinity_cpu': cpu, 'uname': list(os.uname()),
            'rustc': subprocess.check_output(['rustc', '-Vv'], text=True),
            'base_commit': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()}
    files = [ROOT / 'Cargo.lock', ROOT / 'Cargo.toml',
             registration]
    for base in ['research/chr-compiled',
                 'research/chr-persistent', 'research/chr-observe', 'crates/chr-syntax']:
        files.extend(p for p in (ROOT / base).rglob('*') if p.suffix in ['.rs', '.toml', '.py'])
    meta['sources'] = {str(p.relative_to(ROOT)): digest(p) for p in sorted(set(files))}
    cells = [(n, a, outcome, queries, mutation) for n in [0, 512]
             for a in [1, 64] for outcome in ['mostly-fail', 'all-success']
             for queries in [1, 4] for mutation in ['read', 'insert']]
    jobs = []
    rng = random.Random(args.seed)
    for mode, reps in [('warmup', 1), ('primary', 5), ('allocation', 1), ('work', 1)]:
        batch = [(mode, rep, ownership, cell) for rep in range(reps)
                 for ownership in ['clone', 'cow'] for cell in cells]
        rng.shuffle(batch)
        jobs.extend(batch)
    meta['bounds'] = {'process_seconds': 30, 'process_address_bytes': 1024**3,
                      'execution_seconds': 1200, 'build_seconds': 180}
    meta['seed'] = args.seed
    meta['jobs'] = [{'mode': mode, 'rep': rep, 'ownership': ownership, 'cell': cell}
                    for mode, rep, ownership, cell in jobs]
    (OUT / 'metadata.json').write_text(json.dumps(meta, indent=2) + '\n')
    binaries = {}
    for mode, features in [('primary', ['--no-default-features', '--features', 'experiment']),
                           ('allocation', ['--no-default-features', '--features', 'alloc-meter']),
                           ('work', ['--features', 'experiment'])]:
        for ownership in ['clone', 'cow']:
            key = ownership + '-' + mode
            target = ROOT / 'target' / (OUT.name + '-' + key)
            selected = features if ownership == 'clone' else [*features[:-1], features[-1] + ',arena-cow']
            command = ['cargo', 'build', '--locked', '--offline', '--release', '-p', 'chr-compiled',
                       '--bin', 'chr-state-preservation-cost', '--target-dir', str(target), *selected]
            build = {'command': command, 'status': 'running'}
            meta.setdefault('builds', {})[key] = build
            (OUT / 'metadata.json').write_text(json.dumps(meta, indent=2) + '\n')
            before = time.monotonic()
            try:
                with (OUT / (key + '-build.log')).open('w') as log:
                    result = subprocess.run(command, cwd=ROOT, stdout=log,
                                            stderr=subprocess.STDOUT, timeout=180)
                build.update(exit=result.returncode, status='complete' if result.returncode == 0 else 'failed')
                result.check_returncode()
                binaries[key] = str(target / 'release/chr-state-preservation-cost')
                build['binary_sha256'] = digest(pathlib.Path(binaries[key]))
            except (OSError, subprocess.SubprocessError) as error:
                build.update(status='failed', error=repr(error))
                raise
            finally:
                build['wall_s'] = time.monotonic() - before
                (OUT / 'metadata.json').write_text(json.dumps(meta, indent=2) + '\n')


    (OUT / 'metadata.json').write_text(json.dumps(meta, indent=2) + '\n')
    def limits():
        os.sched_setaffinity(0, {cpu})
        resource.setrlimit(resource.RLIMIT_AS, (1024**3, 1024**3))
    check = subprocess.run([binaries['clone-allocation'], 'meter-check'], capture_output=True,
                           text=True, timeout=30, preexec_fn=limits)
    (OUT / 'meter-check.log').write_text(check.stdout + check.stderr)
    check.check_returncode()
    start = time.monotonic()
    with (OUT / 'runs.jsonl').open('x') as log:
        for i, (mode, rep, ownership, cell) in enumerate(jobs):
            if time.monotonic() - start > 1200:
                (OUT / 'total-bound.json').write_text(json.dumps({'completed': i, 'planned': len(jobs)}))
                break
            command = [binaries[ownership + '-' + ('primary' if mode == 'warmup' else mode)], *map(str, cell)]
            before = time.monotonic()
            row = {'mode': mode, 'rep': rep, 'ownership': ownership, 'cell': cell, 'command': command}
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
            except (OSError, subprocess.SubprocessError) as error:
                row['spawn_error'] = repr(error)
            row['wall_s'] = time.monotonic() - before
            log.write(json.dumps(row) + '\n')
            log.flush()
            if i % 16 == 15:
                print(f'{i + 1}/{len(jobs)} processes recorded', flush=True)
    for name, expected in meta['sources'].items():
        assert digest(ROOT / name) == expected, ('source changed', name)
    for mode, binary in binaries.items():
        assert digest(pathlib.Path(binary)) == meta['builds'][mode]['binary_sha256']
    (OUT / 'source-check.log').write_text('All frozen source and binary hashes unchanged after runs.\n')

if __name__ == '__main__':
    main()
