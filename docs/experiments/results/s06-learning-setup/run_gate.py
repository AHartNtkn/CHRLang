"""Run the prospectively fixed learning requested-allocation qualification."""
from pathlib import Path
import itertools
import json
import resource
import subprocess
import hashlib

root=Path.cwd()
raw=root/'docs/experiments/results/s06-learning-setup'
binary=root/'target/debug/examples/learning_ownership'
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
    resource.setrlimit(resource.RLIMIT_CPU,(60,60))
check=subprocess.run([binary,'meter-check'],capture_output=True,text=True,timeout=60,check=True,preexec_fn=limits)
(raw/'meter-check.log').write_text(check.stdout)
results=[]
with (raw/'runs.jsonl').open('w') as output:
    for mode,mask,weight,capacity,count in itertools.product(['recompute','eager','covered'],[0,273,238,511],[1,2],[0,1,4],[1,4,16]):
        pair=[]
        for repeat in range(2):
            proc=subprocess.run([binary,mode,str(mask),str(weight),str(capacity),str(count)],capture_output=True,text=True,timeout=60,check=True,preexec_fn=limits)
            assert not proc.stderr,proc.stderr
            row=json.loads(proc.stdout)
            pair.append(row)
            output.write(json.dumps({'repeat':repeat,**row})+'\n');output.flush()
        assert pair[0]==pair[1],(mode,mask,weight,capacity,count)
        row=pair[0];phases=row['phases'];baseline=row['baseline']
        assert phases[0]['memory']['live_start']==baseline
        assert phases[-1]['memory']['live_end']==baseline
        for a,b in zip(phases,phases[1:]):
            assert a['memory']['live_end']==b['memory']['live_start'],(a,b)
        expected=0
        for i in range(count+1):
            domain=3 if i==0 else 7 if i%2 else 1
            pairs=sum(bool(domain&(1<<a) and domain&(1<<b) and mask&(1<<(3*a+b))) for a in range(3) for b in range(3))
            expected+=pairs*weight**2
        assert row['answers']==expected
        assert row['retained_regions']<=capacity
        if mode=='recompute' or capacity==0: assert row['retained_regions']==0
        results.append({'mode':mode,'mask':mask,'weight':weight,'capacity':capacity,'followups':count,'answers':expected,'retained':row['retained_regions'],'requested':sum(p['memory']['requested_bytes'] for p in phases),'peak_above_baseline':max(p['memory']['peak_live'] for p in phases)-baseline,'learner_drop_bytes':next(p['memory']['live_start']-p['memory']['live_end'] for p in phases if p['name']=='learner_drop')})
files=[binary,Path(__file__),root/'research/chr-direct-conditional/examples/learning_ownership.rs',root/'research/chr-compiled/experiments/finite_learning.rs',root/'research/chr-compiled/experiments/meter.rs',root/'research/chr-direct-conditional/tests/finite_learning_gate.rs',raw/'runs.jsonl']
(raw/'audit.json').write_text(json.dumps({'configurations':len(results),'processes':2*len(results),'results':results,'sha256':{str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in files}},indent=2)+'\n')
print(f'{len(results)} configurations, {2*len(results)} processes: deterministic phases, exact counts, full heap restoration')
