"""Post-run audit correction: first_ns is timing, not a deterministic snapshot field.
The original frozen auditor and all experiment inputs remain intact.
"""
import csv,hashlib,io,json,tarfile
from run_graph_cost_pilot import ROOT,OUT,PREFIX

def main():
    m=json.loads((OUT/f'{PREFIX}-manifest.json').read_text())
    rows=[json.loads(s) for s in (OUT/f'{PREFIX}.jsonl').read_text().splitlines()]
    ids=[f'{f}-{n}-{kind}' for f,n in [('dag',4),('dag',8),('unary',16),('unary',64),('symmetric',4),('symmetric',8),('retention',64),('retention',1024)] for kind in ('repeat','distinct')]
    policies=('eager_clone','eager_compare','eager_graph_compare','graph_compare')
    order=[dict(kind=kind,id=id,policy=p) for kind in ('system','meter') for id in ids for p in policies]
    assert m['order']==order and [{k:r[k] for k in ('kind','id','policy')} for r in rows]==order
    # The post-run field-selector correction is independently hashed below.
    # All experiment inputs and the preregistered auditor are checked in the archive.
    auditor_path='research/chr-persistent/scripts/audit_graph_cost_pilot.py'
    assert all(hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h for p,h in m['sha256'].items() if p!=auditor_path)
    with tarfile.open(OUT/f'{PREFIX}-inputs.tar.gz','r:gz') as a:
        assert all(hashlib.sha256(a.extractfile(p).read()).hexdigest()==h for p,h in m['sha256'].items())
    parsed={};header=None
    phases=('prepared','first','done');readings=('before','prepared','first','done','index_dropped','engine_dropped','output_drop_before','output_dropped')
    for row in rows:
        assert row['exit']==0 and not row['stderr'] and row['wall_seconds']<30
        reader=csv.DictReader(io.StringIO(row['stdout']),delimiter='\t');data=list(reader)
        assert len(data)==1 and len(reader.fieldnames)==185 and len(set(reader.fieldnames))==185
        if header is None:header=reader.fieldnames
        else:assert header==reader.fieldnames
        d=data[0];assert None not in d and all(v is not None for v in d.values())
        assert d['case']==row['id'] and d['policy']==row['policy']
        d={k:(v if k in ('case','policy') else int(v)) for k,v in d.items()}
        expected=32 if row['id'].endswith('distinct') else (2 if row['id'].startswith('symmetric') else 1)
        assert d['expected_answers']==d['done_deliveries']==expected and d['validated']==1
        assert d['done_raw']==d['done_source_completed']==32 and d['done_frontier']==0 and d['done_source_failed']==0
        assert d['prepared_raw']==0 and d['first_raw']==d['first_deliveries']==1
        assert d['prepare_ns']<=d['first_ns']<=d['cold_ns']
        graph=row['policy']=='graph_compare'
        assert d['done_exports']==(expected if graph else 32)
        assert d['done_eager_export_answers']==(0 if graph else 32)
        assert d['done_capture_snapshots']==(32 if graph else 0)
        assert d['done_index_keys']==(expected if row['policy'] in ('graph_compare','eager_clone') else 0)
        assert d['metered']==int(row['kind']=='meter')
        if row['kind']=='system':assert all(d[f'{p}_{f}']==0 for p in readings for f in ('calls','requested','live','peak'))
        else:
            assert d['output_dropped_live']==d['before_live']
            assert d['engine_dropped_live']==d['output_drop_before_live']
            assert d['done_live']>=d['index_dropped_live']>=d['engine_dropped_live']>=d['output_dropped_live']
            for field in ('calls','requested','peak'):
                values=[d[f'{p}_{field}'] for p in readings];assert values==sorted(values)
            assert all(d[f'{p}_peak']>=d[f'{p}_live'] for p in readings)
        parsed[(row['kind'],row['id'],row['policy'])]=d
    counter_keys=[k for k in header if not k.endswith('_ns') and any(k.startswith(p+'_') for p in phases) and not any(k==p+'_'+f for p in phases for f in ('calls','requested','live','peak'))]
    for id in ids:
        source_by_phase={}
        for policy in policies:
            a=parsed[('system',id,policy)];b=parsed[('meter',id,policy)]
            assert all(a[k]==b[k] for k in counter_keys)
            for phase in phases:
                source={k.removeprefix(phase+'_source_'):a[k] for k in counter_keys if k.startswith(phase+'_source_')}
                source['dereferences']-=a[phase+'_eager_export_dereferences'];source['map_visits']-=a[phase+'_eager_export_map_visits']
                source['actual_queue_peak']=a[phase+'_queue_peak']
                if phase in source_by_phase:assert source==source_by_phase[phase],(id,phase,policy)
                else:source_by_phase[phase]=source
        for phase in phases:
            for name in ('term_pairs','occurrence_scans','occurrence_candidates','backtracks'):
                assert parsed[('system',id,'eager_graph_compare')][phase+'_graph_compare_'+name]==parsed[('system',id,'graph_compare')][phase+'_graph_compare_'+name]
                assert parsed[('system',id,'eager_clone')][phase+'_eager_compare_'+name]==parsed[('system',id,'eager_compare')][phase+'_eager_compare_'+name]
    metrics=[]
    for id in ids:
        for p in policies:
            t=parsed[('system',id,p)];a=parsed[('meter',id,p)]
            metrics.append(dict(id=id,policy=p,cold_ns=t['cold_ns'],first_ns=t['first_ns'],prepare_ns=t['prepare_ns'],
                synchronous_lifecycle_ns=sum(t[k] for k in ('cold_ns','index_drop_ns','engine_drop_ns','output_drop_ns')),
                allocation_calls=a['engine_dropped_calls']-a['before_calls']+a['output_dropped_calls']-a['output_drop_before_calls'],
                requested_bytes=a['engine_dropped_requested']-a['before_requested']+a['output_dropped_requested']-a['output_drop_before_requested'],
                prevalidation_peak_above_baseline=a['engine_dropped_peak']-a['before_live'],
                delivery_live_bytes=a['engine_dropped_live']-a['before_live'],
                independently_owned_index_bytes=a['done_live']-a['index_dropped_live'],
                engine_live_bytes_after_index_drop=a['index_dropped_live']-a['engine_dropped_live']))
    result=dict(passed=True,auditor_sha256=hashlib.sha256(open(__file__,'rb').read()).hexdigest(),audit_revision='Exclude first_ns from deterministic snapshot fields; timings are not replay counters',isolated_children=128,workloads=16,policies=4,fields=185,
                matched_source_and_comparator_work=True,system_meter_agree=True,all_measured_ownership_released=True,
                single_observations_not_rankings=True,metrics=metrics,max_child_wall_seconds=max(r['wall_seconds'] for r in rows))
    with (OUT/f'{PREFIX}-audit-v2.json').open('x') as f:json.dump(result,f,indent=2);f.write('\n')
    print(json.dumps({k:v for k,v in result.items() if k!='metrics'}))
if __name__=='__main__':main()
