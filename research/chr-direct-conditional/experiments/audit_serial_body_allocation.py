#!/usr/bin/env python3
"""Independently replay registered cells, phase ownership and allocation contrasts."""
import csv
import hashlib
import itertools
import json
from pathlib import Path
import random
import sys

root = Path(sys.argv[1])
cells = list(itertools.product(
    ['general', 'serial'], ['conditional', 'inferred'],
    ['history', 'choices', 'mixed_same'], [0, 3], [1, 4], ['query']))
expected_order = [(r, *cell) for r in range(2) for cell in cells]
random.Random(7850).shuffle(expected_order)
# The driver enumerates build/mode pairs first, exactly as this product does.
assert json.loads((root / 'meter-order.json').read_text()) == [list(x) for x in expected_order]
assert len(list((root / 'meter').glob('*.json'))) == 96
for name, digest in json.loads((root / 'freeze.json').read_text()).items():
    assert hashlib.sha256(Path(name).read_bytes()).hexdigest() == digest, name
rows = []
records = {}
for build, mode, family, n, reuse, consumer in cells:
    names = ['prepare'] + [
        'input', 'setup', 'first-delivery-or-exhaustion', 'remaining-delivery',
        'search-drop', 'answer-policy', 'input-drop'] * reuse + [
        'cancel-input', 'cancel-setup', 'cancel-unit', 'cancel-search-drop',
        'cancel-input-drop'] * 3 + ['prepared-drop', 'retained-answers-drop']
    repeats = []
    cancellations = []
    for repetition in range(2):
        name = f'{repetition}-{build}-{mode}-{family}-{n}-{reuse}-{consumer}.json'
        record = json.loads((root / 'meter' / name).read_text())
        assert tuple(record[k] for k in ['build', 'mode', 'family', 'n', 'reuse', 'consumer']) == (build, mode, family, n, reuse, consumer)
        assert record['meter'] is True
        assert [x['phase'] for x in record['phases']] == names
        assert [x['steps'] for x in record['cancellation']] == [1, 64, 256]
        cancellations.append(record['cancellation'])
        ms = [x['memory'] for x in record['phases']]
        assert all(a['live_end'] == b['live_start'] for a, b in zip(ms, ms[1:]))
        assert ms[0]['live_start'] == ms[-1]['live_end']
        for i in range(reuse):
            assert ms[7 + 7*i]['live_end'] == ms[0]['live_end']
        for i in range(3):
            assert ms[5 + 7*reuse + 5*i]['live_end'] == ms[0]['live_end']
        assert ms[-2]['live_end'] == ms[0]['live_start']
        repeats.append(ms)
    assert repeats[0] == repeats[1]
    assert cancellations[0] == cancellations[1]
    ms = repeats[0]
    records[build, mode, family, n, reuse] = (names, ms)
    rows.append(dict(build=build, mode=mode, family=family, n=n, reuse=reuse,
        completed_requested_bytes=sum(m['requested_bytes'] for name, m in zip(names, ms) if not name.startswith('cancel-')),
        cancellation_requested_bytes=sum(m['requested_bytes'] for name, m in zip(names, ms) if name.startswith('cancel-')),
        peak_above_start=max(m['peak_live'] for m in ms)-ms[0]['live_start']))
with (root / 'summary.csv').open('w') as handle:
    writer = csv.DictWriter(handle, fieldnames=rows[0], lineterminator='\n')
    writer.writeheader()
    writer.writerows(rows)
contrasts = []
for mode, family, n, reuse in itertools.product(['conditional', 'inferred'], ['history', 'choices', 'mixed_same'], [0, 3], [1, 4]):
    names, general = records['general', mode, family, n, reuse]
    _, serial = records['serial', mode, family, n, reuse]
    changes = {}
    for name, a, b in zip(names, general, serial):
        delta = b['requested_bytes'] - a['requested_bytes']
        if delta:
            changes[name] = changes.get(name, 0) + delta
    contrasts.append(dict(mode=mode, family=family, n=n, reuse=reuse,
        phase_requested_byte_changes=changes,
        completed_requested_byte_change=sum(v for k, v in changes.items() if not k.startswith('cancel-'))))
(root / 'audit.json').write_text(json.dumps(dict(processes=96, exact_pairs=48,
    phase_ownership='pass', frozen_hashes='pass', registered_order='pass',
    timing='not_run', contrasts=contrasts), indent=2) + '\n')
print('96 processes; 48 exact repeats; phase ownership, registered order and frozen hashes pass')
for row in rows:
    if row['n'] == 3 and row['reuse'] == 4:
        print(row)
print('Completed traffic changes:', [(x['family'], x['mode'], x['n'], x['reuse'], x['completed_requested_byte_change']) for x in contrasts])
