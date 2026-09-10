"""Read-only independent audit of frozen nested attribution and ownership receipts."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / 'docs/experiments/results/s02-relevant-attribution'

def read(path):
    return json.loads(path.read_text())

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def semantic_memory(value, baseline):
    if isinstance(value, list):
        return [semantic_memory(item, baseline) for item in value]
    if not isinstance(value, dict):
        return value
    return {key: (item - baseline if key in {'live_start', 'live_end', 'peak_live'}
                  else semantic_memory(item, baseline))
            for key, item in value.items()
            if key not in {'ns', 'first_answer_ns', 'attribution'}}

def result(path):
    rows = [json.loads(line) for line in path.read_text().splitlines() if line.startswith('{')]
    completed = [row for row in rows if row.get('event') == 'result']
    assert len(completed) == 1, path
    return completed[0]

freeze = read(BASE / 'freeze.json')
for name, sha in freeze['sources'].items():
    assert digest(ROOT / name) == sha, name
assert digest(ROOT / freeze['binary']['path']) == freeze['binary']['sha256']
old = read(ROOT / 'docs/experiments/results/s02-relevant-ownership/results.json')
selected = {r['index']: r for r in old if r['cancel'] is None and r['tokens'] == 1
            and r['mode'] in {'contextual', 'shared', 'relevant', 'persistent-shared', 'persistent-relevant'}}
assert len(selected) == 160
summaries = {r['index']: r for r in read(BASE / 'results.json')}
assert summaries.keys() == selected.keys()
assert len(list((BASE / 'runs').glob('*.log'))) == 320
phase_names = ['equality_other', 'key', 'lookup', 'relevant_replay', 'exact_replay', 'capture', 'relevant_record', 'exact_record']
residuals = {}
for index, control in selected.items():
    pair = [result(BASE / 'runs' / f'{index}-{repeat}.log') for repeat in range(2)]
    for run in pair:
        assert run['meter'] and not run['counters']
        assert all(sample['complete'] for sample in run['samples'])
        baseline = run['source_build']['memory']['live_start']
        prior = control['result']
        assert semantic_memory(run, baseline) == semantic_memory(prior, prior['source_build']['memory']['live_start']), index
        phases = run['attribution']
        assert [p['phase'] for p in phases] == phase_names
        assert all(isinstance(v, int) and v >= 0 for p in phases for k, v in p.items() if k != 'phase')
        execution = sum(s['execute_observe']['memory']['requested_bytes'] for s in run['samples'])
        summary = summaries[index]
        assert summary['phases'] == phases and summary['execution_bytes'] == execution
        for key in ['mode', 'family', 'depth', 'queries', 'tokens', 'retention', 'requested_bytes', 'peak_excess']:
            assert summary[key] == control[key], (index, key)
        for field in ['requested_bytes', 'allocation_calls', 'deallocation_calls']:
            assert sum(p[field] for p in phases) <= sum(s['execute_observe']['memory'][field] for s in run['samples'])
        key = tuple(control[k] for k in ['family', 'depth', 'queries', 'retention'])
        residuals.setdefault(key, set()).add(execution - sum(p['requested_bytes'] for p in phases))
    assert pair[0]['attribution'] == pair[1]['attribution']
assert len(residuals) == 32 and all(len(values) == 1 for values in residuals.values())
check = [json.loads(line) for line in (BASE / 'attribution-self-check.log').read_text().splitlines()]
expected = {'equality_other': (1, 64, 1), 'key': (2, 640, 2), 'relevant_replay': (1, 256, 1)}
for phase in check[0]:
    assert tuple(phase[k] for k in ['allocation_calls', 'requested_bytes', 'deallocation_calls']) == expected.get(phase['phase'], (0, 0, 0))
assert check[1]['requested_bytes'] == 960 and check[1]['peak_live'] - check[1]['live_start'] == 576
assert check[1]['live_end'] == check[1]['live_start']
def scenario(row):
    return tuple(row[key] for key in ['family', 'depth', 'queries', 'tokens', 'retention'])
controls = {(scenario(row), row['mode']): row for row in old if row['cancel'] is None}
sensitivity = []
for mode in ['relevant', 'persistent-relevant']:
    group = [row for row in summaries.values() if row['mode'] == mode]
    for control in ['contextual', 'shared', 'persistent-shared', 'scan', 'lowered']:
        actual = optimistic = 0
        for row in group:
            key_bytes = next(p['requested_bytes'] for p in row['phases'] if p['phase'] == 'key')
            other = controls[scenario(row), control]['requested_bytes']
            actual += row['requested_bytes'] < other
            optimistic += row['requested_bytes'] - key_bytes < other
        sensitivity.append({'candidate': mode, 'control': control, 'scenarios': len(group),
                            'actual_lower': actual, 'zero_key_lower': optimistic})
print(json.dumps({'source_hashes': len(freeze['sources']), 'binary_hash': True, 'raw_processes': 320,
                  'control_cells': 160, 'matched_outside_equality_residuals': 32,
                  'known_nested_allocation_check': True, 'primary_timing': False,
                  'sensitivity': sensitivity}, indent=2))
