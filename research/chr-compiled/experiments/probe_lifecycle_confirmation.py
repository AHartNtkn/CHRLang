"""Confirm matched complete sessions with closely interleaved native batches."""
import gzip,hashlib,itertools,json,os,random,resource,shutil,subprocess,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s01-probe-lifecycle-confirmation'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_CPU,(60,60));resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
def invoke(b,c,count):
    cmd=[b,c['family'],str(c['n']),c['policy'],'indexed','false','false',str(count)]
    r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=limits)
    assert r.returncode==0,(cmd,r.stderr)
    sessions=[json.loads(s) for s in r.stdout.splitlines()];assert len(sessions)==count and all(s['validated'] for s in sessions)
    return dict(command=cmd,exit_code=r.returncode,sessions=sessions,stderr=r.stderr)
def main():
    assert not (OUT/'freeze.json').exists();builds={};bins=ROOT/'target/s01-probe-confirmation-binaries';bins.mkdir(exist_ok=True)
    for feature,kind in itertools.product([False,True],['time','meter']):
        flags=(['selective-probe'] if feature else [])+(['alloc-meter'] if kind=='meter' else [])
        cmd=['cargo','build','--release','-p','chr-compiled','--example','probe_lifecycle','--no-default-features','--target-dir','target/s01-probe-confirmation-build']
        if flags:cmd+=['--features',','.join(flags)]
        r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True);(OUT/f'build-{feature}-{kind}.log').write_text(r.stderr.rstrip()+'\n');assert r.returncode==0
        b=bins/f'{feature}-{kind}';shutil.copy2(ROOT/'target/s01-probe-confirmation-build/release/examples/probe_lifecycle',b);builds[f'{feature}-{kind}']=dict(command=cmd,binary=str(b),sha256=sha(b));print('built',feature,kind,flush=True)
    scenarios=[dict(family=f,n=128,policy=p) for f,p in itertools.product(['selective','neutral','duplicate','broad'],['global','active'])]+[dict(family='selective',n=16,policy='global')]
    warmups=[dict(case=c,feature=f,raw=invoke(builds[f'{f}-time']['binary'],c,50)) for c in scenarios for f in [False,True]]
    with gzip.open(OUT/'warmups.json.gz','wt') as g:json.dump(warmups,g)
    jobs=[];rng=random.Random(8205)
    for rep in range(20):
        order=list(enumerate(scenarios));rng.shuffle(order)
        for i,c in order:
            for feature in ([False,True] if (i+rep)%2==0 else [True,False]):jobs.append(dict(case=c,feature=feature,kind='time',rep=rep,count=50))
    for rep in range(2):
        for c,feature in itertools.product(scenarios,[False,True]):jobs.append(dict(case=c,feature=feature,kind='meter',rep=rep,count=1))
    paths=[]
    for folder in ['research/chr-compiled','research/chr-persistent','research/chr-observe','research/chr-cases','crates/chr-syntax']:paths+=list((ROOT/folder).rglob('*.rs'))+[ROOT/folder/'Cargo.toml']
    paths += [ROOT/'Cargo.toml',ROOT/'Cargo.lock',Path(__file__),ROOT/'docs/experiments/registrations/S01-probe-lifecycle-confirmation.md']
    hashes={str(p.relative_to(ROOT)):sha(p) for p in paths}
    with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    (OUT/'freeze.json').write_text(json.dumps(dict(builds=builds,sources=hashes,archive_sha256=sha(OUT/'sources.zip'),jobs=jobs,toolchain=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2)+'\n')
    with gzip.open(OUT/'runs.jsonl.gz','wt') as g:
        for i,j in enumerate(jobs):
            r=invoke(builds[f"{j['feature']}-{j['kind']}"]['binary'],j['case'],j['count']);g.write(json.dumps(dict(job=j,raw=r))+'\n');g.flush()
            if (i+1)%36==0:print(i+1,'/',len(jobs),'complete',flush=True)
    print('Confirmation complete',flush=True)
if __name__=='__main__':main()
