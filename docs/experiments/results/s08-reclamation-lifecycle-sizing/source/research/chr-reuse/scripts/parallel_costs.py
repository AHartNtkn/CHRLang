"""Registered E16 comparative runner. Build/validate separately before executing.

Writes the complete randomized order and source hashes before launching any cell.
Never rebuilds binaries, overwrites previous evidence, or continues past a failed
child. Timing and metered runs use their separate existing executables.
"""
import csv
import hashlib
import io
import json
import os
from pathlib import Path
import platform
import random
import resource
import signal
import subprocess
import sys
import tempfile
import time

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results'
PREFIX = 'E16-costs'
SEED = 160916
TIMEOUT = 30
ADDRESS_SPACE = 1073741824
BASE_IDS = [
    'wide-cheap', 'wide-medium', 'wide-large', 'wide-identity', 'chain-large',
    'skew-first', 'skew-last', 'mixed', 'prefix-drain',
    'app-sk-duplication', 'app-type-synthesis-prefix',
]
IDS = BASE_IDS + [f'distinct-{shape}-d-{depth}'
                  for depth in [4, 6, 8] for shape in ['wide', 'chain']]
MODES = [('Shared', 0), ('Owned', 0), ('Inline', 4),
         ('Threads1', 4), ('Threads2', 4)]
CELLS = [(case, mode, limit) for case in IDS for mode, limit in MODES]
CELLS += [(case, mode, 1) for case in ['wide-large', 'distinct-wide-d-8']
          for mode in ['Inline', 'Threads1', 'Threads2']]


def order():
    """Each repetition shuffles a fresh 91-cell list using the seeded PRNG."""
    rng = random.Random(SEED)
    result = []
    for phase, kind, repetitions in [
            ('warmup', 'time', 2),
            ('time', 'time', 7),
            ('memory', 'memory', 3)]:
        for rep in range(repetitions):
            cells = list(CELLS)
            rng.shuffle(cells)
            for case, mode, limit in cells:
                result.append(dict(ordinal=len(result) + 1, phase=phase,
                                   kind=kind, rep=rep, case=case, mode=mode,
                                   limit=limit))
    assert len(CELLS) == len(set(CELLS)) == 91
    assert len(result) == 1092
    return result


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def source_paths():
    paths = {ROOT / 'Cargo.toml', ROOT / 'Cargo.lock', Path(__file__).resolve(),
             ROOT / 'docs/experiments/registrations/E16-costs.md',
             ROOT / 'research/chr-reuse/scripts/audit_parallel_costs.py'}
    for folder in ['research/chr-reuse', 'research/chr-persistent',
                   'research/chr-cases', 'research/chr-observe',
                   'crates/chr-reference', 'crates/chr-syntax',
                   'crates/chr-programs']:
        paths.update(p for p in (ROOT / folder).rglob('*')
                     if p.is_file() and p.suffix in ['.rs', '.toml'])
    paths.update(ROOT / 'target/release/examples' / exe
                 for exe in ['parallel_cost', 'parallel_memory'])
    return sorted(paths)


def read_if_available(name):
    try:
        return Path(name).read_text()
    except OSError as error:
        return f'unavailable: {error}'


def manifest():
    # This is a measurement-relevant allowlist, not a dump of credentials or
    # unrelated environment variables. Unset values remain explicit nulls.
    environment_names = [
        'RUST_MIN_STACK', 'RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS',
        'RUSTUP_TOOLCHAIN', 'CARGO_TARGET_DIR', 'RUST_BACKTRACE',
        'CARGO_PROFILE_RELEASE_OPT_LEVEL', 'CARGO_PROFILE_RELEASE_LTO',
        'CARGO_PROFILE_RELEASE_CODEGEN_UNITS', 'CARGO_PROFILE_RELEASE_DEBUG',
        'MALLOC_ARENA_MAX', 'MALLOC_CONF', 'GLIBC_TUNABLES', 'LD_PRELOAD',
        'OMP_NUM_THREADS', 'LANG', 'LC_ALL', 'LC_CTYPE', 'TZ',
    ]
    cgroup_names = ['/proc/self/cgroup', '/proc/self/mountinfo',
                    '/sys/fs/cgroup/cpu.max', '/sys/fs/cgroup/cpu.stat',
                    '/sys/fs/cgroup/cpuset.cpus.effective',
                    '/sys/fs/cgroup/memory.max', '/sys/fs/cgroup/memory.current']
    # Also capture the process's actual unified-cgroup directory where visible.
    for line in read_if_available('/proc/self/cgroup').splitlines():
        if line.startswith('0::'):
            group = Path('/sys/fs/cgroup') / line[3:].lstrip('/')
            cgroup_names.extend(str(group / leaf) for leaf in [
                'cpu.max', 'cpu.stat', 'cpuset.cpus.effective',
                'memory.max', 'memory.current'])
    return {
        'purpose': 'registered E16 comparative lifecycle costs; warmups excluded from estimates',
        'created_utc': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()),
        'platform': platform.platform(), 'python': sys.version,
        'affinity': sorted(os.sched_getaffinity(0)),
        'cpuinfo': read_if_available('/proc/cpuinfo'),
        'meminfo': read_if_available('/proc/meminfo'),
        'cgroup': {name: read_if_available(name) for name in sorted(set(cgroup_names))},
        'environment': {name: os.environ.get(name) for name in environment_names},
        'rustc': subprocess.check_output(['rustc', '-Vv'], text=True),
        'git_head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
        'git_status': subprocess.check_output(['git', 'status', '--porcelain'], text=True),
        'timeout_seconds': TIMEOUT, 'rlimit_as_bytes': ADDRESS_SPACE,
        'rss_unit': 'KiB, GNU time process maximum RSS; includes workload setup',
        'request_capacity': 'outstanding limit', 'reply_capacity': 1,
        'seed': SEED,
        'shuffle': 'fresh cell list per repetition, sequential draws from random.Random(seed)',
        'order': order(),
        'sha256': {str(p.relative_to(ROOT)): digest(p) for p in source_paths()},
    }


def limits():
    resource.setrlimit(resource.RLIMIT_AS, (ADDRESS_SPACE, ADDRESS_SPACE))


def durable(stream, value):
    stream.write(json.dumps(value) + '\n')
    stream.flush()
    os.fsync(stream.fileno())


def changed_sources(frozen):
    return [name for name, expected in frozen.items()
            if not (ROOT / name).is_file() or digest(ROOT / name) != expected]


def measurement(stdout, cell):
    reader = csv.DictReader(io.StringIO(stdout), delimiter='\t')
    rows = list(reader)
    names = reader.fieldnames
    if (not names or len(names) != 125 or len(set(names)) != len(names)
            or len(rows) != 1 or None in rows[0]
            or any(value is None for value in rows[0].values())):
        raise ValueError('expected one complete TSV row with 125 unique columns')
    row = rows[0]
    if (row['case'] != cell['case'] or row['mode'] != cell['mode']
            or int(row['outstanding_limit']) != cell['limit']
            or row['metered'] != str(cell['kind'] == 'memory').lower()
            or row['pass'] != 'true'):
        raise ValueError('measurement identity, metering mode, or pass status differs')
    booleans = {'metered', 'pass', 'exhausted', 'has_first'}
    for name, value in row.items():
        if name in booleans:
            if value not in {'true', 'false'}:
                raise ValueError(f'invalid Boolean field {name}')
        elif name not in {'case', 'mode'} and int(value) < 0:
            raise ValueError(f'negative numeric field {name}')
    return row


def run_child(cell, directory):
    exe = 'parallel_cost' if cell['kind'] == 'time' else 'parallel_memory'
    rss = directory / 'rss.txt'
    if rss.exists():
        rss.unlink()
    command = ['/usr/bin/time', '-f', '%M', '-o', str(rss),
               f'target/release/examples/{exe}', cell['case'], cell['mode'],
               str(cell['limit'])]
    result = {**cell, 'command': command}
    started = time.monotonic()
    try:
        child = subprocess.Popen(command, cwd=ROOT, stdout=subprocess.PIPE,
                                 stderr=subprocess.PIPE, text=True, errors='replace',
                                 preexec_fn=limits, start_new_session=True)
    except (OSError, subprocess.SubprocessError) as error:
        result.update(exit='launch-error', stdout='', stderr='', launch_error=str(error))
    else:
        try:
            stdout, stderr = child.communicate(timeout=TIMEOUT)
            result.update(exit=child.returncode, stdout=stdout, stderr=stderr)
        except subprocess.TimeoutExpired:
            # Kill GNU time and its complete child process group, then collect
            # remaining output before preserving the failed row and stopping.
            try:
                os.killpg(child.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            stdout, stderr = child.communicate()
            result.update(exit='timeout', stdout=stdout, stderr=stderr)
    result['process_wall_seconds'] = time.monotonic() - started
    result['rss_report'] = rss.read_text() if rss.exists() else 'unavailable'
    if result['exit'] == 0:
        try:
            result['measurement'] = measurement(result['stdout'], cell)
            result['peak_rss_kib'] = int(result['rss_report'].strip())
        except (ValueError, KeyError, csv.Error) as error:
            result['parse_error'] = str(error)
    return result


def main():
    if len(sys.argv) != 1:
        raise SystemExit('parallel_costs.py takes no arguments; matrix/order are registered')
    os.chdir(ROOT)
    OUT.mkdir(parents=True, exist_ok=True)
    manifest_path = OUT / f'{PREFIX}-manifest.json'
    log_path = OUT / f'{PREFIX}.jsonl'
    if manifest_path.exists() or log_path.exists():
        raise SystemExit('E16 costs evidence already exists; preserve it before a separately recorded retry')
    frozen = manifest()
    with manifest_path.open('x') as stream:
        stream.write(json.dumps(frozen, indent=2) + '\n')
        stream.flush()
        os.fsync(stream.fileno())
    with log_path.open('x') as log, tempfile.TemporaryDirectory(prefix='chr-e16-costs-') as directory:
        previous_rep = None
        for cell in frozen['order']:
            repetition = (cell['phase'], cell['rep'])
            if repetition != previous_rep:
                changed = changed_sources(frozen['sha256'])
                if changed:
                    durable(log, {**cell, 'exit': 'frozen-input-change', 'changed_sources': changed})
                    raise SystemExit('frozen inputs changed; diagnostic row preserved')
                previous_rep = repetition
            result = run_child(cell, Path(directory))
            durable(log, result)
            if result['exit'] != 0 or 'parse_error' in result:
                print(f"Stopped after durable failed row {cell['ordinal']}: {cell}", flush=True)
                raise SystemExit(1)
            if cell['ordinal'] % 25 == 0 or cell['ordinal'] == len(frozen['order']):
                print(cell['ordinal'], '/', len(frozen['order']), cell['phase'],
                      'rep', cell['rep'], cell['case'], cell['mode'], flush=True)
        changed = changed_sources(frozen['sha256'])
        if changed:
            durable(log, {'exit': 'frozen-input-change', 'changed_sources': changed, 'after_batch': True})
            raise SystemExit('frozen inputs changed during final repetition; diagnostic row preserved')


if __name__ == '__main__':
    main()
