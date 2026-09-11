"""Registered paired ownership and randomized counter-free lifecycle sizing."""
import hashlib,itertools,json,os,random,resource,subprocess,time,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s01-intermediate-lifecycle'
MODES=['local-scan','tuples','partial','intermediate','scan','indexed','special-scan','special-indexed']
FAMILIES=['proper','proper-kill','proper-late','proper-keyed','cold','dense']
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def main():
    assert not (BASE/'freeze.json').exists()
    binaries={}
    for kind,feature in [('meter','alloc-meter'),('time',None)]:
        cmd=['cargo','test','-p','chr-relational','--test','multihead_lifecycle','--release','--no-default-features','--target-dir',f'target/s01-intermediate-lifecycle-{kind}','--no-run','--message-format=json']
        if feature:cmd+=['--features',feature]
        r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=300)
        (BASE/f'build-{kind}.json').write_text(json.dumps(dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr))+'\n');assert r.returncode==0
        binary=Path(next(json.loads(x)['executable'] for x in r.stdout.splitlines() if json.loads(x).get('executable')));binaries[kind]=dict(path=str(binary),sha256=sha(binary))
    cells=list(itertools.product(FAMILIES,[4,8],[1,4],['immediate','all'],MODES));assert len(cells)==384
    meter=list(itertools.product(range(2),range(len(cells))));random.Random(81010).shuffle(meter)
    blocks=list(itertools.product(range(5),FAMILIES,[4,8],[1,4],['immediate','all']));rng=random.Random(81011);rng.shuffle(blocks);timing=[]
    for block,(rep,family,width,reuse,consumer) in enumerate(blocks):
        modes=MODES.copy();rng.shuffle(modes)
        for position,mode in enumerate(modes):timing.append(dict(block=block,position=position,rep=rep,index=cells.index((family,width,reuse,consumer,mode))))
    for name,data in [('cells',cells),('meter-order',meter),('time-order',timing)]: (BASE/f'{name}.json').write_text(json.dumps(data,indent=2)+'\n')
    paths=subprocess.check_output(['git','ls-files','crates','research','Cargo.toml','Cargo.lock'],cwd=ROOT,text=True).splitlines();paths=[p for p in paths if p.endswith(('.rs','.toml','.lock'))]+[str(Path(__file__).relative_to(ROOT)),'docs/experiments/registrations/S01-intermediate-lifecycle.md']
    hashes={p:sha(ROOT/p) for p in sorted(set(paths))}
    with zipfile.ZipFile(BASE/'sources.zip','w',compression=zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    (BASE/'freeze.json').write_text(json.dumps(dict(sources=hashes,archive_sha256=sha(BASE/'sources.zip'),binaries=binaries,orders={p:sha(BASE/(p+'.json')) for p in ['cells','meter-order','time-order']},cpu=0,affinity=sorted(os.sched_getaffinity(0)),commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()),indent=2)+'\n')
    def invoke(kind,args,path):
        command=[binaries[kind]['path'],*map(str,args)]
        try:
            r=subprocess.run(command,cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=limits)
            raw=dict(command=command,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr,timeout=False)
        except subprocess.TimeoutExpired as e:
            decode=lambda x:x.decode() if isinstance(x,bytes) else (x or '')
            raw=dict(command=command,exit_code=None,stdout=decode(e.stdout),stderr=decode(e.stderr),timeout=True)
        path.write_text(json.dumps(raw)+'\n');assert raw['exit_code']==0 and not raw['timeout'] and not raw['stderr'],path
    for kind in binaries:
        invoke(kind,[],BASE/f'smoke-{kind}.json');invoke(kind,['clock-check'],BASE/f'clock-{kind}.json')
    start=time.monotonic()
    for kind,jobs in [('meter',meter),('time',timing)]:
        (BASE/kind).mkdir()
        for i,job in enumerate(jobs):
            assert time.monotonic()-start<1800
            index=job[1] if kind=='meter' else job['index'];family,width,reuse,consumer,mode=cells[index]
            invoke(kind,[mode,family,width,reuse,consumer],BASE/kind/f'{i}.json')
            if (i+1)%192==0:print(kind,i+1,'/',len(jobs),flush=True)
    assert all(sha(ROOT/p)==h for p,h in hashes.items())
    (BASE/'campaign.json').write_text(json.dumps(dict(configurations=384,processes=2688,seconds=time.monotonic()-start))+'\n')
if __name__=='__main__':main()
