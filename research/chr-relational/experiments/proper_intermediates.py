"""Freeze and execute the registered CHR source-selection contrast."""
import hashlib,json,os,resource,subprocess,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s01-proper-intermediates'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def main():
    assert not (BASE/'freeze.json').exists()
    cmd=['cargo','test','-p','chr-relational','--test','multihead','--release','--no-default-features','--target-dir','target/s01-proper-intermediates','--no-run','--message-format=json']
    r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=300)
    (BASE/'build.json').write_text(json.dumps(dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr))+'\n');assert r.returncode==0
    binary=Path(next(json.loads(x)['executable'] for x in r.stdout.splitlines() if json.loads(x).get('executable')))
    paths=subprocess.check_output(['git','ls-files','crates','research','Cargo.toml','Cargo.lock'],cwd=ROOT,text=True).splitlines()
    paths=[p for p in paths if p.endswith(('.rs','.toml','.lock'))]+[str(Path(__file__).relative_to(ROOT)),'docs/experiments/registrations/S01-proper-intermediates.md']
    hashes={p:sha(ROOT/p) for p in sorted(set(paths))}
    with zipfile.ZipFile(BASE/'sources.zip','w',compression=zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    (BASE/'freeze.json').write_text(json.dumps(dict(sources=hashes,archive_sha256=sha(BASE/'sources.zip'),binary=dict(path=str(binary),sha256=sha(binary)),commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()),indent=2)+'\n')
    for name,args in [('target-1',['--nocapture','--test-threads=1']),('target-2',['--nocapture','--test-threads=1'])]:
        command=[str(binary),*args]
        try:
            r=subprocess.run(command,cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=limits)
            raw=dict(command=command,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr,timeout=False)
        except subprocess.TimeoutExpired as e:
            decode=lambda s:s.decode() if isinstance(s,bytes) else (s or '')
            raw=dict(command=command,exit_code=None,stdout=decode(e.stdout),stderr=decode(e.stderr),timeout=True)
        (BASE/f'{name}.json').write_text(json.dumps(raw)+'\n');assert raw['exit_code']==0 and not raw['timeout'],name
        print(name,'passed',flush=True)
    assert all(sha(ROOT/p)==h for p,h in hashes.items())
if __name__=='__main__':main()
