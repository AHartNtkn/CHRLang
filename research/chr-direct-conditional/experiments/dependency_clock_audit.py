"""Recheck clock receipts, normalized ownership and exploratory signal scale."""
import hashlib,json,statistics
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s03-dependency-clock'
def read(p):return json.loads(p.read_text())
def norm(v,baseline=0):
    if isinstance(v,list):return [norm(x,baseline) for x in v]
    if isinstance(v,dict):return {k:(x-baseline if k in {'live_start','live_end','peak_live'} else norm(x,baseline)) for k,x in v.items() if k not in {'ns','first_ns'}}
    return v
def raw(name):
    rows=[json.loads(x) for x in (BASE/'runs'/name).read_text().splitlines() if x.startswith('{')]
    return rows[-1]
def check(r,on):
    for i,e in enumerate(r['endpoints']):
        assert (e['first_ns'] is not None)==(on and e['answers']>0)
        duration=next(p['reading']['ns'] for p in r['phases'] if p['query']==i and p['phase']=='execute_observe')
        assert e['first_ns'] is None or 0<=e['first_ns']<=duration
freeze=read(BASE/'freeze.json')
for name,sha in freeze['sources'].items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==sha,name
for binary in freeze['binaries'].values():assert hashlib.sha256((ROOT/binary['path']).read_bytes()).hexdigest()==binary['sha256']
old={r['index']:r for r in read(ROOT/'docs/experiments/results/s03-dependency-ownership/results.json')}
qualification=read(BASE/'qualification.json');assert len(qualification)==80
expected={i for i,r in old.items() if not r['reverse'] and r['retention']=='immediate' and not r['cancel']}
assert {r['index'] for r in qualification}==expected
for cell in qualification:
    i=cell['index'];prior=old[i]['result'];assert len(cell['runs'])==6
    assert {(r['clock'],r['kind'],r['repeat']) for r in cell['runs']}=={(clock,kind,rep) for clock in ['off','on'] for kind,rep in [('meter',0),('meter',1),('ordinary',0)]}
    for row in cell['runs']:
        r=raw(f"qual-{i}-{row['clock']}-{row['kind']}-{row['repeat']}.log");assert r==row['result'];check(r,row['clock']=='on')
        assert norm(r['endpoints'])==prior['endpoints']
        if row['kind']=='meter':
            baseline=r['phases'][0]['reading']['memory']['live_start'];before=prior['phases'][0]['reading']['memory']['live_start']
            assert norm(r,baseline)==norm(prior,before)
extras=[r for r in old.values() if r['family'] in ['known-hit','delayed-miss'] and r['size']==8 and not r['reverse'] and r['retention'] in ['window','all'] and r['cancel']]
assert len(extras)==32
for cell in extras:
    r=raw(f"cancel-{cell['index']}.log");check(r,True);assert norm(r['endpoints'])==cell['result']['endpoints']
sizing=read(BASE/'sizing.json');assert len(sizing)==200
groups={};execution=[];phases=[]
for row in sizing:
    r=raw(f"sizing-{row['index']}-{row['repeat']}.log");assert r==row['result'];check(r,True)
    assert norm(r['endpoints'])==old[row['index']]['result']['endpoints']
    groups.setdefault(row['index'],[]).append(sum(p['reading']['ns'] for p in r['phases']))
    phases.extend(p['reading']['ns'] for p in r['phases'])
    execution.extend(p['reading']['ns'] for p in r['phases'] if p['phase']=='execute_observe')
assert len(groups)==40 and all(len(v)==5 for v in groups.values())
clocks=[raw(f'clock-{i}.log') for i in range(5)];assert clocks==read(BASE/'clocks.json')
assert len(list((BASE/'runs').glob('*.log')))==717
for c in clocks:assert c['samples']==100000 and 0<=c['min']<=c['median']<=c['p99']<=c['max']
ratio=min(execution)/max(c['median'] for c in clocks);assert ratio>=100
print(json.dumps({'workload_processes':712,'clock_processes':5,'meter_self_checks':1,'frozen_source_files':len(freeze['sources']),'normalized_ownership_matches':True,'first_owned_observation_valid':True,'min_execution_ns':min(execution),'min_session_ns':min(min(v) for v in groups.values()),'empty_medians_ns':[c['median'] for c in clocks],'empty_p99_ns':[c['p99'] for c in clocks],'empty_max_ns':max(c['max'] for c in clocks),'min_execution_over_largest_empty_median':ratio,'median_session_max_min':statistics.median(max(v)/min(v) for v in groups.values()),'largest_session_max_min':max(max(v)/min(v) for v in groups.values()),'smallest_phase_ns':min(phases),'primary_confirmation':False},indent=2))
