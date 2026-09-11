"""Registered paired full-lifecycle pilot; counters are off in primary runs."""
import gzip,hashlib,itertools,json,random,time
from pathlib import Path
from continuing_lifecycle_entry import ROOT,BASE,invoke
OUT=ROOT/'docs/experiments/results/s08-binding-coverage-cost'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    assert not (OUT/'freeze.json').exists();OUT.mkdir(exist_ok=True)
    parents={False:json.loads((BASE/'freeze.json').read_text()),True:json.loads((BASE/'coverage-freeze.json').read_text())}
    cases=list(itertools.product([False,True],[32,128],['0','all'],[False,True],[False,True]));assert len(cases)==32
    jobs=[];rng=random.Random(740916)
    for stage,kind,reps,rss in [('warmup','time',1,False),('primary','time',5,False),('allocation','meter',2,False),('rss','time',2,True)]:
        for rep in range(reps):
            order=cases.copy();rng.shuffle(order);jobs.extend(dict(stage=stage,kind=kind,rep=rep,rss=rss,case=c) for c in order)
    assert len(jobs)==320
    for p in parents.values():
        for kind in ['time','meter']:assert sha(Path(p['binaries'][kind]['path']))==p['binaries'][kind]['sha256']
    (OUT/'freeze.json').write_text(json.dumps(dict(jobs=jobs,parent_sha256={name:sha(BASE/name) for name in ['freeze.json','coverage-freeze.json']},runner_sha256=sha(Path(__file__)),registration_sha256=sha(ROOT/'docs/experiments/registrations/S08-binding-coverage-cost.md')),indent=2))
    start=time.monotonic()
    with gzip.open(OUT/'runs.jsonl.gz','wt') as out:
        for i,j in enumerate(jobs):
            assert time.monotonic()-start<1200
            resource,n,keep,packing,coverage=j['case'];binary=parents[coverage]['binaries'][j['kind']]['path']
            raw=invoke([binary]+[str(x).lower() for x in ['conditional',resource,n,keep,4,packing,j['rss']]],60)
            out.write(json.dumps(dict(job=j,raw=raw))+'\n');out.flush();assert raw['exit_code']==0,(j,raw['stderr'])
            if (i+1)%32==0:print(i+1,round(time.monotonic()-start,2),flush=True)
    (OUT/'campaign.json').write_text(json.dumps(dict(processes=len(jobs),seconds=time.monotonic()-start),indent=2))
if __name__=='__main__':main()
