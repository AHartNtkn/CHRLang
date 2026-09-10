"""Audit covered-state learning against frozen eager evidence and raw outcomes."""
from pathlib import Path
from collections import Counter
import hashlib
import json

root = Path(__file__).resolve().parents[3]
raw = root / 'docs/experiments/results/s06-covered-learning'
def rows(path, prefix):
    return sorted(line for line in path.read_text().splitlines() if line.startswith(prefix + ','))
logs = [raw / f'gate-{mode}.log' for mode in ('off', 'default')]
for log in logs:
    text = log.read_text()
    assert 'FAILED' not in text and 'error:' not in text
    assert sorted(int(line.split()[3]) for line in text.splitlines() if line.startswith('test result: ok.')) == [1, 2, 9, 9, 11]
for prefix, count in [('LATE', 1920), ('REGION', 1920), ('COMMON', 8), ('COVERED', 8)]:
    assert rows(logs[0], prefix) == rows(logs[1], prefix)
    assert len(rows(logs[0], prefix)) == count
for prefix in ['REGION', 'COMMON']:
    assert rows(logs[0], prefix) == rows(root / 'docs/experiments/results/s06-learned-regions/gate-off.log', prefix)
counts = Counter()
probes = exclusions = 0
for line in rows(logs[0], 'LATE'):
    r = line.split(',')
    accepted, weight = int(r[1]), int(r[2])
    left, right, alias = int(r[6]), int(r[7]), r[8] == 'true'
    pairs = sum(bool(left & (1 << a) and right & (1 << b) and accepted & (1 << (3*a+b))) for a in range(3) for b in range(3) if not alias or a == b)
    assert int(r[15]) == pairs * weight**2
    baseline, late = int(r[10]), int(r[11])
    counts['lower' if late < baseline else 'same' if late == baseline else 'higher'] += 1
    if late < baseline and pairs:
        counts['successful_lower'] += 1
    probes += int(r[13]); exclusions += int(r[14])
    if int(r[3]) == 0:
        assert baseline == late and int(r[13]) == 0
assert dict(counts) == {'same': 1622, 'lower': 298, 'successful_lower': 32}, counts
for line in rows(logs[0], 'COVERED'):
    _, depth, weight, steps, checks, removed = line.split(',')
    assert (int(steps), int(checks), int(removed)) == (int(depth)+9, int(depth)+10, 1)
assert 'Finished' in (raw / 'clippy.log').read_text() and 'error:' not in (raw / 'clippy.log').read_text()
files = logs + [raw/'clippy.log', Path(__file__), root/'research/chr-compiled/experiments/finite_learning.rs', root/'research/chr-direct-conditional/tests/finite_learning_gate.rs', root/'docs/experiments/registrations/S06-covered-learning.md']
result = {'changed_queries_per_build': 1920, 'counts': dict(counts), 'query_probes': probes, 'discarded_states': exclusions, 'sha256': {str(p.relative_to(root)): hashlib.sha256(p.read_bytes()).hexdigest() for p in files}}
(raw/'audit.json').write_text(json.dumps(result, indent=2)+'\n')
print(json.dumps({k:v for k,v in result.items() if k != 'sha256'}))
