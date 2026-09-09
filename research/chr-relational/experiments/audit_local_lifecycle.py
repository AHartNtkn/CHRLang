#!/usr/bin/env python3
"""Independent receipt/order/phase audit and descriptive statistics; no winner scoring."""
import collections
import hashlib
import itertools
import json
from pathlib import Path
import random
import statistics
import tarfile

ROOT = Path(__file__).resolve().parents[3]
NAMES = ['s02-local-lifecycle-sizing', 's02-local-validation-attribution']
MODES = ['endpoint', 'filtered', 'local-indexed', 'scan', 'indexed', 'special-scan', 'special-indexed']
FAMILIES = ['chain', 'flat', 'shared', 'lowyield']

def normalized(receipt):
    base = receipt['phases'][0]['heap']['live_start']
    result = []
    for phase in receipt['phases']:
        h = phase['heap'].copy()
        for k in ['live_start', 'live_end', 'peak_live']:
            h[k] -= base
        result.append(h)
    return result

def main():
    matrices = []
    for name in NAMES:
        out = ROOT / 'docs/experiments/results' / name
        manifest = json.loads((out / 'manifest.json').read_text())
        rng = random.Random(7204)
        expected_order = []
        cells = list(itertools.product(MODES, FAMILIES, [4, 32, 128], [1, 4], ['complete']))
        for kind, reps in [('time', 5), ('meter', 2), ('work', 2)]:
            for rep in range(reps):
                block = cells.copy()
                rng.shuffle(block)
                expected_order += [(kind, rep, list(c)) for c in block]
        for rep in range(2):
            block = list(itertools.product(MODES, FAMILIES, [32], [1], ['setup', 'cancel']))
            rng.shuffle(block)
            expected_order += [('meter', rep, list(c)) for c in block]
        assert json.loads(json.dumps(expected_order)) == manifest['order']
        rows = [json.loads(s) for s in (out / 'runs.jsonl').read_text().splitlines()]
        assert len(rows) == len(expected_order) == 1624
        archive_path = out / 'source-freeze.tar.gz'
        archive = tarfile.open(archive_path) if archive_path.exists() else None
        for path, digest in manifest['source_hashes'].items():
            data = archive.extractfile(path).read() if archive else (ROOT / path).read_bytes()
            assert hashlib.sha256(data).hexdigest() == digest, path
        if archive:
            archive.close()
        for build in manifest['builds'].values():
            assert hashlib.sha256((ROOT / build['binary']).read_bytes()).hexdigest() == build['sha256']
        groups = collections.defaultdict(list)
        for i, (row, (kind, rep, cell)) in enumerate(zip(rows, expected_order)):
            assert (row['index'], row['kind'], row['rep'], row['cell']) == (i, kind, rep, cell)
            expected_command = [str(ROOT / manifest['builds'][kind]['binary']), cell[0], cell[1], str(cell[2]), str(cell[3]), cell[4]]
            assert row['command'] == expected_command
            r = row['receipt']
            assert (r['family'], r['size'], r['reuse'], r['stop']) == tuple(cell[1:])
            assert r['metrics'] == (kind == 'work') and r['meter'] == (kind == 'meter')
            assert len(r['phases']) == cell[3] * 5 + 2
            if kind == 'meter':
                assert r['restored']
                heaps = [p['heap'] for p in r['phases']]
                assert heaps[0]['live_start'] == heaps[-1]['live_end']
                for a, b in zip(heaps, heaps[1:]):
                    assert a['live_end'] == b['live_start']
                assert all(h['peak_live'] >= max(h['live_start'], h['live_end']) for h in heaps)
            if kind == 'work':
                apps = cell[2] if cell[1] in ['chain', 'flat'] else cell[2] + 1 if cell[1] == 'shared' else 1
                assert len(r['work']) == cell[3]
                assert all(w['applications'] == apps for w in r['work'])
                if cell[0].startswith('special-'):
                    assert all(w['specialized'] == apps and w['cursor_steps'] == 0 for w in r['work'])
            groups[kind, tuple(cell)].append(r)
        for (kind, cell), rs in groups.items():
            if kind == 'meter':
                assert len(rs) == 2 and normalized(rs[0]) == normalized(rs[1])
            if kind == 'work':
                assert len(rs) == 2 and rs[0]['work'] == rs[1]['work']
        summaries = []
        for cell in cells:
            times = [sum(p['ns'] for p in r['phases']) for r in groups['time', cell]]
            heaps = normalized(groups['meter', cell][0])
            summary = {'cell': cell, 'ns_median': statistics.median(times), 'ns_min': min(times), 'ns_max': max(times), 'requested_bytes': sum(h['requested_bytes'] for h in heaps), 'peak_requested_growth': max(h['peak_live'] for h in heaps)}
            summary['phase_ns_medians'] = [statistics.median(r['phases'][i]['ns'] for r in groups['time', cell]) for i in range(cell[3] * 5 + 2)]
            summaries.append(summary)
        (out / 'summary.json').write_text(json.dumps(summaries, indent=2) + '\n')
        audit = {'processes': 1624, 'timing_processes': 840, 'allocation_processes': 448, 'work_processes': 336, 'paired_allocation_cells': 224, 'paired_work_cells': 168, 'cutoffs': 0, 'complete_raw_source_answers_validated_in_runner': True, 'independent_order_commands_phases_freezes_checked': True, 'description': 'Exploratory medians and ranges only; no universal score or confirmatory classification.'}
        (out / 'audit.json').write_text(json.dumps(audit, indent=2) + '\n')
        matrices.append(groups)
    for (kind, cell), rs in matrices[0].items():
        if kind == 'meter':
            assert normalized(rs[0]) == normalized(matrices[1][kind, cell][0]), cell
        if kind == 'work':
            assert rs[0]['work'] == matrices[1][kind, cell][0]['work'], cell
    paired = {'unchanged_normalized_allocation_cells': 224, 'unchanged_work_cells': 168, 'normalization': 'Requested live and peak gauges subtract each process session baseline; argument/binary-path allocation otherwise changes absolute gauges.'}
    (ROOT / 'docs/experiments/results/s02-local-validation-attribution/paired-audit.json').write_text(json.dumps(paired, indent=2) + '\n')
    print('PASS: both 1624-process matrices, exact paired records, phase restoration, source/binary freezes and between-matrix normalized allocation/work signatures.')

if __name__ == '__main__':
    main()
