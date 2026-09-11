"""Freeze and confirm the independent partner-order source/work gate."""
import hashlib,json,os,resource,subprocess,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s01-partner-order-entry'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_CPU,(60,60));resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
def main():
    assert not (OUT/'freeze.json').exists()
    builds={}
    for name in ['on','off']:
        cmd=['cargo','test','-p','chr-compiled','--test','partner_order','--no-run','--message-format=json']
        if name=='off':cmd+=['--no-default-features']
        r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True);(OUT/f'build-{name}.log').write_text(r.stderr.rstrip()+'\n');assert r.returncode==0
        objs=[json.loads(s) for s in r.stdout.splitlines() if s.startswith('{')]
        binary=Path(next(x['executable'] for x in objs if x.get('executable')))
        builds[name]=dict(command=cmd,binary=str(binary),sha256=sha(binary))
    paths=[]
    for folder in ['research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax']:
        paths+=list((ROOT/folder).rglob('*.rs'))+[ROOT/folder/'Cargo.toml']
    paths += [ROOT/'Cargo.toml',ROOT/'Cargo.lock',Path(__file__),ROOT/'docs/experiments/registrations/S01-partner-order-entry.md']
    sources={str(p.relative_to(ROOT)):sha(p) for p in paths}
    with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in sources:z.write(ROOT/p,p)
    (OUT/'freeze.json').write_text(json.dumps(dict(builds=builds,sources=sources,archive_sha256=sha(OUT/'sources.zip'),toolchain=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2)+'\n')
    for name,reps in [('on',2),('off',1)]:
        for rep in range(reps):
            cmd=[builds[name]['binary'],'--nocapture','--test-threads=1']
            r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=limits)
            (OUT/f'{name}-{rep}.json').write_text(json.dumps(dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr),indent=2)+'\n');assert r.returncode==0
    print('Three source/work confirmations complete')
if __name__=='__main__':main()
