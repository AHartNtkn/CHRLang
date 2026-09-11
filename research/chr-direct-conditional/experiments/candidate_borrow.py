"""Frozen before/after candidate-copy lifecycle comparison with existing controls."""
import gzip,hashlib,itertools,json,os,random,resource,subprocess,time,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s03-candidate-borrow';PARENT=ROOT/'docs/experiments/results/s03-post-control-cost'
FAMILIES=['post-input','post-output','post-forward','post-miss','post-duplicate','post-template']
DEMAND=['birth','birth-miss','birth-miss-template']
EXPLICIT=['scan','indexed','sealed','active-scan','active-indexed','native-scan','native-indexed','active-native-scan','active-native-indexed']
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def invoke(cmd):
    try:
        r=subprocess.run(cmd,env=dict(os.environ,DEPENDENCY_FIRST_CLOCK='off'),capture_output=True,text=True,timeout=60,preexec_fn=limits)
        return dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr,timeout=False,first_clock='off')
    except subprocess.TimeoutExpired as e:
        dec=lambda x:x.decode() if isinstance(x,bytes) else x or ''
        return dict(command=cmd,exit_code=None,stdout=dec(e.stdout),stderr=dec(e.stderr),timeout=True,first_clock='off')
def main():
    assert not (BASE/'freeze.json').exists();assert 0 in os.sched_getaffinity(0)
    parent=json.loads((PARENT/'freeze.json').read_text());assert sha(PARENT/'sources.zip')==parent['archive_sha256']
    toolchain=subprocess.check_output(['rustc','-Vv'],text=True);assert toolchain==parent['toolchain']
    bins={}
    for kind in ['time','meter']:
        b=parent['binaries'][kind];assert sha(Path(b['path']))==b['sha256'];bins['old-'+kind]=b
    # Verify the before implementation and the exact relevant runner/library text.
    roots=['research/chr-direct-choice/src/','research/chr-direct-conditional/src/','research/chr-direct-conditional/examples/support/','research/chr-direct-conditional/tests/runtime_support/','research/chr-compiled/src/','research/chr-persistent/src/','research/chr-observe/src/','crates/chr-syntax/src/']
    paths=[p for p in parent['sources'] if any(p.startswith(prefix) for prefix in roots)]
    paths += ['research/chr-direct-conditional/examples/dependency_ownership.rs','research/chr-compiled/experiments/meter.rs','Cargo.toml']
    before={}
    for p in paths:
        content=subprocess.check_output(['git','show','HEAD:'+p]);digest=hashlib.sha256(content).hexdigest()
        assert digest==parent['sources'][p],p
        before[p]=digest
    for kind,feature in [('time','experiment'),('meter','alloc-meter'),('profile','candidate-profile')]:
        cmd=['cargo','build','-p','chr-direct-conditional','--example','dependency_ownership','--release','--no-default-features','--features',feature,'--target-dir',f'target/s03-candidate-borrow-{kind}','--message-format=json']
        r=subprocess.run(cmd,capture_output=True,text=True,timeout=300);(BASE/f'build-{kind}.json').write_text(json.dumps(dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr)));assert r.returncode==0,r.stderr
        artifacts=[json.loads(l) for l in r.stdout.splitlines()];p=Path(next(a['executable'] for a in artifacts if a.get('executable')));bins['new-'+kind]=dict(path=str(p),sha256=sha(p))
        for a in artifacts:
            if a.get('reason')=='compiler-artifact':
                assert not set(a['features'])&{'metrics','kernel-metrics','work-diagnostics'},a
                if kind=='time':assert not set(a['features'])&{'alloc-meter','candidate-profile'},a
    scenarios=list(itertools.product(FAMILIES,[8,32],[False,True],['immediate','window','all']))
    rng=random.Random(710733);jobs=[]
    def add(kind,rep,cells,cancel=False):
        order=list(cells);rng.shuffle(order)
        for scenario,mode in order:jobs.append(dict(kind=kind,rep=rep,scenario=scenario,mode=mode,cancel=cancel,warmup=False))
    for rep in range(2):add('new-meter',rep,[(s,m) for s in scenarios for m in DEMAND])
    add('new-meter',0,[(s,m) for s in scenarios for m in EXPLICIT])
    for rep in range(2):add('new-profile',rep,[(s,m) for s in scenarios if s[-1]=='all' for m in DEMAND])
    for kind in ['new-meter','new-time']:add(kind,0,[(s,m) for s in scenarios for m in DEMAND],cancel=True)
    modes=[('old-time',m) for m in DEMAND+EXPLICIT]+[('new-time',m) for m in DEMAND]
    for rep in [-1,0,1,2,3,4]:
        order=scenarios.copy();rng.shuffle(order)
        for s in order:
            choices=modes.copy();rng.shuffle(choices)
            for kind,mode in choices:jobs.append(dict(kind=kind,rep=rep,scenario=s,mode=mode,cancel=False,warmup=rep==-1))
    assert len(jobs)==8136;(BASE/'jobs.json').write_text(json.dumps(jobs))
    paths=subprocess.check_output(['git','ls-files','crates','research','Cargo.toml','Cargo.lock'],text=True).splitlines();paths=[p for p in paths if p.endswith(('.rs','.toml','.lock'))]+['research/chr-direct-conditional/experiments/candidate_borrow.py','research/chr-direct-conditional/experiments/audit_candidate_borrow.py','docs/experiments/registrations/S03-candidate-borrow.md']
    hashes={p:sha(ROOT/p) for p in sorted(set(paths))}
    with zipfile.ZipFile(BASE/'sources.zip','w',compression=zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    f=dict(sources=hashes,archive_sha256=sha(BASE/'sources.zip'),binaries=bins,jobs_sha256=sha(BASE/'jobs.json'),parent_freeze_sha256=sha(PARENT/'freeze.json'),parent_archive_sha256=sha(PARENT/'sources.zip'),before_source_hashes=before,before_commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),toolchain=toolchain,cpu=0)
    (BASE/'freeze.json').write_text(json.dumps(f,indent=2))
    for kind,args in [(k,['clock-check']) for k in ['old-time','new-time'] for _ in range(3)]+[(k,['meter-check']) for k in ['old-meter','new-meter','new-profile']]:
        i=len(list(BASE.glob('calibration-*.json')));raw=invoke([bins[kind]['path'],*args]);(BASE/f'calibration-{i}.json').write_text(json.dumps(dict(kind=kind,**raw)));assert raw['exit_code']==0,raw
    start=time.monotonic()
    with gzip.open(BASE/'runs.jsonl.gz','wt') as out:
        for i,j in enumerate(jobs):
            assert time.monotonic()-start<1800
            fam,n,rev,ret=j['scenario'];cmd=[bins[j['kind']]['path'],j['mode'],fam,str(n),str(rev).lower(),ret,str(j['cancel']).lower()]
            raw=invoke(cmd);out.write(json.dumps(dict(index=i,**raw))+'\n');out.flush();assert raw['exit_code']==0 and not raw['stderr'] and not raw['timeout'],(i,raw)
            if (i+1)%432==0:print(i+1,'/',len(jobs),'seconds',round(time.monotonic()-start),flush=True)
    assert all(sha(ROOT/p)==h for p,h in hashes.items());(BASE/'campaign.json').write_text(json.dumps(dict(processes=len(jobs),seconds=time.monotonic()-start)))
if __name__=='__main__':main()
