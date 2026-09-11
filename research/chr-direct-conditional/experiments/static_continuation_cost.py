"""Registered matched static-initialization comparison."""
import gzip,hashlib,itertools,json,os,random,resource,subprocess,time,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s10-static-control';PARENT=ROOT/'docs/experiments/results/s10-post-continuation-cost'
MODES=['birth','birth-miss','birth-miss-template','conditional','scan','indexed','active-scan','active-indexed','sealed-scan','sealed-indexed','native-scan','native-indexed','active-native-scan','active-native-indexed']
SHAPES=[('common',0,0),('common',0,16)]+[(f,3,d) for f in ['common','independent','early'] for d in [0,16]]
CONFIGS=[(m,init) for m in MODES for init in ([True] if m.startswith('birth') else [False,True])]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def invoke(cmd):
    try:
        r=subprocess.run(cmd,capture_output=True,text=True,timeout=60,preexec_fn=limits);return dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr)
    except subprocess.TimeoutExpired as e:return dict(command=cmd,exit_code=None,stdout=str(e.stdout),stderr=str(e.stderr))
def main():
    assert not (BASE/'freeze.json').exists();bins={k:ROOT/f'target/s10-static-{k}/release/examples/static_continuation_cost' for k in ['time','meter']}
    for k,c in [('time','clock-check'),('meter','meter-check')]:
        r=invoke([str(bins[k]),c]);(BASE/f'{c}.json').write_text(json.dumps(r,indent=2));assert r['exit_code']==0
    all_cases=[(*shape,h,r,c) for shape,h,r,c in itertools.product(SHAPES,[False,True],[False,True],['immediate','window','all'])]
    primary=[(f,k,d,True,False,'all') for f,k,d in [('common',0,0),('common',3,16),('independent',3,16),('early',3,16)]]
    jobs=[];rng=random.Random(781043)
    for stage,kind,reps,cases,cancel in [('warmup','time',1,primary,False),('primary','time',64,primary,False),('allocation','meter',2,all_cases,False),('cancel-time','time',1,all_cases,True),('cancel-meter','meter',1,all_cases,True)]:
        for rep in range(reps):
            order=list(cases);rng.shuffle(order)
            for case in order:
                configs=CONFIGS.copy();rng.shuffle(configs)
                for mode,init in configs:jobs.append(dict(stage=stage,kind=kind,rep=rep,case=case,mode=mode,initialize=init,cancel=cancel))
    assert len(jobs)==16100;(BASE/'jobs.json').write_text(json.dumps(jobs))
    paths=[]
    for folder in ['research/chr-direct-choice','research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax']:
        paths.extend(p for p in (ROOT/folder).rglob('*.rs') if 'target' not in p.parts);paths.append(ROOT/folder/'Cargo.toml')
    paths.extend([ROOT/'Cargo.toml',ROOT/'Cargo.lock',Path(__file__),ROOT/'research/chr-direct-conditional/experiments/audit_static_continuation_cost.py',ROOT/'docs/experiments/registrations/S10-static-control.md'])
    hashes={str(p.relative_to(ROOT)):sha(p) for p in paths}
    with zipfile.ZipFile(BASE/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    freeze=dict(sources=hashes,archive_sha256=sha(BASE/'sources.zip'),jobs_sha256=sha(BASE/'jobs.json'),binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in bins.items()},parent_sha256=sha(PARENT/'freeze.json'),parent_runs_sha256=sha(PARENT/'runs.jsonl.gz'),toolchain=subprocess.check_output(['rustc','-Vv'],text=True))
    (BASE/'freeze.json').write_text(json.dumps(freeze,indent=2));start=time.monotonic();n=0
    with gzip.open(BASE/'runs.jsonl.gz','wt') as out:
        for j in jobs:
            assert time.monotonic()-start<1200;f,k,d,h,r,c=j['case'];args=[j['mode'],f,k,d,h,r,c,True,j['cancel'],j['initialize']]
            raw=invoke([str(bins[j['kind']])]+[str(x).lower() if isinstance(x,bool) else str(x) for x in args]);out.write(json.dumps(dict(job=j,raw=raw))+'\n');out.flush();n+=1;assert raw['exit_code']==0,(j,raw)
            if n%500==0:print(n,round(time.monotonic()-start,2),flush=True)
    (BASE/'campaign.json').write_text(json.dumps(dict(processes=n,seconds=time.monotonic()-start),indent=2))
if __name__=='__main__':main()
