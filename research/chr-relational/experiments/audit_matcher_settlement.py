"""Independently check frozen readiness receipts and reported work relationships."""
import hashlib
import json
import re
import zipfile
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s02-matcher-settlement'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    frozen = json.loads((OUT / 'freeze.json').read_text())
    assert sha((OUT / 'sources.zip').read_bytes()) == frozen['archive_sha256']
    with zipfile.ZipFile(OUT / 'sources.zip') as archive:
        for path, digest in frozen['sources'].items():
            assert sha(archive.read(path)) == digest
    repeated = []
    for rep in range(2):
        receipt = json.loads((OUT / f'confirmation-{rep}.json').read_text())
        assert receipt['exit_code'] == 0
        assert receipt['command'] == frozen['command']
        output = receipt['stdout']
        assert 'test result: ok. 12 passed; 0 failed' in output
        rows = re.findall(r'READY_[^\n]+', output)
        assert Counter(row.split(',')[0] for row in rows) == {
            'READY_PRIORITY': 24, 'READY_SEPARATE': 72, 'READY_READ': 9,
            'READY_RESOURCE': 12, 'READY_CANCEL': 3,
        }
        separate = re.findall(
            r'READY_SEPARATE,depth=(\d+),possible=(true|false),shared=(true|false),'
            r'outcome=(success|fail|clash),tokens=(\d+),background=(\d+),selected=(\d+),'
            r'full=(\d+),pending=(\[[^\]]*\]),turns=(\d+),full_turns=(\d+)', output)
        assert len(separate) == 72
        keys = set()
        for d, possible, shared, outcome, tokens, bg, selected, full, pending, turns, ft in separate:
            keys.add((d, possible, shared, outcome, tokens))
            d, bg, selected, full, turns, ft = map(int, (d, bg, selected, full, turns, ft))
            assert full == d + (4 if outcome == 'success' else 3)
            assert ft == {'success': 9, 'fail': 8, 'clash': 7}[outcome]
            if shared == 'false':
                assert pending == '[1]' and selected == 0
                assert bg == (4 if outcome == 'fail' else d + 4)
                assert turns == (8 if outcome == 'fail' else d + 8)
            else:
                assert bg + selected == full and turns == ft
                assert pending == ('[]' if outcome == 'clash' else '[0]')
        assert len(keys) == 72
        controls = re.findall(r'depth=(\d+) deep=(true|false) fail=(true|false) interleaved=(\d+) settled=(\d+)', output)
        assert len(controls) == 12
        for d, deep, fail, eager, settled in controls:
            assert int(settled) == int(d) + 1
            assert int(eager) == (2 if deep == 'false' and fail == 'true' else int(d) + 1)
        priority = [line[line.index('PRIORITY,'):] for line in output.splitlines()
                    if 'PRIORITY,' in line and 'READY_' not in line]
        assert len(priority) == 24
        repeated.append((rows, controls, priority))
    assert repeated[0] == repeated[1]
    mutations = json.loads((OUT / 'mutation-audit.json').read_text())
    assert mutations['source_restored_sha256'] == frozen['sources']['research/chr-relational/src/store.rs']
    assert {r['omitted'] for r in mutations['rejected']} == {'guard', 'repeated', 'constructor'}
    for result in mutations['rejected']:
        assert result['exit_code'] == 101
        log = (OUT / f"mutation-{result['omitted']}.log").read_text()
        assert 'test result: FAILED' in log and 'error[E' not in log
    print('Verified frozen sources, 2 × 120 readiness cases, established controls, work/turn formulas, and 3 semantic mutations')


if __name__ == '__main__':
    main()
