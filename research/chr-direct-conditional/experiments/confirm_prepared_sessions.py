"""Registered confirmation of the repaired sustained finite-session comparisons."""
import gzip,hashlib,itertools,json,random,time
from pathlib import Path
from prepared_sessions import ROOT,invoke
BASE=ROOT/'docs/experiments/results/s08-prepared-sessions-confirm'
CONFIGS=[('birth-miss',True),('native-scan',False),('native-scan',True)]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    assert not (BASE/'freeze.json').exists();BASE.mkdir(exist_ok=True)
    binary=ROOT/'target/s08-sessions-repair-time/release/examples/prepared_sessions'
    cases=list(itertools.product(['common','independent'],['immediate','all'],[False,True],CONFIGS));assert len(cases)==24
    jobs=[];rng=random.Random(740914)
    for rep in range(-1,64):
        order=cases.copy();rng.shuffle(order);jobs.extend(dict(rep=rep,case=c) for c in order)
    paths=[Path(__file__),ROOT/'docs/experiments/registrations/S08-prepared-sessions-confirm.md',ROOT/'docs/experiments/results/s08-prepared-sessions-repair/freeze.json',ROOT/'docs/experiments/results/s08-prepared-sessions-repair/audit.json',ROOT/'research/chr-direct-conditional/experiments/prepared_sessions.py']
    (BASE/'freeze.json').write_text(json.dumps(dict(binary=dict(path=str(binary),sha256=sha(binary)),sources={str(p.relative_to(ROOT)):sha(p) for p in paths},jobs=jobs),indent=2))
    start=time.monotonic()
    with gzip.open(BASE/'runs.jsonl.gz','wt') as out:
        for i,j in enumerate(jobs):
            assert time.monotonic()-start<1200
            family,consumer,reuse,(mode,init)=j['case'];args=[mode,family,3,4,True,False,consumer,True,False,init,128,reuse,False]
            r=invoke([str(binary)]+[str(x).lower() for x in args]);out.write(json.dumps(dict(job=j,raw=r))+'\n');out.flush();assert r['exit_code']==0,(j,r)
            if (i+1)%240==0:print(i+1,round(time.monotonic()-start,2),flush=True)
    (BASE/'campaign.json').write_text(json.dumps(dict(processes=len(jobs),seconds=time.monotonic()-start),indent=2))
if __name__=='__main__':main()
