#!/usr/bin/env python3
"""Compare registered anchor-information variants, preserving per-cell uncertainty."""
import csv
import json
from collections import defaultdict
from pathlib import Path
from statistics import median

OUT = Path(__file__).resolve().parents[3] / 'docs/experiments/results/R01-anchor-pilot'


def main():
    records = [json.loads(s) for s in (OUT / 'raw.jsonl').read_text().splitlines()]
    assert len(records) == 1120
    groups = defaultdict(list)
    for r in records:
        assert r.get('returncode') == 0 and 'validation_error' not in r
        p = r['report']
        assert p['completed'] == 4
        assert all(s['exhausted'] and not s['failed'] for s in p['samples'])
        key = tuple(p[k] for k in ['family', 'size', 'execution', 'policy', 'access'])
        groups[(r['variant'], r['mode'], *key)].append(p)
    rows = []
    cells = sorted(set(key[2:] for key in groups))
    for key in cells:
        row = dict(zip(['family', 'size', 'execution', 'policy', 'access'], key))
        for variant in ['before', 'after']:
            reports = groups[(variant, 'time', *key)]
            assert len(reports) == 5
            values = [p['prepare_ns'] + p['prepared_drop_ns'] + sum(s['request_ns'] + s['answer_drop_ns'] for s in p['samples']) for p in reports]
            for field, value in [('median_ns', median(values)), ('min_ns', min(values)), ('max_ns', max(values))]:
                row[variant + '_' + field] = value
            row[variant + '_execution_ns'] = median(sum(s['execution_ns'] for s in p['samples']) / 4 for p in reports)
            work, = groups[(variant, 'work', *key)]
            for k,v in work['samples'][0]['work'].items():
                row[variant + '_' + k] = v
            memory, = groups[(variant, 'memory', *key)]
            assert len(set(s['memory'][4]['live_end'] for s in memory['samples'])) == 1
            row[variant + '_execution_allocations'] = memory['samples'][0]['memory'][1]['allocation_calls']
            row[variant + '_execution_requested_bytes'] = memory['samples'][0]['memory'][1]['requested_bytes']
        assert row['before_applications'] == row['after_applications']
        row['lifecycle_ratio_after_before'] = row['after_median_ns']/row['before_median_ns']
        row['ranges_overlap'] = not (row['after_max_ns'] < row['before_min_ns'] or row['before_max_ns'] < row['after_min_ns'])
        rows.append(row)
    with (OUT / 'summary.tsv').open('w') as f:
        writer = csv.DictWriter(f, fieldnames=list(rows[0]), delimiter='\t')
        writer.writeheader(); writer.writerows(rows)
    print('1120 processes, 4480 complete answers; bounded retention and application counts agree.')
    for r in rows:
        if r['size'] == 64 and r['execution'] == 'Generated':
            print(r['family'],r['policy'],r['access'],
                  round(r['before_median_ns']/1e6,3),round(r['after_median_ns']/1e6,3),
                  'ratio',round(r['lifecycle_ratio_after_before'],3),'overlap',r['ranges_overlap'],
                  'visits',r['before_candidate_visits'],r['after_candidate_visits'],
                  'allocs',r['before_execution_allocations'],r['after_execution_allocations'])


if __name__ == '__main__':
    main()
