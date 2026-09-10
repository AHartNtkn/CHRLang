"""Registered descriptive screen; no inferential or workload-weighted ranking."""
from pathlib import Path
import collections,hashlib,json,statistics
root=Path(__file__).resolve().parents[3];out=root/'docs/experiments/results/s10-mixed-pilot'
v=json.loads((out/'validation.json').read_text())
for name,h in v['hashes'].items():assert hashlib.sha256((root/name).read_bytes()).hexdigest()==h,name
selection=json.loads((out/'selection.json').read_text())['batches']
rows=[json.loads(l) for l in (out/'runs.jsonl').read_text().splitlines()];measured=[r for r in rows if r['repetition']>=0]
assert len(measured)==v['measured'] and len(rows)-len(measured)==v['warmups']
by=collections.defaultdict(list)
for r in measured:by[r['batch'],r['mode'],r['cpu']].append(r)
for r in by.values():assert sorted(x['repetition'] for x in r)==list(range(5))
def values(batch,mode,cpu,metric):return [r[metric] for r in by[batch,mode,cpu]]
def median(batch,mode,metric):return statistics.median(values(batch,mode,0,metric)+values(batch,mode,1,metric))
def screen(batch,mode,metric):
 directions=[];details=[]
 for cpu in [0,1]:
  native=values(batch,'native',cpu,metric);rust=values(batch,mode,cpu,metric)
  ratio=statistics.median(native)/statistics.median(rust)
  direction='gain' if ratio<=.9 and max(native)<min(rust) else 'loss' if ratio>=1.1 and min(native)>max(rust) else 'unresolved'
  directions.append(direction);details.append(dict(cpu=cpu,ratio=ratio,native_range=[min(native),max(native)],rust_range=[min(rust),max(rust)]))
 return dict(direction=directions[0] if directions[0]==directions[1] else 'unresolved',cpu=details)
comparisons=[]
for batch,source in enumerate(selection):
 modes=sorted(m for b,m,c in by if b==batch and c==0 and m!='native')
 assert modes
 best=min(modes,key=lambda m:median(batch,m,'process_ns'))
 service_best=min(modes,key=lambda m:median(batch,m,'service_ns'))
 comparisons.append(dict(batch=batch,family=source['family'],group=source['group'],queries=len(source['sources']),admitted_rust=modes,best_process_rust=best,best_service_rust=service_best,native_process_median_ns=median(batch,'native','process_ns'),rust_process_median_ns=median(batch,best,'process_ns'),process=screen(batch,best,'process_ns'),service_against_process_control=screen(batch,best,'service_ns'),service_against_service_control=screen(batch,service_best,'service_ns')))
result=dict(processes=len(rows),measured=len(measured),comparisons=comparisons,process_screen=dict(collections.Counter(c['process']['direction'] for c in comparisons)),service_screen=dict(collections.Counter(c['service_against_service_control']['direction'] for c in comparisons)))
(out/'analysis.json').write_text(json.dumps(result,indent=2)+'\n')
print('Process screen:',result['process_screen'],'service screen:',result['service_screen'])
for c in comparisons:print(c['batch'],c['family'],c['group'],'control',c['best_process_rust'],'process ratio',round(c['native_process_median_ns']/c['rust_process_median_ns'],2),c['process']['direction'],'service',c['service_against_service_control']['direction'])
