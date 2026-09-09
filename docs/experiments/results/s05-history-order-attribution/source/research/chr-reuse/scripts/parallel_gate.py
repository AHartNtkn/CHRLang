"""Run and audit the E16 semantic gate from the repository root (no timing claims)."""
import collections
import csv
import hashlib
import json
import os
import pathlib
import platform
import resource
import subprocess
import time

OUT = pathlib.Path('docs/experiments/results')
PREFIX = 'E16-gate-v2'
COMMAND = ['target/debug/examples/parallel_gate']

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def bounds():
    resource.setrlimit(resource.RLIMIT_AS, (1073741824, 1073741824))

paths = [pathlib.Path('Cargo.lock'), pathlib.Path('docs/experiments/registrations/E16.md'), pathlib.Path(__file__)]
for folder in ['research/chr-reuse', 'research/chr-persistent', 'research/chr-cases', 'research/chr-observe', 'crates/chr-reference', 'crates/chr-syntax', 'crates/chr-programs']:
    paths.extend(p for p in pathlib.Path(folder).rglob('*') if p.is_file() and p.suffix in ['.rs', '.toml'])
manifest = {
    'purpose': 'E16 semantic gate; no performance inference',
    'created_utc': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()),
    'platform': platform.platform(), 'affinity': sorted(os.sched_getaffinity(0)),
    'rustc': subprocess.check_output(['rustc', '-Vv'], text=True),
    'timeout_seconds': 300, 'rlimit_as_bytes': 1073741824, 'command': COMMAND,
    'sha256': {str(p): digest(p) for p in sorted(set(paths))},
}
(OUT / f'{PREFIX}-manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
for suffix in ['', '-replay']:
    with (OUT / f'{PREFIX}{suffix}.tsv').open('w') as stdout, (OUT / f'{PREFIX}{suffix}.stderr').open('w') as stderr:
        subprocess.run(COMMAND, stdout=stdout, stderr=stderr, timeout=300, preexec_fn=bounds, check=True)
a, b = [list(csv.DictReader((OUT / f'{PREFIX}{s}.tsv').open(), delimiter='\t')) for s in ['', '-replay']]
assert len(a) == len(b) == 512
keys = lambda rows: [(r['case'], r['mode'], r['limit']) for r in rows]
assert keys(a) == keys(b) and len(set(keys(a))) == 512
assert len(set(r['case'] for r in a)) == 64
configs = {('Owned', '0'), ('Shared', '0'), ('Inline', '1'), ('Inline', '4'), ('Threads(1)', '1'), ('Threads(1)', '4'), ('Threads(2)', '1'), ('Threads(2)', '4')}
variable = {'owner_buffered_peak', 'worker_completion'}
assert all({k: v for k, v in x.items() if k not in variable} == {k: v for k, v in y.items() if k not in variable} for x, y in zip(a, b))
fields = ['issued', 'committed_equations', 'lookahead_visits', 'projection_dereferences', 'uncommitted_at_shutdown', 'calls', 'pairs', 'resolve_nodes', 'occurs_nodes']
for case in set(r['case'] for r in a):
    assert {(r['mode'], r['limit']) for r in a if r['case'] == case} == configs
    for limit in ['1', '4']:
        group = [r for r in a if r['case'] == case and r['limit'] == limit]
        assert len({tuple(r[f] for f in fields) for r in group}) == 1
assert all(r['status'] == 'pass' for r in a + b)
summary = {
    'rows_per_run': 512, 'cases': 64,
    'configurations': dict(collections.Counter(r['mode'] + '/' + r['limit'] for r in a)),
    'status': 'all pass',
    'independent_reference': 'answers, raw completions, failed branches and exhaustion checked',
    'deterministic_replay': 'all fields except declared scheduling-dependent columns identical',
    'new_driver_aggregate_work': 'identical at matched case and outstanding limit',
    'variable_columns': sorted(variable),
    'changed_rows': {k: sum(x[k] != y[k] for x, y in zip(a, b)) for k in sorted(variable)},
    'max_outstanding': max(int(r['max_outstanding']) for r in a),
    'max_uncommitted_at_shutdown': max(int(r['uncommitted_at_shutdown']) for r in a),
    'sha256': {f: digest(OUT / f) for f in [f'{PREFIX}.tsv', f'{PREFIX}-replay.tsv', f'{PREFIX}-manifest.json']},
}
assert all(digest(pathlib.Path(p)) == value for p, value in manifest['sha256'].items())
(OUT / f'{PREFIX}-audit.json').write_text(json.dumps(summary, indent=2) + '\n')
print(json.dumps(summary, indent=2))
