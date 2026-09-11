"""Freeze and repeat the bounded quiescent draining semantic/work gate."""
import hashlib,json,os,resource,subprocess,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s02-quiescent-drain'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    assert not (OUT/'freeze.json').exists()
    cmd=['cargo','test','-p','chr-relational','--no-default-features','--lib','--no-run','--message-format=json']
    r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,check=True)
    (OUT/'build.log').write_text(r.stderr.rstrip()+'\n')
    artifacts=[json.loads(s) for s in r.stdout.splitlines() if s.startswith('{')]
    binary=Path(next(x['executable'] for x in artifacts if x.get('executable') and x.get('target',{}).get('name')=='chr_relational'))
    paths=[]
    for folder in ['research/chr-relational','research/chr-integrated','research/chr-compiled','research/chr-persistent','crates/chr-syntax']:
        paths+=list((ROOT/folder).rglob('*.rs'))+[ROOT/folder/'Cargo.toml']
    paths += [ROOT/'research/chr-direct-conditional/tests/runtime_support/mod.rs',Path(__file__),ROOT/'research/chr-relational/experiments/check_quiescent_drain.py',ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'docs/experiments/registrations/S02-quiescent-drain.md']
    hashes={str(p.relative_to(ROOT)):sha(p) for p in paths}
    with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    command=[str(binary),'--nocapture','--test-threads=1']
    (OUT/'freeze.json').write_text(json.dumps(dict(build=cmd,command=command,binary_sha256=sha(binary),sources=hashes,archive_sha256=sha(OUT/'sources.zip')),indent=2)+'\n')
    def limits():
        os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_CPU,(60,60));resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
    data=[]
    for rep in range(2):
        r=subprocess.run(command,cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=limits)
        (OUT/f'confirmation-{rep}.json').write_text(json.dumps(dict(command=command,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr),indent=2)+'\n')
        assert r.returncode==0
        priority=[s[s.index('PRIORITY,'):] for s in r.stdout.splitlines() if 'PRIORITY,' in s and 'READY_' not in s]
        controls=[s[s.index('depth='):] for s in r.stdout.splitlines() if ' deep=' in s]
        readiness=[s[s.index('READY_'):] for s in r.stdout.splitlines() if 'READY_' in s]
        assert len(readiness)==120
        drain=[s[s.index('DRAIN'): ] for s in r.stdout.splitlines() if 'DRAIN' in s]
        assert len(drain)==579
        assert len(priority)==24 and len(controls)==12
        data.append(dict(priority=priority,controls=controls,readiness=readiness,drain=drain))
    assert data[0]==data[1]
    (OUT/'audit.json').write_text(json.dumps(dict(confirmations=2,drain_rows=579,readiness_cases=120,priority_cases=24,established_controls=12,exact_work_repeats=True,rows=data[0]),indent=2)+'\n')
    print('Two complete library confirmations; 579 drain, 120 readiness, 24 priority and 12 established-control rows repeat exactly')
if __name__=='__main__':main()
