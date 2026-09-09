#!/usr/bin/env python3
"""Audit repeated-update receipts, unchanged controls, and paired calibration."""
import collections
import hashlib
import itertools
import json
from pathlib import Path
import random
import statistics
import tarfile
from audit_local_lifecycle import normalized

ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / 'docs/experiments/results'
MODES = ['endpoint', 'filtered', 'local-indexed', 'scan', 'indexed', 'special-scan', 'special-indexed']

def read_groups(name):
    g = collections.defaultdict(list)
    for line in (BASE / name / 'runs.jsonl').read_text().splitlines():
        r = json.loads(line)
        g[r['kind'], tuple(r['cell'])].append(r['receipt'])
    return g

def main():
    matrices = []
    for name in ['s02-repeated-alias-lifecycle', 's02-filtered-wake-attribution']:
        out = BASE / name
        m = json.loads((out / 'manifest.json').read_text())
        rng = random.Random(7205)
        cells = list(itertools.product(MODES, ['aliases-1', 'aliases-8', 'aliases-32'], [32, 128], [1, 4], ['complete']))
        order = []
        for kind, reps in [('time', 5), ('meter', 2), ('work', 2)]:
            for rep in range(reps):
                block = cells.copy()
                rng.shuffle(block)
                order += [(kind, rep, list(c)) for c in block]
        for rep in range(2):
            block = list(itertools.product(MODES, ['aliases-32'], [128], [1], ['setup', 'cancel']))
            rng.shuffle(block)
            order += [('meter', rep, list(c)) for c in block]
        assert json.loads(json.dumps(order)) == m['order'] and len(order) == 784
        archive = tarfile.open(out / 'source-freeze.tar.gz') if (out / 'source-freeze.tar.gz').exists() else None
        for p, digest in m['source_hashes'].items():
            data = archive.extractfile(p).read() if archive else (ROOT / p).read_bytes()
            assert hashlib.sha256(data).hexdigest() == digest, p
        if archive:
            archive.close()
        for b in m['builds'].values():
            assert hashlib.sha256((ROOT / b['binary']).read_bytes()).hexdigest() == b['sha256']
        rows = [json.loads(l) for l in (out / 'runs.jsonl').read_text().splitlines()]
        assert len(rows) == len(order)
        for i, (r, (kind, rep, cell)) in enumerate(zip(rows, order)):
            assert (r['index'], r['kind'], r['rep'], r['cell']) == (i, kind, rep, cell)
            assert r['command'] == [str(ROOT / m['builds'][kind]['binary']), cell[0], cell[1], str(cell[2]), str(cell[3]), cell[4]]
            a = r['receipt']
            assert (a['family'], a['size'], a['reuse'], a['stop']) == tuple(cell[1:])
            assert a['metrics'] == (kind == 'work') and a['meter'] == (kind == 'meter')
            assert len(a['phases']) == 5 * cell[3] + 2
            if kind == 'meter':
                h = normalized(a)
                assert a['restored'] and h[-1]['live_end'] == 0
                assert all(x['live_end'] == y['live_start'] for x, y in zip(h, h[1:]))
                assert all(x['peak_live'] >= max(x['live_start'], x['live_end']) for x in h)
            if kind == 'work':
                assert len(a['work']) == cell[3]
                assert all(w['applications'] == int(cell[1].split('-')[1]) for w in a['work'])
                if cell[0].startswith('special-'):
                    assert all(w['specialized'] == w['applications'] and w['cursor_steps'] == 0 for w in a['work'])
        gates = [json.loads(l) for l in (out / 'work-gate.jsonl').read_text().splitlines()]
        assert [tuple(g['cell']) for g in gates] == cells
        groups = read_groups(name)
        for g in gates:
            assert g['receipt']['work'] == groups['work', tuple(g['cell'])][0]['work']
        for (kind, cell), rs in groups.items():
            if kind == 'meter':
                assert len(rs) == 2 and normalized(rs[0]) == normalized(rs[1])
            if kind == 'work':
                assert len(rs) == 2 and rs[0]['work'] == rs[1]['work']
        summaries = []
        for c in cells:
            rs = groups['time', c]
            h = normalized(groups['meter', c][0])
            s = {'cell': c, 'requested_bytes': sum(x['requested_bytes'] for x in h), 'peak_requested_growth': max(x['peak_live'] for x in h)}
            for clock in ['ns', 'cpu_ns']:
                if clock not in rs[0]['phases'][0]:
                    continue
                values = [sum(p[clock] for p in r['phases']) for r in rs]
                s[clock] = {'median': statistics.median(values), 'min': min(values), 'max': max(values)}
            summaries.append(s)
        (out / 'summary.json').write_text(json.dumps(summaries, indent=2) + '\n')
        (out / 'audit.json').write_text(json.dumps({'processes': 784, 'preflight_work_gates': 84, 'timing_processes': 420, 'meter_processes': 196, 'work_processes': 168, 'paired_allocation_cells': 98, 'paired_work_cells': 84, 'cutoffs': 0, 'order_commands_freezes_source_work_phase_restoration': 'pass'}, indent=2) + '\n')
        matrices.append(groups)
    original = read_groups('s02-local-validation-attribution')
    for mode, n, reuse in itertools.product(MODES, [32, 128], [1, 4]):
        old = (mode, 'lowyield', n, reuse, 'complete')
        new = (mode, 'aliases-1', n, reuse, 'complete')
        assert normalized(original['meter', old][0]) == normalized(matrices[0]['meter', new][0])
        assert original['work', old][0]['work'] == matrices[0]['work', new][0]['work']
    changed = []
    for (kind, cell), rs in matrices[0].items():
        other = matrices[1][kind, cell][0]
        if kind == 'work':
            assert rs[0]['work'] == other['work']
        if kind == 'meter' and normalized(rs[0]) != normalized(other):
            assert cell[0] == 'filtered' and cell[-1] == 'complete'
            before, after = normalized(rs[0]), normalized(other)
            for i, (a, b) in enumerate(zip(before, after)):
                if i in [2 + 5 * q for q in range(cell[3])]:
                    assert b['requested_bytes'] < a['requested_bytes']
                else:
                    assert a == b
            changed.append(cell)
    assert len(changed) == 12
    # Independently reconstruct the paired calibration ordering and binary identities.
    rows = [json.loads(l) for l in (BASE / 's02-repeated-alias-lifecycle/calibration.jsonl').read_text().splitlines()]
    rng = random.Random(7206)
    expected = []
    for rep in range(10):
        modes = MODES.copy()
        rng.shuffle(modes)
        for mode in modes:
            tags = ['old', 'new']
            rng.shuffle(tags)
            expected += [(rep, mode, tag) for tag in tags]
    assert len(rows) == 140 and [(r['rep'], r['mode'], r['tag']) for r in rows] == expected
    for r in rows:
        name, family = ('s02-local-validation-attribution', 'lowyield') if r['tag'] == 'old' else ('s02-repeated-alias-lifecycle', 'aliases-1')
        m = json.loads((BASE / name / 'manifest.json').read_text())
        assert r['command'] == [str(ROOT / m['builds']['time']['binary']), r['mode'], family, '128', '4', 'complete']
    paired = {'unchanged_one_update_control_cells': 28, 'unchanged_work_cells_after_repair': 84, 'changed_filtered_allocation_cells': 12, 'unchanged_other_allocation_cells': 86, 'calibration_processes': 140}
    (BASE / 's02-filtered-wake-attribution/paired-audit.json').write_text(json.dumps(paired, indent=2) + '\n')
    print('PASS: two 784-process matrices, 168 preflight gates, 140 calibration processes, exact replays and repair confined to Filtered execution allocation.')

if __name__ == '__main__':
    main()
