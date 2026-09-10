"""Independent paired-block confirmation with prospectively fixed scenarios."""
from pathlib import Path
import collections,hashlib,itertools,json,os,random,resource,statistics as st,subprocess,time
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s06-learning-cost-confirmation'
SCENARIOS=[(0,64,16),(484,64,16),(511,64,16),(484,64,1)];MODES=['recompute','eager','covered']
def main():
    prior=json.loads((ROOT/'docs/experiments/results/s06-learning-cost-pilot/manifest.json').read_text())
    for p,h in prior['hashes'].items():assert hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h
    assert {0,1}<=os.sched_getaffinity(0) and not (OUT/'runs.jsonl').exists()
    rng=random.Random(20260919);order=[]
    for block in [-1,*range(15)]:
        jobs=list(itertools.product(range(4),[0,1]));rng.shuffle(jobs)
        for scenario,cpu in jobs:
            modes=MODES.copy();rng.shuffle(modes)
            order.extend((scenario,cpu,block,mode) for mode in modes)
    paths=[Path(__file__),ROOT/'docs/experiments/registrations/S06-learning-cost-confirmation.md']
    (OUT/'manifest.json').write_text(json.dumps(dict(order=order,scenarios=SCENARIOS,hashes={**prior['hashes'],**{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}}),indent=2)+'\n')
    started=time.monotonic();rows=[]
    with (OUT/'runs.jsonl').open('w') as log:
        for scenario,cpu,block,mode in order:
            assert time.monotonic()-started<600
            mask,depth,count=SCENARIOS[scenario]
            def bounds():
                os.sched_setaffinity(0,{cpu});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(20,20))
            binary=ROOT/'target/s06-learning-prefix-entry/primary/release/examples/learning_ownership'
            p=subprocess.run([binary,mode,str(mask),'1','4',str(count),'none',str(depth)],capture_output=True,text=True,timeout=60,preexec_fn=bounds)
            assert p.returncode==0 and not p.stderr,(scenario,cpu,block,mode,p.stderr)
            r=json.loads(p.stdout);assert not r['diagnostics'] and not r['allocation_meter']
            expected=0
            for i in range(count+1):
                domain=3 if i==0 else 7 if i%2 else 1
                expected+=sum(bool(domain&(1<<a) and domain&(1<<b) and mask&(1<<(3*a+b))) for a in range(3) for b in range(3))
            assert r['answers']==expected and r['queries']==count+1
            item=dict(scenario=scenario,cpu=cpu,block=block,mode=mode,total_ns=sum(p['ns'] for p in r['phases']),row=r)
            rows.append(item);log.write(json.dumps(item)+'\n');log.flush()
    by={(r['scenario'],r['cpu'],r['block'],r['mode']):r for r in rows};assert len(by)==384
    comparisons=[]
    for s,mode in itertools.product(range(4),['eager','covered']):
        cpus=[]
        for cpu in [0,1]:
            a=[by[s,cpu,b,mode]['total_ns'] for b in range(15)];base=[by[s,cpu,b,'recompute']['total_ns'] for b in range(15)];ratios=[x/y for x,y in zip(a,base)];median=st.median(ratios)
            gain=sum(r<1 for r in ratios);loss=sum(r>1 for r in ratios)
            direction='gain' if median<=.9 and gain>=13 else 'loss' if median>=1.1 and loss>=13 else 'unresolved'
            cpus.append(dict(cpu=cpu,median_ratio=median,ratios=ratios,below_one=gain,above_one=loss,direction=direction,candidate_range=[min(a),max(a)],recompute_range=[min(base),max(base)],extrema_gain=max(a)<min(base),extrema_loss=min(a)>max(base)))
        direction=cpus[0]['direction'] if cpus[0]['direction']==cpus[1]['direction'] else 'unresolved'
        comparisons.append(dict(scenario=SCENARIOS[s],mode=mode,direction=direction,cpus=cpus))
    (OUT/'analysis.json').write_text(json.dumps(dict(warmups=24,measured=360,comparisons=comparisons),indent=2)+'\n')
    (OUT/'validation.json').write_text(json.dumps({p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in [OUT/'manifest.json',OUT/'runs.jsonl',OUT/'analysis.json']},indent=2)+'\n')
    for c in comparisons:print(c['scenario'],c['mode'],c['direction'],[round(p['median_ratio'],3) for p in c['cpus']])
if __name__=='__main__':main()
