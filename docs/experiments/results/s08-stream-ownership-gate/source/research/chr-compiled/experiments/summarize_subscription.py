#!/usr/bin/env python3
"""Audit registered source, lifecycle restoration, allocation replay and paired costs."""
from pathlib import Path
from collections import defaultdict
import hashlib,json,statistics,csv,subprocess
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s01-subscription-lifecycle'
freeze=json.loads((OUT/'freeze.json').read_text())
for f,h in freeze['sources'].items():assert hashlib.sha256(subprocess.check_output(['git','show',f"{freeze['source_commit']}:{f}"],cwd=ROOT)).hexdigest()==h,f
for b in freeze['binaries'].values():assert hashlib.sha256(Path(b['path']).read_bytes()).hexdigest()==b['sha256']
order=json.loads((OUT/'order.json').read_text());assert len(order)==1008
rows=[];groups=defaultdict(lambda:defaultdict(list))
def primary(v):return [v['preparation']]+[q[k] for q in v['queries'] for k in ['setup','execution','observation','engine_disposal','answer_disposal']]+[v['prepared_disposal']]
def all_phases(v):return primary(v)+list(v['cancellation'].values())
for cell in order:
    k,rep,f,n,r,m=[cell[x] for x in ['kind','rep','family','n','rounds','mode']]
    receipt=json.loads((OUT/f'{k}-{rep}-{f}-{n}-{r}-{m}.json').read_text())
    assert receipt['command']==[freeze['binaries'][k]['path'],'cell',m,f,str(n),str(r)]
    assert receipt['exit_code']==0 and not receipt.get('cutoff')
    v=json.loads(receipt['stdout'])
    assert (v['mode'],v['family'],v['n'],v['rounds'],v['meter'])==(m,f,n,r,k=='meter')
    assert len(v['queries'])==2
    for phase in all_phases(v):
        assert phase['ns']>=0
        mem=phase['memory']
        if k=='time':assert mem is None
        else:
            assert all(isinstance(x,int) and x>=0 for x in mem.values())
            assert mem['peak_live']>=max(mem['live_start'],mem['live_end'])
    if k=='meter':
        base=v['preparation']['memory']
        for q in v['queries']:
            assert q['setup']['memory']['live_start']==base['live_end']
            assert q['answer_disposal']['memory']['live_end']==base['live_end']
        c=v['cancellation'];assert c['setup']['memory']['live_start']==base['live_end']
        assert c['disposal']['memory']['live_end']==base['live_end']
        assert v['prepared_disposal']['memory']['live_start']==base['live_end']
        assert v['prepared_disposal']['memory']['live_end']==base['live_start']
    q=v['queries'][0]
    item={**cell,'total_ns':sum(p['ns'] for p in primary(v)),
          'preparation_ns':v['preparation']['ns'],
          'setup_ns':sum(q['setup']['ns'] for q in v['queries']),
          'execution_ns':sum(q['execution']['ns'] for q in v['queries']),
          'observation_ns':sum(q['observation']['ns'] for q in v['queries']),
          'disposal_ns':v['prepared_disposal']['ns']+sum(q['engine_disposal']['ns']+q['answer_disposal']['ns'] for q in v['queries']),
          'first_cold_ns':v['preparation']['ns']+q['setup']['ns']+q['execution']['ns']+q['observation']['ns'],
          'cancellation_ns':sum(p['ns'] for p in v['cancellation'].values()),
          'requested_bytes':sum(p['memory']['requested_bytes'] for p in primary(v)) if k=='meter' else None,
          'peak_increment_bytes':max(p['memory']['peak_live'] for p in primary(v))-v['preparation']['memory']['live_start'] if k=='meter' else None}
    rows.append(item);groups[(f,n,r,m)][k].append((rep,item,v))
assert len(groups)==144
summary=[]
for (f,n,r,m),g in sorted(groups.items()):
    assert sorted(x[0] for x in g['time'])==list(range(5))
    assert sorted(x[0] for x in g['meter'])==list(range(2))
    assert [p['memory'] for p in all_phases(g['meter'][0][2])]==[p['memory'] for p in all_phases(g['meter'][1][2])],(f,n,r,m,'allocation replay')
    item=dict(family=f,n=n,rounds=r,mode=m)
    for key in ['total_ns','preparation_ns','setup_ns','execution_ns','observation_ns','disposal_ns','first_cold_ns','cancellation_ns']:item[key]=statistics.median(x[1][key] for x in g['time'])
    for key in ['requested_bytes','peak_increment_bytes']:item[key]=g['meter'][0][1][key]
    values={rep:row for rep,row,_ in g['time']}
    for control in ['indexed','eager','subscribed','compiled']:
        refs={rep:row for rep,row,_ in groups[(f,n,r,control)]['time']}
        pairs=[values[i]['total_ns']/refs[i]['total_ns'] for i in range(5)]
        item['vs_'+control]=dict(median=statistics.median(pairs),minimum=min(pairs),maximum=max(pairs),pairs=pairs)
    summary.append(item)
(OUT/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
with (OUT/'phases.csv').open('w') as file:
    w=csv.DictWriter(file,fieldnames=rows[0].keys());w.writeheader();w.writerows(rows)
(OUT/'audit.json').write_text(json.dumps(dict(processes=len(rows),cells=len(groups),exact_allocation_replays=len(groups),source_and_binary_freeze=True,complete_answers_validated_by_runner=True,query_cancellation_and_prepared_lifetimes_restored=True),indent=2)+'\n')
for s in summary:
    if s['mode']=='subscribed':
        ratio=s['vs_indexed'];print(s['family'],s['n'],s['rounds'],f"{s['total_ns']/1e6:.3f}ms ratio {ratio['median']:.3f} [{ratio['minimum']:.3f},{ratio['maximum']:.3f}]")
