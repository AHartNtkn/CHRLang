"""Observed compilation amortization; no extrapolated crossover claim."""
import json
from pathlib import Path
import statistics as stats
from artifact_amortization import CASES
from artifact_cost_pilot import validate

OUT=Path(__file__).resolve().parents[3]/'docs/experiments/results/s01-native-amortization'


def read(name):return json.loads((OUT/(name+'.json')).read_text())
def assess(ratios):
    median=stats.median(ratios)
    status='gain' if median<=.9 and max(ratios)<1 else 'loss' if median>=1.1 and min(ratios)>1 else 'unresolved'
    return dict(median_ratio=median,min_ratio=min(ratios),max_ratio=max(ratios),status=status)


def main():
    read('summary')
    disposal=read('disposal');costs={}
    for family in ['chain','payload']:
        costs[family]=[]
        for rep in range(5):
            name=f'ordinary-{family}-{rep}'
            emission=json.loads(read('emit-'+name)['stdout'])
            cost=sum(emission[k] for k in ['generation_ns','source_write_ns','buffer_drop_ns'])
            cost+=read('compile-'+name)['wall_seconds']*1e9
            cost+=sum(d['unlink_ns'] for d in disposal if Path(d['path']).name in [name,name+'.rs'])
            costs[family].append(cost)
    results=[];checked=0
    for f,n,q in CASES:
        runtime={};process={};allocation={}
        for m in ['planned','native']:
            runtime[m]=[];process[m]=[]
            for b in range(8):
                result=read(f'time-{b}-{f}-{q}-{m}');rs=validate(result,f,m,n,q);checked+=q
                if b:
                    runtime[m].append(rs[-1]['lifecycle_ns'])
                    process[m].append(result['wall_seconds'])
            memories=[]
            for rep in [0,1]:
                rs=validate(read(f'alloc-{rep}-{f}-{q}-{m}'),f,m,n,q,True);checked+=q
                memories.append([r['memory'] for r in rs])
            assert memories[0]==memories[1]
            allocation[m]=dict(requested_bytes=sum(p['requested_bytes'] for r in memories[0] for p in r.values()),absolute_requested_peak=max(p['peak_live'] for r in memories[0] for p in r.values()))
        native=runtime['native'];planned=runtime['planned'];cost=costs[f]
        inclusive=[(a+stats.median(cost))/b for a,b in zip(native,planned)]
        row=dict(family=f,size=n,queries=q,runtime=assess([a/b for a,b in zip(native,planned)]),compilation_inclusive=assess(inclusive),
                 runtime_ns=runtime,artifact_cost_ns=cost,allocation=allocation,process_wall_seconds=process,
                 amortized_across_observed_ranges=max(native)+max(cost)<min(planned),
                 not_amortized_across_observed_ranges=min(native)+min(cost)>max(planned))
        results.append(row)
        print(f,q,'runtime',round(row['runtime']['median_ratio'],3),'inclusive',round(row['compilation_inclusive']['median_ratio'],3),row['compilation_inclusive']['status'],'range recovery',row['amortized_across_observed_ranges'],flush=True)
    assert checked==696320
    output=dict(results=results,complete_queries=checked,diagnostic_replay_cells=10,
                interpretation='Observed query counts and descriptive paired criterion; no universal crossover or architecture ranking')
    (OUT/'analysis.json').write_text(json.dumps(output,indent=2)+'\n')


if __name__=='__main__':main()
