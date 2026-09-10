"""Qualify primary and diagnostic learning paths; never compare timings here."""
from pathlib import Path
import itertools,json,subprocess,resource,hashlib,time
root=Path(__file__).resolve().parents[3]
raw=root/'docs/experiments/results/s06-learning-prefix-entry'
def bounds():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
    resource.setrlimit(resource.RLIMIT_CPU,(20,20))
def allocation_signature(row):
    return [{k:v for k,v in p.items() if k!='ns'} for p in row['phases']]
configs=[(m,a,w,c,4,x,d) for m,a,w,c,x,d in itertools.product(['recompute','eager','covered'],[0,484,511],[1,2],[0,4],['none','step1'],[0,16,64])]
assert len(configs)==216
started=time.monotonic()
results=[]
assert not (raw/'runs.jsonl').exists(), 'Preserve prior outcomes'
with (raw/'runs.jsonl').open('w') as output:
    for mode,mask,weight,capacity,count,cancel,depth in configs:
        assert time.monotonic()-started<600, "whole entry bound"
        pair=[]
        for build in ['primary','diagnostic','diagnostic']:
            binary=root/f'target/s06-learning-prefix-entry/{build}/release/examples/learning_ownership'
            p=subprocess.run([binary,mode,str(mask),str(weight),str(capacity),str(count),cancel,str(depth)],capture_output=True,text=True,timeout=60,check=True,preexec_fn=bounds)
            assert not p.stderr,p.stderr
            row=json.loads(p.stdout);pair.append(row);assert row['depth']==depth
            output.write(json.dumps({'build':build,**row})+'\n');output.flush()
            assert row['diagnostics']==row['allocation_meter']==(build=='diagnostic')
            expected=0
            for i in range(count+1):
                domain=3 if i==0 else 7 if i%2 else 1
                n=0 if cancel=='step1' and i==1 else sum(bool(domain&(1<<a) and domain&(1<<b) and mask&(1<<(3*a+b))) for a in range(3) for b in range(3))*weight**2
                assert (row['first_owned_ns'][i] is not None)==bool(n)
                expected+=n
            assert row['answers']==expected
            assert row['retained_regions']<=capacity
            phases=row['phases']
            assert sum(p['name']=='session_drop' for p in phases)==count+1
            if build=='primary':
                assert row['baseline'] is None and all(p['memory'] is None for p in phases)
            else:
                baseline=row['baseline']
                assert phases[0]['memory']['live_start']==phases[-1]['memory']['live_end']==baseline
                for a,b in zip(phases,phases[1:]):assert a['memory']['live_end']==b['memory']['live_start']
        assert pair[0]['answers']==pair[1]['answers']==pair[2]['answers']
        assert pair[0]['retained_regions']==pair[1]['retained_regions']==pair[2]['retained_regions']
        assert allocation_signature(pair[1])==allocation_signature(pair[2])
        results.append({'mode':mode,'mask':mask,'weight':weight,'capacity':capacity,'followups':count,'cancel':cancel,'depth':depth,'answers':pair[0]['answers'],'retained_regions':pair[0]['retained_regions']})
files=[Path(__file__),root/'research/chr-compiled/experiments/finite_learning.rs',root/'research/chr-direct-conditional/examples/learning_ownership.rs',root/'research/chr-direct-conditional/tests/finite_learning_gate.rs',root/'research/chr-direct-conditional/Cargo.toml',raw/'runs.jsonl',root/'target/s06-learning-prefix-entry/primary/release/examples/learning_ownership',root/'target/s06-learning-prefix-entry/diagnostic/release/examples/learning_ownership',root/'docs/experiments/registrations/S06-learning-prefix-entry.md']
(raw/'audit.json').write_text(json.dumps({'configurations':len(results),'processes':len(results)*3,'results':results,'sha256':{str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in files}},indent=2)+'\n')
print(f'{len(results)} configurations/{len(results)*3} processes: primary/diagnostic outcomes, allocation repeatability and ownership passed')
