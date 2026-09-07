"""Bounded exploratory E16 pilot; run after release builds and all validation."""
import csv
import hashlib
import io
import json
import os
from pathlib import Path
import platform
import resource
import signal
import subprocess
import time
import sys

OUT = Path('docs/experiments/results')
DISTINCT = sys.argv[1:] == ['--distinct']
assert not sys.argv[1:] or DISTINCT
PREFIX = 'E16-distinct-pilot' if DISTINCT else 'E16-pilot'
IDS = ['wide-cheap', 'wide-medium', 'wide-large', 'wide-identity', 'chain-large',
       'skew-first', 'skew-last', 'mixed', 'prefix-drain',
       'app-sk-duplication', 'app-type-synthesis-prefix']
MODES = [('Shared', 0), ('Owned', 0), ('Inline', 4), ('Threads1', 4), ('Threads2', 4)]
CELLS = [(case, mode, limit) for case in IDS for mode, limit in MODES]
CELLS += [('wide-large', mode, 1) for mode in ['Inline', 'Threads1', 'Threads2']]
if DISTINCT:
    CELLS = [(f'distinct-{shape}-d-{d}', mode, limit) for d in [4, 6, 8] for shape in ['wide', 'chain'] for mode, limit in MODES]

def digest(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def bounds(): resource.setrlimit(resource.RLIMIT_AS, (1073741824, 1073741824))
paths = [Path('Cargo.toml'), Path('Cargo.lock'), Path(__file__), Path(f'docs/experiments/registrations/{PREFIX}.md')]
for folder in ['research/chr-reuse', 'research/chr-persistent', 'research/chr-cases', 'research/chr-observe', 'crates/chr-syntax', 'crates/chr-programs']:
    paths += [p for p in Path(folder).rglob('*') if p.is_file() and p.suffix in ['.rs', '.toml']]
paths += [Path('target/release/examples') / exe for exe in ['parallel_cost', 'parallel_memory']]
cgroup = {}
for name in ['/proc/self/cgroup', '/sys/fs/cgroup/cpu.max', '/sys/fs/cgroup/memory.max']:
    p = Path(name)
    cgroup[name] = p.read_text() if p.exists() else 'unavailable at this path'
manifest = {
    'purpose': 'exploratory sizing and lifecycle pilot; no confirmatory performance ranking',
    'created_utc': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()),
    'platform': platform.platform(), 'affinity': sorted(os.sched_getaffinity(0)),
    'cpuinfo': Path('/proc/cpuinfo').read_text(), 'cgroup': cgroup,
    'rustc': subprocess.check_output(['rustc', '-Vv'], text=True),
    'RUST_MIN_STACK': os.environ.get('RUST_MIN_STACK', 'unset; library default'),
    'timeout_seconds': 30, 'rlimit_as_bytes': 1073741824,
    'order': [{'kind': kind, 'case': case, 'mode': mode, 'limit': limit}
              for kind in ['time', 'memory'] for case, mode, limit in CELLS],
    'sha256': {str(p): digest(p) for p in sorted(set(paths))},
}
(OUT / f'{PREFIX}-manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
with (OUT / f'{PREFIX}.jsonl').open('w') as log:
    for i, cell in enumerate(manifest['order']):
        exe = 'parallel_cost' if cell['kind'] == 'time' else 'parallel_memory'
        rss = Path('/tmp') / f'chr-e16-rss-{os.getpid()}.txt'
        command = ['/usr/bin/time', '-f', '%M', '-o', str(rss),
                   f'target/release/examples/{exe}', cell['case'], cell['mode'], str(cell['limit'])]
        if rss.exists(): rss.unlink()
        p = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, preexec_fn=bounds, start_new_session=True)
        try:
            stdout, stderr = p.communicate(timeout=30)
            result = {**cell, 'exit': p.returncode, 'stdout': stdout, 'stderr': stderr}
            if p.returncode == 0:
                try:
                    rows = list(csv.DictReader(io.StringIO(stdout), delimiter='\t'))
                    if len(rows) != 1 or None in rows[0] or any(v is None for v in rows[0].values()):
                        raise ValueError('expected one complete TSV row')
                    result['measurement'] = rows[0]
                except (ValueError, csv.Error) as error:
                    result['parse_error'] = str(error)
        except subprocess.TimeoutExpired:
            os.killpg(p.pid, signal.SIGKILL)
            stdout, stderr = p.communicate()
            result = {**cell, 'exit': 'timeout', 'stdout': stdout, 'stderr': stderr}
        result['rss_report'] = rss.read_text() if rss.exists() else 'unavailable'
        log.write(json.dumps(result) + '\n'); log.flush()
        print(i + 1, '/' + str(len(manifest['order'])), cell['kind'], cell['case'], cell['mode'], result['exit'], flush=True)
assert all(digest(Path(p)) == value for p, value in manifest['sha256'].items())
