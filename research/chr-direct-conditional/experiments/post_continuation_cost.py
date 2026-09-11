"""Registered complete-path pilot; separate ordinary, allocation and cancellation runs."""
import gzip, hashlib, itertools, json, os, random, resource, subprocess, time, zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
BASE=ROOT/'docs/experiments/results/s10-post-continuation-cost'
MODES=['birth','birth-miss','birth-miss-template','conditional','scan','indexed','active-scan','active-indexed','sealed-scan','sealed-indexed','native-scan','native-indexed','active-native-scan','active-native-indexed']
SHAPES=[('common',0,0),('common',0,16)]+[(f,3,d) for f in ['common','independent','early'] for d in [0,16]]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def invoke(cmd):
    try:
        r=subprocess.run(cmd,capture_output=True,text=True,timeout=60,preexec_fn=limits)
        return dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr,timeout=False)
    except subprocess.TimeoutExpired as e:
        dec=lambda x:x.decode() if isinstance(x,bytes) else x or ''
        return dict(command=cmd,exit_code=None,stdout=dec(e.stdout),stderr=dec(e.stderr),timeout=True)
def main():
    assert not (BASE/'freeze.json').exists()
    bins={kind:ROOT/f'target/s10-post-continuation-{kind}/release/examples/post_continuation_cost' for kind in ['time','meter']}
    for kind,command in [('time','clock-check'),('meter','meter-check')]:
        r=invoke([str(bins[kind]),command]);(BASE/f'{command}.json').write_text(json.dumps(r,indent=2));assert r['exit_code']==0
    scenarios=[(*shape,h,r,consumer) for shape,h,r,consumer in itertools.product(SHAPES,[False,True],[False,True],['immediate','window','all'])]
    rng=random.Random(781041);jobs=[]
    for stage,kind,reps,cancel in [('warmup','time',1,False),('primary','time',5,False),('allocation','meter',2,False),('cancel-time','time',1,True),('cancel-meter','meter',1,True)]:
        for rep in range(reps):
            order=scenarios.copy();rng.shuffle(order)
            for case in order:
                modes=list(itertools.product(MODES,[False,True]));rng.shuffle(modes)
                for mode,counted in modes:jobs.append(dict(stage=stage,kind=kind,rep=rep,case=case,mode=mode,counted=counted,cancel=cancel))
    assert len(jobs)==26880
    (BASE/'jobs.json').write_text(json.dumps(jobs))
    paths=[]
    for folder in ['research/chr-direct-choice','research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax']:
        paths.extend(p for p in (ROOT/folder).rglob('*.rs') if 'target' not in p.parts)
    paths.extend([ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'docs/experiments/registrations/S10-post-continuation-cost.md',Path(__file__),ROOT/'research/chr-direct-conditional/experiments/audit_post_continuation_cost.py'])
    hashes={str(p.relative_to(ROOT)):sha(p) for p in paths}
    with zipfile.ZipFile(BASE/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    freeze=dict(sources=hashes,archive_sha256=sha(BASE/'sources.zip'),jobs_sha256=sha(BASE/'jobs.json'),binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in bins.items()},toolchain=subprocess.check_output(['rustc','-Vv'],text=True))
    (BASE/'freeze.json').write_text(json.dumps(freeze,indent=2))
    start=time.monotonic();completed=0
    with gzip.open(BASE/'runs.jsonl.gz','wt') as out:
        for job in jobs:
            assert time.monotonic()-start<1200,'campaign bound'
            f,k,d,h,r,c=job['case'];args=[job['mode'],f,k,d,h,r,c,job['counted'],job['cancel']]
            cmd=[str(bins[job['kind']])]+[str(x).lower() if isinstance(x,bool) else str(x) for x in args]
            raw=invoke(cmd);out.write(json.dumps(dict(job=job,raw=raw))+'\n');out.flush();completed+=1
            if raw['exit_code']!=0:raise RuntimeError(f'failed job {completed}: {job}: {raw["stderr"]}')
            if completed%500==0:print(completed,round(time.monotonic()-start,2),flush=True)
    (BASE/'campaign.json').write_text(json.dumps(dict(completed=completed,elapsed_seconds=time.monotonic()-start),indent=2))
if __name__=='__main__':main()
