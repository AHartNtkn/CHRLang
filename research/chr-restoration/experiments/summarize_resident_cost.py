#!/usr/bin/env python3
"""Audit the fixed S04 matrix and summarize paired primary endpoints."""
import csv
import hashlib
import json
from pathlib import Path
import statistics as st
import subprocess
from collections import defaultdict
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s04-resident-cost'
order=json.loads((OUT/'order.json').read_text())
freeze=json.loads((OUT/'freeze.json').read_text())
for f,h in freeze['sources'].items():
    source=(ROOT/f).read_bytes()
    assert hashlib.sha256(source).hexdigest()==h,f
for b in freeze['binaries'].values(): assert hashlib.sha256(Path(b['path']).read_bytes()).hexdigest()==b['sha256']

def phases(value):
    return [value['preparation']]+[q[p] for q in value['queries'] for p in ['setup','execution_observation','engine_disposal','answer_disposal']]+[value['prepared_disposal']]

def diagnostics(value):
    return [p['memory'] for p in phases(value)]+[value['cancellation'][p]['memory'] for p in ['setup','execution_observation','engine_disposal','answer_disposal']]

groups=defaultdict(lambda:defaultdict(list))
rows=[]
for cell in order:
    name=f"{cell['kind']}-{cell['rep']}-{cell['family']}-{cell['mode']}-{cell['reuse']}"
    receipt=json.loads((OUT/(name+'.json')).read_text())
    key=('cow-' if cell['mode']=='indexed-cow' else '')+cell['kind']
    expected_command=[freeze['binaries'][key]['path'], 'indexed' if cell['mode']=='indexed-cow' else cell['mode'], cell['family'], str(cell['reuse'])]
    assert receipt['command']==expected_command
    assert receipt['exit_code']==0 and not receipt.get('cutoff')
    value=json.loads(receipt['stdout'])
    assert value['family']==cell['family'] and value['reuse']==cell['reuse']
    assert value['mode']==('indexed' if cell['mode']=='indexed-cow' else cell['mode'])
    assert value['meter']==(cell['kind']=='meter') and value['cow']==(cell['mode']=='indexed-cow')
    assert len(value['queries'])==cell['reuse']
    counts={'linear':1,'small':8,'retained':8,'mutation':8,'work':8,'deep':64,'spine':7,'early':1,'late':1,'compatible-small':1,'compatible-large':1,'compatible-alias':1}
    assert all(q['count']==counts[cell['family']] and q['first_ns'] is not None for q in value['queries'])
    ps=phases(value)
    assert all(p['ns']>=0 for p in ps)
    for phase in ps+list(value['cancellation'].values()):
        if cell['kind']=='time':
            assert phase['memory'] is None
        else:
            memory=phase['memory']
            assert all(isinstance(n,int) and n>=0 for n in memory.values())
            assert memory['peak_live']>=max(memory['live_start'],memory['live_end'])
    assert all(0<=q['first_ns']<=q['execution_observation']['ns'] for q in value['queries'])
    base=value['preparation']['memory']
    row={**cell,'total_ns':sum(p['ns'] for p in ps),
         'execution_ns':sum(q['execution_observation']['ns'] for q in value['queries']),
         'setup_ns':sum(q['setup']['ns'] for q in value['queries']),
         'preparation_ns':value['preparation']['ns'],
         'disposal_ns':value['prepared_disposal']['ns']+sum(q['engine_disposal']['ns']+q['answer_disposal']['ns'] for q in value['queries']),
         'first_prepared_owner_ns':value['preparation']['ns']+value['queries'][0]['setup']['ns']+value['queries'][0]['first_ns'],
         'cancel_ns':sum(p['ns'] for p in value['cancellation'].values()),
         'requested_bytes':sum(p['memory']['requested_bytes'] for p in ps) if base else None,
         'peak_increment_bytes':max(p['memory']['peak_live'] for p in ps)-base['live_start'] if base else None}
    if base:
        chronological=[value['preparation']]+[q[p] for q in value['queries'] for p in ['setup','execution_observation','engine_disposal','answer_disposal']]+[value['cancellation'][p] for p in ['setup','execution_observation','engine_disposal','answer_disposal']]+[value['prepared_disposal']]
        assert all(a['memory']['live_end']==b['memory']['live_start'] for a,b in zip(chronological,chronological[1:])),(cell,'unmeasured retention')
        assert value['prepared_disposal']['memory']['live_end']==base['live_start']
    groups[(cell['family'],cell['mode'],cell['reuse'])][cell['kind']].append((cell['rep'],row,value))
    rows.append(row)
assert len(rows)==1344 and len(groups)==192
summaries=[]
for (family,mode,reuse),g in sorted(groups.items()):
    assert sorted(rep for rep,_,_ in g['time'])==list(range(5))
    assert sorted(rep for rep,_,_ in g['meter'])==list(range(2))
    assert diagnostics(g['meter'][0][2])==diagnostics(g['meter'][1][2]),(family,mode,reuse,'allocation replay')
    time={rep:r for rep,r,_ in g['time']}
    item={'family':family,'mode':mode,'reuse':reuse}
    for field in ['total_ns','execution_ns','setup_ns','preparation_ns','disposal_ns','first_prepared_owner_ns','cancel_ns']:
        item[field]=st.median(r[field] for r in time.values())
        item[field+'_range']=[min(r[field] for r in time.values()),max(r[field] for r in time.values())]
    for field in ['requested_bytes','peak_increment_bytes']:
        item[field]=g['meter'][0][1][field]
    for control in ['copy','indexed','indexed-cow']:
        refs={rep:r for rep,r,_ in groups[(family,control,reuse)]['time']}
        ratios=[time[i]['total_ns']/refs[i]['total_ns'] for i in range(5)]
        item['vs_'+control]=dict(median=st.median(ratios),minimum=min(ratios),maximum=max(ratios))
    summaries.append(item)
with (OUT/'phases.csv').open('w') as f:
    w=csv.DictWriter(f,fieldnames=rows[0].keys(),lineterminator='\n');w.writeheader();w.writerows(rows)
(OUT/'summary.json').write_text(json.dumps(summaries,indent=2)+'\n')
(OUT/'audit.json').write_text(json.dumps({'processes':len(rows),'cells':len(groups),'exact_allocation_replays':len(groups),'source_and_binary_freeze':True,'all_complete_observations_validated_in_runner':True,'all_phase_lifecycle_allocations_restored':True},indent=2)+'\n')
for family in ['small','retained','mutation','work','deep','spine','early','late','linear']:
    candidates=[s for s in summaries if s['family']==family and s['reuse']==8]
    print(family,', '.join(f"{s['mode']} {s['total_ns']/1e6:.3f}ms/{s['requested_bytes']/1048576:.2f}MiB" for s in candidates))
