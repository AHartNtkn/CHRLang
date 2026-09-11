"""Registered stable-selection ownership/work comparison in the existing harness."""
import hashlib,itertools,json,os,resource,subprocess,time,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s02-selection-ownership'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def main():
    assert not (BASE/'freeze.json').exists()
    binaries={}
    for kind,features in [('ordinary',[]),('meter',['alloc-meter']),('work',['local-work,compiled-work'])]:
        cmd=['cargo','test','-p','chr-relational','--test','local_lifecycle','--release','--no-default-features','--target-dir',f'target/s02-selection-ownership-{kind}','--no-run','--message-format=json']
        if features:cmd+=['--features',features[0]]
        r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=300)
        (BASE/f'build-{kind}.json').write_text(json.dumps(dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr))+'\n');assert r.returncode==0
        binary=Path(next(json.loads(x)['executable'] for x in r.stdout.splitlines() if json.loads(x).get('executable')))
        binaries[kind]=dict(path=str(binary),sha256=sha(binary))
    cells=list(itertools.product(['flat','sparse','chain','competition'],[2,6],['local','local-filtered','scan','indexed','sealed','chr-scan','chr-indexed','chr-sealed'],['immediate','all'],['complete','cancel']))
    assert len(cells)==256;(BASE/'cells.json').write_text(json.dumps(cells,indent=2)+'\n')
    paths=subprocess.check_output(['git','ls-files','crates','research','Cargo.toml','Cargo.lock'],cwd=ROOT,text=True).splitlines();paths=[p for p in paths if p.endswith(('.rs','.toml','.lock'))]
    paths+=['research/chr-relational/tests/support/selection_source.rs','research/chr-relational/tests/support/selection_lifecycle.rs',str(Path(__file__).relative_to(ROOT)),'docs/experiments/registrations/S02-selection-ownership.md']
    hashes={p:sha(ROOT/p) for p in sorted(set(paths))}
    with zipfile.ZipFile(BASE/'sources.zip','w',compression=zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    (BASE/'freeze.json').write_text(json.dumps(dict(sources=hashes,archive_sha256=sha(BASE/'sources.zip'),binaries=binaries,cells_sha256=sha(BASE/'cells.json'),cpu=0,affinity=sorted(os.sched_getaffinity(0)),commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()),indent=2)+'\n')
    def invoke(kind,args,path):
        command=[binaries[kind]['path'],*map(str,args)]
        try:
            r=subprocess.run(command,cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=limits)
            raw=dict(command=command,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr,timeout=False)
        except subprocess.TimeoutExpired as e:
            decode=lambda x:x.decode() if isinstance(x,bytes) else (x or '')
            raw=dict(command=command,exit_code=None,stdout=decode(e.stdout),stderr=decode(e.stderr),timeout=True)
        path.write_text(json.dumps(raw)+'\n');assert raw['exit_code']==0 and not raw['timeout'] and not raw['stderr'],path
    invoke('meter',['selection','meter-check'],BASE/'meter-check.json')
    invoke('ordinary',[],BASE/'existing-smoke.json')
    (BASE/'runs').mkdir();start=time.monotonic();count=0
    for i,(family,n,mode,retention,stop) in enumerate(cells):
        for kind,rep in [('meter',0),('meter',1),('ordinary',0)]+([('work',0),('work',1)] if retention=='all' and stop=='complete' else []):
            assert time.monotonic()-start<1800
            invoke(kind,['selection',mode,family,n,retention,stop],BASE/'runs'/f'{i}-{kind}-{rep}.json');count+=1
        if (i+1)%32==0:print(f'{i+1}/256 configurations',flush=True)
    assert count==896;assert all(sha(ROOT/p)==h for p,h in hashes.items())
    (BASE/'campaign.json').write_text(json.dumps(dict(configurations=256,processes=count,seconds=time.monotonic()-start))+'\n')
if __name__=='__main__':main()
