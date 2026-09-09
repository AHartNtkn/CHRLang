#!/usr/bin/env python3
"""Audit T050's full registered manifest and summarize bounded lifecycle evidence."""
import argparse
import hashlib
import json
import pathlib
import random
import statistics

ROOT = pathlib.Path(__file__).resolve().parents[3]
PHASES = ('query', 'setup', 'service', 'engine_drop', 'output_drop')

def phases(d):
    return [d['prepare'], d['prepared_drop'],
            *(s[k] for s in d['samples'] for k in PHASES)]

def complete(d):
    return d and d['attempted'] == d['queries'] and all(s['exhausted'] for s in d['samples'])

def check_row(r):
    failures=[]
    answers=0
    if r.get('exit') != 0 or 'result' not in r:
        failures.append({k:r.get(k) for k in ('mode','rep','cell','exit','timeout','spawn_error','stderr')})
        return failures, answers
    d = r['result']; n,a,o,q = r['cell']
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
            assert s['work']['applications'] == s['n']+1+a+(s['n']+1)*expected
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
    assert meta['seed'] == 50050
    cells = [(n,a,o,q) for n in (0,64,512) for a in (1,8,64)
             for o in ('mostly-fail','all-success') for q in (1,4)]
    rng = random.Random(50050)
    planned = []
    for mode, reps in [('warmup',1),('primary',5),('allocation',1),('work',1)]:
        batch = [(mode,rep,cell) for rep in range(reps) for cell in cells]
        rng.shuffle(batch)
        planned.extend(batch)
    manifest = [(j['mode'],j['rep'],tuple(j['cell'])) for j in meta['jobs']]
    assert manifest == planned
    rows = [json.loads(l) for l in (out/'runs.jsonl').read_text().splitlines()] if (out/'runs.jsonl').exists() else []
    assert [(r['mode'],r['rep'],tuple(r['cell'])) for r in rows] == planned[:len(rows)]
    failures = []
    answers = 0
    for r in rows:
        row_failures, count = check_row(r)
        failures.extend(row_failures)
        answers += count
    summary=[]
    for cell in cells:
        group=[r for r in rows if tuple(r['cell'])==cell]
        prim=[r['result'] for r in group if r['mode']=='primary' and complete(r.get('result'))]
        item={'cell':cell,'recorded':len(group),'complete':len(group)==8 and all(complete(r.get('result')) for r in group)}
        if prim:
            vals=[sum(p['ns'] for p in phases(d))/d['queries']/1e6 for d in prim]
            item.update(median_ms=statistics.median(vals),min_ms=min(vals),max_ms=max(vals),primary_repetitions=len(prim))
            item['prepare_ms'] = statistics.median(d['prepare']['ns']/1e6 for d in prim)
            item['prepared_drop_ms'] = statistics.median(d['prepared_drop']['ns']/1e6 for d in prim)
            item['first_service_ms'] = statistics.median(d['samples'][0]['first_observation_ns']/1e6 for d in prim)
            item['phase_ms_per_query']={k:statistics.median(sum(s[k]['ns'] for s in d['samples'])/d['queries']/1e6 for d in prim) for k in PHASES}
        alloc=[r['result'] for r in group if r['mode']=='allocation' and complete(r.get('result'))]
        if alloc:
            d,=alloc
            traffic=sum(p['memory']['requested_bytes'] for p in phases(d))
            split_bytes=sum(s['diagnostic']['split']['memory']['requested_bytes'] for s in d['samples'] if s['diagnostic']['split']['memory'])
            split_ns=sum(s['diagnostic']['split']['ns'] for s in d['samples'])
            splits=sum(s['splits'] for s in d['samples'])
            item.update(requested_bytes_per_query=traffic/d['queries'],
                        peak_above_baseline=max(p['memory']['peak_live'] for p in phases(d))-d['baseline_live'],
                        split_bytes_per_event=split_bytes/splits if splits else None,
                        diagnostic_split_ns_per_event=split_ns/splits if splits else None,
                        diagnostic_split_traffic_fraction=split_bytes/traffic if traffic else 0,
                        diagnostic_split_time_fraction=split_ns/sum(p['ns'] for p in phases(d)),
                        diagnostic=[s['diagnostic'] for s in d['samples']])
        work=[r['result'] for r in group if r['mode']=='work' and complete(r.get('result'))]
        if work: item['work']=[s['work'] for s in work[0]['samples']]
        summary.append(item)
    for item in summary:
        n,a,o,q=item['cell']
        if n==512 and a>1 and o=='mostly-fail' and item['complete']:
            low=next(s for s in summary if s['cell']==(0,a,o,q))
            material=max(item['diagnostic_split_traffic_fraction'],item['diagnostic_split_time_fraction'])>=0.25
            growth=(item['split_bytes_per_event']>low['split_bytes_per_event'] or item['diagnostic_split_ns_per_event']>low['diagnostic_split_ns_per_event']) if low['complete'] else None
            item['owner_inspection_trigger']=material and growth if growth is not None else None
    freeze_errors=['missing-build:'+mode for mode in ('primary','allocation','work') if mode not in meta.get('builds',{})]
    for name,h in meta['sources'].items():
        if hashlib.sha256((ROOT/name).read_bytes()).hexdigest()!=h: freeze_errors.append(name)
    for mode,b in meta.get('builds',{}).items():
        if b.get('status')!='complete': freeze_errors.append('build:'+mode);continue
        target=pathlib.Path(b['command'][b['command'].index('--target-dir')+1])/'release/chr-state-preservation-cost'
        if hashlib.sha256(target.read_bytes()).hexdigest()!=b['binary_sha256']: freeze_errors.append('binary:'+mode)
    result={'planned':len(planned),'recorded':len(rows),'missing':len(planned)-len(rows),'failures':failures,
            'validated_answers':answers,'complete_cells':sum(s['complete'] for s in summary),'freeze_errors':freeze_errors}
    (out/'audit.json').write_text(json.dumps(result,indent=2)+'\n')
    (out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    print(json.dumps(result))
    return result

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--output',type=pathlib.Path,required=True)
    audit(parser.parse_args().output.resolve())
