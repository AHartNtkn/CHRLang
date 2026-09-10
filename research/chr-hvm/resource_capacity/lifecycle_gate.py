"""Complete-answer and ownership qualification; no timing comparisons."""
from pathlib import Path
import hashlib,itertools,json,math,resource,subprocess,time
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s06-capacity-lifecycle-entry'
MODES=['capacity','scan','index','specialized-scan','specialized-index']
CONFIGS=list(itertools.product(MODES,[0,2,4],['empty','tight','spare'],[1,2],[1,4]))
def main():
    assert len(CONFIGS)==180 and not (OUT/'runs.jsonl').exists()
    paths=[Path(__file__),ROOT/'docs/experiments/registrations/S06-capacity-lifecycle-entry.md',ROOT/'research/chr-compiled/experiments/resource_capacity.rs',ROOT/'research/chr-direct-conditional/examples/capacity_lifecycle.rs',ROOT/'research/chr-direct-conditional/tests/resource_capacity_entry.rs',ROOT/'research/chr-direct-conditional/Cargo.toml']+[ROOT/f'target/s06-capacity-lifecycle-entry/{build}/release/examples/capacity_lifecycle' for build in ['primary','diagnostic']]
    hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
    (OUT/'manifest.json').write_text(json.dumps(dict(configurations=CONFIGS,hashes=hashes),indent=2)+'\n');start=time.monotonic()
    def bounds():
        resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(20,20))
    results=[]
    with (OUT/'runs.jsonl').open('w') as log:
        for index,(mode,n,supply,weight,reuse) in enumerate(CONFIGS):
            records=[]
            for build in ['primary','diagnostic','diagnostic']:
                assert time.monotonic()-start<600,'launcher resource bound'
                binary=ROOT/f'target/s06-capacity-lifecycle-entry/{build}/release/examples/capacity_lifecycle'
                p=subprocess.run([binary,mode,str(n),supply,str(weight),str(reuse)],capture_output=True,text=True,timeout=60,preexec_fn=bounds)
                assert p.returncode==0 and not p.stderr,(index,build,p.returncode,p.stderr)
                r=json.loads(p.stdout);records.append(r);log.write(json.dumps(dict(config=index,build=build,row=r))+'\n');log.flush()
                assert r['diagnostics']==r['meter']==(build=='diagnostic')
                counts=[]
                for q in range(reuse):
                    value=1 if n==0 else 0 if supply=='empty' else ((0 if q%2 else math.comb(n,n//2)) if supply=='tight' else (2 if q%2 else 2**n)) * weight**n
                    counts.append(value)
                assert r['counts']==counts,(index,r['counts'],counts)
                phases=r['phases'];assert len(phases)==5+3*reuse
                if build=='primary':
                    assert all(p['memory'] is None for p in phases)
                    assert all(w[1:]==[0,0] for w in r['work'])
                else:
                    assert phases[0]['memory']['live_start']==phases[-1]['memory']['live_end']
                    for a,b in zip(phases,phases[1:]):assert a['memory']['live_end']==b['memory']['live_start']
            assert records[0]['counts']==records[1]['counts']==records[2]['counts']
            assert [w[0] for w in records[0]['work']]==[w[0] for w in records[1]['work']]
            def signature(r):return {k:([{pk:pv for pk,pv in p.items() if pk!='ns'} for p in v] if k=='phases' else v) for k,v in r.items()}
            assert signature(records[1])==signature(records[2])
            results.append(dict(config=CONFIGS[index],counts=records[0]['counts'],requested=sum(p['memory']['requested_bytes'] for p in records[1]['phases']),work=records[1]['work']))
            if (index+1)%30==0:print(index+1,'/180 configurations qualify',flush=True)
    for p,h in hashes.items():assert hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h,p
    (OUT/'audit.json').write_text(json.dumps(dict(configurations=180,processes=540,results=results,hashes={**hashes,str((OUT/'runs.jsonl').relative_to(ROOT)):hashlib.sha256((OUT/'runs.jsonl').read_bytes()).hexdigest()}),indent=2)+'\n')
    print('540 processes: complete answers, diagnostics pairing, repeated allocations and restored ownership pass')
if __name__=='__main__':main()
