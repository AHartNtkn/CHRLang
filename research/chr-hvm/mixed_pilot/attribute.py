"""Attribute existing observations without treating hypothetical savings as measurements."""
from pathlib import Path
import json,statistics
root=Path(__file__).resolve().parents[3];out=root/'docs/experiments/results/s10-mixed-pilot'
rows=[json.loads(l) for l in (out/'runs.jsonl').read_text().splitlines()];rows=[r for r in rows if r['repetition']>=0]
analysis=json.loads((out/'analysis.json').read_text());result=[]
for c in analysis['comparisons']:
 batch=c['batch'];native=[r for r in rows if r['batch']==batch and r['mode']=='native'];rust=[r for r in rows if r['batch']==batch and r['mode']==c['best_process_rust']]
 events=[json.loads(r['result']['stderr']) for r in native]
 bound=[]
 for cpu in [0,1]:
  n=[r['service_ns'] for r in native if r['cpu']==cpu];r=[r['process_ns'] for r in rust if r['cpu']==cpu]
  bound.append(min(n)>max(r))
 result.append(dict(batch=batch,family=c['family'],group=c['group'],native_service_alone_exceeds_rust_whole_process_both_cpus=all(bound),native_median_ns={
 'process':statistics.median(r['process_ns'] for r in native),
 'service':statistics.median(r['service_ns'] for r in native),
 'session':statistics.median(e['session_elapsed_ns'] for e in events),
 'native_invocation':statistics.median(e['phases']['native_process_ns'] for e in events),
 'emission':statistics.median(e['phases']['emission_ns'] for e in events),
 'outside_host_session':statistics.median(r['process_ns']-e['session_elapsed_ns'] for r,e in zip(native,events))},rust_process_median_ns=statistics.median(r['process_ns'] for r in rust)))
assert all(r['native_service_alone_exceeds_rust_whole_process_both_cpus'] for r in result if r['family']=='substantive')
(out/'attribution.json').write_text(json.dumps(result,indent=2)+'\n')
print('All four substantive batches remain slower on both CPUs even if every native non-service cost were zero')
