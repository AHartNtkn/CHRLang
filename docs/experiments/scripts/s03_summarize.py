#!/usr/bin/env python3
import csv,json,statistics,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results'/(sys.argv[1] if len(sys.argv)>1 else 's03-lifecycle')
records=[json.loads(x) for x in (OUT/'raw.jsonl').read_text().splitlines()]
order=json.loads((OUT/'order.json').read_text())
assert len(records)==len(order) and all(r['exit_code']==0 for r in records)
assert all(all(r[k]==v for k,v in cell.items()) for r,cell in zip(records,order))
groups={}
for r in records:groups.setdefault((r['kind'],r['family'],r['mode']),[]).append(r)
rows=[]
for (kind,family,mode),rs in sorted(groups.items()):
    assert len(rs)==(5 if kind=='time' else 2)
    phases=[]
    for r in rs:
        m=r['measurement'];q=m['queries'];assert len(q)==16
        assert m['total_ns']==m['preparation']['ns']+m['prepared_drop']['ns']+sum(x[p]['ns'] for x in q for p in ['setup','execution_observation','engine_drop','answer_drop'])
        phases.append(dict(total_ns=m['total_ns'],preparation_ns=m['preparation']['ns'],setup_ns=sum(x['setup']['ns'] for x in q),execution_ns=sum(x['execution_observation']['ns'] for x in q),disposal_ns=m['prepared_drop']['ns']+sum(x[p]['ns'] for x in q for p in ['engine_drop','answer_drop']),first_ns_sum=sum(x['first_ns'] for x in q),raw=sum(x['raw'] for x in q)))
    row=dict(kind=kind,family=family,mode=mode,repetitions=len(rs),**{k:statistics.median(p[k] for p in phases) for k in phases[0]})
    row.update(min_ns=min(p['total_ns'] for p in phases),max_ns=max(p['total_ns'] for p in phases))
    if kind=='memory':
        fingerprints=[]
        for r in rs:
            m=r['measurement'];q=m['queries'];ps=[m['preparation'],m['prepared_drop']]+[x[p] for x in q for p in ['setup','execution_observation','engine_drop','answer_drop']]
            fingerprints.append([(p['memory']['requested_bytes'],p['memory']['allocation_calls'],p['memory']['live_start'],p['memory']['live_end'],p['memory']['peak_live']) for p in ps])
            assert all(x['answer_drop']['memory']['live_end']==x['setup']['memory']['live_start'] for x in q),(family,mode,'query owner retention')
        row['memory_exact_replay']=fingerprints[0]==fingerprints[1]
        row['requested_bytes']=sum(p[0] for p in fingerprints[0]);row['allocation_calls']=sum(p[1] for p in fingerprints[0]);row['peak_live']=max(p[4] for p in fingerprints[0])
    rows.append(row)
fields=list(dict.fromkeys(k for r in rows for k in r))
with (OUT/'summary.tsv').open('w') as f:
    w=csv.DictWriter(f,fields,delimiter='\t',lineterminator='\n',restval='NA');w.writeheader();w.writerows(rows)
ratios=[]
for family in ['binary','nested','duplicate','plain','shared','discriminate']:
    for control in ['global-scan','global-index','conditional','words','active-index','graph-original']:
        if ('time',family,control) not in groups:continue
        graph={r['repetition']:r['measurement']['total_ns'] for r in groups['time',family,'graph']}
        other={r['repetition']:r['measurement']['total_ns'] for r in groups['time',family,control]}
        values=[graph[k]/other[k] for k in sorted(graph)]
        med=statistics.median(values)
        outcome='inconclusive'
        if all(v<1 for v in values) and med<=0.8:outcome='graph faster in pilot'
        if all(v>1 for v in values) and med>=1.2:outcome='graph slower in pilot'
        ratios.append(dict(family=family,control=control,graph_over_control=values,median=med,outcome=outcome))
(OUT/'ratios.json').write_text(json.dumps(ratios,indent=2)+'\n')
cal=json.loads((OUT/'time-calibrate.log').read_text())
threshold=20*66*cal['median_ns']
assert all(r['min_ns']>=threshold for r in rows if r['kind']=='time'),threshold
print('All query-owned requested memory restored; lifecycle resolution threshold',threshold,'ns')
for r in rows:
 if r['kind']=='time':print(r['family'],r['mode'],'total_us',round(r['total_ns']/1000,2),'execution_us',round(r['execution_ns']/1000,2),'first_us_sum',round(r['first_ns_sum']/1000,2))
print('Memory deterministic:',all(r.get('memory_exact_replay',True) for r in rows))
for r in ratios:print(r['family'],r['control'],round(r['median'],3),r['outcome'])
