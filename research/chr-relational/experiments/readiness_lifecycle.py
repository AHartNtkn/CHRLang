"""Registered serial lifecycle sizing, with frozen builds and separate meters."""
import hashlib
import itertools
import json
import os
import random
import resource
import subprocess
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s02-readiness-lifecycle'
MODES = ['full', 'selective', 'batch8', 'batch256', 'compiled']
CPU = min(os.sched_getaffinity(0))


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def limits():
    os.sched_setaffinity(0, {CPU})
    resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
    resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))


def invoke(binary, args):
    command = [str(binary), *map(str, args)]
    try:
        r = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, timeout=60, preexec_fn=limits)
        return dict(command=command, exit_code=r.returncode, stdout=r.stdout, stderr=r.stderr)
    except subprocess.TimeoutExpired as e:
        return dict(command=command, exit_code='timeout', stdout=str(e.stdout), stderr=str(e.stderr))


def main():
    assert not (OUT / 'freeze.json').exists(), 'Do not replace a frozen run'
    binaries, builds = {}, {}
    for flavor in ['ordinary', 'meter']:
        command = ['cargo', 'test', '-p', 'chr-relational', '--test', 'readiness_lifecycle',
                   '--release', '--no-default-features', '--no-run', '--message-format=json',
                   '--target-dir', str(ROOT / 'target' / 'readiness-lifecycle' / flavor)]
        if flavor == 'meter':
            command += ['--features', 'alloc-meter']
        r = subprocess.run(command, cwd=ROOT, capture_output=True, text=True)
        (OUT / f'build-{flavor}.log').write_text(r.stderr.rstrip() + '\n')
        assert r.returncode == 0
        artifacts = [json.loads(s) for s in r.stdout.splitlines() if s.startswith('{')]
        binary = Path(next(x['executable'] for x in artifacts if x.get('executable') and x.get('target', {}).get('name') == 'readiness_lifecycle'))
        binaries[flavor] = binary
        builds[flavor] = dict(command=command, binary=str(binary), sha256=sha(binary))
        print('Built', flavor, flush=True)
    entry = []
    check = invoke(binaries['meter'], ['meter-check'])
    (OUT / 'meter-check.json').write_text(json.dumps(check, indent=2) + '\n')
    assert check['exit_code'] == 0
    for flavor, binary in binaries.items():
        for mode, shared, outcome in itertools.product(MODES, ['false', 'true'], ['success', 'fail', 'clash', 'cancel']):
            receipt = invoke(binary, [mode, 4, shared, outcome, 2])
            entry.append(dict(flavor=flavor, mode=mode, shared=shared, outcome=outcome, receipt=receipt))
            (OUT / 'entry.json').write_text(json.dumps(entry, indent=2) + '\n')
            assert receipt['exit_code'] == 0, receipt
            assert json.loads(receipt['stdout'])['validated']
    print('80 entry processes and meter self-check passed', flush=True)
    floor = invoke(binaries['ordinary'], ['clock-floor'])
    (OUT / 'clock-floor.json').write_text(json.dumps(floor, indent=2) + '\n')
    assert floor['exit_code'] == 0
    paths = []
    for folder in ['research/chr-relational', 'research/chr-compiled', 'research/chr-persistent', 'crates/chr-syntax']:
        paths += list((ROOT / folder).rglob('*.rs')) + [ROOT / folder / 'Cargo.toml']
    paths += [ROOT / 'Cargo.toml', ROOT / 'Cargo.lock', Path(__file__), ROOT / 'research/chr-direct-conditional/tests/runtime_support/mod.rs', ROOT / 'docs/experiments/registrations/S02-readiness-lifecycle.md']
    hashes = {str(p.relative_to(ROOT)): sha(p) for p in paths}
    with zipfile.ZipFile(OUT / 'sources.zip', 'w', zipfile.ZIP_DEFLATED) as archive:
        for p in hashes: archive.write(ROOT / p, p)
    freeze = dict(builds=builds, sources=hashes, archive_sha256=sha(OUT / 'sources.zip'), cpu=CPU, seed=7204,
                  rustc=subprocess.check_output(['rustc', '-Vv'], text=True),
                  host=subprocess.check_output(['uname', '-a'], text=True),
                  parent=subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip())
    (OUT / 'freeze.json').write_text(json.dumps(freeze, indent=2) + '\n')
    warmups = [invoke(binaries['ordinary'], [m, 4, 'false', 'success', 1]) for m in MODES]
    (OUT / 'warmups.json').write_text(json.dumps(warmups, indent=2) + '\n')
    assert all(r['exit_code'] == 0 for r in warmups)
    cells = list(itertools.product(MODES, [4, 64], ['false', 'true'], ['success', 'fail', 'clash', 'cancel'], [1, 16]))
    rng = random.Random(7204)
    with (OUT / 'samples.jsonl').open('x') as stream:
        for flavor, repetitions in [('ordinary', 5), ('meter', 2)]:
            for rep in range(repetitions):
                order = cells.copy(); rng.shuffle(order)
                for mode, depth, shared, outcome, count in order:
                    receipt = invoke(binaries[flavor], [mode, depth, shared, outcome, count])
                    row = dict(flavor=flavor, rep=rep, mode=mode, depth=depth, shared=shared, outcome=outcome, count=count, receipt=receipt)
                    stream.write(json.dumps(row) + '\n'); stream.flush()
                    assert receipt['exit_code'] == 0, row
                    assert json.loads(receipt['stdout'])['validated']
                print(flavor, 'block', rep, 'complete', flush=True)
    print('1,120 isolated lifecycle samples complete', flush=True)


if __name__ == '__main__':
    main()
