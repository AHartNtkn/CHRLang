"""Counter-free complete timing after exact call ownership qualification."""
import json,random,statistics,subprocess,sys,shutil
from pathlib import Path
sys.dont_write_bytecode=True
import call_trace_ownership as own
ROOT=own.ROOT
RAW=ROOT/'docs/experiments/results/s05-call-trace-timing'
def invoke(cmd,path):
    assert not path.exists()
    try:
        p=subprocess.run(cmd,capture_output=True,text=True,timeout=45,preexec_fn=own.bounds);r=dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
    except subprocess.TimeoutExpired as e:
        r=dict(command=cmd,timeout=True,stdout=(e.stdout or b'').decode(),stderr=(e.stderr or b'').decode())
    path.write_text(json.dumps(r,indent=2)+'\n');assert r.get('returncode')==0 and not r['stderr'];return r

def audit():
    freeze=json.loads((RAW/'freeze.json').read_text());assert own.sha(own.binary('primary'))==freeze['binary_hash']
    for p,h in freeze['sources'].items():assert own.sha(RAW/'registered'/p)==h,p
    floor=100*max(x['p99_ns']for x in freeze['clocks']);values={};phases={}
    schedule=json.loads((RAW/'schedule.json').read_text());assert len(schedule)==5760
    for k,i in schedule:
        r=json.loads((RAW/f'{k:02}-{i:03}.json').read_text());assert r['command']==[str(own.binary('primary')),*map(str,own.cells()[i])]
        h,mem=own.parse(r);assert mem is None
        rows=[json.loads(x)for x in r['stdout'].splitlines()][1:];assert all(isinstance(x['ns'],int)for x in rows)
        c=tuple(own.cells()[i]);assert (c,k) not in values;values[c,k]=sum(x['ns']for x in rows);phases[c,k]=rows
    assert len(values)==5760
    out=[]
    for c in own.cells():
        if c[0]!='trace':continue
        for mode in ['direct','memo','memo16']:
            other=(mode,*c[1:]);ratios=[values[c,k]/values[other,k]for k in range(10)];m=statistics.median(ratios)
            qualified=min(statistics.median(values[x,k]for k in range(10))for x in [c,other])>=floor
            verdict='gain' if qualified and m<=.9 and max(ratios)<1 else 'loss' if qualified and m>=1.1 and min(ratios)>1 else 'unresolved'
            # Post-measurement diagnostic bound, not an additional timing verdict.
            free_disposal=statistics.median((values[c,k]-sum(r['ns']for r in phases[c,k]if r['phase'].endswith('_dispose')))/values[other,k]for k in range(10))
            out.append(dict(scenario=list(c[1:]),control=mode,median=m,min=min(ratios),max=max(ratios),clock_qualified=qualified,verdict=verdict,trace_ns=statistics.median(values[c,k]for k in range(10)),control_ns=statistics.median(values[other,k]for k in range(10)),free_trace_disposal_median=free_disposal))
    return dict(floor_ns=floor,comparisons=out)
if __name__=='__main__':
    if sys.argv[1]=='run':
        own.audit();RAW.mkdir(exist_ok=False)
        previous=json.loads((ROOT/'docs/experiments/results/s05-recognition-stride-timing/freeze.json').read_text())
        clock=ROOT/previous['binary'];assert own.sha(clock)==previous['binary_hash']
        clocks=[json.loads(invoke([str(clock),'clock-check'],RAW/f'clock-{i}.json')['stdout'])for i in range(3)]
        paths=[Path(__file__),Path(own.__file__),ROOT/'docs/experiments/registrations/S05-call-trace-timing.md']
        for p in paths:
            dst=RAW/'registered'/p.relative_to(ROOT);dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,dst)
        (RAW/'freeze.json').write_text(json.dumps(dict(binary_hash=own.sha(own.binary('primary')),clocks=clocks,sources={str(p.relative_to(ROOT)):own.sha(p)for p in paths}),indent=2)+'\n')
        rng=random.Random(607095);schedule=[]
        for k in range(10):
            order=list(range(576));rng.shuffle(order);schedule.extend((k,i)for i in order)
        (RAW/'schedule.json').write_text(json.dumps(schedule)+'\n')
        for number,(k,i)in enumerate(schedule):
            invoke([str(own.binary('primary')),*map(str,own.cells()[i])],RAW/f'{k:02}-{i:03}.json')
            if number%576==575:print(number+1,'/ 5760 terminal processes',flush=True)
        (RAW/'analysis.json').write_text(json.dumps(audit(),indent=2)+'\n')
    else:
        assert audit()==json.loads((RAW/'analysis.json').read_text());print('5760 terminal sessions, 432 comparisons, phase sums and frozen inputs verified.')
