"""Register/freeze new post controls, bridge old allocation, and run paired costs."""
import hashlib,itertools,json,os,random,resource,subprocess,time,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s03-post-control-cost';PARENT=ROOT/'docs/experiments/results/s03-post-ownership'
FAMILIES=['post-input','post-output','post-forward','post-miss','post-duplicate','post-template'];OLD=['birth','birth-miss','birth-miss-template','scan','indexed','sealed'];NEW=['active-scan','active-indexed','native-scan','native-indexed','active-native-scan','active-native-indexed'];MODES=OLD+NEW
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def norm(x,base):
    if isinstance(x,list):return [norm(v,base) for v in x]
    if isinstance(x,dict):return {k:(v-base if k in ['live_start','live_end','peak_live'] else norm(v,base)) for k,v in x.items() if k!='ns'}
    return x
def result(raw):return next(json.loads(l) for l in raw['stdout'].splitlines() if l.startswith('{') and json.loads(l).get('event')=='result')
def limits():
    os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def main():
    assert not (BASE/'freeze.json').exists();assert 0 in os.sched_getaffinity(0)
    binaries={}
    for kind,feature in [('time','experiment'),('meter','alloc-meter')]:
        cmd=['cargo','build','-p','chr-direct-conditional','--example','dependency_ownership','--release','--no-default-features','--features',feature,'--target-dir',f'target/s03-post-control-cost-{kind}','--message-format=json']
        r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=300);(BASE/f'build-{kind}.json').write_text(json.dumps(dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr))+'\n');assert r.returncode==0
        path=Path(next(json.loads(l)['executable'] for l in r.stdout.splitlines() if json.loads(l).get('executable')));binaries[kind]=dict(path=str(path),sha256=sha(path))
    scenarios=list(itertools.product(FAMILIES,[8,32],[False,True],['immediate','window','all']));cells=[[*s,m] for s in scenarios for m in MODES];assert len(cells)==864
    meter=[dict(index=i,rep=r) for i,c in enumerate(cells) for r in range(1 if c[-1] in OLD else 2)];random.Random(71320).shuffle(meter)
    blocks=list(itertools.product(range(5),range(72)));rng=random.Random(71321);rng.shuffle(blocks);timing=[]
    for block,(rep,index) in enumerate(blocks):
        modes=MODES.copy();rng.shuffle(modes)
        for pos,mode in enumerate(modes):timing.append(dict(block=block,rep=rep,index=cells.index([*scenarios[index],mode]),position=pos))
    warmup=list(range(864));random.Random(71322).shuffle(warmup)
    for name,data in [('cells',cells),('meter-order',meter),('time-order',timing),('warmup-order',warmup)]: (BASE/f'{name}.json').write_text(json.dumps(data)+'\n')
    paths=subprocess.check_output(['git','ls-files','crates','research','Cargo.toml','Cargo.lock'],text=True).splitlines();paths=[p for p in paths if p.endswith(('.rs','.toml','.lock'))]+['research/chr-compiled/tests/post_controls.rs','research/chr-direct-conditional/experiments/post_control_cost.py','research/chr-direct-conditional/experiments/audit_post_control_cost.py','docs/experiments/registrations/S03-post-control-cost.md']
    hashes={p:sha(ROOT/p) for p in sorted(set(paths))}
    with zipfile.ZipFile(BASE/'sources.zip','w',compression=zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    f=dict(sources=hashes,archive_sha256=sha(BASE/'sources.zip'),binaries=binaries,orders={n:sha(BASE/f'{n}.json') for n in ['cells','meter-order','time-order','warmup-order']},parent_freeze_sha256=sha(PARENT/'freeze.json'),parent_archive_sha256=sha(PARENT/'sources.zip'),cpu=0,affinity=sorted(os.sched_getaffinity(0)),toolchain=subprocess.check_output(['rustc','-Vv'],text=True),platform=os.uname().release,commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip())
    (BASE/'freeze.json').write_text(json.dumps(f,indent=2)+'\n');pc={tuple(c):i for i,c in enumerate(json.loads((PARENT/'cells.json').read_text()))}
    def invoke(kind,args,path):
        cmd=[binaries[kind]['path'],*args]
        try:
            r=subprocess.run(cmd,cwd=ROOT,env=dict(os.environ,DEPENDENCY_FIRST_CLOCK='off'),capture_output=True,text=True,timeout=60,preexec_fn=limits);raw=dict(command=cmd,first_clock='off',exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr,timeout=False)
        except subprocess.TimeoutExpired as e:
            dec=lambda x:x.decode() if isinstance(x,bytes) else x or ''
            raw=dict(command=cmd,first_clock='off',exit_code=None,stdout=dec(e.stdout),stderr=dec(e.stderr),timeout=True)
        path.write_text(json.dumps(raw)+'\n');assert raw['exit_code']==0 and not raw['stderr'] and not raw['timeout'],path;return raw
    for i in range(5):invoke('time',['clock-check'],BASE/f'clock-{i}.json')
    for i,(fam,mode) in enumerate(itertools.product(FAMILIES,NEW)):invoke('meter',[mode,fam,'32','true','window','true'],BASE/f'cancel-{i}.json')
    def args(index):
        fam,size,rev,c,m=cells[index];return [m,fam,str(size),str(rev).lower(),c,'false']
    start=time.monotonic()
    for kind,order,bkind in [('meter',meter,'meter'),('warmup',warmup,'time'),('time',timing,'time')]:
        (BASE/kind).mkdir()
        for i,j in enumerate(order):
            assert time.monotonic()-start<1800;index=j if kind=='warmup' else j['index'];raw=invoke(bkind,args(index),BASE/kind/f'{i}.json')
            fam,size,rev,c,m=cells[index]
            if kind=='meter' and m in OLD:
                prior=result(json.loads((PARENT/'runs'/f'{pc[fam,size,rev,m,c,False]}-meter-0.json').read_text()));now=result(raw)
                assert norm(now,now['phases'][0]['reading']['memory']['live_start'])==norm(prior,prior['phases'][0]['reading']['memory']['live_start']),(index,'allocation bridge mismatch')
            if (i+1)%432==0:print(kind,i+1,'/',len(order),flush=True)
    assert all(sha(ROOT/p)==h for p,h in hashes.items())
    (BASE/'campaign.json').write_text(json.dumps(dict(allocation_bridges=432,new_allocation_processes=864,primary_processes=4320,warmups=864,cancellation_gates=36,seconds=time.monotonic()-start))+'\n')
if __name__=='__main__':main()
