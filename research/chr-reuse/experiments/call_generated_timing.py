"""Paired ordinary timing using already-qualified, frozen call artifacts."""
import json,random,statistics,sys,shutil
from pathlib import Path
sys.dont_write_bytecode=True
import call_compiled_cost as old
import call_generated_entry as gen
import call_trace_ownership as own
ROOT=own.ROOT
RAW=ROOT/'docs/experiments/results/s05-call-generated-timing'
MODES=old.MODES+['generated','generated-sealed']
CELLS=[(m,*c[1:])for m in MODES for c in old.CELLS if c[0]=='trace']
PAIRS=[('trace',m)for m in MODES[1:]]+[('generated','indexed'),('generated-sealed','sealed')]
def binary(mode):
    return ROOT/'target/s05-call-generated-source-check-primary/release/call-generated-artifact' if mode.startswith('generated')else old.binary('primary')
def audit():
    freeze=json.loads((RAW/'freeze.json').read_text())
    for p,h in freeze['binaries'].items():assert own.sha(ROOT/p)==h
    for p,h in freeze['sources'].items():assert own.sha(RAW/'registered'/p)==h
    values={};obs={}
    schedule=json.loads((RAW/'schedule.json').read_text());assert len(schedule)==10080
    for k,i in schedule:
        c=CELLS[i];r=json.loads((RAW/f'{k:02}-{i:04}.json').read_text())
        assert r['command']==[str(binary(c[0])),*map(str,c)]
        h,mem=own.parse(r);assert mem is None
        o=(h['counts'],h['retained'])
        if c[1:]in obs:assert obs[c[1:]]==o
        obs[c[1:]]=o
        rows=[json.loads(x)for x in r['stdout'].splitlines()][1:]
        assert all(isinstance(x['ns'],int)for x in rows)
        assert (c,k)not in values;values[c,k]=sum(x['ns']for x in rows)
    assert len(values)==10080
    clocks=[json.loads(json.loads((RAW/f'clock-{i}.json').read_text())['stdout'])for i in range(3)]
    floor=100*max(x['p99_ns']for x in clocks)
    memory={}
    for x in json.loads((old.RAW/'analysis.json').read_text())['comparisons']:
        memory['trace',*x['scenario']]=x['trace_memory'];memory[x['control'],*x['scenario']]=x['control_memory']
    for x in json.loads((ROOT/'docs/experiments/results/s05-call-generated-source-check/analysis.json').read_text()):memory[tuple(x['cell'])]=x['memory']
    out=[]
    for c in CELLS:
        if c[0]!='trace':continue
        for a,b in PAIRS:
            left=(a,*c[1:]);right=(b,*c[1:]);rs=[values[left,k]/values[right,k]for k in range(10)];med=statistics.median(rs)
            times=[statistics.median(values[x,k]for k in range(10))for x in [left,right]];qualified=min(times)>=floor
            verdict='gain'if qualified and med<=.9 and max(rs)<1 else'loss'if qualified and med>=1.1 and min(rs)>1 else'uncertain'
            out.append(dict(scenario=list(c[1:]),candidate=a,control=b,median=med,min=min(rs),max=max(rs),clock_qualified=qualified,verdict=verdict,candidate_ns=times[0],control_ns=times[1],candidate_memory=memory[left],control_memory=memory[right]))
    return dict(floor_ns=floor,comparisons=out)
if __name__=='__main__':
    if sys.argv[1]=='run':
        old.audit();gen.RAW=ROOT/'docs/experiments/results/s05-call-generated-source-check';gen.TARGET='s05-call-generated-source-check';gen.audit()
        RAW.mkdir(exist_ok=False)
        previous=json.loads((ROOT/'docs/experiments/results/s05-recognition-stride-timing/freeze.json').read_text());clock=ROOT/previous['binary'];assert own.sha(clock)==previous['binary_hash']
        for i in range(3):old.invoke([str(clock),'clock-check'],RAW/f'clock-{i}.json')
        paths=[Path(__file__),Path(old.__file__),Path(own.__file__),ROOT/'docs/experiments/registrations/S05-call-generated-timing.md']
        for p in paths:
            dst=RAW/'registered'/p.relative_to(ROOT);dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,dst)
        bins={binary(m)for m in MODES}|{clock}
        (RAW/'freeze.json').write_text(json.dumps(dict(binaries={str(p.relative_to(ROOT)):own.sha(p)for p in bins},sources={str(p.relative_to(ROOT)):own.sha(p)for p in paths}),indent=2)+'\n')
        rng=random.Random(607097);schedule=[]
        for k in range(10):
            order=list(range(1008));rng.shuffle(order);schedule.extend((k,i)for i in order)
        (RAW/'schedule.json').write_text(json.dumps(schedule)+'\n')
        for number,(k,i)in enumerate(schedule):
            c=CELLS[i];old.invoke([str(binary(c[0])),*map(str,c)],RAW/f'{k:02}-{i:04}.json')
            if number%1008==1007:print(number+1,'/10080 processes',flush=True)
        (RAW/'analysis.json').write_text(json.dumps(audit(),indent=2)+'\n')
    else:
        assert audit()==json.loads((RAW/'analysis.json').read_text());print('10080 processes, 1152 comparisons, output agreement and frozen inputs verified.')
