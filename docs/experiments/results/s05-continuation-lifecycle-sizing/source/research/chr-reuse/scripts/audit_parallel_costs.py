"""Independent matrix-key, semantic/work and range audit for E16 cold costs."""
import hashlib
import json
import statistics
import random
from pathlib import Path

OUT = Path('docs/experiments/results')
CASES = ['wide-cheap','wide-medium','wide-large','wide-identity','chain-large',
         'skew-first','skew-last','mixed','prefix-drain','app-sk-duplication',
         'app-type-synthesis-prefix'] + [f'distinct-{s}-d-{d}' for d in [4,6,8] for s in ['wide','chain']]
CELLS = [(c,m,k) for c in CASES for m,k in [('Shared',0),('Owned',0),('Inline',4),('Threads1',4),('Threads2',4)]]
CELLS += [(c,m,1) for c in ['wide-large','distinct-wide-d-8'] for m in ['Inline','Threads1','Threads2']]
rows=[json.loads(line) for line in (OUT/'E16-costs.jsonl').read_text().splitlines()]
manifest=json.loads((OUT/'E16-costs-manifest.json').read_text())
# This expected matrix is constructed independently of the runner manifest.
expected={(phase,rep,c,m,k) for phase,n in [('warmup',2),('time',7),('memory',3)] for rep in range(n) for c,m,k in CELLS}
def key(r): return (r['phase'],r['rep'],r['case'],r['mode'],r['limit'])
assert len(rows)==len(expected)==1092
assert {key(r) for r in rows}==expected
assert [key(r) for r in rows]==[key(r) for r in manifest['order']]
expected_blocks=[(phase,rep) for phase,n in [('warmup',2),('time',7),('memory',3)] for rep in range(n) for _ in CELLS]
assert [(r['phase'],r['rep']) for r in rows]==expected_blocks
rng=random.Random(160916)
shuffled=[]
for phase,n in [('warmup',2),('time',7),('memory',3)]:
    for rep in range(n):
        cells=list(CELLS); rng.shuffle(cells)
        shuffled.extend((phase,rep,c,m,k) for c,m,k in cells)
assert [key(r) for r in rows]==shuffled and manifest['seed']==160916
source=['source_steps','applications','introductions','equations','splits','source_failed','source_completed','source_duplicates']
work=['calls','computed','hits','key_nodes','replay_nodes','owned_pairs','owned_resolve','owned_occurs','cache_entries']
broker=['lookahead_visits','projection_dereferences','issued','committed_equations','uncommitted_at_shutdown']
for r in rows:
    assert r['exit']==0 and 'parse_error' not in r, r
    m=r['measurement']; assert len(m)==125 and m['pass']=='true'
    assert (m['case'],m['mode'],int(m['outstanding_limit']))==(r['case'],r['mode'],r['limit'])
    assert m['metered']==str(r['phase']=='memory').lower()
    assert all(m['stop_'+k]==m['joined_'+k] for k in source)
    if r['limit']:
        assert int(m['joined_issued'])==int(m['joined_received'])==int(m['joined_calls'])
        assert int(m['joined_issued'])-int(m['joined_committed_equations'])==int(m['joined_uncommitted_at_shutdown'])
        assert int(m['joined_outstanding'])==int(m['joined_buffered'])==0
        assert int(m['joined_max_outstanding'])<=r['limit']
        if r['case']=='prefix-drain': assert int(m['joined_uncommitted_at_shutdown'])==4
    if r['case'].startswith('distinct-') and r['mode']!='Shared':
        d=int(r['case'].split('-')[-1]); pairs=8*2**(d+1) if '-wide-' in r['case'] else 8*(2**(d+1)-1)+1
        assert int(m['joined_owned_pairs'])==pairs
    if r['phase']=='memory':
        for p in ['through_join','search_drop','output_drop']:
            assert int(m[p+'_peak_live'])>=max(int(m[p+'_baseline_live']),int(m[p+'_final_live']))
        assert m['through_join_final_live']==m['search_drop_baseline_live']
        assert m['search_drop_final_live']==m['output_drop_baseline_live']
for c in CASES:
    group=[r for r in rows if r['case']==c]
    assert len({tuple(r['measurement']['joined_'+k] for k in source) for r in group})==1
    for limit in [1,4]:
        g=[r for r in group if r['limit']==limit]
        if g: assert len({tuple(r['measurement']['joined_'+k] for k in work+broker) for r in g})==1
for c,mode,k in CELLS:
    group=[r for r in rows if (r['case'],r['mode'],r['limit'])==(c,mode,k)]
    assert len({tuple(r['measurement']['joined_'+f] for f in work) for r in group})==1

def span(values):
    return {'min':min(values),'median':statistics.median(values),'max':max(values)}
summary={}
for c,mode,k in CELLS:
    group=[r for r in rows if (r['case'],r['mode'],r['limit'])==(c,mode,k)]
    time=[r['measurement'] for r in group if r['phase']=='time']
    memory=[r['measurement'] for r in group if r['phase']=='memory']
    value={'case':c,'mode':mode,'limit':k,'time_samples':len(time),'memory_samples':len(memory)}
    for f in ['construct_ns','first_answer_ns','search_ns','stop_ns','shutdown_ns','joined_ns','search_drop_ns','search_dropped_ns','output_drop_ns']:
        value[f]=span([int(r[f]) for r in time])
    value['release_ns']=span([int(r['shutdown_ns'])+int(r['search_drop_ns']) for r in time])
    for p in ['through_join','search_drop','output_drop']:
        for f in ['allocation_calls','requested_bytes','baseline_live','peak_live','final_live']:
            value[p+'_'+f]=span([int(r[p+'_'+f]) for r in memory])
    for phase in ['time','memory']:
        value[phase+'_max_rss_kib']=span([int(r['rss_report']) for r in group if r['phase']==phase])
    value['joined_work']={f:time[0]['joined_'+f] for f in source+work+broker}
    summary[(c,mode,k)]=value

def compare(c,a,b,metric):
    x,y=summary[(c,*a)][metric],summary[(c,*b)][metric]
    direction='b_lower' if y['max']<x['min'] else 'a_lower' if x['max']<y['min'] else 'overlap'
    return {'case':c,'a':a,'b':b,'metric':metric,'disposition':direction,'a_range':x,'b_range':y,'median_b_over_a':y['median']/x['median'] if x['median'] else None}
comparisons=[]
for c in CASES:
    for a,b in [(('Shared',0),('Threads1',4)),(('Shared',0),('Threads2',4)),(('Owned',0),('Threads1',4)),(('Owned',0),('Threads2',4)),(('Inline',4),('Threads1',4)),(('Inline',4),('Threads2',4)),(('Threads1',4),('Threads2',4))]:
        for metric in ['search_dropped_ns','first_answer_ns','through_join_peak_live']:
            comparisons.append(compare(c,a,b,metric))
for c in ['wide-large','distinct-wide-d-8']:
    for mode in ['Inline','Threads1','Threads2']:
        for metric in ['search_dropped_ns','first_answer_ns','through_join_peak_live']:
            comparisons.append(compare(c,(mode,1),(mode,4),metric))
result={'status':'pass','rows':1092,'warmup':182,'measured_time':637,'memory':273,
        'semantic_work':'exact registered keys, source/work agreement and complete shutdown checks pass',
        'cells':list(summary.values()),'comparisons':comparisons,
        'sha256':{name:hashlib.sha256((OUT/name).read_bytes()).hexdigest() for name in ['E16-costs.jsonl','E16-costs-manifest.json']}}
(OUT/'E16-costs-audit.json').write_text(json.dumps(result,indent=2)+'\n')
print({k:v for k,v in result.items() if k not in ['cells','comparisons']})
