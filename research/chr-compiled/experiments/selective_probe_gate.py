"""Freeze and confirm the prepared selective-probe intervention."""
import hashlib,json,os,resource,subprocess,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s01-selective-probe'
TESTS=['partner_order','partner_order_bound','selective_probe']
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_CPU,(60,60));resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
def main():
    assert not (OUT/'freeze.json').exists()
    builds={}
    for feature in [False,True]:
        for metrics in [False,True]:
            name=f'{feature}-{metrics}'
            cmd=['cargo','test','-p','chr-compiled','--no-run','--message-format=json']
            for test in TESTS:cmd+=['--test',test]
            if feature:cmd+=['--features','selective-probe']
            if not metrics:cmd+=['--no-default-features']
            r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True);(OUT/f'build-{name}.log').write_text(r.stderr.rstrip()+'\n');assert r.returncode==0
            objs=[json.loads(s) for s in r.stdout.splitlines() if s.startswith('{')]
            binaries={o['target']['name']:dict(path=o['executable'],sha256=sha(Path(o['executable']))) for o in objs if o.get('executable') and o['target']['name'] in TESTS}
            assert set(binaries)==set(TESTS);builds[name]=dict(command=cmd,binaries=binaries)
    paths=[]
    for folder in ['research/chr-compiled','research/chr-persistent','research/chr-observe','research/chr-cases','crates/chr-syntax']:
        paths+=list((ROOT/folder).rglob('*.rs'))+[ROOT/folder/'Cargo.toml']
    paths += [ROOT/'Cargo.toml',ROOT/'Cargo.lock',Path(__file__),ROOT/'docs/experiments/registrations/S01-selective-probe.md']
    sources={str(p.relative_to(ROOT)):sha(p) for p in paths}
    with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in sources:z.write(ROOT/p,p)
    (OUT/'freeze.json').write_text(json.dumps(dict(builds=builds,sources=sources,archive_sha256=sha(OUT/'sources.zip'),toolchain=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2)+'\n')
    for feature in [False,True]:
        for metrics in [False,True]:
            name=f'{feature}-{metrics}'
            for test in TESTS:
                for rep in range(2 if metrics else 1):
                    cmd=[builds[name]['binaries'][test]['path'],'--nocapture','--test-threads=1']
                    r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=limits)
                    (OUT/f'{name}-{test}-{rep}.json').write_text(json.dumps(dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr),indent=2)+'\n');assert r.returncode==0
    print('18 frozen qualification processes completed')
if __name__=='__main__':main()
