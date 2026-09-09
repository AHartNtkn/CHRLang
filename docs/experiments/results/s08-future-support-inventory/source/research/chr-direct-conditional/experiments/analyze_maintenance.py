#!/usr/bin/env python3
"""Summarize all registered cells, retaining failure and phase boundaries."""
import json, pathlib, statistics
root=pathlib.Path(__file__).resolve().parents[3]
out=root/'docs/experiments/results/r03-conditional-maintenance'
rows=[json.loads(line) for line in (out/'runs.jsonl').read_text().splitlines()]
assert len(rows)==384,len(rows)
fail=[r for r in rows if r.get('exit')!=0 or 'result' not in r]
assert not fail,fail
for r in rows:
 d=r['result'];assert d['metrics']==(r['mode']=='work');assert d['allocation_meter']==(r['mode']=='allocation')
 assert len(d['samples'])==r['cell'][3]
 for i,s in enumerate(d['samples']):
  assert s['size']==r['cell'][2]+i%2
  assert s['answers']==(1 if d['family']=='plain' else (1<<s['size']) if d['family']=='output' else 16)
  assert (s['work'] is not None)==(r['mode']=='work')
def total(d):
 return (d['preparation']['ns']+d['prepared_disposal']['ns']+sum(s[p]['ns'] for s in d['samples'] for p in ['setup','execution_observation','engine_disposal']))/len(d['samples'])/1e6
summary=[]
for family,n,q in sorted({tuple(r['cell'][1:]) for r in rows}):
 item={'family':family,'size':n,'queries':q}
 for backend in ['baseline','conditional','explicit']:
  subset=[r for r in rows if r['cell']==[backend,family,n,q]]
  primary=[r['result'] for r in subset if r['mode']=='primary'];assert len(primary)==5
  vals=[total(d) for d in primary]
  memory=next(r['result'] for r in subset if r['mode']=='allocation')
  work=next(r['result'] for r in subset if r['mode']=='work')
  phases=[memory['preparation'],memory['prepared_disposal']]+[s[p] for s in memory['samples'] for p in ['setup','execution_observation','engine_disposal']]
  item[backend]={'median_ms':statistics.median(vals),'min_ms':min(vals),'max_ms':max(vals),'first_ms':statistics.median(d['samples'][0]['first_answer_ns']/1e6 for d in primary),'prepare_ms':statistics.median(d['preparation']['ns']/1e6 for d in primary),'answer_disposal_ms_per_query':statistics.median(sum(s['answer_disposal_after_validation']['ns'] for s in d['samples'])/q/1e6 for d in primary),'requested_bytes_per_query':sum(p['memory']['requested_bytes'] for p in phases)/q,'peak_requested_live_bytes':max(p['memory']['peak_live'] for p in phases),'work':work['samples'][0]['work']}
 item['conditional_over_baseline']=item['conditional']['median_ms']/item['baseline']['median_ms']
 item['conditional_over_explicit']=item['conditional']['median_ms']/item['explicit']['median_ms']
 summary.append(item)
(out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
lines=['family\tsize\tqueries\tconditional_ms\texplicit_ms\tratio\tconditional_requested_bytes\texplicit_requested_bytes\tconditional_apps\texplicit_apps']
for r in summary:
 c=r['conditional'];e=r['explicit'];lines.append(f"{r['family']}\t{r['size']}\t{r['queries']}\t{c['median_ms']:.6f}\t{e['median_ms']:.6f}\t{r['conditional_over_explicit']:.3f}\t{c['requested_bytes_per_query']:.0f}\t{e['requested_bytes_per_query']:.0f}\t{c['work']['applications']}\t{e['work']['applications']}")
(out/'summary.tsv').write_text('\n'.join(lines)+'\n')
print('\n'.join(lines))
print('baseline ratios:',[(r['family'],r['size'],r['queries'],round(r['conditional_over_baseline'],3)) for r in summary])
print('validated process queries:',sum(len(r['result']['samples']) for r in rows),'answers:',sum(s['answers'] for r in rows for s in r['result']['samples']))
