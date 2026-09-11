"""Frozen ordinary timing pilot over existing post ownership controls."""
import hashlib,itertools,json,os,random,resource,subprocess,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s03-post-timing';PARENT=ROOT/'docs/experiments/results/s03-post-ownership'
FAMILIES=['post-input','post-output','post-forward','post-miss','post-duplicate','post-template'];MODES=['birth','birth-miss','birth-miss-template','scan','indexed','sealed']
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def main():
    assert not (BASE/'freeze.json').exists();parent=json.loads((PARENT/'freeze.json').read_text());binary=parent['binaries']['ordinary'];assert sha(Path(binary['path']))==binary['sha256'];assert sha(PARENT/'sources.zip')==parent['source_archive_sha256'];assert 0 in os.sched_getaffinity(0)
    scenarios=list(itertools.product(FAMILIES,[8,32],[False,True],['immediate','window','all']));cells=[(*s,m) for s in scenarios for m in MODES];assert len(cells)==432
    blocks=list(itertools.product(range(5),range(len(scenarios))));rng=random.Random(71310);rng.shuffle(blocks);jobs=[]
    for block,(rep,index) in enumerate(blocks):
        modes=MODES.copy();rng.shuffle(modes)
        for pos,mode in enumerate(modes):jobs.append(dict(block=block,rep=rep,index=cells.index((*scenarios[index],mode)),position=pos))
    warmup=list(range(432));random.Random(71311).shuffle(warmup)
    for name,data in [('cells',cells),('jobs',jobs),('warmup-order',warmup)]: (BASE/f'{name}.json').write_text(json.dumps(data)+'\n')
    paths=['docs/experiments/registrations/S03-post-timing.md','research/chr-direct-conditional/experiments/post_timing.py','research/chr-direct-conditional/experiments/audit_post_timing.py']
    freeze=dict(binary=binary,parent_freeze_sha256=sha(PARENT/'freeze.json'),parent_archive_sha256=sha(PARENT/'sources.zip'),parent_build_sha256=sha(PARENT/'build-ordinary.jsonl'),sources={p:sha(ROOT/p) for p in paths},source_text={p:(ROOT/p).read_text() for p in paths},orders={n:sha(BASE/f'{n}.json') for n in ['cells','jobs','warmup-order']},cpu=0,affinity=sorted(os.sched_getaffinity(0)),platform=os.uname().release,commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip())
    (BASE/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    def invoke(args,path):
        cmd=[binary['path'],*args]
        try:
            r=subprocess.run(cmd,cwd=ROOT,env=dict(os.environ,DEPENDENCY_FIRST_CLOCK='off'),capture_output=True,text=True,timeout=60,preexec_fn=limits);raw=dict(command=cmd,first_clock='off',exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr,timeout=False)
        except subprocess.TimeoutExpired as e:
            dec=lambda x:x.decode() if isinstance(x,bytes) else x or ''
            raw=dict(command=cmd,first_clock='off',exit_code=None,stdout=dec(e.stdout),stderr=dec(e.stderr),timeout=True)
        path.write_text(json.dumps(raw)+'\n');assert raw['exit_code']==0 and not raw['stderr'] and not raw['timeout'],path
    for rep in range(5):invoke(['clock-check'],BASE/f'clock-{rep}.json')
    for i,(fam,mode) in enumerate(itertools.product(FAMILIES,MODES)):invoke([mode,fam,'8','false','window','true'],BASE/f'cancel-{i}.json')
    def args(index):
        fam,size,reverse,consumer,mode=cells[index];return [mode,fam,str(size),str(reverse).lower(),consumer,'false']
    start=time.monotonic()
    for kind,order in [('warmup',warmup),('runs',jobs)]:
        (BASE/kind).mkdir()
        for i,j in enumerate(order):
            assert time.monotonic()-start<1800;invoke(args(j if kind=='warmup' else j['index']),BASE/kind/f'{i}.json')
            if (i+1)%216==0:print(kind,i+1,'/',len(order),flush=True)
    assert all(sha(ROOT/p)==h for p,h in freeze['sources'].items());assert sha(Path(binary['path']))==binary['sha256']
    (BASE/'campaign.json').write_text(json.dumps(dict(primary_processes=2160,warmups=432,cancellation_gates=36,seconds=time.monotonic()-start))+'\n')
if __name__=='__main__':main()
