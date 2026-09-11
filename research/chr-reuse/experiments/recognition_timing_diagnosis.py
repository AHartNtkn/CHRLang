"""Post-campaign bounded interpretation without changing primary verdicts."""
import sys
sys.dont_write_bytecode=True
import gzip,json,statistics
from recognition_timing import OUT,MODES
rows=list(map(json.loads,gzip.open(OUT/'runs.jsonl.gz','rt')))
cells={(tuple(r['job']['group']),r['job']['variant'],r['job']['block']):r['rows'][1:] for r in rows}
def total(rows,free_disposal=False):return sum(r['ns'] for r in rows if not (free_disposal and r['phase'].endswith('_dispose')))
bounds=[]
for mode in ['memo4','memo16']:
 i=MODES.index(mode)
 for g in sorted({k[0] for k in cells}):
  ratios=[total(cells[g,i,b],True)/total(cells[g,MODES.index('direct'),b]) for b in range(10)]
  bounds.append(dict(mode=mode,group=g,median=statistics.median(ratios),min=min(ratios),max=max(ratios)))
g=(1,4,1,'all',1,False)
case=dict(group=g,median_session_ns={m:statistics.median(total(cells[g,i,b]) for b in range(10)) for i,m in enumerate(MODES)})
q=[total(cells[g,1,b])/total(cells[g,7,b]) for b in range(10)]
case['exact_reverse_stride4_over_sealed']=dict(median=statistics.median(q),min=min(q),max=max(q))
result=dict(disposal_bound='Every *_dispose phase of the stride candidate is set to zero; Direct unchanged. Descriptive bound, not a new primary verdict.',bounds=bounds,sealed_advantage_case=case)
(OUT/'diagnosis.json').write_text(json.dumps(result,indent=2)+'\n')
for m in ['memo4','memo16']:
 q=[x['median'] for x in bounds if x['mode']==m];print(m,'free disposal median range',min(q),max(q),'below one',sum(x<1 for x in q))
