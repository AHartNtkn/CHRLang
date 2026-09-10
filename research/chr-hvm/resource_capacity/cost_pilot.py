"""Registered paired-block capacity lifecycle pilot and descriptive analysis."""
from pathlib import Path
import argparse,collections,hashlib,itertools,json,math,os,random,resource,statistics as st,subprocess,time
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s06-capacity-cost-pilot'
MODES=['capacity','scan','index','specialized-scan','specialized-index']
SCENARIOS=list(itertools.product([0,2,4],['empty','tight','spare'],[1,2],[1,4]))
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def analyze():
    manifest=json.loads((OUT/'manifest.json').read_text());valid=json.loads((OUT/'validation.json').read_text())
    for f,h in manifest['hashes'].items():assert digest(ROOT/f)==h,f
    for f,h in valid.items():assert digest(OUT/f)==h,f
    rows=[json.loads(l) for l in (OUT/'runs.jsonl').read_text().splitlines()]
    assert [[r['scenario'],r['cpu'],r['block'],r['mode']] for r in rows]==manifest['order']
    assert len(rows)==2880 and sum(r['block']>=0 for r in rows)==2520
    by={(r['scenario'],r['cpu'],r['block'],r['mode']):r for r in rows};assert len(by)==2880
    comparisons=[]
    for i,control in itertools.product(range(36),MODES[1:]):
        cpus=[]
        for cpu in [0,1]:
            a=[by[i,cpu,b,'capacity']['total_ns'] for b in range(7)];c=[by[i,cpu,b,control]['total_ns'] for b in range(7)];ratios=[x/y for x,y in zip(a,c)];median=st.median(ratios)
            direction='gain' if median<=.9 and max(ratios)<1 else 'loss' if median>=1.1 and min(ratios)>1 else 'unresolved'
            cpus.append(dict(cpu=cpu,median_ratio=median,ratios=ratios,direction=direction,capacity_range=[min(a),max(a)],control_range=[min(c),max(c)]))
        direction=cpus[0]['direction'] if cpus[0]['direction']==cpus[1]['direction'] else 'unresolved'
        def phases(mode):
            rs=[by[i,cpu,b,mode] for cpu in [0,1] for b in range(7)]
            names={p['name'] for p in rs[0]['row']['phases']}
            return dict(total_ns=st.median(r['total_ns'] for r in rs),phases={name:st.median(sum(p['ns'] for p in r['row']['phases'] if p['name']==name) for r in rs) for name in sorted(names)})
        comparisons.append(dict(scenario=SCENARIOS[i],control=control,direction=direction,cpus=cpus,capacity=phases('capacity'),ordinary=phases(control)))
    result=dict(comparisons=comparisons,counts={m:dict(collections.Counter(c['direction'] for c in comparisons if c['control']==m)) for m in MODES[1:]})
    (OUT/'analysis.json').write_text(json.dumps(result,indent=2)+'\n');print(result['counts'])
    for c in comparisons:
        if c['direction']!='gain':print(c['scenario'],c['control'],c['direction'],[round(p['median_ratio'],3) for p in c['cpus']])
def run():
    entry=json.loads((ROOT/'docs/experiments/results/s06-capacity-lifecycle-entry/audit.json').read_text())
    for f,h in entry['hashes'].items():assert digest(ROOT/f)==h
    assert {0,1}<=os.sched_getaffinity(0) and not (OUT/'runs.jsonl').exists()
    order=[];rng=random.Random(20260920)
    for block in [-1,*range(7)]:
        jobs=list(itertools.product(range(36),[0,1]));rng.shuffle(jobs)
        for i,cpu in jobs:
            modes=MODES.copy();rng.shuffle(modes);order.extend((i,cpu,block,m) for m in modes)
    paths=[Path(__file__),ROOT/'docs/experiments/registrations/S06-capacity-cost-pilot.md']
    hashes={**entry['hashes'],**{str(p.relative_to(ROOT)):digest(p) for p in paths}}
    (OUT/'manifest.json').write_text(json.dumps(dict(scenarios=SCENARIOS,order=order,hashes=hashes),indent=2)+'\n');start=time.monotonic()
    with (OUT/'runs.jsonl').open('w') as log:
        for j,(i,cpu,block,mode) in enumerate(order):
            assert time.monotonic()-start<1000,'whole pilot bound'
            n,supply,weight,reuse=SCENARIOS[i]
            def bounds():
                os.sched_setaffinity(0,{cpu});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(20,20))
            binary=ROOT/'target/s06-capacity-lifecycle-entry/primary/release/examples/capacity_lifecycle'
            p=subprocess.run([binary,mode,str(n),supply,str(weight),str(reuse)],capture_output=True,text=True,timeout=60,preexec_fn=bounds)
            assert p.returncode==0 and not p.stderr,(i,mode,p.returncode,p.stderr)
            row=json.loads(p.stdout);assert not row['diagnostics'] and not row['meter'] and all(p['memory'] is None for p in row['phases'])
            counts=[1 if n==0 else 0 if supply=='empty' else ((0 if q%2 else math.comb(n,n//2)) if supply=='tight' else (2 if q%2 else 2**n))*weight**n for q in range(reuse)]
            assert row['counts']==counts
            log.write(json.dumps(dict(scenario=i,cpu=cpu,block=block,mode=mode,total_ns=sum(p['ns'] for p in row['phases']),row=row))+'\n');log.flush()
            if (j+1)%360==0:print(j+1,'/2880 primary processes validated',flush=True)
    (OUT/'validation.json').write_text(json.dumps({p.name:digest(p) for p in [OUT/'manifest.json',OUT/'runs.jsonl']},indent=2)+'\n')
    analyze()
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--analyze',action='store_true');a=p.parse_args()
    analyze() if a.analyze else run()
