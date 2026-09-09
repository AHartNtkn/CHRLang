"""Repeated matched-policy audit and preregistered descriptive comparisons."""
import csv,hashlib,io,json,random,statistics,tarfile
from run_graph_costs import ROOT,OUT,PREFIX

POLICIES=('eager_clone','eager_compare','eager_graph_compare','graph_compare')
READINGS=('before','prepared','first','done','index_dropped','engine_dropped','output_drop_before','output_dropped')
PHASES=('prepared','first','done')
def memory_metrics(d):
    return dict(allocation_calls=d['engine_dropped_calls']-d['before_calls']+d['output_dropped_calls']-d['output_drop_before_calls'],
        requested_bytes=d['engine_dropped_requested']-d['before_requested']+d['output_dropped_requested']-d['output_drop_before_requested'],
        peak_above_baseline=d['engine_dropped_peak']-d['before_live'],
        index_release=d['done_live']-d['index_dropped_live'],engine_release=d['index_dropped_live']-d['engine_dropped_live'],
        delivered_bytes=d['engine_dropped_live']-d['before_live'])
def times(d):return dict(cold_ns=d['cold_ns'],first_ns=d['first_ns'],prepare_ns=d['prepare_ns'],lifecycle_ns=sum(d[k] for k in ('cold_ns','index_drop_ns','engine_drop_ns','output_drop_ns')))
def main():
    m=json.loads((OUT/f'{PREFIX}-manifest.json').read_text());rows=[json.loads(s) for s in (OUT/f'{PREFIX}.jsonl').read_text().splitlines()]
    ids=[f'{f}-{n}-{k}' for f,n in [('dag',4),('dag',8),('unary',16),('unary',64),('symmetric',4),('symmetric',8),('retention',64),('retention',1024)] for k in ('repeat','distinct')]
    rng=random.Random(140907);order=[]
    for phase,kind,n in [('warmup','system',2),('timing','system',7),('memory','meter',3)]:
        for block in range(n):
            cells=[dict(id=id,policy=p) for id in ids for p in POLICIES];rng.shuffle(cells)
            order.extend(dict(phase=phase,kind=kind,block=block,**c) for c in cells)
    assert len(rows)==768 and m['order']==order and [{k:r[k] for k in ('phase','kind','block','id','policy')} for r in rows]==order
    assert all(hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h for p,h in m['sha256'].items())
    with tarfile.open(OUT/f'{PREFIX}-inputs.tar.gz','r:gz') as a:
        assert all(hashlib.sha256(a.extractfile(p).read()).hexdigest()==h for p,h in m['sha256'].items())
    samples={};work={};source={};header=None
    for row in rows:
        assert row['exit']==0 and not row['stderr'] and row['wall_seconds']<30
        reader=csv.DictReader(io.StringIO(row['stdout']),delimiter='\t');values=list(reader)
        assert len(values)==1 and len(reader.fieldnames)==len(set(reader.fieldnames))==185
        if header is None:header=reader.fieldnames
        else:assert header==reader.fieldnames
        d=values[0];assert None not in d and all(v is not None for v in d.values())
        assert d['case']==row['id'] and d['policy']==row['policy']
        d={k:(v if k in ('case','policy') else int(v)) for k,v in d.items()}
        expected=32 if row['id'].endswith('distinct') else (2 if row['id'].startswith('symmetric') else 1)
        assert d['validated']==1 and d['expected_answers']==d['done_deliveries']==expected
        assert d['done_raw']==d['done_source_completed']==32 and d['done_frontier']==d['done_source_failed']==0
        assert d['prepared_raw']==0 and d['first_raw']==d['first_deliveries']==1 and d['prepare_ns']<=d['first_ns']<=d['cold_ns']
        graph=row['policy']=='graph_compare'
        assert d['done_exports']==(expected if graph else 32) and d['done_eager_export_answers']==(0 if graph else 32)
        assert d['done_capture_snapshots']==(32 if graph else 0)
        assert d['done_index_keys']==(expected if row['policy'] in ('eager_clone','graph_compare') else 0)
        memory_keys=[p+'_'+f for p in READINGS for f in ('calls','requested','live','peak')]
        assert d['metered']==int(row['kind']=='meter')
        if row['kind']=='system':assert all(d[k]==0 for k in memory_keys)
        else:
            assert d['output_dropped_live']==d['before_live'] and d['engine_dropped_live']==d['output_drop_before_live']
            assert d['done_live']>=d['index_dropped_live']>=d['engine_dropped_live']>=d['output_dropped_live']
            for f in ('calls','requested','peak'):
                values=[d[p+'_'+f] for p in READINGS];assert values==sorted(values)
            assert all(d[p+'_peak']>=d[p+'_live'] for p in READINGS)
        deterministic={k:v for k,v in d.items() if k not in memory_keys and not k.endswith('_ns') and k!='metered'}
        key=(row['id'],row['policy'])
        if key in work:assert work[key]==deterministic
        else:work[key]=deterministic
        for phase in PHASES:
            values={k.removeprefix(phase+'_source_'):v for k,v in d.items() if k.startswith(phase+'_source_')}
            values['dereferences']-=d[phase+'_eager_export_dereferences'];values['map_visits']-=d[phase+'_eager_export_map_visits']
            values['actual_queue_peak']=d[phase+'_queue_peak'];key=(row['id'],phase)
            if key in source:assert source[key]==values
            else:source[key]=values
        samples[(row['phase'],row['block'],row['id'],row['policy'])]=d
    allocations=[];timing=[];comparisons=[]
    for id in ids:
        for phase in PHASES:
            for name in ('term_pairs','occurrence_scans','occurrence_candidates','backtracks'):
                assert work[(id,'eager_graph_compare')][phase+'_graph_compare_'+name]==work[(id,'graph_compare')][phase+'_graph_compare_'+name]
                assert work[(id,'eager_clone')][phase+'_eager_compare_'+name]==work[(id,'eager_compare')][phase+'_eager_compare_'+name]
        for p in POLICIES:
            values=[memory_metrics(samples[('memory',i,id,p)]) for i in range(3)]
            assert values[0]==values[1]==values[2],(id,p,'allocation replay')
            allocations.append(dict(id=id,policy=p,**values[0]))
            ts=[times(samples[('timing',i,id,p)]) for i in range(7)]
            timing.append(dict(id=id,policy=p,metrics={k:dict(min=min(x[k] for x in ts),median=statistics.median(x[k] for x in ts),max=max(x[k] for x in ts)) for k in ts[0]}))
        for candidate,control in [('eager_compare','eager_clone'),('eager_graph_compare','eager_compare'),('graph_compare','eager_graph_compare'),('graph_compare','eager_compare')]:
            for metric in ('cold_ns','first_ns','prepare_ns','lifecycle_ns'):
                a=[times(samples[('timing',i,id,candidate)])[metric] for i in range(7)]
                b=[times(samples[('timing',i,id,control)])[metric] for i in range(7)]
                ratios=[x/y for x,y in zip(a,b)]
                disposition='lower_separated' if max(a)<min(b) else ('higher_separated' if min(a)>max(b) else 'overlapping')
                comparisons.append(dict(id=id,candidate=candidate,control=control,metric=metric,ratios=ratios,median_ratio=statistics.median(ratios),min_ratio=min(ratios),max_ratio=max(ratios),disposition=disposition))
    result=dict(passed=True,children=768,timed=448,warmup=128,memory=192,source_and_observer_work_agree=True,exact_allocation_replay=True,
                all_measured_ownership_released=True,allocations=allocations,timing=timing,comparisons=comparisons,max_child_wall_seconds=max(r['wall_seconds'] for r in rows))
    with (OUT/f'{PREFIX}-audit.json').open('x') as f:json.dump(result,f,indent=2);f.write('\n')
    print(json.dumps({k:v for k,v in result.items() if k not in ('allocations','timing','comparisons')}))
if __name__=='__main__':main()
