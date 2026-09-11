"""Freeze and execute the structural-prefix source/work screen; no timings."""
import hashlib,json,os,resource,subprocess,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
BASE=ROOT/'docs/experiments/results/s01-generated-prefix'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
    resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def main():
    assert not (BASE/'freeze.json').exists()
    binaries={}
    for mode,features in [('work',['--features','metrics']),('plain',[])]:
        cmd=['cargo','test','-p','chr-compiled','--test','structural_prefix','--release','--no-default-features',*features,'--target-dir',f'target/s01-generated-prefix-{mode}','--no-run','--message-format=json']
        r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=300)
        (BASE/f'build-{mode}.json').write_text(json.dumps(dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr))+'\n')
        assert r.returncode==0,r.stderr
        binary=Path(next(json.loads(l)['executable'] for l in r.stdout.splitlines() if json.loads(l).get('executable')))
        binaries[mode]=dict(path=str(binary),sha256=sha(binary))
    paths=subprocess.check_output(['git','ls-files','crates','research','Cargo.toml','Cargo.lock'],cwd=ROOT,text=True).splitlines()
    paths=[p for p in paths if p.endswith(('.rs','.toml','.lock'))]+['research/chr-compiled/tests/structural_prefix.rs','research/chr-compiled/experiments/structural_prefix_source.rs','research/chr-compiled/experiments/structural_prefix.py','research/chr-compiled/experiments/audit_structural_prefix.py','docs/experiments/registrations/S01-generated-prefix.md']
    hashes={p:sha(ROOT/p) for p in sorted(set(paths))}
    with zipfile.ZipFile(BASE/'sources.zip','w',compression=zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    freeze=dict(sources=hashes,archive_sha256=sha(BASE/'sources.zip'),binaries=binaries,commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip())
    (BASE/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    for mode,b in binaries.items():
        for repeat in [1,2]:
            cmd=[b['path'],'--nocapture','--test-threads=1']
            try:
                r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=limits)
                raw=dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr,timeout=False)
            except subprocess.TimeoutExpired as e:
                decode=lambda x:x.decode() if isinstance(x,bytes) else (x or '')
                raw=dict(command=cmd,exit_code=None,stdout=decode(e.stdout),stderr=decode(e.stderr),timeout=True)
            (BASE/f'{mode}-{repeat}.json').write_text(json.dumps(raw)+'\n')
            assert raw['exit_code']==0 and not raw['timeout'],(mode,repeat)
            print(mode,repeat,'passed',flush=True)
    assert all(sha(ROOT/p)==h for p,h in hashes.items())
if __name__=='__main__':main()
