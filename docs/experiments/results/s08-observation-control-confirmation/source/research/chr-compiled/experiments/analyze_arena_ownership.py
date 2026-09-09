#!/usr/bin/env python3
"""Audit T052's matched arena ownership lifecycle comparison."""
import argparse
import hashlib
import json
import pathlib
import random
import statistics
from analyze_state_preservation import phases, complete, PHASES
ROOT = pathlib.Path(__file__).resolve().parents[3]

def check_row(r):
    failures=[]
    answers=0
    if r.get('exit') != 0 or 'result' not in r:
        failures.append({k:r.get(k) for k in ('mode','rep','cell','exit','timeout','spawn_error','stderr')})
        return failures, answers
    d = r['result']; n,a,o,q,mutation = r['cell']
    assert d['arena_cow'] == (r['ownership'] == 'cow')
    assert d['insertion'] == (mutation == 'insert')
    assert (d['n'], d['alternatives'], d['all_success'], d['queries']) == (n,a,o=='all-success',q)
    assert d['metrics'] == d['kernel_metrics'] == d['observer_metrics'] == (r['mode']=='work')
    assert d['metered'] == (r['mode']=='allocation')
    assert d['budget_per_query'] == 20_000_000
    assert len(d['samples']) == d['attempted'] <= q
    if not complete(d): failures.append({'cell':r['cell'],'mode':r['mode'],'rep':r['rep'],'incomplete':True})
    for i,s in enumerate(d['samples']):
        assert s['n'] == n+i%2
        assert s['total_ns'] == sum(s[k]['ns'] for k in PHASES)
        assert (s['first_observation_ns'] is not None) == (s['answers'] > 0)
        if s['first_observation_ns'] is not None:
            assert 0 <= s['first_observation_ns'] <= s['service']['ns']
        if d['metered'] or d['metrics']: assert 1 <= s['maximum_frontier'] <= a
        else: assert s['maximum_frontier'] is None
        expected = a if d['all_success'] else 1
        if s['exhausted']:
            assert (s['answers'],s['failed'],s['splits']) == (expected,a-expected,a-1)
        else: assert s['ticks'] == d['budget_per_query']
        assert (s['work'] is not None) == (r['mode']=='work')
        if s['work'] is not None and s['exhausted']:
            assert s['work']['applications'] == s['n']+1+a*(2 if mutation=='insert' else 1)+(s['n']+1)*expected
        answers += s['answers']
        diag = s['diagnostic']
        assert (diag is not None) == d['metered']
        if diag is not None:
            assert sum(diag[k]['count'] for k in ('split','progress','failed','complete','exhausted')) == s['ticks']
            for k,count in [('split',s['splits']),('failed',s['failed']),('complete',s['answers']),
                            ('observation',s['answers']),('answer_retention',s['answers']),
                            ('terminal_disposal',s['failed']+s['answers']),('split_metadata_disposal',s['splits']),
                            ('exhausted',int(s['exhausted']))]:
                assert diag[k]['count'] == count
            assert sum(c['ns'] for c in diag.values()) <= s['service']['ns']
            for key in ('requested_bytes','allocation_calls','deallocation_calls','net_live_delta'):
                assert sum(c['memory'][key] for c in diag.values() if c['memory'] is not None) == s['service']['memory'][key]
    if d['metered']: assert d['baseline_live'] == d['final_live']
    return failures, answers

def audit(out):
    meta = json.loads((out/'metadata.json').read_text())
    assert meta['seed'] == 52052
    assert meta['bounds'] == {'process_seconds':30,'process_address_bytes':1024**3,'execution_seconds':1200,'build_seconds':180}
    cells = [(n,a,o,q,m) for n in (0,512) for a in (1,64)
             for o in ('mostly-fail','all-success') for q in (1,4) for m in ('read','insert')]
    rng = random.Random(52052)
    planned=[]
    for mode,reps in [('warmup',1),('primary',5),('allocation',1),('work',1)]:
        batch=[(mode,rep,ownership,cell) for rep in range(reps) for ownership in ('clone','cow') for cell in cells]
        rng.shuffle(batch); planned.extend(batch)
    identity=lambda r:(r['mode'],r['rep'],r['ownership'],tuple(r['cell']))
    assert list(map(identity,meta['jobs'])) == planned
    rows=[json.loads(line) for line in (out/'runs.jsonl').read_text().splitlines()] if (out/'runs.jsonl').exists() else []
    assert list(map(identity,rows)) == planned[:len(rows)]
    failures=[]; answers=0; valid=set()
    for row in rows:
        try:
            key=row['ownership']+'-'+('primary' if row['mode']=='warmup' else row['mode'])
            command=meta['builds'][key]['command']
            binary=str(pathlib.Path(command[command.index('--target-dir')+1])/'release/chr-state-preservation-cost')
            assert row['command']==[binary,*map(str,row['cell'])]
            found,count=check_row(row)
            failures.extend(found); answers+=count
            if not found: valid.add(id(row))
        except (AssertionError,KeyError,TypeError) as error:
            failures.append({'cell':row['cell'],'ownership':row['ownership'],'mode':row['mode'],'invalid':repr(error)})
    summary=[]
    for cell in cells:
        item={'cell':cell,'configurations':{}}
        for ownership in ('clone','cow'):
            group=[r for r in rows if tuple(r['cell'])==cell and r['ownership']==ownership]
            config={'recorded':len(group),'complete':len(group)==8 and all(id(r) in valid for r in group)}
            good=[r for r in group if id(r) in valid]
            primary=[r['result'] for r in good if r['mode']=='primary']
            if primary:
                values=[sum(p['ns'] for p in phases(d))/d['queries']/1e6 for d in primary]
                config.update(median_ms=statistics.median(values),min_ms=min(values),max_ms=max(values),primary_repetitions=len(primary))
                config['prepare_ms']=statistics.median(d['prepare']['ns']/1e6 for d in primary)
                config['first_service_ms']=statistics.median(d['samples'][0]['first_observation_ns']/1e6 for d in primary)
                config['phase_ms_per_query']={k:statistics.median(sum(s[k]['ns'] for s in d['samples'])/d['queries']/1e6 for d in primary) for k in PHASES}
            alloc=[r['result'] for r in good if r['mode']=='allocation']
            if alloc:
                d,=alloc
                config['requested_bytes_per_query']=sum(p['memory']['requested_bytes'] for p in phases(d))/d['queries']
                config['peak_above_baseline']=max(p['memory']['peak_live'] for p in phases(d))-d['baseline_live']
                config['phase_requested_bytes_per_query']={k:sum(s[k]['memory']['requested_bytes'] for s in d['samples'])/d['queries'] for k in PHASES}
            work=[r['result'] for r in good if r['mode']=='work']
            if work: config['work']=[s['work'] for s in work[0]['samples']]
            item['configurations'][ownership]=config
        clone=item['configurations']['clone']; cow=item['configurations']['cow']
        if clone['complete'] and cow['complete']:
            item['range_disposition']='cow-lower' if cow['max_ms']<clone['min_ms'] else 'clone-lower' if clone['max_ms']<cow['min_ms'] else 'overlap'
            item['cow_over_clone']=cow['median_ms']/clone['median_ms']
            if cow.get('work')!=clone.get('work'):
                failures.append({'cell':cell,'source_work_mismatch':True})
        summary.append(item)
    freeze=[]
    for name,h in meta['sources'].items():
        if hashlib.sha256((ROOT/name).read_bytes()).hexdigest()!=h: freeze.append(name)
    for ownership in ('clone','cow'):
        for mode in ('primary','allocation','work'):
            key=ownership+'-'+mode; build=meta.get('builds',{}).get(key,{})
            if build.get('status')!='complete': freeze.append('build:'+key); continue
            command=build['command']; features=command[command.index('--features')+1]
            expected={'primary':'experiment','allocation':'alloc-meter','work':'experiment'}[mode]+(',arena-cow' if ownership=='cow' else '')
            assert features==expected and ('--no-default-features' in command)==(mode!='work')
            binary=pathlib.Path(command[command.index('--target-dir')+1])/'release/chr-state-preservation-cost'
            if hashlib.sha256(binary.read_bytes()).hexdigest()!=build['binary_sha256']: freeze.append('binary:'+key)
    result={'planned':len(planned),'recorded':len(rows),'missing':len(planned)-len(rows),'failures':failures,'validated_answers':answers,'freeze_errors':freeze,
            'complete_pairs':sum(all(c['complete'] for c in s['configurations'].values()) for s in summary)}
    (out/'audit.json').write_text(json.dumps(result,indent=2)+'\n')
    (out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    print(json.dumps(result));return result

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--output',type=pathlib.Path,required=True)
    result=audit(parser.parse_args().output.resolve())
    raise SystemExit(bool(result['missing'] or result['failures'] or result['freeze_errors']))
