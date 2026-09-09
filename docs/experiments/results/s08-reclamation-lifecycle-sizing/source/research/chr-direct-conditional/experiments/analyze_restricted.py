#!/usr/bin/env python3
"""Audit and summarize R06 restricted-publication outcomes without treating incomplete cells as rankings."""
import json
import itertools
import pathlib
import statistics

ROOT = pathlib.Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/r06-restricted-publication'

def phases(d):
    return [d['prepare'], d['setup'],
            *([d['terminal_service']['service']] if d['terminal_service'] else []), *d['disposal'].values(),
            *(p for sample in d['samples'] for p in [sample['service'], sample['consumer']])]

def total(d):
    return sum(p['ns'] for p in phases(d)) / 1e6

def main():
    rows = [json.loads(line) for line in (OUT / 'runs.jsonl').read_text().splitlines()]
    failures = []
    seen = set()
    for row in rows:
        key = (row['mode'], row['rep'], tuple(row['cell']))
        assert key not in seen, key
        seen.add(key)
        if row.get('exit') != 0 or 'result' not in row:
            failures.append({k: row.get(k) for k in ['mode', 'rep', 'cell', 'exit', 'timeout', 'stderr']})
            continue
        d = row['result']
        assert [d[k] for k in ['backend', 'family', 'extent', 'n', 'consumer']] == [('conditional' if row['cell'][0] == 'baseline' else row['cell'][0]), *row['cell'][1:]]
        assert d['metrics'] == d['compiled_metrics'] == (row['mode'] == 'work')
        assert d['allocator_meter'] == (row['mode'] == 'allocation')
        assert d['delivered'] == len(d['samples'])
        assert [s['answer'] for s in d['samples']] == list(range(1, d['delivered']+1))
        assert sum(p['ns'] for p in phases(d)) == d['measured_ns']
        assert sum(s['ticks'] for s in d['samples']) + (d['terminal_service']['ticks'] if d['terminal_service'] else 0) == d['total_ticks']
        if d['complete']:
            assert d['delivered'] == d['n']
            assert d['exhausted'] == (d['extent'] == 'finite') and not d['cutoff']
        else:
            assert d['cutoff'] and not d['exhausted']
            assert d['total_ticks'] == d['tick_limit'] == 20_000_000
            failures.append({'mode': row['mode'], 'rep': row['rep'], 'cell': row['cell'],
                             'cutoff': True, 'delivered': d['delivered']})
        if row['mode'] == 'allocation':
            assert d['final']['memory']['live_end'] == d['baseline']['memory']['live_end'], row['cell']
    prior = [json.loads(line) for line in (ROOT/'docs/experiments/results/r06-streaming-lifetime/runs.jsonl').read_text().splitlines()]
    prior_map = {(r['mode'],r['rep'],tuple(r['cell'])): r['result'] for r in prior}
    for r in rows:
        if r['cell'][0] != 'baseline' or 'result' not in r:
            continue
        d=r['result']
        old=prior_map[r['mode'],r['rep'],tuple(['conditional',*r['cell'][1:]])]
        for key in ['total_ticks','delivered','exhausted','cutoff','complete','final_diagnostics']:
            assert d[key]==old[key], ('baseline replay',r['cell'],key)
        assert [(s['ticks'],s['diagnostics']) for s in d['samples']]==[(s['ticks'],s['diagnostics']) for s in old['samples']]
    cells = set(itertools.product(['baseline', 'conditional', 'specialized'], ['ground', 'aliases'],
                                  ['unbounded', 'finite'], [16, 64, 128], ['drop', 'retain']))
    mode_reps = {('warmup', 0), ('allocation', 0), ('work', 0),
                 *(('primary', rep) for rep in range(5))}
    expected = {(mode, rep, cell) for cell in cells for mode, rep in mode_reps}
    assert seen <= expected, seen - expected
    summary = []
    for cell in sorted(cells):
        group = [r for r in rows if tuple(r['cell']) == cell]
        primary = [r['result'] for r in group if r['mode'] == 'primary' and
                   r.get('result', {}).get('complete')]
        mem = [r['result'] for r in group if r['mode'] == 'allocation' and
               r.get('result', {}).get('complete')]
        item = {'cell': cell, 'recorded': len(group), 'primary_complete': len(primary),
                'cell_complete': {(r['mode'], r['rep']) for r in group} == mode_reps and all(r.get('result', {}).get('complete') for r in group)}
        if primary:
            vals = [total(d) for d in primary]
            item.update(median_ms=statistics.median(vals), min_ms=min(vals), max_ms=max(vals),
                        prepare_ms=statistics.median(d['prepare']['ns']/1e6 for d in primary),
                        setup_ms=statistics.median(d['setup']['ns']/1e6 for d in primary),
                        first_service_ms=statistics.median(d['samples'][0]['service']['ns']/1e6 for d in primary),
                        early_service_us=statistics.median(statistics.mean(s['service']['ns'] for s in d['samples'][:max(1,d['n']//4)])/1e3 for d in primary),
                        late_service_us=statistics.median(statistics.mean(s['service']['ns'] for s in d['samples'][-max(1,d['n']//4):])/1e3 for d in primary),
                        disposal_ms=statistics.median(sum(p['ns'] for p in d['disposal'].values())/1e6 for d in primary),
                        validation_ms=statistics.median(sum(s['validation_ns'] for s in d['samples'])/1e6 for d in primary))
        if mem:
            d, = mem
            baseline = d['baseline']['memory']['live_end']
            item.update(requested_bytes=sum(p['memory']['requested_bytes'] for p in phases(d)),
                        peak_above_baseline=max(p['memory']['peak_live'] for p in phases(d))-baseline,
                        retained_above_baseline=d['samples'][-1]['consumer']['memory']['live_end']-baseline,
                        after_terminal_above_baseline=(d['terminal_service']['service']['memory']['live_end'] if d['terminal_service'] else d['samples'][-1]['consumer']['memory']['live_end'])-baseline,
                        post_disposal_above_baseline=d['final']['memory']['live_end']-baseline,
                        retained_samples=[s['consumer']['memory']['live_end']-baseline for s in d['samples']])
        work = [r['result'] for r in group if r['mode'] == 'work' and r.get('result', {}).get('complete')]
        if work:
            item['total_ticks'] = work[0]['total_ticks']
            item['last_diagnostics'] = work[0]['samples'][-1]['diagnostics']
        partial = [r for r in group if r.get('result') and not r['result']['complete']]
        item['partial'] = []
        for r in partial:
            d=r['result']
            p={'mode':r['mode'],'rep':r['rep'],'delivered':d['delivered'],
               'ticks':d['total_ticks'],'measured_ms':total(d),'diagnostics':d['final_diagnostics']}
            if r['mode']=='allocation':
                p['retained_at_cutoff']=d['terminal_service']['service']['memory']['live_end']-d['baseline']['memory']['live_end']
            item['partial'].append(p)
        summary.append(item)
    (OUT / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
    audit = {'planned_processes': 576, 'recorded_processes': len(rows), 'failures': failures,
             'complete_cells': sum(s['cell_complete'] for s in summary),
             'missing_runs': sorted(expected - seen),
             'validated_raw_answers': sum(r.get('result',{}).get('delivered',0) for r in rows)}
    (OUT / 'audit.json').write_text(json.dumps(audit, indent=2) + '\n')
    lines = ['backend\tfamily\textent\tn\tconsumer\tcomplete\tmedian_ms\tlate_us\tlive_bytes\trequested_bytes']
    for s in summary:
        lines.append('\t'.join(map(str,[*s['cell'],s['cell_complete'],s.get('median_ms'),s.get('late_service_us'),s.get('retained_above_baseline'),s.get('requested_bytes')])))
    (OUT / 'summary.tsv').write_text('\n'.join(lines) + '\n')
    print(json.dumps(audit))
    print('\n'.join(lines))

if __name__ == '__main__':
    main()
