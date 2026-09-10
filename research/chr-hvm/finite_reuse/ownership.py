"""Prospective finite reuse allocation gate; no timing ranking."""
import hashlib
import itertools
import json
import resource
import subprocess
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
RAW = ROOT / 'docs/experiments/results/s05-finite-reuse-ownership'
BIN = ROOT / 'target/s05-finite-reuse-ownership'
MODES = ['recompute', 'eager', 'covered', 'reuse']
main = [(m,a,w,c,n,'none',d) for m,a,w,c,n,d in itertools.product(MODES,[0,484,511],[1,2],[0,1,4],[1,16],[0,64])]
configs = main + [(m,a,1,4,16,'step1',64) for m,a in itertools.product(MODES,[0,484,511])]
assert len(configs) == 300

def limits():
    resource.setrlimit(resource.RLIMIT_AS, (1<<30, 1<<30))
    resource.setrlimit(resource.RLIMIT_CPU, (60,60))

def signature(row):
    return {k: ([{a:b for a,b in p.items() if a!='ns'} for p in v] if k=='phases' else v) for k,v in row.items() if k!='first_owned_ns'}

def expected(config):
    _, mask, weight, _, count, cancel, _ = config
    result=0
    for i in range(count+1):
        if i==1 and cancel=='step1': continue
        domain=3 if i==0 else 7 if i%2 else 1
        result += weight**2 * sum(bool(domain&(1<<a) and domain&(1<<b) and mask&(1<<(3*a+b))) for a in range(3) for b in range(3))
    return result

if __name__ == '__main__':
    rawfile=RAW/'runs.jsonl'
    if rawfile.exists() or (RAW/'manifest.json').exists():
        raise SystemExit('refusing to overwrite existing receipt')
    binaries={k:BIN/k/'release/examples/learning_ownership' for k in ['primary','diagnostic']}
    files=[Path(__file__),*binaries.values(),ROOT/'research/chr-direct-conditional/examples/learning_ownership.rs',ROOT/'research/chr-direct-conditional/tests/finite_learning_gate.rs',ROOT/'research/chr-direct-conditional/examples/support/finite_bridge.rs',*[ROOT/'research/chr-compiled/experiments'/f for f in ['finite_phase.rs','finite_learning.rs','finite_reuse.rs','meter.rs']],ROOT/'docs/experiments/registrations/S05-finite-reuse-ownership.md']
    hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in files}
    (RAW/'manifest.json').write_text(json.dumps({'configurations':configs,'replays':['primary','diagnostic','diagnostic'],'sha256':hashes},indent=2)+'\n')
    check=subprocess.run([binaries['diagnostic'],'meter-check'],capture_output=True,text=True,timeout=60,check=True,preexec_fn=limits)
    (RAW/'meter-check.log').write_text(check.stdout)
    deadline=time.monotonic()+600
    summaries=[]
    with rawfile.open('x') as out:
        for cell,config in enumerate(configs):
            rows=[]
            for repeat,kind in enumerate(['primary','diagnostic','diagnostic']):
                assert time.monotonic()<deadline,'launcher bound'
                proc=subprocess.run([binaries[kind],*map(str,config)],capture_output=True,text=True,timeout=60,check=True,preexec_fn=limits)
                assert not proc.stderr,proc.stderr
                row=json.loads(proc.stdout)
                out.write(json.dumps({'cell':cell,'repeat':repeat,'kind':kind,**row})+'\n');out.flush()
                assert row['answers']==expected(config),(config,row['answers'])
                assert row['allocation_meter']==(kind=='diagnostic')
                assert row['diagnostics']==(kind=='diagnostic')
                assert row['retained_regions']<=config[3] and row['retained_results']<=config[3]
                if kind=='diagnostic':
                    phases=row['phases'];baseline=row['baseline']
                    assert phases[0]['memory']['live_start']==baseline
                    assert phases[-1]['memory']['live_end']==baseline
                    for a,b in zip(phases,phases[1:]): assert a['memory']['live_end']==b['memory']['live_start'],(config,a,b)
                else: assert all(p['memory'] is None for p in row['phases'])
                rows.append(row)
            assert signature(rows[1])==signature(rows[2]),config
            for key in ['answers','retained_regions','retained_results','queries']:
                assert rows[0][key]==rows[1][key],(config,key)
            row=rows[1]; phases=row['phases'];baseline=row['baseline']
            summaries.append({'config':config,'answers':row['answers'],'regions':row['retained_regions'],'results':row['retained_results'],'requested':sum(p['memory']['requested_bytes'] for p in phases),'peak':max(p['memory']['peak_live'] for p in phases)-baseline,'retention_drop':next(p['memory']['live_start']-p['memory']['live_end'] for p in phases if p['name']=='learner_drop'),'phases':[{k:v for k,v in p.items() if k!='ns'} for p in phases]})
            if (cell+1)%50==0: print(f'{cell+1}/300 configurations validated',flush=True)
    for name,h in hashes.items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==h,name
    (RAW/'audit.json').write_text(json.dumps({'configurations':len(summaries),'processes':3*len(summaries),'diagnostic_pairs':len(summaries),'results':summaries,'runs_sha256':hashlib.sha256(rawfile.read_bytes()).hexdigest()},indent=2)+'\n')
    print('300 configurations, 900 processes: exact answers, deterministic allocations, full heap restoration',flush=True)
