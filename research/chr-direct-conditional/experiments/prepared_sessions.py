"""Registered preparation-reuse pilot with separate timing/allocation/RSS runs."""
import sys,gzip,hashlib,itertools,json,os,random,resource,subprocess,time,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s08-prepared-sessions'
REPAIR=sys.argv[1:]==['repair']
assert not sys.argv[1:] or REPAIR
if REPAIR: BASE=ROOT/'docs/experiments/results/s08-prepared-sessions-repair'
CONFIGS=[('birth-miss',True),('conditional',False),('conditional',True),('native-scan',False),('native-scan',True),('active-native-scan',True),('sealed-scan',True)]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def invoke(cmd):
    try:
        r=subprocess.run(cmd,capture_output=True,text=True,timeout=60,preexec_fn=limits)
        return dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr)
    except subprocess.TimeoutExpired as e:return dict(command=cmd,exit_code=None,stdout=str(e.stdout),stderr=str(e.stderr))
def main():
    assert not (BASE/'freeze.json').exists()
    bins={k:ROOT/f'target/s08-sessions{"-repair" if REPAIR else ""}-{k}/release/examples/prepared_sessions' for k in ['time','meter']}
    for kind,command in [('time','clock-check'),('meter','meter-check')]:
        r=invoke([str(bins[kind]),command]);(BASE/f'{command}.json').write_text(json.dumps(r,indent=2));assert r['exit_code']==0
    cases=list(itertools.product(['common','independent'],[1,16,128],['immediate','window','all'],[False,True],CONFIGS));assert len(cases)==252
    jobs=[];rng=random.Random(740913)
    for stage,kind,reps,cancel,rss in [('warmup','time',1,False,False),('primary','time',5,False,False),('allocation','meter',2,False,False),('rss','time',2,False,True),('cancel-time','time',1,True,False),('cancel-meter','meter',1,True,False)]:
        for rep in range(reps):
            order=cases.copy();rng.shuffle(order)
            jobs.extend(dict(stage=stage,kind=kind,rep=rep,case=case,cancel=cancel,rss=rss) for case in order)
    assert len(jobs)==3024
    paths=[]
    for folder in ['research/chr-direct-choice','research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax']:
        paths.extend(p for p in (ROOT/folder).rglob('*.rs') if 'target' not in p.parts);paths.append(ROOT/folder/'Cargo.toml')
    paths.extend([ROOT/'Cargo.toml',ROOT/'Cargo.lock',Path(__file__),ROOT/('docs/experiments/registrations/S08-prepared-sessions-repair.md' if REPAIR else 'docs/experiments/registrations/S08-prepared-sessions.md')])
    hashes={str(p.relative_to(ROOT)):sha(p) for p in paths}
    with zipfile.ZipFile(BASE/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    (BASE/'jobs.json').write_text(json.dumps(jobs))
    (BASE/'freeze.json').write_text(json.dumps(dict(sources=hashes,archive_sha256=sha(BASE/'sources.zip'),jobs_sha256=sha(BASE/'jobs.json'),binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in bins.items()},toolchain=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2))
    start=time.monotonic();n=0
    with gzip.open(BASE/'runs.jsonl.gz','wt') as out:
        for j in jobs:
            assert time.monotonic()-start<1200
            f,q,c,reuse,(mode,initialize)=j['case'];args=[mode,f,3,4,True,False,c,True,j['cancel'],initialize,q,reuse,j['rss']]
            raw=invoke([str(bins[j['kind']])]+[str(x).lower() for x in args]);out.write(json.dumps(dict(job=j,raw=raw))+'\n');out.flush();n+=1
            assert raw['exit_code']==0,(j,raw)
            if n%252==0:print(n,round(time.monotonic()-start,2),flush=True)
    (BASE/'campaign.json').write_text(json.dumps(dict(processes=n,seconds=time.monotonic()-start),indent=2))
if __name__=='__main__':main()
