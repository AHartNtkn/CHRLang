"""Registered finite-session timing and separate heap measurements."""
import gzip
import hashlib
import itertools
import json
import os
from pathlib import Path
import random
import resource
import subprocess
import time
import zipfile

ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / 'docs/experiments/results/s05-inert-lifecycle'
MODES = ['direct', 'whole', 'compact', 'separate', 'memo', 'scan', 'indexed',
         'sealed', 'active-scan', 'active-indexed']


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def bounds():
    os.sched_setaffinity(0, {0})
    resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
    resource.setrlimit(resource.RLIMIT_CPU, (60, 60))


def launch(command):
    try:
        result = subprocess.run(command, capture_output=True, text=True,
                                timeout=60, preexec_fn=bounds)
        return dict(command=command, exit_code=result.returncode,
                    stdout=result.stdout, stderr=result.stderr, timeout=False)
    except subprocess.TimeoutExpired as error:
        def decode(value):
            return value.decode() if isinstance(value, bytes) else value or ''
        return dict(command=command, exit_code=None, stdout=decode(error.stdout),
                    stderr=decode(error.stderr), timeout=True)


def main():
    assert not (BASE / 'freeze.json').exists(), 'Existing campaign must be inspected, not restarted'
    assert 0 in os.sched_getaffinity(0)
    binaries = {}
    for kind, features in [('meter', ['--features', 'alloc-meter']), ('plain', [])]:
        cmd = ['cargo', 'build', '-p', 'chr-reuse', '--example', 'inert_ownership',
               '--release', '--no-default-features', *features,
               '--target-dir', f'target/s05-inert-lifecycle-{kind}', '--message-format=json']
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=300)
        (BASE / f'build-{kind}.json').write_text(json.dumps(dict(
            command=cmd, exit_code=result.returncode, stdout=result.stdout, stderr=result.stderr)))
        assert result.returncode == 0, result.stderr
        artifacts = [json.loads(line) for line in result.stdout.splitlines()]
        executable = Path(next(a['executable'] for a in artifacts if a.get('executable')))
        for artifact in artifacts:
            if artifact.get('reason') != 'compiler-artifact':
                continue
            assert not set(artifact['features']) & {'metrics', 'kernel-metrics', 'stage-alloc'}, artifact
            if kind == 'plain':
                assert 'alloc-meter' not in artifact['features'], artifact
        binaries[kind] = dict(path=str(executable), sha256=sha(executable))
    paths = subprocess.check_output(['git', 'ls-files', 'crates', 'research',
                                     'Cargo.toml', 'Cargo.lock'], text=True).splitlines()
    paths = [p for p in paths if p.endswith(('.rs', '.toml', '.lock'))]
    paths += ['research/chr-reuse/experiments/inert_lifecycle.py',
              'research/chr-reuse/experiments/audit_inert_lifecycle.py',
              'docs/experiments/registrations/S05-inert-lifecycle.md']
    hashes = {p: sha(ROOT / p) for p in sorted(set(paths))}
    with zipfile.ZipFile(BASE / 'sources.zip', 'w', compression=zipfile.ZIP_DEFLATED) as archive:
        for path in hashes:
            archive.write(ROOT / path, path)
    scenarios = list(itertools.product(range(6), [0, 4, 32, 128], [False, True], [1, 4], ['0', '1', 'all']))
    rng = random.Random(750511)
    jobs = []
    for kind, repetitions in [('meter', 2), ('plain', 5)]:
        for rep in range(repetitions):
            order = scenarios.copy()
            rng.shuffle(order)
            for family, depth, distinct, reuse, keep in order:
                modes = MODES.copy()
                rng.shuffle(modes)
                for mode in modes:
                    jobs.append(dict(kind=kind, rep=rep, args=[mode, str(family), str(depth),
                        str(reuse), keep, '0', str(distinct).lower()]))
    (BASE / 'jobs.json').write_text(json.dumps(jobs))
    (BASE / 'freeze.json').write_text(json.dumps(dict(binaries=binaries, sources=hashes,
        archive_sha256=sha(BASE / 'sources.zip'), jobs_sha256=sha(BASE / 'jobs.json'),
        cpu=0, affinity=sorted(os.sched_getaffinity(0)),
        toolchain=subprocess.check_output(['rustc', '-Vv'], text=True)), indent=2))
    clocks = [launch([binaries['plain']['path'], 'clock-check']) for _ in range(3)]
    (BASE / 'clocks.json').write_text(json.dumps(clocks))
    assert all(c['exit_code'] == 0 for c in clocks)
    failures = {}
    start = time.monotonic()
    with gzip.open(BASE / 'runs.jsonl.gz', 'wt') as output:
        for index, job in enumerate(jobs):
            if time.monotonic() - start > 1800:
                break
            key = (job['kind'], tuple(job['args']))
            if failures.get(key, 0) >= 2:
                raw = dict(skipped='two prior failures in this build')
            else:
                raw = launch([binaries[job['kind']]['path'], *job['args']])
                if raw['exit_code'] != 0:
                    failures[key] = failures.get(key, 0) + 1
            output.write(json.dumps(dict(index=index, **raw)) + '\n')
            output.flush()
            if (index + 1) % 288 == 0:
                print(index + 1, '/', len(jobs), 'seconds', round(time.monotonic() - start), flush=True)
        else:
            index = len(jobs) - 1
    assert all(sha(ROOT / path) == digest for path, digest in hashes.items())
    (BASE / 'campaign.json').write_text(json.dumps(dict(recorded=index + 1,
        scheduled=len(jobs), seconds=time.monotonic() - start)))


if __name__ == '__main__':
    main()
