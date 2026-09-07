"""Audit complete E16 pilot outcomes and extract exploratory lifecycle figures."""
import hashlib
import json
from pathlib import Path
OUT = Path('docs/experiments/results')
rows = [json.loads(line) for line in (OUT/'E16-pilot.jsonl').read_text().splitlines()]
assert len(rows) == 116
assert len({(r['kind'],r['case'],r['mode'],r['limit']) for r in rows}) == 116
assert all(r['exit'] == 0 and 'parse_error' not in r and r['measurement']['pass'] == 'true' for r in rows)
source = ['source_steps','applications','introductions','equations','splits','source_failed','source_completed','source_duplicates']
work = ['calls','computed','hits','key_nodes','replay_nodes','owned_pairs','owned_resolve','owned_occurs','cache_entries']
for r in rows:
    m=r['measurement']
    assert len(m)==125
    assert m['case']==r['case'] and m['mode']==r['mode'] and int(m['outstanding_limit'])==r['limit']
    assert m['metered']==str(r['kind']=='memory').lower()
    assert all(m['stop_'+k]==m['joined_'+k] for k in source)
    if r['limit']:
        assert int(m['joined_issued'])==int(m['joined_received'])==int(m['joined_calls'])
        assert int(m['joined_issued'])-int(m['joined_committed_equations'])==int(m['joined_uncommitted_at_shutdown'])
        assert int(m['joined_outstanding'])==int(m['joined_buffered'])==0
        assert int(m['joined_max_outstanding'])<=r['limit']
        if r['case']=='prefix-drain': assert int(m['joined_uncommitted_at_shutdown'])==4
    if r['kind']=='memory':
        for phase in ['through_join','search_drop','output_drop']:
            assert int(m[phase+'_peak_live'])>=max(int(m[phase+'_baseline_live']),int(m[phase+'_final_live']))
        assert m['through_join_final_live']==m['search_drop_baseline_live']
        assert m['search_drop_final_live']==m['output_drop_baseline_live']
for case in {r['case'] for r in rows}:
    group=[r for r in rows if r['case']==case]
    assert len({tuple(r['measurement']['joined_'+k] for k in source) for r in group})==1
    for limit in {r['limit'] for r in group}:
        if not limit: continue
        matching=[r for r in group if r['limit']==limit]
        assert len({tuple(r['measurement']['joined_'+k] for k in work) for r in matching})==1
for t in rows[:58]:
    m=next(r for r in rows[58:] if (r['case'],r['mode'],r['limit'])==(t['case'],t['mode'],t['limit']))
    assert all(t['measurement']['joined_'+k]==m['measurement']['joined_'+k] for k in source+work)
summary={'rows':116,'time_rows':58,'memory_rows':58,'all_checks':'pass',
         'max_rss_kib':max(int(r['rss_report']) for r in rows),
         'max_metered_peak_bytes':max(int(r['measurement']['through_join_peak_live']) for r in rows[58:]),
         'observations':[]}
for r in rows[:58]:
    m=r['measurement'];mem=next(x['measurement'] for x in rows[58:] if (x['case'],x['mode'],x['limit'])==(r['case'],r['mode'],r['limit']))
    summary['observations'].append({'case':r['case'],'mode':r['mode'],'limit':r['limit'],
        'cold_through_search_drop_ms':int(m['search_dropped_ns'])/1e6,
        'first_answer_ms':int(m['first_answer_ns'])/1e6,
        'construct_ms':int(m['construct_ns'])/1e6,
        'search_ms':int(m['search_ns'])/1e6,
        'release_ms':(int(m['shutdown_ns'])+int(m['search_drop_ns']))/1e6,
        'heap_peak_bytes':int(mem['through_join_peak_live']),
        'heap_requested_bytes':int(mem['through_join_requested_bytes']),
        'post_search_live_bytes':int(mem['search_drop_final_live']),
        'post_output_live_bytes':int(mem['output_drop_final_live']),
        'max_outstanding':int(m['joined_max_outstanding']),
        'owned_pairs':int(m['joined_owned_pairs'])})
summary['sha256']={name:hashlib.sha256((OUT/name).read_bytes()).hexdigest() for name in ['E16-pilot.jsonl','E16-pilot-manifest.json']}
(OUT/'E16-pilot-audit.json').write_text(json.dumps(summary,indent=2)+'\n')
print({k:v for k,v in summary.items() if k!='observations'})
