"""Audit saved artifact-gate receipts without rerunning or ranking timings."""
import copy
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s01-artifact-boundary'


def validate_run(record, family, mode, size):
    assert record['exit_code'] == 0
    rows = [json.loads(line) for line in record['stdout'].splitlines()]
    assert len(rows) == 4
    header = rows[-1]
    phases = ['setup_ns', 'execute_ns', 'observation_ns', 'engine_drop_ns', 'answer_drop_ns']
    for index, query in enumerate(rows[:-1]):
        assert query['validated'] is True
        assert query['query'] == index and query['size'] == size + index % 2
        assert all(type(query[key]) is int and query[key] >= 0 for key in phases)
        assert query['query_ns'] == sum(query[key] for key in phases)
    assert header['family'] == family and header['mode'] == mode
    assert header['queries'] == 3 and header['counters'] is False
    assert header['allocator'] == 'ordinary'
    assert header['source_disposal'] == 'included in preparation'
    preparation = ['source_ns', 'prepare_ns', 'prepared_drop_ns']
    assert all(type(header[key]) is int and header[key] >= 0 for key in preparation)
    assert header['lifecycle_ns'] == sum(q['query_ns'] for q in rows[:-1]) + sum(header[k] for k in preparation)


def main():
    freeze = json.loads((OUT / 'artifacts.json').read_text())
    for path, digest in freeze['shared_rlibs'].items():
        assert hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == digest, path
    for family, artifact in freeze['artifacts'].items():
        for suffix, key in [('.rs', 'source'), ('', 'binary')]:
            data = (ROOT / 'target/s01-artifact-boundary' / (family + suffix)).read_bytes()
            assert hashlib.sha256(data).hexdigest() == artifact[key + '_sha256']
            assert len(data) == artifact[key + '_bytes']
    modes = ['generic', 'planned', 'specialized', 'native', 'native-generic-repair', 'native-planned', 'native-specialized']
    checked = 0
    for family in ['chain', 'payload', 'subscription', 'dispatch16']:
        for mode in modes:
            for size in [0, 5]:
                record = json.loads((OUT / f'run-{family}-{mode}-{size}.json').read_text())
                validate_run(record, family, mode, size)
                checked += 1
    mismatch = json.loads((OUT / 'source-mismatch.json').read_text())
    assert mismatch['exit_code'] == 2 and not mismatch['stdout']
    assert 'does not match supplied rules' in mismatch['stderr']
    # These faults test acceptance of incomplete/false accounting receipts;
    # independent source semantics are checked inside the Rust executable.
    original = json.loads((OUT / 'run-chain-generic-0.json').read_text())
    detected = []
    for fault in ['missing-disposal', 'wrong-total', 'false-validation', 'partial-output']:
        bad = copy.deepcopy(original)
        rows = [json.loads(line) for line in bad['stdout'].splitlines()]
        if fault == 'missing-disposal':
            del rows[0]['engine_drop_ns']
        elif fault == 'wrong-total':
            rows[-1]['lifecycle_ns'] += 1
        elif fault == 'false-validation':
            rows[0]['validated'] = False
        else:
            rows.pop()
        bad['stdout'] = '\n'.join(json.dumps(row) for row in rows)
        try:
            validate_run(bad, 'chain', 'generic', 0)
        except (AssertionError, KeyError):
            detected.append(fault)
        else:
            raise AssertionError(f'accepted {fault}')
    assert checked == 56
    result = dict(processes=checked, queries=checked * 3, mismatch_rejections=1,
                  accounting_faults_detected=detected, artifact_hashes_verified=True,
                  interpretation='Receipt integrity and accounting only; no performance ranking.')
    (OUT / 'receipt-audit.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result))


if __name__ == '__main__':
    main()
