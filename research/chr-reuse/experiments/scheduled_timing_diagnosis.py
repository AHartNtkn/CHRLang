"""Post-campaign diagnostics; never replace registered timing verdicts."""
import sys
sys.dont_write_bytecode=True
import gzip,json,statistics
from scheduled_timing import OUT,VARIANTS
rows=list(map(json.loads,gzip.open(OUT/'runs.jsonl.gz','rt')))
results=json.loads((OUT/'analysis.json').read_text())
cells={(tuple(r['job']['group']),r['job']['variant'],r['job']['block']):r['sessions'] for r in rows}
def total(sessions,without_disposal=False):
 return statistics.mean(sum(p['ns'] for p in d['phases'] if not (without_disposal and p['phase'].endswith('_drop'))) for d in sessions)
si=VARIANTS.index((True,'templates'));di=VARIANTS.index((False,'direct'))
bounds=[]
for g in sorted({k[0] for k in cells}):
 ratios=[total(cells[g,si,b],True)/total(cells[g,di,b]) for b in range(10)]
 bounds.append(dict(group=g,median=statistics.median(ratios),min=min(ratios),max=max(ratios)))
unresolved=[r for r in results if r['reference']==[False,'direct'] and r['candidate']==[True,'templates'] and r['verdict']=='unresolved']
contrary=[]
for r in unresolved:
 g=tuple(r['group']);ratios=[total(cells[g,si,b])/total(cells[g,di,b]) for b in range(10)];b=min(range(10),key=lambda b:ratios[b])
 contrary.append(dict(group=g,registered_median=r['median'],block=b,ratio=ratios[b],scheduled_session_ns=[sum(p['ns'] for p in d['phases']) for d in cells[g,si,b]],direct_session_ns=[sum(p['ns'] for p in d['phases']) for d in cells[g,di,b]]))
output=dict(disposal_bound_definition='Per block scheduled process mean with every *_drop phase set to zero, divided by unchanged Direct process mean. Descriptive impossible-free-disposal bound, not a timing verdict.',disposal_bounds=bounds,unresolved_direct_contrary_blocks=contrary)
(OUT/'diagnosis.json').write_text(json.dumps(output,indent=2)+'\n')
print('Zero-disposal scheduled/Direct median range:',min(r['median'] for r in bounds),max(r['median'] for r in bounds))
print(json.dumps(contrary,indent=2))
