"""Supplementary post-run audit of existing pilot lifecycle fields; no reruns."""
import json
from pathlib import Path
out = Path('docs/experiments/results')
rows = [json.loads(s) for s in (out/'E16-regions-pilot.jsonl').read_text().splitlines()]
owner = ['certificate_predicates','certificate_edges','certificate_terms','owner_steps','owner_source_turns','product_jobs_created','products','duplicates','renamed_nodes','peak_product_jobs','empty_refutations']
for r in rows:
 m=r['measurement']; n=lambda k:int(m[k]); q=r['quantum']
 for k in owner + ['factor_count','cached_answers','pending_jobs']:
  assert m['stop_'+k] == m['joined_'+k], (r['case'],k)
 assert n('operational_ns') == sum(n(k+'_ns') for k in ['construct','search','shutdown','search_drop'])
 assert n('construct_ns') <= n('wall_stop_ns') <= n('wall_joined_ns') <= n('wall_through_drop_including_diagnostics_ns')
 assert n('wall_stop_ns') == n('construct_ns') + n('search_ns')
 if m['has_first']=='true': assert n('construct_ns') <= n('first_answer_ns') <= n('wall_stop_ns')
 else: assert n('first_answer_ns') == n('answers') == 0
 if r['mode'] != 'Baseline':
  for p in ['stop_','joined_']:
   t=lambda k:n(p+'transport_'+k)
   assert t('accepted') == n(p+'owner_source_turns')
   assert t('accepted') <= t('received') <= t('issued')
   assert t('issued')-t('received') == t('outstanding')-t('buffered')
   assert t('received')-t('accepted') == t('buffered')+t('unaccepted_at_shutdown')
   assert t('owner_buffered_peak') <= t('max_outstanding') <= min(r['limit'],n(p+'factor_count'))
   assert t('accepted') <= n(p+'accepted_source_steps') <= q*t('accepted')
  assert n('joined_transport_cancelled_requests') <= n('joined_transport_unaccepted_at_shutdown')
  assert n('joined_transport_cancelled_requests') <= n('joined_transport_unserved_quantum_slots') <= q*n('joined_transport_cancelled_requests')
  assert n('joined_actual_source_steps')+n('joined_transport_unserved_quantum_slots') <= q*n('joined_transport_issued')
 if r['kind']=='memory':
  for p in ['through_join','search_drop','output_drop']:
   assert n(p+'_peak_live') >= max(n(p+'_baseline_live'),n(p+'_final_live'))
  assert n('through_join_final_live') == n('search_drop_baseline_live')
  assert n('search_drop_final_live') == n('output_drop_baseline_live')
 else:
  for name in m:
   if name.endswith(('_allocation_calls','_requested_bytes','_baseline_live','_peak_live','_final_live')): assert n(name)==0
result={'pass':True,'children':len(rows),'purpose':'post-run supplementary lifecycle/accounting checks; no comparative runs'}
(out/'E16-regions-pilot-lifecycle-audit.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result))
