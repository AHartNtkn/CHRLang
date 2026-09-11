"""Registered allocation/work qualification for nonground source posts."""
import hashlib,itertools,json,os,re,resource,subprocess,time,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s03-post-ownership'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0})
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def main():
    assert not (BASE/'freeze.json').exists();assert 0 in os.sched_getaffinity(0)
    binaries={}
    for kind,feature in [('ordinary','experiment'),('meter','alloc-meter')]:
        cmd=['cargo','build','-p','chr-direct-conditional','--example','dependency_ownership','--release','--no-default-features','--features',feature,'--target-dir',f'target/s03-post-ownership-{kind}','--message-format=json']
        r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=300)
        (BASE/f'build-{kind}.jsonl').write_text(r.stdout);(BASE/f'build-{kind}.log').write_text(r.stderr)
        assert r.returncode==0,r.stderr
        binary=Path(next(json.loads(x)['executable'] for x in r.stdout.splitlines() if json.loads(x).get('executable')))
        binaries[kind]={'path':str(binary),'sha256':sha(binary)}
    work=Path(re.search(r'\((target/[^\s()]+)\)',(BASE/'work-0.log').read_text()).group(1))
    binaries['work']={'path':str(ROOT/work),'sha256':sha(work)}
    families=['post-input','post-output','post-forward','post-miss','post-duplicate','post-template']
    modes=['birth','birth-miss','birth-miss-template','scan','indexed','sealed']
    cells=list(itertools.product(families,[8,32],[False,True],modes,['immediate','window','all'],[False,True]))
    assert len(cells)==864
    (BASE/'cells.json').write_text(json.dumps(cells,indent=2)+'\n')
    paths=subprocess.check_output(['git','ls-files','crates','research','Cargo.toml','Cargo.lock'],cwd=ROOT,text=True).splitlines()
    paths=[p for p in paths if p.endswith(('.rs','.toml','.lock'))]
    paths += ['research/chr-direct-conditional/tests/runtime_support/post_source.rs','research/chr-direct-conditional/tests/post_work.rs',str(Path(__file__).relative_to(ROOT)),'docs/experiments/registrations/S03-post-ownership.md']
    sources={p:sha(ROOT/p) for p in sorted(set(paths))}
    with zipfile.ZipFile(BASE/'sources.zip','w',compression=zipfile.ZIP_DEFLATED) as archive:
        for path in sources:archive.write(ROOT/path,path)
    freeze=dict(sources=sources,source_archive_sha256=sha(BASE/'sources.zip'),binaries=binaries,cells_hash=sha(BASE/'cells.json'),cpu=0,affinity=sorted(os.sched_getaffinity(0)),parent_commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip())
    (BASE/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    def invoke(kind,args,path):
        command=[binaries[kind]['path'],*map(str,args)]
        try:
            r=subprocess.run(command,cwd=ROOT,env=dict(os.environ,DEPENDENCY_FIRST_CLOCK='off'),capture_output=True,text=True,timeout=60,preexec_fn=limits)
            raw=dict(command=command,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr,timeout=False)
        except subprocess.TimeoutExpired as error:
            decode=lambda x:x.decode() if isinstance(x,bytes) else (x or '')
            raw=dict(command=command,exit_code=None,stdout=decode(error.stdout),stderr=decode(error.stderr),timeout=True)
        path.write_text(json.dumps(raw)+'\n');assert raw['exit_code']==0 and not raw['timeout'],path
        return raw['stdout']
    invoke('meter',['meter-check'],BASE/'meter-check.json')
    for rep in range(2):invoke('work',['--nocapture','--test-threads=1'],BASE/f'work-{rep+1}.json')
    (BASE/'runs').mkdir();start=time.monotonic()
    with (BASE/'results.jsonl').open('x') as output:
        for i,(family,size,reverse,mode,retention,cancel) in enumerate(cells):
            args=[mode,family,size,str(reverse).lower(),retention,str(cancel).lower()]
            for kind,rep in [('meter',0),('meter',1),('ordinary',0)]:
                assert time.monotonic()-start<1800
                raw=invoke(kind,args,BASE/'runs'/f'{i}-{kind}-{rep}.json')
                result=next(json.loads(x) for x in raw.splitlines() if x.startswith('{') and json.loads(x).get('event')=='result')
                assert len(result['endpoints'])==4 and all(e['first_ns'] is None for e in result['endpoints'])
                assert [e['complete'] for e in result['endpoints']]==([False,True,False,True] if cancel else [True]*4)
                output.write(json.dumps(dict(index=i,kind=kind,rep=rep,result=result))+'\n');output.flush()
            if (i+1)%144==0:print(f'completed {i+1}/864 configurations',flush=True)
    assert all(sha(ROOT/p)==h for p,h in sources.items())
    (BASE/'campaign.json').write_text(json.dumps(dict(configurations=864,workload_processes=2592,work_processes=2,seconds=time.monotonic()-start))+'\n')
    print('All configurations completed')
if __name__=='__main__':main()
