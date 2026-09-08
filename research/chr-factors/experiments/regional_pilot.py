#!/usr/bin/env python3
"""Execute the prospectively registered R08 pilot; preserve every outcome."""
import hashlib
import json
import os
from pathlib import Path
import random
import resource
import subprocess
import time

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/r08-regional-lifecycle'
FINITE = ['one-zero', 'one-work', 'two-work', 'owner-product', 'asym-first',
          'asym-last', 'duplicate-eight', 'mixed-add-infer']
MODES = ['Inline', 'Threads1', 'Threads2']
CELLS = [(c, m, 64, 4) for c in FINITE for m in MODES]
CELLS += [(c, m, 1, 4) for c in ['one-work', 'two-work'] for m in MODES]
CELLS += [(c, 'Specialized', 0, 0) for c in FINITE]
CELLS += [(c, m, q, 4) for c in ['stream-prefix', 'refute-loop'] for q in [1, 8] for m in MODES]
assert len(CELLS) == len(set(CELLS)) == 50
CPUS = {0, 2, 4}

def digest(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

def capacity():
    allowed = os.sched_getaffinity(0)
    assert CPUS <= allowed, (CPUS, allowed)
    topology = {}
    for cpu in sorted(CPUS):
        base = Path(f'/sys/devices/system/cpu/cpu{cpu}/topology')
        topology[str(cpu)] = {name: (base / name).read_text().strip()
                             for name in ['core_id', 'physical_package_id', 'thread_siblings_list']}
    assert len({(x['physical_package_id'], x['core_id']) for x in topology.values()}) == 3
    return {'parent_allowed': sorted(allowed), 'child_affinity': sorted(CPUS), 'topology': topology}

def cgroup_limits():
    relative = Path('/proc/self/cgroup').read_text().strip().split('::', 1)[1]
    current = Path('/sys/fs/cgroup' + relative)
    root = Path('/sys/fs/cgroup')
    result = {}
    while True:
        row = {name: (current / name).read_text().strip() if (current / name).exists() else None
               for name in ['cpu.max', 'cpuset.cpus.effective', 'memory.max']}
        result[str(current)] = row
        if row['cpu.max'] and not row['cpu.max'].startswith('max '):
            quota, period = map(int, row['cpu.max'].split())
            assert quota / period >= 3, ('insufficient shared CPU quota', current, row)
        if current == root:
            break
        assert root in current.parents
        current = current.parent
    return result

def bounds():
    os.sched_setaffinity(0, CPUS)
    assert os.sched_getaffinity(0) == CPUS
    resource.setrlimit(resource.RLIMIT_AS, (1073741824, 1073741824))

def main():
    os.chdir(ROOT)
    assert not OUT.exists(), 'refuse to overwrite evidence directory'
    hardware = capacity()
    quotas = cgroup_limits()
    OUT.mkdir()
    binaries = {}
    for kind in ['primary', 'allocation', 'work']:
        example = 'region_lifecycle_memory' if kind == 'allocation' else 'region_lifecycle'
        target = ROOT / f'target/r08-regional-{kind}'
        cmd = ['cargo', 'build', '-p', 'chr-factors', '--release', '--features', 'lifecycle',
               '--example', example, '--target-dir', str(target)]
        if kind != 'work':
            cmd += ['--no-default-features']
        with (OUT / f'{kind}-build.log').open('w') as log:
            subprocess.run(cmd, stdout=log, stderr=subprocess.STDOUT, check=True)
        binaries[kind] = target / 'release/examples' / example
    paths = [ROOT / 'Cargo.toml', ROOT / 'Cargo.lock', Path(__file__).resolve(),
             ROOT / 'research/chr-factors/experiments/analyze_regional.py',
             ROOT / 'docs/experiments/registrations/R08-regional-lifecycle.md']
    for folder in ['research/chr-factors', 'research/chr-persistent', 'research/chr-observe',
                   'research/chr-compiled', 'research/chr-cases', 'crates/chr-syntax', 'crates/chr-programs']:
        paths += [p for p in (ROOT / folder).rglob('*') if p.is_file() and p.suffix in ['.rs', '.toml']]
    paths += list(binaries.values())
    frozen = {str(p.relative_to(ROOT)): digest(p) for p in sorted(set(paths))}
    rng = random.Random(43044)
    order = []
    for kind, rep in [('warmup', 0), *[('primary', i) for i in range(5)], ('allocation', 0), ('work', 0)]:
        block = [{'kind': kind, 'rep': rep, 'cell': list(c)} for c in CELLS]
        rng.shuffle(block)
        order.extend(block)
    meta = {'created_utc': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()),
            'hardware': hardware, 'ancestor_limits': quotas, 'cpuinfo': Path('/proc/cpuinfo').read_text(),
            'cgroup': {p: Path(p).read_text() if Path(p).exists() else None for p in
                       ['/proc/self/cgroup', '/sys/fs/cgroup/cpu.max', '/sys/fs/cgroup/memory.max']},
            'rustc': subprocess.check_output(['rustc', '-Vv'], text=True),
            'order': order, 'source_and_binary_hashes': frozen,
            'timeout_seconds': 30, 'address_space_bytes': 1073741824, 'batch_seconds': 1200}
    (OUT / 'metadata.json').write_text(json.dumps(meta, indent=2) + '\n')
    started = time.monotonic()
    with (OUT / 'runs.jsonl').open('w') as output:
        for i, item in enumerate(order):
            if time.monotonic() - started >= 1200:
                break
            kind = item['kind'] if item['kind'] != 'warmup' else 'primary'
            cmd = [str(binaries[kind]), *map(str, item['cell'])]
            row = dict(item)
            t0 = time.monotonic()
            try:
                p = subprocess.run(cmd, capture_output=True, text=True, timeout=30, preexec_fn=bounds)
                row.update(exit=p.returncode, stderr=p.stderr)
                if p.returncode == 0:
                    try:
                        row['result'] = json.loads(p.stdout)
                    except json.JSONDecodeError:
                        row.update(parse_error=True, stdout=p.stdout)
                else:
                    row['stdout'] = p.stdout
            except subprocess.TimeoutExpired:
                row.update(timeout=True, exit=None)
            row['process_seconds'] = time.monotonic() - t0
            output.write(json.dumps(row) + '\n')
            output.flush()
            if (i + 1) % 50 == 0:
                print(f'{i + 1}/{len(order)} processes recorded', flush=True)
            if row.get('parse_error') or row.get('exit') not in [0, None]:
                print('Stopped for process/configuration/semantic error', flush=True)
                break
    changed = [name for name, sha in frozen.items() if digest(ROOT / name) != sha]
    (OUT / 'source-check.json').write_text(json.dumps({'changed': changed}, indent=2) + '\n')
    assert not changed, changed
    print('Pilot stopped; all recorded outcomes retained and source freeze checked.', flush=True)

if __name__ == '__main__':
    main()
