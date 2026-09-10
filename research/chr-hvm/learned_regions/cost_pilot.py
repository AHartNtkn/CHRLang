"""Registered primary lifecycle pilot; validation and allocation are separate runs."""
from pathlib import Path
import hashlib,itertools,json,os,random,resource,subprocess,time
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s06-learning-cost-pilot'
ENTRY=ROOT/'docs/experiments/results/s06-learning-prefix-entry/audit.json'
MODES=['recompute','eager','covered']
CONFIGS=list(itertools.product(MODES,[0,484,511],[0,16,64],[1,4,16]))
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    entry=json.loads(ENTRY.read_text())
    for f,h in entry['sha256'].items():assert digest(ROOT/f)==h,f
    assert {0,1}<=os.sched_getaffinity(0)
    assert not (OUT/'runs.jsonl').exists(),'Preserve prior experiment'
    rng=random.Random(20260918);order=[]
    for rep in [-1,0,1,2,3,4]:
        jobs=[(i,cpu,rep) for i in range(len(CONFIGS)) for cpu in [0,1]];rng.shuffle(jobs);order+=jobs
    files=[Path(__file__),ENTRY,ROOT/'docs/experiments/registrations/S06-learning-cost-pilot.md']+[ROOT/p for p in entry['sha256'] if not p.endswith('runs.jsonl')]
    freeze={str(p.relative_to(ROOT)):digest(p) for p in files}
    (OUT/'manifest.json').write_text(json.dumps(dict(configurations=CONFIGS,order=order,hashes=freeze),indent=2)+'\n')
    started=time.monotonic()
    def run(i,build,cpu):
        assert time.monotonic()-started<900,'whole pilot bound'
        mode,mask,depth,count=CONFIGS[i]
        def bounds():
            os.sched_setaffinity(0,{cpu});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(20,20))
        binary=ROOT/f'target/s06-learning-prefix-entry/{build}/release/examples/learning_ownership'
        p=subprocess.run([binary,mode,str(mask),'1','4',str(count),'none',str(depth)],capture_output=True,text=True,timeout=60,preexec_fn=bounds)
        assert p.returncode==0 and not p.stderr,(i,build,p.returncode,p.stderr)
        row=json.loads(p.stdout);assert row['diagnostics']==row['allocation_meter']==(build=='diagnostic')
        assert (row['mode'],row['accepted'],row['depth'],row['queries'])==(mode,mask,depth,count+1)
        expected=0
        for q in range(count+1):
            domain=3 if q==0 else 7 if q%2 else 1
            n=sum(bool(domain&(1<<a) and domain&(1<<b) and mask&(1<<(3*a+b))) for a in range(3) for b in range(3))
            expected+=n;assert (row['first_owned_ns'][q] is not None)==bool(n)
        assert row['answers']==expected and row['retained_regions']<=4
        phases=row['phases']
        if build=='diagnostic':
            assert phases[0]['memory']['live_start']==phases[-1]['memory']['live_end']==row['baseline']
            for a,b in zip(phases,phases[1:]):assert a['memory']['live_end']==b['memory']['live_start']
        else:assert row['baseline'] is None and all(p['memory'] is None for p in phases)
        return dict(config=i,build=build,cpu=cpu,row=row,total_ns=sum(p['ns'] for p in phases),service_ns=sum(p['ns'] for p in phases if p['name'] in ['finite_service','caller_transport_execute_observe_drop']))
    with (OUT/'qualification.jsonl').open('w') as log:
        for i in range(len(CONFIGS)):
            records=[]
            for build in ['primary','diagnostic','diagnostic']:
                r=run(i,build,0);records.append(r);log.write(json.dumps(r)+'\n');log.flush()
            assert len({(r['row']['answers'],r['row']['retained_regions']) for r in records})==1
            def allocation(r):return [{k:v for k,v in p.items() if k!='ns'} for p in r['row']['phases']]
            assert allocation(records[1])==allocation(records[2])
    print('All81 configurations qualify in243 processes',flush=True)
    with (OUT/'runs.jsonl').open('w') as log:
        for n,(i,cpu,rep) in enumerate(order):
            r=run(i,'primary',cpu);r['repetition']=rep;log.write(json.dumps(r)+'\n');log.flush()
            if (n+1)%162==0:print(n+1,'/',len(order),'primary processes complete',flush=True)
    for f,h in freeze.items():assert digest(ROOT/f)==h,f
    (OUT/'validation.json').write_text(json.dumps(dict(qualification=243,warmups=162,measured=810,hashes={p.name:digest(p) for p in [OUT/'manifest.json',OUT/'qualification.jsonl',OUT/'runs.jsonl']}),indent=2)+'\n')
if __name__=='__main__':main()
