#!/usr/bin/env python3
"""Summarize the frozen exploratory pilot without treating queries as replicates."""
import csv
import json
from collections import defaultdict
from pathlib import Path
from statistics import median

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/R01-native-pilot'


def main():
    records = [json.loads(line) for line in (OUT / 'raw.jsonl').read_text().splitlines()]
    assert len(records) == 640
    groups = defaultdict(list)
    for record in records:
        assert record.get('returncode') == 0 and 'validation_error' not in record
        report = record['report']
        assert report['completed'] == 4
        assert all(s['exhausted'] and not s['failed'] for s in report['samples'])
        key = tuple(report[k] for k in ['family', 'size', 'execution', 'policy', 'access'])
        groups[(record['mode'], *key)].append(report)
    rows = []
    for (mode, *key), reports in sorted(groups.items()):
        if mode != 'time':
            continue
        assert len(reports) == 5
        row = dict(zip(['family', 'size', 'execution', 'policy', 'access'], key))
        data = defaultdict(list)
        for report in reports:
            samples = report['samples']
            data['prepare_ns'].append(report['prepare_ns'])
            data['prepared_drop_ns'].append(report['prepared_drop_ns'])
            data['harness_ns'].append(report['harness_ns'])
            data['first_query_ns'].append(samples[0]['request_ns'] + samples[0]['answer_drop_ns'])
            data['reused_query_ns'].append(sum(s['request_ns'] + s['answer_drop_ns'] for s in samples[1:]) / 3)
            data['lifecycle_ns'].append(report['prepare_ns'] + report['prepared_drop_ns'] + sum(s['request_ns'] + s['answer_drop_ns'] for s in samples))
            for field in ['setup_ns', 'execution_ns', 'observe_ns', 'engine_drop_ns', 'answer_drop_ns']:
                data[field].append(sum(s[field] for s in samples) / 4)
        for field, values in data.items():
            row[field] = median(values)
            row[field + '_min'] = min(values)
            row[field + '_max'] = max(values)
        work = groups[('work', *key)]
        assert len(work) == 2
        assert [s['work'] for s in work[0]['samples']] == [s['work'] for s in work[1]['samples']]
        for field in work[0]['samples'][0]['work']:
            row['work_' + field] = work[0]['samples'][0]['work'][field]
        memory, = groups[('memory', *key)]
        assert len(set(s['memory'][4]['live_end'] for s in memory['samples'])) == 1
        row['execution_requested_bytes'] = memory['samples'][0]['memory'][1]['requested_bytes']
        row['execution_allocation_calls'] = memory['samples'][0]['memory'][1]['allocation_calls']
        row['query_peak_live_bytes'] = max(m['peak_live'] for s in memory['samples'] for m in s['memory'])
        row['post_answer_live_bytes'] = memory['samples'][0]['memory'][4]['live_end']
        md = memory['prepared_drop_memory']
        row['prepared_released_bytes'] = md['live_start'] - md['live_end']
        rows.append(row)
    with (OUT / 'summary.tsv').open('w') as f:
        writer = csv.DictWriter(f, fieldnames=list(rows[0]), delimiter='\t')
        writer.writeheader()
        writer.writerows(rows)
    print('Validated 640 processes / 2560 complete answers, 80 cells; counted replicas match; query retained bytes stable.')
    print('Size 64: four-query lifecycle microseconds (five-process median)')
    for family in ['Build', 'Chain', 'Delayed', 'Collision', 'Repair']:
        print(family)
        for row in rows:
            if row['family'] == family and row['size'] == 64:
                print(row['execution'],row['policy'],row['access'],round(row['lifecycle_ns']/1000,1),
                      'exec/query',round(row['execution_ns']/1000,1),'prepare',round(row['prepare_ns']/1000,1),
                      'candidates',row['work_candidate_visits'],'allocs',row['execution_allocation_calls'])


if __name__ == '__main__':
    main()
