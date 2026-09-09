#!/usr/bin/env python3
"""Audit the complete T059 manifest and within-freeze lifecycle comparisons."""
import argparse, hashlib, json, pathlib, random, statistics
ROOT=pathlib.Path(__file__).resolve().parents[3]
CONFIGS={'conditional':'','specialized':'','specialized-cow':'arena-cow'}
PHASES=['query','setup','execution_observation','engine_drop','outputs_drop']
def phases(d): return [d['prepare'],d['prepared_drop'],*(s[k] for s in d['samples'] for k in PHASES)]
def check_row(r):
    assert r.get('exit')==0 and 'result' in r
    d=r['result'];depth,q,placement,outcome=r['cell']; config=r['configuration']; mode=r['mode'];conditional=config=='conditional';clash=outcome=='clash'
    assert d['schema_version']==1
    assert [d['depth'],d['queries'],d['placement'],d['outcome']]==[depth,q,placement,outcome]
    assert d['backend']==('conditional' if conditional else 'specialized')
    assert d['carrier_contraction'] is False and d['arena_cow']==config.endswith('-cow')
    assert d['metrics']==d['compiled_metrics']==d['kernel_metrics']==d['observer_metrics']==(mode=='work')
    assert d['allocator_meter']==(mode=='allocation')
    assert d['complete'] and d['completed']==q==len(d['samples'])
    assert d['tick_limit']==20_000_000
    assert d['measured_ns']==sum(p['ns'] for p in phases(d))
    assert d['validation_ns']==sum(s['validation_ns'] for s in d['samples'])
    for i,s in enumerate(d['samples']):
        assert s['depth']==depth+i%2
        assert s['exhausted'] and not s['cutoff'] and s['answers']==(0 if clash else 16)
        assert 0<s['ticks']<=d['tick_limit']
        if clash:
            assert s['first_answer_ns'] is None and s['first_answer_request_ns'] is None
        else:
            assert 0<=s['first_answer_ns']<=s['execution_observation']['ns']
            assert s['first_answer_request_ns']==s['query']['ns']+s['setup']['ns']+s['first_answer_ns']
        assert s['failure_leaves']==(None if conditional else 16 if clash else 0)
        assert s['splits']==(None if conditional else 15)
        assert (s['work'] is not None)==(mode=='work')
        if mode=='work':
            w=s['work'];before=placement=='before'
            expected=(2 if clash else 18) if conditional and before else 33 if not conditional and before and not clash else 17
            assert w['applications']==expected
            if conditional:
                assert w['conditional_equality_jobs']==(9 if before else 24)
                assert w['conditional_equality_ticks']>=w['conditional_equality_jobs']
                assert w['pairs'] is None and w['occurs_visits'] is None
            else:
                assert w['conditional_equality_jobs'] is None and w['conditional_equality_ticks'] is None
                assert w['pairs']==(62+16*s['depth'] if clash else 94+32*s['depth'])
                assert isinstance(w['occurs_visits'],int) and w['occurs_visits']>=0
    for p in phases(d): assert (p['memory'] is not None)==(mode=='allocation')
    if mode=='allocation': assert d['baseline']['memory']['live_end']==d['final']['memory']['live_end']
    return (0 if clash else 16)*q

def audit(out):
    configs=CONFIGS
    meta=json.loads((out/'metadata.json').read_text())
    assert meta['seed']==59059
    assert meta['bounds']=={'process_seconds':30,'process_address_bytes':1024**3,'execution_seconds':1200,'build_seconds':180}
    cells=[(depth,q,placement,outcome) for depth in (0,16,64) for q in (1,4) for placement in ('before','after') for outcome in ('success','clash')]
    planned=[]; rng=random.Random(59059)
    for mode,reps in [('warmup',1),('primary',5),('allocation',1),('work',1)]:
        batch=[(mode,rep,c,cell) for rep in range(reps) for c in configs for cell in cells]
        rng.shuffle(batch);planned.extend(batch)
    key=lambda r:(r['mode'],r['rep'],r['configuration'],tuple(r['cell']))
    assert list(map(key,meta['jobs']))==planned
    rows=[json.loads(l) for l in (out/'runs.jsonl').read_text().splitlines()] if (out/'runs.jsonl').exists() else []
    assert list(map(key,rows))==planned[:len(rows)]
    failures=[]; valid=set();answers=0
    for r in rows:
        try:
            mode='primary' if r['mode']=='warmup' else r['mode'];c=r['configuration']
            command=meta['builds'][c+'-'+mode]['command']
            binary=str(pathlib.Path(command[command.index('--target-dir')+1])/'release/chr-equation-cost')
            assert r['command']==[binary,'conditional' if c=='conditional' else 'specialized',*map(str,r['cell'])]
            answers+=check_row(r);valid.add(id(r))
        except (AssertionError,KeyError,TypeError) as error:
            failures.append({'configuration':r['configuration'],'cell':r['cell'],'mode':r['mode'],'rep':r['rep'],'error':repr(error),'exit':r.get('exit'),'stderr':r.get('stderr'),'timeout':r.get('timeout')})
    summary=[]
    for cell in cells:
        item={'cell':cell,'configurations':{}}
        for c in configs:
            group=[r for r in rows if tuple(r['cell'])==cell and r['configuration']==c]
            good=[r for r in group if id(r) in valid]
            t={'recorded':len(group),'complete':len(group)==8 and len(good)==8}
            prim=[r['result'] for r in good if r['mode']=='primary']
            if prim:
                vals=[d['measured_ns']/d['queries']/1e6 for d in prim]
                t.update(median_ms=statistics.median(vals),min_ms=min(vals),max_ms=max(vals),primary_repetitions=len(prim),
                         prepare_ms=statistics.median(d['prepare']['ns']/1e6 for d in prim),
                         first_service_ms=None if cell[3]=='clash' else statistics.median(d['samples'][0]['first_answer_ns']/1e6 for d in prim),
                         phase_ms={k:statistics.median(sum(s[k]['ns'] for s in d['samples'])/d['queries']/1e6 for d in prim) for k in PHASES})
            alloc=[r['result'] for r in good if r['mode']=='allocation']
            if alloc:
                d,=alloc;t.update(requested_bytes_per_query=sum(p['memory']['requested_bytes'] for p in phases(d))/d['queries'],peak_above_baseline=max(p['memory']['peak_live'] for p in phases(d))-d['baseline']['memory']['live_end'])
            work=[r['result'] for r in good if r['mode']=='work']
            if work:t['work']=[s['work'] for s in work[0]['samples']]
            item['configurations'][c]=t
        item['comparisons']={}
        for a,b in [('specialized','conditional'),('specialized-cow','conditional'),('specialized-cow','specialized')]:
            if a not in configs or b not in configs: continue
            x,y=item['configurations'][a],item['configurations'][b]
            if x['complete'] and y['complete']:
                item['comparisons'][a+'/'+b]={'median_ratio':x['median_ms']/y['median_ms'],'range_disposition':'first-lower' if x['max_ms']<y['min_ms'] else 'second-lower' if y['max_ms']<x['min_ms'] else 'overlap'}
        summary.append(item)
    freeze=[]
    for name,h in meta['sources'].items():
        if hashlib.sha256((ROOT/name).read_bytes()).hexdigest()!=h:freeze.append(name)
    for c,extra in configs.items():
        for mode in ('primary','allocation','work'):
            b=meta.get('builds',{}).get(c+'-'+mode,{})
            if b.get('status')!='complete':freeze.append('build:'+c+'-'+mode);continue
            command=b['command'];expected=('alloc-meter' if mode=='allocation' else 'experiment')+(','+extra if extra else '')
            assert command[command.index('--features')+1]==expected and ('--no-default-features' in command)==(mode!='work')
            binary=pathlib.Path(command[command.index('--target-dir')+1])/'release/chr-equation-cost'
            if hashlib.sha256(binary.read_bytes()).hexdigest()!=b['binary_sha256']:freeze.append('binary:'+c+'-'+mode)
    result={'planned':len(planned),'recorded':len(rows),'missing':len(planned)-len(rows),'failures':failures,'validated_answers':answers,'complete_cells':sum(c['complete'] for s in summary for c in s['configurations'].values()),'freeze_errors':freeze}
    (out/'audit.json').write_text(json.dumps(result,indent=2)+'\n');(out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(result));return result
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--output',type=pathlib.Path,required=True)
    args=p.parse_args()
    r=audit(args.output.resolve());raise SystemExit(bool(r['missing'] or r['failures'] or r['freeze_errors']))
