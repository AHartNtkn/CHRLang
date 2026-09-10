"""Frozen paired primary costs for matched finite reuse and learning."""
import hashlib,itertools,json,os,random,resource,statistics as st,subprocess,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s05-finite-reuse-costs'
MODES=['recompute','eager','covered','reuse']
SCENARIOS=[(484,1,4,16,64),(511,2,4,16,64),(0,1,4,16,64),(484,1,1,16,64),(484,1,4,1,0)]

def analyze():
    rows=[json.loads(line) for line in (OUT/'runs.jsonl').read_text().splitlines()]
    by={(r['scenario'],r['cpu'],r['block'],r['mode']):r for r in rows}
    assert len(rows)==len(by)==640
    comparisons=[]
    for s in range(5):
        for base,mode in itertools.combinations(MODES,2):
            cpus=[]
            for cpu in [0,1]:
                a=[by[s,cpu,b,mode]['total_ns'] for b in range(15)]
                ref=[by[s,cpu,b,base]['total_ns'] for b in range(15)]
                ratios=[x/y for x,y in zip(a,ref)];median=st.median(ratios)
                direction='gain' if median<=.9 and sum(r<1 for r in ratios)>=13 else 'loss' if median>=1.1 and sum(r>1 for r in ratios)>=13 else 'unresolved'
                cpus.append(dict(cpu=cpu,median_ratio=median,ratios=ratios,below_one=sum(r<1 for r in ratios),above_one=sum(r>1 for r in ratios),direction=direction,candidate_median_ns=st.median(a),control_median_ns=st.median(ref),candidate_range=[min(a),max(a)],control_range=[min(ref),max(ref)]))
            direction=cpus[0]['direction'] if cpus[0]['direction']==cpus[1]['direction'] else 'unresolved'
            comparisons.append(dict(scenario=s,config=SCENARIOS[s],mode=mode,control=base,direction=direction,cpus=cpus))
    (OUT/'analysis.json').write_text(json.dumps(dict(warmups=40,measured=600,comparisons=comparisons),indent=2)+'\n')
    for c in comparisons:
        print(c['scenario'],c['mode'],'vs',c['control'],c['direction'],[round(x['median_ratio'],3) for x in c['cpus']])

def run():
    assert {0,1}<=os.sched_getaffinity(0)
    if (OUT/'manifest.json').exists() or (OUT/'runs.jsonl').exists():raise SystemExit('refusing to overwrite receipt')
    prior=json.loads((ROOT/'docs/experiments/results/s05-finite-reuse-ownership/manifest.json').read_text())
    hashes=prior['sha256'].copy()
    for path in [Path(__file__),ROOT/'docs/experiments/registrations/S05-finite-reuse-costs.md']:
        hashes[str(path.relative_to(ROOT))]=hashlib.sha256(path.read_bytes()).hexdigest()
    for f,h in hashes.items():assert hashlib.sha256((ROOT/f).read_bytes()).hexdigest()==h,f
    rng=random.Random(20260921);order=[]
    for block in [-1,*range(15)]:
        jobs=list(itertools.product(range(5),[0,1]));rng.shuffle(jobs)
        for scenario,cpu in jobs:
            modes=MODES.copy();rng.shuffle(modes)
            order.extend((scenario,cpu,block,mode) for mode in modes)
    (OUT/'manifest.json').write_text(json.dumps(dict(order=order,scenarios=SCENARIOS,hashes=hashes),indent=2)+'\n')
    start=time.monotonic()
    binary=ROOT/'target/s05-finite-reuse-ownership/primary/release/examples/learning_ownership'
    with (OUT/'runs.jsonl').open('x') as log:
        for n,(scenario,cpu,block,mode) in enumerate(order):
            assert time.monotonic()-start<600
            mask,weight,capacity,count,depth=SCENARIOS[scenario]
            def bounds():
                os.sched_setaffinity(0,{cpu});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(20,20))
            p=subprocess.run([binary,mode,str(mask),str(weight),str(capacity),str(count),'none',str(depth)],capture_output=True,text=True,timeout=60,preexec_fn=bounds)
            assert p.returncode==0 and not p.stderr,(scenario,cpu,block,mode,p.stderr)
            row=json.loads(p.stdout);assert not row['diagnostics'] and not row['allocation_meter']
            expected=0
            for i in range(count+1):
                domain=3 if i==0 else 7 if i%2 else 1
                expected+=weight**2*sum(bool(domain&(1<<a) and domain&(1<<b) and mask&(1<<(3*a+b))) for a in range(3) for b in range(3))
            assert row['answers']==expected and row['queries']==count+1
            log.write(json.dumps(dict(scenario=scenario,cpu=cpu,block=block,mode=mode,total_ns=sum(p['ns'] for p in row['phases']),row=row))+'\n');log.flush()
            if (n+1)%160==0:print(f'{n+1}/640 processes',flush=True)
    for f,h in hashes.items():assert hashlib.sha256((ROOT/f).read_bytes()).hexdigest()==h,f
    analyze()
    (OUT/'validation.json').write_text(json.dumps({p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in [OUT/'manifest.json',OUT/'runs.jsonl',OUT/'analysis.json']},indent=2)+'\n')

if __name__=='__main__':
    import sys
    if sys.argv[1:]==['--analyze']:analyze()
    elif not sys.argv[1:]:run()
    else:raise SystemExit('usage: costs.py [--analyze]')
