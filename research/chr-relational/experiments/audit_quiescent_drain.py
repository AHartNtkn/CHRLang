"""Check bounded drain receipts against the registered semantic/work claims."""
import hashlib
import json
import re
import zipfile
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s02-quiescent-drain'


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
        assert receipt['exit_code'] == 0 and receipt['command'] == frozen['command']
        output = receipt['stdout']
        assert 'test result: ok. 14 passed; 0 failed' in output
        rows = re.findall(r'DRAIN[^\n]+', output)
        assert Counter(row.split(',')[0] for row in rows) == {
            'DRAIN_SOURCE': 351, 'DRAIN': 216, 'DRAIN_CANCEL': 9, 'DRAIN_BRANCH': 3,
        }
        work_rows = re.findall(
            r'DRAIN,depth=(\d+),possible=(true|false),shared=(true|false),'
            r'outcome=(success|fail|clash),tokens=(\d+),budget=(\d+),turns=(\d+),work=(\d+)', output)
        assert len(work_rows) == 216
        assert len({tuple(row[:6]) for row in work_rows}) == 216
        for d, possible, shared, outcome, tokens, budget, turns, work in work_rows:
            d, budget, turns, work = map(int, (d, budget, turns, work))
            if shared == 'false':
                assert work == (4 if outcome == 'fail' else d + 4)
                assert turns == (8 if outcome == 'fail' else 8 + (d + budget - 1) // budget)
            else:
                assert work == d + (4 if outcome == 'success' else 3)
        assert len(re.findall('READY_', output)) == 120
        assert len(re.findall(r'depth=\d+ deep=', output)) == 12
        assert len([s for s in output.splitlines() if 'PRIORITY,' in s and 'READY_' not in s]) == 24
        repeated.append(rows)
    assert repeated[0] == repeated[1]
    mutations = json.loads((OUT / 'mutation-audit.json').read_text())
    assert mutations['source_restored_sha256'] == frozen['sources']['research/chr-relational/src/execute.rs']
    assert len(mutations['rejected']) == 2
    for receipt in mutations['rejected']:
        assert receipt['exit_code'] == 101
        log = (OUT / f"mutation-{receipt['mutation']}.log").read_text()
        assert receipt['assertion'] in log and 'test result: FAILED' in log and 'error[E' not in log
    print('Verified 2 frozen 14-test confirmations; 351 source rows, 216 paired work rows, 12 lifecycle/service rows, and both mutations')


if __name__ == '__main__':
    main()
