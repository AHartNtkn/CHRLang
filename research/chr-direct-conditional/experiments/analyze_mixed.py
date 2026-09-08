#!/usr/bin/env python3
"""Audit every R05 mixed-pipeline endpoint and summarize complete cells."""
import argparse
import json
import pathlib
import statistics

ROOT=pathlib.Path(__file__).resolve().parents[3]

PAIRS=[(0,0),(0,16),(8,8),(16,0),(0,64),(32,32),(64,0)]

def phases(d):
    return [d['prepare'],d['prepared_drop'],*(s[p] for s in d['samples'] for p in ['setup','execution_observation','engine_drop','outputs_drop'])]

def total(d):
    return sum(p['ns'] for p in phases(d))/d['queries']/1e6

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', type=pathlib.Path, required=True)
    args = parser.parse_args()
    OUT = args.output.resolve()
    rows=[json.loads(l) for l in (OUT/'runs.jsonl').read_text().splitlines()]
    cells={(b,pre,post,q) for b in ['conditional','specialized'] for pre,post in PAIRS for q in [1,4]}
    reps={('warmup',0),('allocation',0),('work',0),*(('primary',i) for i in range(5))}
    expected={(mode,rep,cell) for cell in cells for mode,rep in reps}
    meta = json.loads((OUT/'metadata.json').read_text())
    manifest = [(j['mode'],j['rep'],tuple(j['cell'])) for j in meta['jobs']]
    assert len(manifest) == len(expected) and set(manifest) == expected
    assert [(r['mode'],r['rep'],tuple(r['cell'])) for r in rows] == manifest[:len(rows)]
    seen=set(); failures=[]; predictions=[]
    for r in rows:
        key=(r['mode'],r['rep'],tuple(r['cell']))
        assert key in expected and key not in seen,key
        seen.add(key)
        if r.get('exit')!=0 or 'result' not in r:
            failures.append({k:r.get(k) for k in ['mode','rep','cell','exit','timeout','stderr']});continue
        d=r['result']
        assert [d[k] for k in ['backend','pre','post','queries']]==r['cell']
        assert d['metrics']==d['compiled_metrics']==d['observer_metrics']==(r['mode']=='work')
        assert d['allocator_meter']==(r['mode']=='allocation')
        assert sum(p['ns'] for p in phases(d))==d['measured_ns']
        assert d['completed']==sum(s['exhausted'] for s in d['samples'])
        assert len(d['samples'])<=d['queries']
        if d['complete']:
            assert d['completed']==d['queries']==len(d['samples'])
        else:
            failures.append({'mode':r['mode'],'rep':r['rep'],'cell':r['cell'],'completed':d['completed']})
        for i,s in enumerate(d['samples']):
            assert (s['pre'],s['post'])==(d['pre']+i%2,d['post']+i%2)
            assert s['cutoff']!=s['exhausted']
            assert (s['work'] is not None)==(r['mode']=='work')
            if s['exhausted']: assert s['answers']==16
            else: assert s['ticks']==d['tick_limit']==20_000_000
            if r['mode']=='work' and s['exhausted']:
                predicted=s['pre']+16*s['post']+34 if d['backend']=='conditional' else 1+16*(s['pre']+s['post']+3)
                predictions.append({'cell':r['cell'],'query':i,'actual':s['work']['applications'],'predicted':predicted,'agrees':s['work']['applications']==predicted})
        if r['mode']=='allocation': assert d['final']['memory']['live_end']==d['baseline']['memory']['live_end'],r['cell']
    summary=[]
    for cell in sorted(cells):
        group=[r for r in rows if tuple(r['cell'])==cell]
        primary=[r['result'] for r in group if r['mode']=='primary' and r.get('result',{}).get('complete')]
        item={'cell':cell,'complete':{(r['mode'],r['rep']) for r in group}==reps and all(r.get('result',{}).get('complete') for r in group),'recorded':len(group),'primary_complete':len(primary)}
        if primary:
            vals=[total(d) for d in primary]
            item.update(median_ms=statistics.median(vals),min_ms=min(vals),max_ms=max(vals),
                        prepare_ms=statistics.median(d['prepare']['ns']/1e6 for d in primary),
                        execution_ms_per_query=statistics.median(sum(s['execution_observation']['ns'] for s in d['samples'])/d['queries']/1e6 for d in primary),
                        first_service_ms=statistics.median(d['samples'][0]['first_answer_ns']/1e6 for d in primary),
                        disposal_ms_per_query=statistics.median((d['prepared_drop']['ns']+sum(s['engine_drop']['ns']+s['outputs_drop']['ns'] for s in d['samples']))/d['queries']/1e6 for d in primary))
        mem=[r['result'] for r in group if r['mode']=='allocation' and r.get('result',{}).get('complete')]
        if mem:
            d,=mem; base=d['baseline']['memory']['live_end']
            item.update(requested_bytes_per_query=sum(p['memory']['requested_bytes'] for p in phases(d))/d['queries'],
                        peak_above_baseline=max(p['memory']['peak_live'] for p in phases(d))-base,
                        live_after_execution=[s['execution_observation']['memory']['live_end']-base for s in d['samples']])
        work=[r['result'] for r in group if r['mode']=='work' and r.get('result',{}).get('complete')]
        if work: item['work']=[s['work'] for s in work[0]['samples']]
        summary.append(item)
    audit={'planned':224,'recorded':len(rows),'failures':failures,'missing':sorted(expected-seen),
           'complete_cells':sum(s['complete'] for s in summary),
           'validated_answers':sum(s['answers'] for r in rows for s in r.get('result',{}).get('samples',[])),
           'prediction_disagreements':[p for p in predictions if not p['agrees']]}
    (OUT/'audit.json').write_text(json.dumps(audit,indent=2)+'\n')
    (OUT/'work-predictions.json').write_text(json.dumps(predictions,indent=2)+'\n')
    (OUT/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    lines=['backend\tpre\tpost\tqueries\tcomplete\tmedian_ms\tmin_ms\tmax_ms\trequested_bytes_per_query\tpeak_bytes']
    for s in summary: lines.append('\t'.join(map(str,[*s['cell'],s['complete'],s.get('median_ms'),s.get('min_ms'),s.get('max_ms'),s.get('requested_bytes_per_query'),s.get('peak_above_baseline')])))
    (OUT/'summary.tsv').write_text('\n'.join(lines)+'\n')
    print(json.dumps(audit));print('\n'.join(lines))

if __name__=='__main__': main()
