#!/usr/bin/env python3
import json,statistics,csv
from pathlib import Path
OUT=Path(__file__).resolve().parents[1]/'results/s01-request-crossover'
rows=[json.loads(x) for x in (OUT/'raw.jsonl').read_text().splitlines()]
assert len(rows)==288 and all(r['exit_code']==0 for r in rows)
def phases(m):
 yield m['preparation'];yield m['prepared_disposal']
 for q in m['queries']:
  for k in ['setup','execution','observation','engine_disposal','answer_disposal']:yield q[k]
def total(r):return sum(p['ns'] for p in phases(r['measurement']))
def diag(r):return [(p['memory']['allocation_calls'],p['memory']['requested_bytes'],p['memory']['deallocation_calls'],p['memory']['live_end']-p['memory']['live_start'],p['memory']['peak_live']-p['memory']['live_start']) for p in phases(r['measurement'])]
cells=[];ratios=[]
for family in ['stable-selective','consume-dense']:
 for n in [8,32]:
  for rounds in [1,8,64]:
   def select(mode,kind):return sorted([r for r in rows if (r['family'],r['n'],r['rounds'],r['mode'],r['kind'])==(family,n,rounds,mode,kind)],key=lambda r:r['repetition'])
   for mode in ['direct','retained']:
    time,mem=select(mode,'time'),select(mode,'memory');assert len(time)==10 and len(mem)==2;assert diag(mem[0])==diag(mem[1])
    values=[total(r) for r in time]
    entry=dict(family=family,n=n,rounds=rounds,mode=mode,median_ms=statistics.median(values)/1e6,min_ms=min(values)/1e6,max_ms=max(values)/1e6,requested_mib=sum(p['memory']['requested_bytes'] for p in phases(mem[0]['measurement']))/2**20)
    for k in ['preparation','prepared_disposal']:entry[k+'_ms']=statistics.median(r['measurement'][k]['ns'] for r in time)/1e6
    for k in ['setup','execution','observation','engine_disposal','answer_disposal']:entry[k+'_ms']=statistics.median(r['measurement']['queries'][0][k]['ns'] for r in time)/1e6
    cells.append(entry)
   a,b=select('retained','time'),select('direct','time');paired=[total(x)/total(y) for x,y in zip(a,b)]
   result=dict(family=family,n=n,rounds=rounds,median=statistics.median(paired),min=min(paired),max=max(paired),ratios=paired)
   ratios.append(result);print(family,n,rounds,round(result['median'],3),round(result['min'],3),round(result['max'],3))
(OUT/'summary.json').write_text(json.dumps(dict(cells=cells,paired_ratios=ratios),indent=2)+'\n')
with (OUT/'summary.csv').open('w') as f:
 w=csv.DictWriter(f,fieldnames=list(cells[0]),lineterminator='\n');w.writeheader();w.writerows(cells)
print('All 24 allocation cells replay exactly.')
