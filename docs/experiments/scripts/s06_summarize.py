#!/usr/bin/env python3
"""S06 registered lifecycle and within-block ratios, preserving reuse strata."""
import json,statistics,csv
from pathlib import Path
OUT=Path(__file__).resolve().parents[1]/'results/s06-table-lifecycle'
rows=[json.loads(x) for x in (OUT/'raw.jsonl').read_text().splitlines()]
assert len(rows)==252 and all(r['exit_code']==0 for r in rows)
def phases(m):
 yield m['preparation'];yield m['prepared_disposal']
 for q in m['queries']:
  for k in ['setup','execution_observation','engine_disposal','answer_disposal']:yield q[k]
def lifecycle(r):return sum(p['ns'] for p in phases(r['measurement']))
def diagnostics(r):return [(p['memory']['allocation_calls'],p['memory']['requested_bytes'],p['memory']['deallocation_calls'],p['memory']['live_end']-p['memory']['live_start'],p['memory']['peak_live']-p['memory']['live_start']) for p in phases(r['measurement'])]
cells=[];ratios=[]
for family in sorted({r['family'] for r in rows}):
 for reuse in [1,16]:
  def selected(mode,kind):return sorted([r for r in rows if (r['family'],r['reuse'],r['mode'],r['kind'])==(family,reuse,mode,kind)],key=lambda r:r['repetition'])
  for mode in ['direct','global-scan','global-index']:
   time,mem=selected(mode,'time'),selected(mode,'memory');assert len(time)==5 and len(mem)==2
   assert diagnostics(mem[0])==diagnostics(mem[1]),(family,reuse,mode)
   ns=[lifecycle(r) for r in time]
   e=dict(family=family,reuse=reuse,mode=mode,median_ms=statistics.median(ns)/1e6,min_ms=min(ns)/1e6,max_ms=max(ns)/1e6,requested_mib=sum(p['memory']['requested_bytes'] for p in phases(mem[0]['measurement']))/2**20)
   for k in ['preparation','prepared_disposal']:e[k+'_ms']=statistics.median(r['measurement'][k]['ns'] for r in time)/1e6
   for k in ['setup','execution_observation','engine_disposal','answer_disposal']:e[k+'_ms']=statistics.median(sum(q[k]['ns'] for q in r['measurement']['queries']) for r in time)/1e6
   e['answers']=sum(q['count'] for q in time[0]['measurement']['queries'])
   assert all(sum(q['count'] for q in r['measurement']['queries'])==e['answers'] for r in time+mem)
   cells.append(e)
  for mode in ['global-scan','global-index']:
   a,b=selected('direct','time'),selected(mode,'time');paired=[lifecycle(x)/lifecycle(y) for x,y in zip(a,b)]
   ratios.append(dict(family=family,reuse=reuse,control=mode,median=statistics.median(paired),min=min(paired),max=max(paired),ratios=paired))
   print(family,reuse,mode,round(statistics.median(paired),4),round(min(paired),4),round(max(paired),4))
(OUT/'summary.json').write_text(json.dumps(dict(cells=cells,paired_ratios=ratios),indent=2)+'\n')
with (OUT/'summary.csv').open('w') as f:
 w=csv.DictWriter(f,fieldnames=list(cells[0]),lineterminator='\n');w.writeheader();w.writerows(cells)
print('All 36 allocation diagnostics replay exactly.')
