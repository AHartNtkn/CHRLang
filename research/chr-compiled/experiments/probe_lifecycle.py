"""Freeze and run the registered full-lifecycle probe pilot."""
import gzip,hashlib,itertools,json,os,random,resource,shutil,subprocess
from pathlib import Path
import zipfile
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s01-probe-lifecycle'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_CPU,(60,60));resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
def invoke(binary,c):
    cmd=[binary,c['family'],str(c['n']),c['policy'],c['access'],str(c['keep']).lower(),str(c['cancel']).lower()]
    r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=limits)
    raw=dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr)
    assert r.returncode==0,raw
    assert json.loads(r.stdout)['validated']
    return raw

def main():
    assert not (OUT/'freeze.json').exists()
    builds={};bins=ROOT/'target/s01-probe-lifecycle-binaries';bins.mkdir(exist_ok=True)
    for feature,kind in itertools.product([False,True],['time','meter']):
        name=f'{feature}-{kind}';features=[]
        if feature:features+=['selective-probe']
        if kind=='meter':features+=['alloc-meter']
        cmd=['cargo','build','--release','-p','chr-compiled','--example','probe_lifecycle','--no-default-features','--target-dir','target/s01-probe-lifecycle-build']
        if features:cmd+=['--features',','.join(features)]
        r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True);(OUT/f'build-{name}.log').write_text(r.stderr.rstrip()+'\n');assert r.returncode==0
        binary=bins/name;shutil.copy2(ROOT/'target/s01-probe-lifecycle-build/release/examples/probe_lifecycle',binary)
        builds[name]=dict(command=cmd,binary=str(binary),sha256=sha(binary));print('built',name,flush=True)
    entry=[];warmups=[]
    for name,b in builds.items():
        for family,policy in itertools.product(['selective','neutral','duplicate','broad'],['global','active']):
            c=dict(family=family,n=16,policy=policy,access='indexed',keep=True,cancel=True,feature=name.startswith('True'))
            entry.append(dict(build=name,case=c,raw=invoke(b['binary'],c)))
            if name.endswith('time'):warmups.append(dict(build=name,case=c,raw=invoke(b['binary'],c)))
    (OUT/'entry.json').write_text(json.dumps(entry)+'\n');(OUT/'warmups.json').write_text(json.dumps(warmups)+'\n')
    cases=[]
    for feature,family,n,policy,access,keep,cancel in itertools.product([False,True],['selective','neutral','duplicate','broad'],[16,128],['global','active'],['scan','indexed'],[False,True],[False,True]):
        if feature and access=='scan':continue
        cases.append(dict(feature=feature,family=family,n=n,policy=policy,access=access,keep=keep,cancel=cancel))
    assert len(cases)==192
    jobs=[];rng=random.Random(8204)
    for kind,reps in [('time',5),('meter',2)]:
        for rep in range(reps):
            order=cases.copy();rng.shuffle(order);jobs.extend(dict(kind=kind,rep=rep,case=c) for c in order)
    sources=[]
    for folder in ['research/chr-compiled','research/chr-persistent','research/chr-observe','research/chr-cases','crates/chr-syntax']:
        sources+=list((ROOT/folder).rglob('*.rs'))+[ROOT/folder/'Cargo.toml']
    sources += [ROOT/'Cargo.toml',ROOT/'Cargo.lock',Path(__file__),ROOT/'docs/experiments/registrations/S01-probe-lifecycle.md']
    hashes={str(p.relative_to(ROOT)):sha(p) for p in sources}
    with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    (OUT/'freeze.json').write_text(json.dumps(dict(builds=builds,sources=hashes,archive_sha256=sha(OUT/'sources.zip'),jobs=jobs,toolchain=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2)+'\n')
    with gzip.open(OUT/'runs.jsonl.gz','wt') as f:
        for i,j in enumerate(jobs):
            b=builds[f"{j['case']['feature']}-{j['kind']}"]
            f.write(json.dumps(dict(job=j,raw=invoke(b['binary'],j['case'])))+'\n');f.flush()
            if (i+1)%192==0:print('block',i+1,'/',len(jobs),'complete',flush=True)
    print('1344 full lifecycle samples complete',flush=True)
if __name__=='__main__':main()
