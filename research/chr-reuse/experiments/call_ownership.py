#!/usr/bin/env python3
"""Replay the registered complete-caller live-heap ownership diagnostic."""
import hashlib
import json
import resource
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s05-caller-ownership-gate'
BINARY = ROOT / 'target/release/examples/call_ownership'
OUT.mkdir(exist_ok=True)
SOURCES = [
    'research/chr-reuse/src/calls.rs',
    'research/chr-persistent/src/state.rs',
    'research/chr-persistent/src/continuations.rs',
    'research/chr-reuse/examples/call_ownership.rs',
    'research/chr-reuse/experiments/call_ownership.py',
    'research/chr-reuse/Cargo.toml',
    'Cargo.lock',
]
(OUT / 'freeze.sha256').write_text(''.join(
    f'{hashlib.sha256(path.read_bytes()).hexdigest()}  {path.relative_to(ROOT)}\n'
    for path in [*(ROOT / p for p in SOURCES), BINARY]
))
(OUT / 'toolchain.txt').write_text(subprocess.check_output(['rustc', '-Vv'], text=True))
(OUT / 'source-head.txt').write_text(subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True))
(OUT / 'source.patch').write_bytes(subprocess.check_output(['git', 'diff', '--binary', '--', 'research/chr-persistent', 'research/chr-reuse'], cwd=ROOT))


def limits():
    resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))


for iteration in (1, 2):
    with (OUT / f'run-{iteration}.jsonl').open('w') as stdout, (OUT / f'run-{iteration}.stderr').open('w') as stderr:
        result = subprocess.run([str(BINARY)], cwd=ROOT, stdout=stdout, stderr=stderr, timeout=60, preexec_fn=limits)
    assert result.returncode == 0, (iteration, result.returncode)
rows = [json.loads(line) for line in (OUT / 'run-1.jsonl').read_text().splitlines()]
replay = [json.loads(line) for line in (OUT / 'run-2.jsonl').read_text().splitlines()]
assert len(rows) == 16 and rows == replay
assert {(r['memo'], r['depth'], r['cancel']) for r in rows} == {(m, d, c) for m in (False, True) for d in (0, 8) for c in range(4)}
for row in rows:
    assert row['baseline'] == row['disposed']
    assert row['released'][0] == row['released'][1]
    assert row['answers'] == [[0, 0], [0, 0], [1, 1], [2, 2]][row['cancel']]
    if not row['memo']:
        assert row['released'][0] == row['prepared']
(OUT / 'audit.json').write_text(json.dumps({
    'configurations': 16, 'queries_per_process': 32, 'exact_replay': True,
    'disposal_baselines': True, 'repeated_query_retention_stable': True,
    'timeout_seconds': 60, 'address_space_bytes': 1 << 30,
}, indent=2) + '\n')
print('16 configurations, 32 queries per process; exact replay and all owner baselines pass')
