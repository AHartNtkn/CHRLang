"""Registered descriptive contrasts and compilation-inclusive sensitivity."""
import json
import statistics as stats
from pathlib import Path
from artifact_cost_pilot import PROFILES, modes, subscription, artifact, validate

ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s01-native-cost-pilot'


def read(name):return json.loads((OUT/(name+'.json')).read_text())
def rows(block,opt,f,n,q,m):
    result=read(f'time-{block}-{opt}-{f}-{n}-{q}-{m}')
    return validate(result,f,m,n,q)
def values(opt,f,n,q,m):return [rows(b,opt,f,n,q,m)[-1]['lifecycle_ns'] for b in range(1,8)]
def compare(a,b):
    ratios=[x/y for x,y in zip(a,b)]
    median=stats.median(ratios)
    status='gain' if median<=.9 and max(ratios)<1 else 'loss' if median>=1.1 and min(ratios)>1 else 'unresolved'
    return dict(median_ratio=median,min_ratio=min(ratios),max_ratio=max(ratios),status=status)
def compile_cost(opt,f,m):
    a=artifact(f,m)
    return [read(f'compile-ordinary-{opt}-{a}-{r}')['wall_seconds']*1e9 + sum(json.loads(read(f'emit-ordinary-{opt}-{a}-{r}')['stdout'])[k] for k in ['generation_ns','source_write_ns','buffer_drop_ns']) + sum(d['unlink_ns'] for d in read('disposal') if Path(d['path']).name in [f'ordinary-{opt}-{a}-{r}',f'ordinary-{opt}-{a}-{r}.rs']) for r in range(5)]


def main():
    summary=read('summary');contrasts=[];link=[];inclusive=[];memory=[]
    for f,n,qs in PROFILES:
        for q in qs:
            for opt in ['off','thin']:
                pairs=[('planned','generic'),('native','generic'),('native','planned'),('native','native-generic-repair'),('native','native-planned'),('specialized','planned'),('native-specialized','native')]
                if subscription(f):
                    pairs += [(m,b) for m in ['retained-indexed','retained-eager','retained-subscribed'] for b in ['planned','native']]
                    pairs += [('retained-eager','retained-indexed'),('retained-subscribed','retained-indexed')]
                for a,b in pairs:
                    contrasts.append(dict(family=f,size=n,queries=q,link=opt,candidate=a,control=b,**compare(values(opt,f,n,q,a),values(opt,f,n,q,b))))
                # Range of complete observed runtime plus observed one-time native artifact cost.
                # Generic artifacts are reusable across rulesets; no per-ruleset generic compilation charge.
                cost=compile_cost(opt,f,'native')
                native=values(opt,f,n,q,'native');baseline=values(opt,f,n,q,'planned')
                inclusive.append(dict(family=f,size=n,queries=q,link=opt,native_runtime_median_ns=stats.median(native),planned_runtime_median_ns=stats.median(baseline),native_artifact_cost_ns=cost,
                                      native_total_min_ns=min(native)+min(cost),native_total_max_ns=max(native)+max(cost),planned_min_ns=min(baseline),planned_max_ns=max(baseline)))
            for m in modes(f):
                link.append(dict(family=f,size=n,queries=q,mode=m,**compare(values('thin',f,n,q,m),values('off',f,n,q,m))))
                pairs=[]
                for rep in [0,1]:
                    rs=validate(read(f'alloc-{rep}-{f}-{n}-{q}-{m}'),f,m,n,q,True)
                    pairs.append([r['memory'] for r in rs])
                assert pairs[0]==pairs[1]
                memory.append(dict(family=f,size=n,queries=q,mode=m,requested_bytes=sum(p['requested_bytes'] for row in pairs[0] for p in row.values()),absolute_requested_peak=max(p['peak_live'] for row in pairs[0] for p in row.values())))
    # Descriptive extrapolation only, beyond measured reuse: no observed crossover claim.
    models=[]
    for f,n,qs in PROFILES:
        q=max(qs)
        for opt in ['off','thin']:
            med={}
            for m in ['native','planned']:
                rs=[rows(b,opt,f,n,q,m) for b in range(1,8)]
                med[m]=dict(steady=stats.median(sum(r['query_ns'] for r in x[1:-1])/(q-1) for x in rs),fixed=stats.median(sum(x[-1][k] for k in ['source_ns','prepare_ns','prepared_drop_ns'])+x[0]['query_ns'] for x in rs))
            saving=med['planned']['steady']-med['native']['steady']
            cost=stats.median(compile_cost(opt,f,'native'))
            models.append(dict(family=f,size=n,link=opt,measured_reuse=q,steady_saved_ns=saving,
                               modeled_queries=(1+max(0,(cost+med['native']['fixed']-med['planned']['fixed'])/saving)) if saving>0 else None,
                               scope='Linear model from measured later queries; not observed crossover or sustained lifetime evidence'))
    result=dict(runtime_contrasts=contrasts,link_contrasts=link,compilation_inclusive=inclusive,allocation=memory,reuse_models=models,
                validation=dict(measured_blocks=7,timing_cells=286,diagnostic_cells=len(memory),exact_allocation_replay=True),
                limits='Descriptive paired criterion, no cross-family aggregate or universal winner. Compiler costs include experimental harness linkage. Absolute requested peaks include oracle/process allocations.')
    (OUT/'analysis.json').write_text(json.dumps(result,indent=2)+'\n')
    for f,n,qs in PROFILES:
        selected=[r for r in contrasts if r['family']==f and r['size']==n and r['queries']==max(qs) and r['candidate']=='native' and r['control']=='planned']
        print(f,n,max(qs),[(r['link'],round(r['median_ratio'],3),r['status']) for r in selected])
    print('validated',len(contrasts),'runtime contrasts;',len(link),'link contrasts;',len(memory),'allocation cells')


if __name__=='__main__':main()
