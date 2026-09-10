"""Audit the bounded equality/backlog comparison and its frozen inputs."""
from pathlib import Path
import hashlib
import json
import re

ROOT = Path(__file__).resolve().parents[3]
RAW = ROOT / 'docs/experiments/results/s08-equality-overlap'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def rows(receipt):
    return [json.loads(line) for line in receipt['stdout'].splitlines()
            if line.startswith('{')]


def audit():
    resolution = json.loads((RAW / 'resolved-inputs.json').read_text())
    inputs = 0
    for manifest, mapping in resolution.items():
        expected = json.loads((ROOT / manifest).read_text())
        assert set(expected) == set(mapping)
        for original, actual in mapping.items():
            assert digest(ROOT / actual) == expected[original], (manifest, original)
            inputs += 1
    modes = ['base', 'selective', 'reverse', 'shortcut', 'shortcut-reverse',
             'backlog-base', 'backlog-shortcut', 'lead-forward', 'lead-reverse']
    outcomes = {}
    cutoffs = 0
    for mode in modes:
        pair = [json.loads((RAW / f'{mode}-{rep}.json').read_text()) for rep in range(2)]
        assert pair[0]['stdout'] == pair[1]['stdout']
        for r in pair:
            cutoff = mode in ['shortcut-reverse', 'backlog-shortcut']
            assert r['returncode'] == (101 if cutoff else 0)
            if cutoff:
                assert 'service cutoff after 116' in r['stderr']
                cutoffs += 1
            else:
                assert not r['stderr']
            rs = rows(r)
            assert [v['answers'] for v in rs] == ([1, 8, 32, 64] if cutoff else [1, 8, 32, 64, 128])
            for v in rs:
                assert sum(v['stages']) == v['calls']
                assert len(v['counts']) == 7
                assert all(len(c) == 6 for c in v['counts'])
            if mode.startswith('lead-'):
                pending = re.findall(r'pending=(\d+)', r['stdout'])
                assert len(pending) == 5 and max(map(int, pending)) <= 2
        outcomes[mode] = rows(pair[0])
    assert outcomes['backlog-base'] == outcomes['reverse']
    assert outcomes['backlog-shortcut'] == outcomes['shortcut-reverse']
    assert outcomes['lead-forward'] == outcomes['shortcut']
    expected_calls = {'base': 3198109, 'selective': 3198239, 'reverse': 4606799,
                      'shortcut': 2116705, 'lead-forward': 2116705, 'lead-reverse': 3488701}
    for mode, calls in expected_calls.items():
        assert outcomes[mode][-1]['calls'] == calls
    tests = {}
    for prefix, executables, expected_count in [('test-', 6, 33), ('lead-test-', 7, 39), ('default-test-', 6, 33)]:
        logs = list(RAW.glob(prefix + '*.log'))
        count = 0
        for p in logs:
            # Build stderr is a separate receipt, not a test executable.
            if p.name == 'test-build.log':
                continue
            result = re.search(r'test result: ok\. (\d+) passed; 0 failed;', p.read_text())
            assert result, p
            count += int(result[1])
        actual = len([p for p in logs if p.name != 'test-build.log'])
        assert (actual, count) == (executables, expected_count), (prefix, actual, count)
        tests[prefix] = dict(executables=actual, passed=count)
    for name in ['clippy.log', 'lead-clippy.log']:
        s = (RAW / name).read_text()
        assert 'Finished' in s and 'error:' not in s
    return dict(manifests=len(resolution), verified_inputs=inputs, diagnostic_processes=18,
                completed_processes=14, expected_cutoffs=cutoffs, tests=tests,
                calls_at_128=expected_calls, claim='Logical service counts; no timing or ownership ranking')


if __name__ == '__main__':
    print(json.dumps(audit(), indent=2))
