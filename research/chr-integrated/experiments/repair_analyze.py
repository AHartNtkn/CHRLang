#!/usr/bin/env python3
"""Per-cell R02 lifecycle distributions; no cross-workload weights or winner."""
import csv
import json
from collections import defaultdict
from pathlib import Path
from statistics import median
OUT=Path(__file__).resolve().parents[3]/'docs/experiments/results/R02-repair-pilot'

def main():
    records=[json.loads(s) for s in (OUT/'raw.jsonl').read_text().splitlines()]
    assert len(records)==392
    groups=defaultdict(list)
    for row in records:
        assert row.get('returncode')==0 and 'validation_error' not in row
        r=row['report'];assert r['completed']==4
        assert all(s['exhausted'] and not s['failed'] for s in r['samples'])
        key=(row['stage'],)+tuple(r[k] for k in ['family','size','depth','variant'])
        groups[(row['mode'],*key)].append(r)
    rows=[]
    for key in sorted(set(k[1:] for k in groups)):
        row=dict(zip(['stage','family','size','depth','variant'],key))
        runs=groups[('time',*key)];assert len(runs)==5
        fields=defaultdict(list)
        for r in runs:
            ss=r['samples']
            fields['lifecycle_ns'].append(r['prepare_ns']+r['prepared_drop_ns']+sum(s['request_ns']+s['answer_drop_ns'] for s in ss))
            fields['prepare_ns'].append(r['prepare_ns']);fields['prepared_drop_ns'].append(r['prepared_drop_ns']);fields['harness_ns'].append(r['harness_ns'])
            fields['first_query_ns'].append(ss[0]['request_ns']+ss[0]['answer_drop_ns'])
            fields['reused_query_ns'].append(sum(s['request_ns']+s['answer_drop_ns'] for s in ss[1:])/3)
            for k in ['setup_ns','execution_ns','observe_ns','engine_drop_ns','answer_drop_ns']:
                fields[k].append(sum(s[k] for s in ss)/4)
        for k,vs in fields.items():
            row[k]=median(vs);row[k+'_min']=min(vs);row[k+'_max']=max(vs)
        work,=groups[('work',*key)]
        row['work']=json.dumps(work['samples'][0]['work'],sort_keys=True)
        memory,=groups[('memory',*key)]
        assert len(set(s['memory'][4]['live_end'] for s in memory['samples']))==1
        for i,phase in enumerate(['setup','execution','observe','engine_drop','answer_drop']):
            m=memory['samples'][0]['memory'][i]
            row[phase+'_allocation_calls']=m['allocation_calls'];row[phase+'_requested_bytes']=m['requested_bytes']
        row['query_peak_live_bytes']=max(m['peak_live'] for s in memory['samples'] for m in s['memory'])
        row['post_answer_live_bytes']=memory['samples'][0]['memory'][4]['live_end']
        m=memory['prepared_drop_memory'];row['prepared_released_bytes']=m['live_start']-m['live_end']
        rows.append(row)
    with (OUT/'summary.tsv').open('w') as f:
        w=csv.DictWriter(f,fieldnames=list(rows[0]),delimiter='\t');w.writeheader();w.writerows(rows)
    print('392 processes / 1568 complete answers, 56 cells; allocation no-growth checked.')
    for r in rows:
        if r['size']==64:
            print(r['stage'],r['family'],r['depth'],r['variant'],round(r['lifecycle_ns']/1e6,3),
                  'range',round(r['lifecycle_ns_min']/1e6,3),round(r['lifecycle_ns_max']/1e6,3),
                  'exec',round(r['execution_ns']/1000,1),'allocs',r['execution_allocation_calls'],'peak',r['query_peak_live_bytes'])

if __name__=='__main__':main()
