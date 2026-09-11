"""Prospective continuing-state semantic, owner and scale qualification."""
import gzip,hashlib,itertools,json,os,resource,subprocess,time,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s08-continuing-lifecycle'
MODES=['direct','compact-live','scan','sealed','conditional','dependencies','templates','dependencies-reclaim','templates-reclaim']
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def invoke(cmd,seconds):
    def limits():
        os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(seconds,seconds))
    try:
        r=subprocess.run(cmd,capture_output=True,text=True,timeout=seconds,preexec_fn=limits);return dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr)
    except subprocess.TimeoutExpired as e:return dict(command=cmd,exit_code=None,stdout=str(e.stdout),stderr=str(e.stderr))
def main():
    assert not (BASE/'freeze.json').exists()
    bins={k:ROOT/f'target/s08-continuing-{k}/release/examples/continuing_lifecycle' for k in ['time','meter']}
    jobs=[dict(mode=m,resource=r,packing=p,kind=k,demand=n) for n,packing in [(32,[False,True]),(512,[False])] for m,r,p,k in itertools.product(MODES,[False,True],packing,bins)]
    assert len(jobs)==108
    paths=[]
    for folder in ['research/chr-reuse','research/chr-direct-choice','research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax']:
        paths.extend(p for p in (ROOT/folder).rglob('*.rs') if 'target' not in p.parts);paths.append(ROOT/folder/'Cargo.toml')
    paths.extend([ROOT/'Cargo.toml',ROOT/'Cargo.lock',Path(__file__),ROOT/'docs/experiments/registrations/S08-continuing-lifecycle-entry.md'])
    hashes={str(p.relative_to(ROOT)):sha(p) for p in paths}
    with zipfile.ZipFile(BASE/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    (BASE/'freeze.json').write_text(json.dumps(dict(sources=hashes,archive_sha256=sha(BASE/'sources.zip'),jobs=jobs,binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in bins.items()},toolchain=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2))
    start=time.monotonic()
    with gzip.open(BASE/'qualification.jsonl.gz','wt') as out:
        for i,j in enumerate(jobs):
            args=[j['mode'],j['resource'],j['demand'],'all',4,j['packing'],True]
            r=invoke([str(bins[j['kind']])]+[str(x).lower() for x in args],120 if j['demand']==512 else 60)
            out.write(json.dumps(dict(job=j,raw=r))+'\n');out.flush();print(i+1,j,r['exit_code'],round(time.monotonic()-start,2),flush=True)
            assert r['exit_code']==0,(j,r['stderr'])
    (BASE/'campaign.json').write_text(json.dumps(dict(processes=len(jobs),seconds=time.monotonic()-start),indent=2))
if __name__=='__main__':main()
