"""Freeze and execute the registered context selection operation gate."""
import hashlib,json,os,resource,subprocess,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s08-context-selection'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0})
    resource.setrlimit(resource.RLIMIT_CPU,(120,120))
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
def main():
    assert not (OUT/'freeze.json').exists()
    cmd=['cargo','test','-p','chr-direct-choice','--lib','--no-default-features','--release','--no-run','--message-format=json']
    r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True)
    (OUT/'build.log').write_text(r.stderr.rstrip()+'\n');assert r.returncode==0
    objects=[json.loads(s) for s in r.stdout.splitlines() if s.startswith('{')]
    binary=Path(next(x['executable'] for x in objects if x.get('executable') and x.get('target',{}).get('name')=='chr_direct_choice'))
    paths=[]
    for folder in ['research/chr-direct-choice','crates/chr-syntax']:
        paths+=list((ROOT/folder).rglob('*.rs'))+[ROOT/folder/'Cargo.toml']
    paths+=[ROOT/'Cargo.toml',ROOT/'Cargo.lock',Path(__file__),ROOT/'docs/experiments/registrations/S08-context-selection.md']
    sources={str(p.relative_to(ROOT)):sha(p) for p in paths}
    with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in sources:z.write(ROOT/p,p)
    (OUT/'freeze.json').write_text(json.dumps(dict(command=cmd,binary=str(binary),binary_sha256=sha(binary),sources=sources,archive_sha256=sha(OUT/'sources.zip'),toolchain=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2)+'\n')
    for name,args in [('truth',['ordered_inclusion_matches_independent_assignment_truth','--nocapture']),('operations',['demand::context::tests::native_selection_sizing','--exact','--ignored','--nocapture'])]:
        command=[str(binary),*args,'--test-threads=1']
        r=subprocess.run(command,cwd=ROOT,capture_output=True,text=True,timeout=150,preexec_fn=limits)
        (OUT/(name+'.json')).write_text(json.dumps(dict(command=command,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr),indent=2)+'\n');assert r.returncode==0
        print(name,'completed',flush=True)
if __name__=='__main__':main()
