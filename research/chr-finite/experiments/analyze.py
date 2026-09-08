#!/usr/bin/env python3
import csv,json
from pathlib import Path
from collections import defaultdict
from statistics import median
OUT=Path(__file__).resolve().parents[3]/'docs/experiments/results/R04-lifecycle-pilot'
def main():
    records=[json.loads(x) for x in (OUT/'raw.jsonl').read_text().splitlines()];assert len(records)==720
    cells=defaultdict(list)
    for r in records:
        assert 'validation_error' not in r
        cells[tuple(r[k] for k in ['family','size','variant','queries'])].append(r)
    summaries=[]
    for key,rs in sorted(cells.items()):
        row=dict(zip(['family','size','variant','queries'],key));timing=[r for r in rs if r['mode']=='time'];assert len(timing)==5
        row['complete_timing_runs']=sum(r.get('complete',False) for r in timing)
        row['resource_failures']=sum(r.get('timeout',False) or r.get('resource_failure',False) for r in rs)
        fields=defaultdict(list)
        if row['complete_timing_runs']==5:
            for record in timing:
                r=record['report'];ss=r['samples'];fields['lifecycle_ns'].append(r['prepare_ns']+r['prepared_drop_ns']+sum(s['request_ns']+s['answer_drop_ns'] for s in ss))
                for k in ['prepare_ns','prepared_drop_ns']:fields[k].append(r[k])
                for k in ['setup_ns','execution_ns','observe_ns','result_drop_ns','query_drop_ns','request_ns','answer_drop_ns']:
                    fields[k].append(sum(s[k] for s in ss))
                fields['first_query_ns'].append(ss[0]['request_ns']+ss[0]['answer_drop_ns'])
                if ss[0]['first_answer_ns'] is not None:fields['first_answer_ns'].append(ss[0]['first_answer_ns'])
                row['raw_by_query']=json.dumps([s['raw'] for s in ss])
            for k,vs in fields.items():row[k]=median(vs);row[k+'_min']=min(vs);row[k+'_max']=max(vs)
        memory,=[r for r in rs if r['mode']=='memory'];row['memory_complete']=memory.get('complete',False);row['max_rss_kib']=memory.get('max_rss_kib','')
        if 'report' in memory:
            m=memory['report'];row['before_rss_kib']=m['before_rss_kib'];row['prepared_rss_kib']=m['prepared_rss_kib'];row['final_rss_kib']=m['final_rss_kib'];row['post_answer_rss_kib']=json.dumps([s['after_answer_rss_kib'] for s in m['samples']])
        summaries.append(row)
    names=list(dict.fromkeys(k for row in summaries for k in row))
    with (OUT/'summary.tsv').open('w') as f:w=csv.DictWriter(f,fieldnames=names,delimiter='\t');w.writeheader();w.writerows(summaries)
    print(len(records),'processes;',sum(r.get('complete',False) for r in records),'complete;',sum(len(r.get('report',{}).get('samples',[])) for r in records),'query records')
    for r in summaries:
        if r['queries']==4:print(r['family'],r['size'],r['variant'],round(r.get('lifecycle_ns',0)/1e6,3),r['complete_timing_runs'],'rss',r['max_rss_kib'])
if __name__=='__main__':main()
