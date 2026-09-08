#!/usr/bin/env python3
"""Registered paired cold lifecycle pilot. --plan never builds or executes CHR."""
import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import random
import subprocess
import tarfile
import tempfile
import time

from regional_pilot import ROOT, FINITE, bounds, capacity, cgroup_limits, digest

OUT = ROOT / 'docs/experiments/results/r01-closed-subtree-lifecycle'
VERSIONS = {'baseline': 'a852a82', 'current': '8b60e95'}
CELLS = [(c, m, 64 if m == 'Inline' else 0, 4 if m == 'Inline' else 0)
         for c in FINITE for m in ['Inline', 'Specialized']]
CELLS += [('two-work', m, 64, 4) for m in ['Threads1', 'Threads2']]
FOLDERS = ['research/chr-factors', 'research/chr-persistent', 'research/chr-observe',
           'research/chr-compiled', 'research/chr-cases', 'crates/chr-syntax', 'crates/chr-programs']
REGISTRATION = 'docs/experiments/registrations/R01-closed-subtree-lifecycle.md'


def manifest():
    rng = random.Random(44045)
    order = []
    for kind, rep in [('warmup', 0), *[('primary', i) for i in range(5)], ('allocation', 0), ('work', 0)]:
        block = [dict(version=v, kind=kind, rep=rep, cell=list(c)) for v in VERSIONS for c in CELLS]
        rng.shuffle(block)
        order.extend(block)
    return order


def source_paths(root):
    return sorted({'Cargo.toml', 'Cargo.lock'} | {
        str(p.relative_to(root)) for folder in FOLDERS for p in (root / folder).rglob('*')
        if p.is_file() and p.suffix in ['.rs', '.toml']})


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, default=OUT)
    parser.add_argument('--plan', action='store_true')
    args = parser.parse_args()
    if args.plan:
        print(json.dumps(manifest(), indent=2)); return
    from analyze_closed_subtree import validate_result
    out = args.out.resolve()
    assert not out.exists(), 'refuse to overwrite evidence'
    hardware, quotas = capacity(), cgroup_limits()
    out.mkdir(parents=True)
    scratch = Path(tempfile.mkdtemp(prefix='chr-closed-subtree-'))
    roots = {'baseline': scratch / 'baseline', 'current': ROOT}
    commits = {v: subprocess.check_output(['git', 'rev-parse', ref], cwd=ROOT, text=True).strip()
               for v, ref in VERSIONS.items()}
    archive = scratch / 'baseline.tar'
    with archive.open('wb') as stream:
        subprocess.run(['git', 'archive', '--format=tar', commits['baseline']], cwd=ROOT, stdout=stream, check=True)
    roots['baseline'].mkdir()
    with tarfile.open(archive) as stream:
        members = stream.getmembers()
        for member in members:
            path = PurePosixPath(member.name)
            assert not path.is_absolute() and '..' not in path.parts
            assert member.isfile() or member.isdir(), ('unexpected archive entry', member.name)
        stream.extractall(roots['baseline'], members=members)
    sources = {}
    frozen = {str(archive): digest(archive)}
    for version, root in roots.items():
        sources[version] = {p: digest(root / p) for p in source_paths(root)}
        for path, sha in sources[version].items():
            original = subprocess.check_output(['git', 'show', f'{commits[version]}:{path}'], cwd=ROOT)
            assert hashlib.sha256(original).hexdigest() == sha, (version, path, 'source differs from selected commit')
            frozen[str(root / path)] = sha
    # The lifecycle/fixture control is literally identical between treatments.
    for path in sources['baseline']:
        if path.startswith('research/chr-factors/examples/'):
            assert sources['baseline'][path] == sources['current'][path], path
    harness = [Path(__file__).resolve(), Path(__file__).with_name('analyze_closed_subtree.py'),
               Path(__file__).with_name('regional_pilot.py'), ROOT / REGISTRATION]
    frozen.update({str(p): digest(p) for p in harness})
    binaries, commands = {}, {}
    rustc = subprocess.check_output(['rustc', '-Vv'], cwd=ROOT, text=True)
    for version, root in roots.items():
        assert subprocess.check_output(['rustc', '-Vv'], cwd=root, text=True) == rustc
        binaries[version], commands[version] = {}, {}
        for kind in ['primary', 'allocation', 'work']:
            example = 'region_lifecycle_memory' if kind == 'allocation' else 'region_lifecycle'
            target = scratch / f'target-{version}-{kind}'
            command = ['cargo', 'build', '--locked', '-p', 'chr-factors', '--release', '--features', 'lifecycle',
                       '--example', example, '--target-dir', str(target)]
            if kind != 'work': command.append('--no-default-features')
            commands[version][kind] = command
            print(f'Building {version}/{kind}', flush=True)
            with (out / f'{version}-{kind}-build.log').open('w') as log:
                subprocess.run(command, cwd=root, stdout=log, stderr=subprocess.STDOUT, check=True)
            binary = target / 'release/examples' / example
            binaries[version][kind] = str(binary)
            frozen[str(binary)] = digest(binary)
    assert all(Path(p).exists() and digest(Path(p)) == sha for p, sha in frozen.items()), 'build changed frozen input'
    meta = dict(created_utc=time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()), schema_version=1, seed=44045, order=manifest(), commits=commits,
                roots={v: str(p) for v, p in roots.items()}, sources=sources, frozen=frozen,
                binaries=binaries, build_commands=commands, rustc=rustc, hardware=hardware,
                ancestor_limits=quotas, cpuinfo=Path('/proc/cpuinfo').read_text(),
                timeout_seconds=30, address_space_bytes=1073741824, batch_seconds=1200)
    write_json(out / 'metadata.json', meta)
    started, stopped = time.monotonic(), None
    try:
        with (out / 'runs.jsonl').open('w') as output:
            for index, item in enumerate(meta['order']):
                if time.monotonic() - started >= 1200: stopped = stopped or 'batch-budget'
                row = dict(item)
                if stopped:
                    row.update(attempted=False, reason=stopped)
                else:
                    row['attempted'] = True
                    kind = 'primary' if item['kind'] == 'warmup' else item['kind']
                    command = [binaries[item['version']][kind], *map(str, item['cell'])]
                    begin = time.monotonic()
                    try:
                        result = subprocess.run(command, cwd=roots[item['version']], capture_output=True,
                                                text=True, timeout=30, preexec_fn=bounds)
                        row.update(exit=result.returncode, stdout=result.stdout, stderr=result.stderr)
                        if result.returncode:
                            stopped = 'process-error'
                        else:
                            try:
                                row['result'] = json.loads(result.stdout)
                                validate_result(row)
                            except (ValueError, AssertionError, KeyError, TypeError) as error:
                                row['validation_error'] = str(error) or repr(error)
                                stopped = 'semantic-configuration-or-retention-error'
                    except subprocess.TimeoutExpired as error:
                        row.update(timeout=True, exit=None, stdout=(error.stdout or b'').decode(errors='replace'),
                                   stderr=(error.stderr or b'').decode(errors='replace'))
                    row['process_seconds'] = time.monotonic() - begin
                output.write(json.dumps(row) + '\n'); output.flush()
                if (index + 1) % 36 == 0: print(f'{index + 1}/288 outcomes recorded', flush=True)
    finally:
        changed = [p for p, sha in frozen.items() if not Path(p).exists() or digest(Path(p)) != sha]
        write_json(out / 'source-check.json', dict(changed=changed, frozen=frozen))
    assert not changed, changed
    print(f'Evidence preserved at {out}; no outcomes omitted.')


if __name__ == '__main__':
    main()
