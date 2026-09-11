"""Prospectively registered ownership attribution; no timing inference."""
import gzip, hashlib, itertools, json, os, random, resource, subprocess, time, zipfile
from pathlib import Path
ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s08-preparation-ownership'
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0, {0})
    resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
    resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
def main():
    assert not (OUT / 'freeze.json').exists()
    bins = {k: ROOT / f'target/s08-preparation-ownership/release/examples/{v}_continuation_cost' for k,v in [('retained','static'),('detached','detached')]}
    cases = list(itertools.product([('common',0,0),('common',3,16),('independent',3,16),('early',3,16)], [False,True], ['immediate','window','all'], [False,True], ['birth-miss','conditional','native-scan']))
    jobs = list(itertools.product(range(2), cases, bins));random.Random(740812).shuffle(jobs)
    assert len(jobs) == 576
    paths = [p for base in ['research','crates'] for p in (ROOT/base).rglob('*.rs') if 'target' not in p.parts and 'chr-choice-graph' not in p.parts]
    paths += [ROOT/'Cargo.lock', ROOT/'Cargo.toml', Path(__file__), ROOT/'docs/experiments/registrations/S08-preparation-ownership.md']
    with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in paths: z.write(p, p.relative_to(ROOT))
    (OUT/'freeze.json').write_text(json.dumps(dict(sources={str(p.relative_to(ROOT)):sha(p) for p in paths}, archive_sha256=sha(OUT/'sources.zip'), binaries={k:sha(p) for k,p in bins.items()}, jobs=jobs, toolchain=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2))
    start=time.monotonic(); results={}
    with gzip.open(OUT/'runs.jsonl.gz','wt') as out:
        for rep,case,kind in jobs:
            assert time.monotonic()-start < 600
            (family,k,d),history,consumer,cancel,mode=case
            args=[mode,family,k,d,history,False,consumer,True,cancel,True]
            cmd=[str(bins[kind])]+[str(x).lower() for x in args]
            r=subprocess.run(cmd,capture_output=True,text=True,timeout=60,preexec_fn=limits)
            out.write(json.dumps(dict(rep=rep,case=case,kind=kind,command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr))+'\n');out.flush()
            assert r.returncode==0,(cmd,r.stderr)
            result=json.loads(r.stdout.splitlines()[-1]);assert result['meter']
            key=json.dumps((case,kind));memory=[p['reading']['memory'] for p in result['phases']]
            if key in results: assert memory == results[key],key
            results[key]=memory
    (OUT/'campaign.json').write_text(json.dumps(dict(processes=len(jobs),allocation_pairs=len(results),seconds=time.monotonic()-start),indent=2))
if __name__ == '__main__': main()
