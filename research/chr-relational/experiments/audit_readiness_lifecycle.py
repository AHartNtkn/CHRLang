"""Audit frozen lifecycle data; report paired sizing ratios without workload weights."""
import hashlib
import json
import re
import statistics
import zipfile
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s02-readiness-lifecycle'


def phases(sample):
    return [sample['preparation'], *[p for q in sample['queries'] for p in
            (q['setup'], q['execution_observation'], q['engine_drop'])],
            sample['preparation_drop'], sample['consumer_drop']]


def normalized(sample):
    base = sample['preparation']['heap']['live_start']
    heaps = []
    for p in phases(sample):
        h = p['heap'].copy()
        for key in ['live_start', 'live_end', 'peak_live']: h[key] -= base
        heaps.append(h)
    assert heaps[-1]['live_end'] == 0
    return dict(heaps=heaps, advances=[q['advances'] for q in sample['queries']])


def main():
    frozen = json.loads((OUT / 'freeze.json').read_text())
    digest = lambda data: hashlib.sha256(data).hexdigest()
    assert digest((OUT / 'sources.zip').read_bytes()) == frozen['archive_sha256']
    with zipfile.ZipFile(OUT / 'sources.zip') as archive:
        for path, h in frozen['sources'].items(): assert digest(archive.read(path)) == h
    floor = json.loads(json.loads((OUT / 'clock-floor.json').read_text())['stdout'])['median_ns']
    entries = json.loads((OUT / 'entry.json').read_text()); assert len(entries) == 80
    assert all(e['receipt']['exit_code'] == 0 for e in entries)
    assert json.loads((OUT / 'meter-check.json').read_text())['exit_code'] == 0
    rows = [json.loads(s) for s in (OUT / 'samples.jsonl').read_text().splitlines()]
    assert len(rows) == 1120
    cells = defaultdict(dict)
    for row in rows:
        assert row['receipt']['exit_code'] == 0, row
        data = json.loads(row['receipt']['stdout'])
        assert data['validated'] and data['metered'] == (row['flavor'] == 'meter')
        assert len(data['queries']) == row['count']
        cell = (row['depth'], row['shared'], row['outcome'], row['count'], row['mode'])
        key = (row['flavor'], row['rep'])
        assert key not in cells[cell]
        cells[cell][key] = data
    assert len(cells) == 160
    summaries = []
    for cell, samples in sorted(cells.items()):
        assert set(samples) == {('ordinary', i) for i in range(5)} | {('meter', i) for i in range(2)}
        a, b = normalized(samples['meter', 0]), normalized(samples['meter', 1])
        assert a == b, cell
        totals = [sum(p['ns'] for p in phases(samples['ordinary', i])) for i in range(5)]
        traffic = sum(h['requested_bytes'] for h in a['heaps'])
        peak = max(h['peak_live'] for h in a['heaps'])
        summary = dict(depth=cell[0], shared=cell[1], outcome=cell[2], count=cell[3], mode=cell[4],
                       median_ns=statistics.median(totals), min_ns=min(totals), max_ns=max(totals),
                       requested_bytes=traffic, peak_excess=peak,
                       preparation_bytes=a['heaps'][0]['requested_bytes'],
                       execution_bytes=sum(p['heap']['requested_bytes'] for q in [samples['meter', 0]['queries']] for p in [x['execution_observation'] for x in q]),
                       advances=a['advances'])
        summaries.append(summary)
    ratios = []
    for cell, samples in sorted(cells.items()):
        if cell[4] == 'full': continue
        for control in ['full', 'compiled']:
            if cell[4] == control: continue
            reference = cells[(*cell[:4], control)]
            paired = [sum(p['ns'] for p in phases(samples['ordinary', i])) /
                      sum(p['ns'] for p in phases(reference['ordinary', i])) for i in range(5)]
            med = statistics.median(paired)
            floor_ok = all(statistics.median([sum(p['ns'] for p in phases(s['ordinary', i])) for i in range(5)]) > 20*floor for s in [samples, reference])
            direction = ('lower' if med < .9 and max(paired) < 1 else
                         'higher' if med > 1.1 and min(paired) > 1 else 'unresolved') if floor_ok else 'clock-unresolved'
            ratios.append(dict(depth=cell[0], shared=cell[1], outcome=cell[2], count=cell[3], mode=cell[4],
                               control=control, median=med, minimum=min(paired), maximum=max(paired), disposition=direction))
    result = dict(processes=1120, allocation_pairs=160, final_ownership_restored=True, clock_floor_ns=floor,
                  cells=summaries, paired_ratios=ratios)
    (OUT / 'analysis.json').write_text(json.dumps(result, indent=2) + '\n')
    print('Verified 1,120 samples, 160 exact allocation pairs and final ownership restoration')
    for control in ['full', 'compiled']:
        for mode in ['selective', 'batch8', 'batch256', 'compiled']:
            rs = [r for r in ratios if r['control'] == control and r['mode'] == mode]
            if rs:
                print(mode, '/', control, {d: sum(r['disposition'] == d for r in rs) for d in ['lower','higher','unresolved','clock-unresolved']}, 'median ratio range', min(r['median'] for r in rs), max(r['median'] for r in rs))


if __name__ == '__main__': main()
