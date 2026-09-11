"""Exploratory paired ratios; no confidence or architecture-selection claims."""
import collections
import json
from pathlib import Path
import statistics

BASE = Path(__file__).resolve().parents[3] / 'docs/experiments/results/s05-inert-lifecycle'
CONTROLS = ['direct', 'separate', 'scan', 'indexed', 'sealed', 'active-scan', 'active-indexed']


def main():
    audit = json.loads((BASE / 'audit.json').read_text())
    groups = collections.defaultdict(dict)
    for cell in audit['cells']:
        if cell['kind'] == 'plain':
            groups[tuple(cell['args'][1:])][cell['args'][0]] = cell
    comparisons = []
    for scenario, cells in sorted(groups.items()):
        memo = cells['memo']
        for mode, control in sorted(cells.items()):
            if mode == 'memo':
                continue
            left = {s['rep']: s for s in memo['samples']}
            right = {s['rep']: s for s in control['samples']}
            ratios = [left[r]['total_ns'] / right[r]['total_ns'] for r in sorted(left)]
            comparisons.append(dict(scenario=scenario, control=mode,
                paired_median_ratio=statistics.median(ratios), ratios=ratios,
                sensitive=memo['sensitive'] or control['sensitive']))
    counts = {}
    for mode in CONTROLS:
        rows = [r for r in comparisons if r['control'] == mode]
        counts[mode] = dict(scenarios=len(rows), sensitive=sum(r['sensitive'] for r in rows),
                           below_09=sum(r['paired_median_ratio'] < .9 for r in rows),
                           above_11=sum(r['paired_median_ratio'] > 1.1 for r in rows))
    best = []
    for scenario, cells in sorted(groups.items()):
        mode = min(CONTROLS, key=lambda m: cells[m]['median_ns'])
        contrast = next(r for r in comparisons if tuple(r['scenario']) == scenario and r['control'] == mode)
        best.append(dict(scenario=scenario, fastest_observed_control=mode, **{k:v for k,v in contrast.items() if k not in ['scenario','control']}))
    output = dict(note='Exploratory unweighted counts; fastest control is selected on these same samples, not a validated policy.',
                  comparisons=comparisons, counts=counts, best_control=best)
    (BASE / 'comparisons.json').write_text(json.dumps(output, indent=2))
    bounds = []
    for scenario, cells in sorted(groups.items()):
        memo, direct = cells['memo'], cells['direct']
        if memo['sensitive'] or direct['sensitive']:
            continue
        left = {v['rep']: v for v in memo['samples']}
        right = {v['rep']: v for v in direct['samples']}
        ordinary = [left[r]['total_ns'] / right[r]['total_ns'] for r in sorted(left)]
        zero_disposal = [(left[r]['total_ns'] - sum(t for p, t in left[r]['phases'].items()
                         if p.endswith('_dispose'))) / right[r]['total_ns'] for r in sorted(left)]
        bounds.append(dict(scenario=scenario, minimum_paired_ratio=min(ordinary),
                           median_zero_disposal_ratio=statistics.median(zero_disposal)))
    (BASE / 'disposal-bound.json').write_text(json.dumps(dict(
        scope='Exploratory arithmetic bound: zero all measured memo disposal, leave Direct unchanged; no replacement implementation.',
        scenarios=len(bounds), minimum_single_paired_ratio=min(r['minimum_paired_ratio'] for r in bounds),
        minimum_median_zero_disposal_ratio=min(r['median_zero_disposal_ratio'] for r in bounds),
        rows=bounds), indent=2))
    print(json.dumps(counts, indent=2))
    print('memo vs fastest observed', 'below .9', sum(r['paired_median_ratio'] < .9 for r in best),
          'above 1.1', sum(r['paired_median_ratio'] > 1.1 for r in best),
          'sensitive', sum(r['sensitive'] for r in best))
    for scenario, cells in sorted(groups.items()):
        if scenario[1] == '128' and scenario[2:]==('4','all','0','true'):
            print(scenario, {m:round(c['median_ns']/1e6, 4) for m,c in cells.items()})


if __name__ == '__main__':
    main()
