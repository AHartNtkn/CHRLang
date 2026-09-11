"""Prospectively registered resource-copy allocation attribution."""
import hashlib,json,os,resource,subprocess,time,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
BASE=ROOT/'docs/experiments/results/s03-candidate-copy'
PARENT=ROOT/'docs/experiments/results/s03-post-ownership'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0})
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
    resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def main():
    assert not (BASE/'freeze.json').exists()
    cmd=['cargo','build','-p','chr-direct-conditional','--example','dependency_ownership','--release','--no-default-features','--features','candidate-profile','--target-dir','target/s03-candidate-copy-profile','--message-format=json']
    r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=300)
    (BASE/'build.json').write_text(json.dumps(dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr)))
    assert r.returncode==0,r.stderr
    binary=Path(next(json.loads(x)['executable'] for x in r.stdout.splitlines() if json.loads(x).get('executable')))
    cells=[dict(parent_index=i,cell=c) for i,c in enumerate(json.loads((PARENT/'cells.json').read_text())) if c[3] in ['birth','birth-miss','birth-miss-template'] and c[4]=='all']
    assert len(cells)==144
    (BASE/'cells.json').write_text(json.dumps(cells,indent=2)+'\n')
    paths=subprocess.check_output(['git','ls-files','crates','research','Cargo.toml','Cargo.lock'],cwd=ROOT,text=True).splitlines()
    paths=[p for p in paths if p.endswith(('.rs','.toml','.lock'))]
    paths+=['research/chr-direct-choice/src/demand/candidate_profile.rs','research/chr-direct-conditional/examples/support/candidate_profile.rs',str(Path(__file__).relative_to(ROOT)),'research/chr-direct-conditional/experiments/audit_candidate_copy.py','docs/experiments/registrations/S03-candidate-copy.md']
    sources={p:sha(ROOT/p) for p in sorted(set(paths))}
    with zipfile.ZipFile(BASE/'sources.zip','w',compression=zipfile.ZIP_DEFLATED) as archive:
        for p in sources:archive.write(ROOT/p,p)
    freeze=dict(sources=sources,archive_sha256=sha(BASE/'sources.zip'),binary=dict(path=str(binary),sha256=sha(binary)),cells_sha256=sha(BASE/'cells.json'),parent={p:sha(PARENT/p) for p in ['freeze.json','cells.json','results.jsonl']},cpu=0,affinity=sorted(os.sched_getaffinity(0)),commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip())
    (BASE/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    def invoke(args,path):
        command=[str(binary),*map(str,args)]
        try:
            r=subprocess.run(command,cwd=ROOT,env=dict(os.environ,DEPENDENCY_FIRST_CLOCK='off'),capture_output=True,text=True,timeout=60,preexec_fn=limits)
            raw=dict(command=command,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr,timeout=False)
        except subprocess.TimeoutExpired as e:
            decode=lambda x:x.decode() if isinstance(x,bytes) else (x or '')
            raw=dict(command=command,exit_code=None,stdout=decode(e.stdout),stderr=decode(e.stderr),timeout=True)
        path.write_text(json.dumps(raw)+'\n')
        assert raw['exit_code']==0 and not raw['timeout'] and not raw['stderr'],path
    invoke(['meter-check'],BASE/'meter-check.json')
    (BASE/'runs').mkdir()
    start=time.monotonic()
    for i,row in enumerate(cells):
        family,size,reverse,mode,retention,cancel=row['cell']
        for rep in range(2):
            assert time.monotonic()-start<1800
            invoke([mode,family,size,str(reverse).lower(),retention,str(cancel).lower()],BASE/'runs'/f'{i}-{rep}.json')
        if (i+1)%24==0:print(f'{i+1}/144 configurations',flush=True)
    assert all(sha(ROOT/p)==h for p,h in sources.items())
    (BASE/'campaign.json').write_text(json.dumps(dict(configurations=144,processes=288,seconds=time.monotonic()-start))+'\n')
if __name__=='__main__':main()
