"""Freeze and confirm the context inclusion gate and ordinary operation sizing."""
import hashlib,json,os,resource,subprocess,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s08-context-inclusion'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_CPU,(60,60));resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
def main():
    assert not (OUT/'freeze.json').exists()
    builds={}
    for name,features in [('lookup','completed-traversal,work-diagnostics'),('ordered','ordered-context,completed-traversal,work-diagnostics'),('operations','')]:
        cmd=['cargo','test','-p','chr-direct-choice','--lib','--no-default-features','--no-run','--message-format=json']
        if name=='operations':cmd+=['--release']
        if features:cmd+=['--features',features]
        r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True);(OUT/f'build-{name}.log').write_text(r.stderr.rstrip()+'\n');assert r.returncode==0
        objects=[json.loads(s) for s in r.stdout.splitlines() if s.startswith('{')]
        b=Path(next(x['executable'] for x in objects if x.get('executable') and x.get('target',{}).get('name')=='chr_direct_choice'))
        builds[name]=dict(command=cmd,binary=str(b),sha256=sha(b))
    paths=[]
    for folder in ['research/chr-direct-choice','crates/chr-syntax']:
        paths+=list((ROOT/folder).rglob('*.rs'))+[ROOT/folder/'Cargo.toml']
    paths += [ROOT/'Cargo.toml',ROOT/'Cargo.lock',Path(__file__),ROOT/'docs/experiments/registrations/S08-context-inclusion.md']
    hashes={str(p.relative_to(ROOT)):sha(p) for p in paths}
    with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    (OUT/'freeze.json').write_text(json.dumps(dict(builds=builds,sources=hashes,archive_sha256=sha(OUT/'sources.zip'),toolchain=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2)+'\n')
    for name in ['lookup','ordered']:
        for rep in range(2):
            cmd=[builds[name]['binary'],'--nocapture','--test-threads=1']
            r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=limits)
            (OUT/f'{name}-{rep}.json').write_text(json.dumps(dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr),indent=2)+'\n');assert r.returncode==0
    cmd=[builds['operations']['binary'],'demand::context::tests::native_operation_sizing','--exact','--ignored','--nocapture','--test-threads=1']
    try:
        r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=limits)
        receipt=dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr)
    except subprocess.TimeoutExpired as e:receipt=dict(command=cmd,exit_code='timeout',stdout=str(e.stdout),stderr=str(e.stderr))
    (OUT/'operations.json').write_text(json.dumps(receipt,indent=2)+'\n');assert receipt['exit_code']==0
    print('Four semantic/work confirmations and native operation sizing completed')
if __name__=='__main__':main()
