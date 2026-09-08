#!/usr/bin/env python3
"""Summarize frozen S02 lifecycle endpoints; first latency is never added twice."""
import json,statistics,csv
from pathlib import Path
OUT=Path(__file__).resolve().parents[1]/'results/s01-script-repair'
rows=[json.loads(x) for x in (OUT/'raw.jsonl').read_text().splitlines()]
assert len(rows)==336 and all(r['exit_code']==0 for r in rows)
all_rows=rows
rows=[r for r in all_rows if r['version']=='corrected']
def phases(m):
 yield m['preparation'];yield m['prepared_disposal']
 for q in m['queries']:
  for k in ['setup','execution','observation','engine_disposal','answer_disposal']:yield q[k]
def lifecycle(r):return sum(p['ns'] for p in phases(r['measurement']))
summary=[]
for family in sorted({r['family'] for r in rows}):
 for mode in ['direct','retained','global-index','global-scan']:
  time=sorted([r for r in rows if r['family']==family and r['mode']==mode and r['kind']=='time'],key=lambda r:r['repetition'])
  mem=[r for r in rows if r['family']==family and r['mode']==mode and r['kind']=='memory']
  assert len(time)==5 and len(mem)==2
  # Live baselines include harness rows and strings; deterministic replay still
  # verifies each phase's requested traffic and net retention independently.
  diag=lambda r:[(p['memory']['allocation_calls'],p['memory']['requested_bytes'],p['memory']['deallocation_calls'],p['memory']['live_end']-p['memory']['live_start'],p['memory']['peak_live']-p['memory']['live_start']) for p in phases(r['measurement'])]
  assert diag(mem[0])==diag(mem[1]),(family,mode)
  ns=[lifecycle(r) for r in time]
  entry=dict(family=family,mode=mode,median_ms=statistics.median(ns)/1e6,min_ms=min(ns)/1e6,max_ms=max(ns)/1e6,requested_mib=sum(p['memory']['requested_bytes'] for p in phases(mem[0]['measurement']))/2**20,allocation_calls=sum(p['memory']['allocation_calls'] for p in phases(mem[0]['measurement'])))
  for component in ['preparation','prepared_disposal']:
   entry[component+'_ms']=statistics.median(r['measurement'][component]['ns'] for r in time)/1e6
  for component in ['setup','execution','observation','engine_disposal','answer_disposal']:
   entry[component+'_ms']=statistics.median(sum(q[component]['ns'] for q in r['measurement']['queries']) for r in time)/1e6
  for n in [4,8]:
   for rounds in [1,8]:
    entry[f'n{n}_r{rounds}_median_us']=statistics.median(sum(q[k]['ns'] for k in ['setup','execution','observation','engine_disposal','answer_disposal']) for r in time for q in r['measurement']['queries'] if q['n']==n and q['rounds']==rounds)/1000
  summary.append(entry)
ratios=[]
for family in sorted({r['family'] for r in rows}):
 for control in ['direct','global-index','global-scan']:
  def vals(mode):return {r['repetition']:lifecycle(r) for r in rows if r['kind']=='time' and r['mode']==mode and r['family']==family}
  a,b=vals('retained'),vals(control);paired=[a[i]/b[i] for i in range(5)]
  ratios.append(dict(family=family,control=control,median=statistics.median(paired),min=min(paired),max=max(paired),ratios=paired))
(OUT/'summary.json').write_text(json.dumps(dict(cells=summary,paired_ratios=ratios),indent=2)+'\n')
with (OUT/'summary.csv').open('w') as f:
 writer=csv.DictWriter(f,fieldnames=list(summary[0]),lineterminator='\n');writer.writeheader();writer.writerows(summary)
for r in ratios:print(r['family'],r['control'],round(r['median'],3),'range',round(r['min'],3),round(r['max'],3))
for r in summary:
 if r['mode'] in ['direct','retained']:print(r['family'],r['mode'],round(r['median_ms'],3),'ms',round(r['requested_mib'],3),'MiB')
print('All 32 allocation diagnostics replay exactly.')

corrections=[]
for family in sorted({r['family'] for r in all_rows}):
 for mode in ['direct','retained']:
  def selected(version,kind):return sorted([r for r in all_rows if r['family']==family and r['mode']==mode and r['version']==version and r['kind']==kind],key=lambda r:r['repetition'])
  before,after=selected('original','time'),selected('corrected','time')
  paired=[lifecycle(a)/lifecycle(b) for a,b in zip(after,before)]
  mb,ma=selected('original','memory'),selected('corrected','memory')
  assert diag(mb[0])==diag(mb[1])
  requested=lambda r:sum(p['memory']['requested_bytes'] for p in phases(r['measurement']))
  entry=dict(family=family,mode=mode,median_ratio=statistics.median(paired),min_ratio=min(paired),max_ratio=max(paired),ratios=paired,original_requested_bytes=requested(mb[0]),corrected_requested_bytes=requested(ma[0]))
  corrections.append(entry)
  print('CORRECTION',family,mode,round(entry['median_ratio'],3),round(entry['min_ratio'],3),round(entry['max_ratio'],3))
(OUT/'correction.json').write_text(json.dumps(corrections,indent=2)+'\n')
subcells=[]
for family in sorted({r['family'] for r in rows}):
 for n,rounds in [(4,1),(4,8),(8,1),(8,8)]:
  def values(mode):
   return {r['repetition']:sum(sum(q[k]['ns'] for k in ['setup','execution','observation','engine_disposal','answer_disposal']) for q in r['measurement']['queries'] if q['n']==n and q['rounds']==rounds) for r in rows if r['kind']=='time' and r['family']==family and r['mode']==mode}
  a,b=values('retained'),values('direct');paired=[a[i]/b[i] for i in range(5)]
  subcells.append(dict(family=family,n=n,rounds=rounds,median=statistics.median(paired),min=min(paired),max=max(paired),ratios=paired))
(OUT/'query-ratios.json').write_text(json.dumps(subcells,indent=2)+'\n')
