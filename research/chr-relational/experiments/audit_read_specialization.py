"""Reconstruct frozen raw processes and the registered paired comparisons."""
import collections, hashlib, json, statistics
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
BASE=ROOT/'docs/experiments/results/s02-read-specialization'
def read(path):return json.loads(path.read_text())
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def phases(v):
    result=[v['source_build'],v['preparation']]
    for sample in v['samples']:
        result += [sample[k] for k in ['input_build','setup','execute_observe','engine_drop','answer_drop','answer_hold'] if sample[k] is not None]
    return result+[v['prepared_drop'],v['consumer_drop']]
def normalize(v, semantic=False):
    baseline=0 if semantic else v['source_build']['memory']['live_start']
    def walk(x):
        if isinstance(x,list):return [walk(y) for y in x]
        if isinstance(x,dict):return {k:(y-baseline if k in ['live_start','live_end','peak_live'] else walk(y)) for k,y in x.items() if k not in ['ns','first_answer_ns','attribution'] and not (semantic and k in ['memory','meter'])}
        return x
    return walk(v)
f=read(BASE/'freeze.json')
for path,digest in f['sources'].items():assert sha(ROOT/path)==digest,path
parent=read(ROOT/'docs/experiments/results/s02-read-cost/freeze.json')
for path,digest in parent['sources'].items():assert any(p.exists() and sha(p)==digest for p in [ROOT/path,ROOT/'docs/experiments/results/s02-read-cost/source-snapshot'/path]),path
for binary in f['binaries'].values():assert sha(Path(binary['path']))==binary['sha256']
assert sha(BASE/'order.json')==f['order_hash'] and sha(BASE/'cases.json')==f['cases_hash']
cases=read(BASE/'cases.json');order=read(BASE/'order.json')
rows=[json.loads(x) for x in (BASE/'results.jsonl').read_text().splitlines()]
assert len(rows)==len(order)==len(list((BASE/'runs').glob('*.json')))==190
clocks=[]
for i in range(5):
    raw=read(BASE/f'clock-{i}.json');assert raw['exit_code']==0 and raw['args']==['clock-check']
    clocks.append(json.loads(raw['stdout'])['median_ns'])
floor=max(clocks);groups=collections.defaultdict(list)
for i,(row,item) in enumerate(zip(rows,order)):
    assert row['index']==i and all(row[k]==v for k,v in item.items())
    raw=read(BASE/'runs'/f'{i}.json');assert raw['exit_code']==0 and not raw['stderr']
    v=row['result'];assert [json.loads(x) for x in raw['stdout'].splitlines() if x.startswith('{')][-1]==v
    family,depth,q=cases[item['case']];q=4 if item['cancel'] else q
    assert raw['args']==[item['mode'],family,depth,q,1,0]+([1] if item['cancel'] else [])
    assert v['mode']==item['mode'] and v['family']==family and v['resource'] and v['retained'] and not v['counters']
    assert v['meter']==(item['build']=='meter')
    assert [s['complete'] for s in v['samples']]==([False,True,False,True] if item['cancel'] else [True]*q)
    n=int(family.rsplit('-',1)[1]);answer_count=n//2 if 'mixed' in family else n
    assert [s['answers'] for s in v['samples']]==([0,answer_count,0,answer_count] if item['cancel'] else [answer_count]*q)
    ps=phases(v);row['total_ns']=sum(p['ns'] for p in ps);row['threshold_ns']=100*floor*(len(ps)+q)
    if item['build']=='meter':
        ms=[p['memory'] for p in ps];baseline=ms[0]['live_start']
        assert ms[-1]['live_end']==baseline and all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]))
        row['traffic']=sum(m['requested_bytes'] for m in ms);row['peak']=max(m['peak_live'] for m in ms)-baseline
    else:assert all(p['memory'] is None for p in ps)
    groups[item['case'],item['mode'],item['cancel'],item['build']].append(row)
summary=[];pairs=0
for (case,mode,cancel,build),rs in groups.items():
    if build!='meter':continue
    assert len(rs)==2 and normalize(rs[0]['result'])==normalize(rs[1]['result']);pairs+=1
    ordinary=groups[case,mode,cancel,'ordinary'];assert len(ordinary)==(1 if cancel else 6)
    for r in ordinary:assert normalize(r['result'],True)==normalize(rs[0]['result'],True)
    primary=[r for r in ordinary if not r['warmup']];times=[r['total_ns'] for r in primary]
    summary.append(dict(case=case,mode=mode,cancel=cancel,traffic=rs[0]['traffic'],peak=rs[0]['peak'],median_ns=statistics.median(times),min_ns=min(times),max_ns=max(times),qualified=all(r['total_ns']>r['threshold_ns'] for r in primary)))
parent_rows=[json.loads(x) for x in (ROOT/'docs/experiments/results/s02-read-cost/results.jsonl').read_text().splitlines()]
parent_cases=read(ROOT/'docs/experiments/results/s02-read-cost/scenarios.json')
matched=0
for (case,mode,cancel,build),rs in groups.items():
    if cancel or build!='meter' or mode=='sealed':continue
    family,depth,q=cases[case]
    parent_case=parent_cases.index([family,depth,q,None])
    prior=next(r for r in parent_rows if r['scenario']==parent_case and r['mode']==mode and r['build']=='meter')
    assert normalize(prior['result'])==normalize(rs[0]['result'])
    matched+=1
assert matched==15
comparisons=[]
for case in range(5):
    for candidate,control in [('sealed','indexed'),('validated','sealed'),('validated','contextual')]:
        def samples(mode):return {r['rep']:r for r in groups[case,mode,False,'ordinary'] if not r['warmup']}
        aa=samples(candidate);bb=samples(control);assert set(aa)==set(bb)==set(range(5))
        ratios=[aa[i]['total_ns']/bb[i]['total_ns'] for i in range(5)]
        qualified=all(r['total_ns']>r['threshold_ns'] for r in list(aa.values())+list(bb.values()))
        status='insufficient-signal' if not qualified else 'gain' if max(ratios)<.90 else 'loss' if min(ratios)>1.10 else 'unresolved'
        comparisons.append(dict(case=case,candidate=candidate,control=control,status=status,min_ratio=min(ratios),median_ratio=statistics.median(ratios),max_ratio=max(ratios)))
phase_rows=[]
for r in rows:
    if r['build']!='ordinary' or r['warmup'] or r['cancel'] or r['case']!=1 or r['mode'] not in ['sealed','validated']:continue
    v=r['result'];phase_rows.append(dict(rep=r['rep'],mode=r['mode'],source_ns=v['source_build']['ns'],prepare_ns=v['preparation']['ns'],setup_ns=sum(s['setup']['ns'] for s in v['samples']),execute_observe_ns=sum(s['execute_observe']['ns'] for s in v['samples'])))
assert phase_rows==read(BASE/'favorable-phase-breakdown.json')
assert pairs==30
print(json.dumps(dict(processes=190,parent_allocation_matches=matched,exact_allocation_pairs=pairs,clock_floor_ns=floor,summary=summary,comparisons=comparisons),indent=2))
