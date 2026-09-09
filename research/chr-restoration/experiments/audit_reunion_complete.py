#!/usr/bin/env python3
"""Independently reconstruct expected receipts, lifecycle totals and diagnostic pairs."""
from pathlib import Path
import hashlib
import itertools
import json
import statistics

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s04-reunion-complete'
frozen = json.loads((OUT / 'freeze.json').read_text())
for rel, digest in frozen['sources'].items():
    assert hashlib.sha256((ROOT / rel).read_bytes()).hexdigest() == digest, rel
for b in frozen['binaries'].values():
    assert hashlib.sha256(Path(b['path']).read_bytes()).hexdigest() == b['sha256']
summary = {tuple(x['cell']): x for x in json.loads((OUT / 'summary.json').read_text())}
modes = ['copy', 'reunion', 'scan', 'indexed', 'specialized-scan', 'specialized-indexed',
         'short-copy', 'short-reunion', 'short-specialized-scan', 'short-specialized-indexed',
         'factored', 'indexed-cow']
expected_files = set()
count = 0
for cell in itertools.product(['plain', 'equal', 'late', 'payload'], [2, 4], [0, 12, 48], [1, 4], modes):
    family, owners, depth, reuse, mode = cell
    key = '-'.join(map(str, cell))
    samples = []
    meters = []
    for kind, reps in [('ordinary', 5), ('meter', 2)]:
        for rep in range(reps):
            name = f'{kind}-{rep}-{key}.json'; expected_files.add(name)
            receipt = json.loads((OUT / name).read_text())
            variant = kind + ('-cow' if mode == 'indexed-cow' else '')
            assert receipt['command'] == [frozen['binaries'][variant]['path'], mode.removesuffix('-cow'), family, str(owners), str(depth), str(reuse)]
            assert receipt['exit_code'] == 0 and not receipt['stderr'], name
            data = json.loads(receipt['stdout'])
            assert [data[k] for k in ['family', 'owners', 'depth', 'reuse', 'mode']] == [family, owners, depth, reuse, mode.removesuffix('-cow')]
            assert data['meter'] == (kind == 'meter') and data['cow'] == (mode == 'indexed-cow')
            blocks = ['input', 'lower', 'setup', 'lowered-drop', 'execute', 'engine-drop', 'answers-drop', 'input-drop']
            names = ['prepare'] + blocks * reuse + ['cancel-input', 'cancel-lower', 'cancel-setup', 'cancel-lowered-drop', 'cancel-first', 'cancel-engine-drop', 'cancel-answers-drop', 'cancel-input-drop', 'prepared-drop']
            assert [p['phase'] for p in data['phases']] == names
            assert len(data['first_ns']) == reuse + 1
            for i, t in enumerate(data['first_ns']):
                assert 0 < t <= data['phases'][1 + 8 * i + 4]['ns']
            if kind == 'ordinary':
                assert all(p['memory'] is None for p in data['phases'])
                samples.append(sum(p['ns'] for p in data['phases'][:1 + 8 * reuse]) + data['phases'][-1]['ns'])
            else:
                memory = [p['memory'] for p in data['phases']]
                assert all(a['live_end'] == b['live_start'] for a, b in zip(memory, memory[1:]))
                assert memory[-1]['live_end'] == memory[0]['live_start']
                assert all(memory[8 * (i + 1)]['live_end'] == memory[0]['live_end'] for i in range(reuse + 1))
                meters.append(memory)
            count += 1
    assert meters[0] == meters[1]
    assert summary[cell]['total_ns'] == dict(median=statistics.median(samples), minimum=min(samples), maximum=max(samples))
    full = meters[0][:1 + 8 * reuse] + meters[0][-1:]
    assert summary[cell]['requested_bytes'] == sum(m['requested_bytes'] for m in full)
    assert summary[cell]['peak_growth'] == max(m['peak_live'] for m in full) - full[0]['live_start']
assert expected_files == {p.name for p in OUT.glob('*.json') if p.name.startswith(('ordinary-', 'meter-'))}
assert count == 4032 and len(summary) == 576
(OUT / 'independent-audit.json').write_text(json.dumps(dict(processes=count, configurations=len(summary),
    exact_memory_pairs=576, source_and_binary_hashes=True, commands_and_phases=True,
    totals_reconstructed=True, live_continuity_and_disposal=True), indent=2) + '\n')
print('Independent audit passes: 4032 receipts, 576 configurations and exact allocation pairs.')
