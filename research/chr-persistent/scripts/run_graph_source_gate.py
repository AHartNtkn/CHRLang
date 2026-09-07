"""Registered per-case completion correspondence; not timing evidence."""
import hashlib,json,os,platform,resource,signal,subprocess,tarfile,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results';PREFIX='E14-graph-source-gate-v2'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def bound():resource.setrlimit(resource.RLIMIT_AS,(1073741824,1073741824))
def main():
    paths=[OUT/f'{PREFIX}{s}' for s in ('-manifest.json','.jsonl','-inputs.tar.gz')]
    if any(p.exists() for p in paths):raise FileExistsError('evidence exists')
    command=['cargo','test','-p','chr-persistent','--test','graph_registry','--release','--no-run','--message-format=json']
    build=subprocess.run(command,cwd=ROOT,text=True,capture_output=True)
    if build.returncode:raise RuntimeError(build.stderr)
    artifacts=[json.loads(s) for s in build.stdout.splitlines() if s.startswith('{')]
    binary=Path(next(a['executable'] for a in artifacts if a.get('reason')=='compiler-artifact' and a.get('target',{}).get('name')=='graph_registry' and a.get('executable')))
    inputs=[ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'docs/experiments/registrations/E14-graph-source-gate.md']
    for base in ('research/chr-persistent','research/chr-observe','research/chr-cases','crates/chr-syntax','crates/chr-reference','crates/chr-programs'):
        inputs+=sorted((ROOT/base).rglob('*.rs'))+[ROOT/base/'Cargo.toml']
    inputs+=sorted((ROOT/'research/chr-persistent/scripts').glob('*.py'))
    tc=ROOT/'rust-toolchain.toml'
    if tc.exists():inputs.append(tc)
    frozen={str(p.relative_to(ROOT)):digest(p) for p in inputs}
    order=[dict(phase=p,index=i) for p in ('gate','replay') for i in range(64)]
    with paths[0].open('x') as mf,paths[1].open('x') as rf,tarfile.open(paths[2],'x:gz') as archive:
        for p in inputs:archive.add(p,arcname=str(p.relative_to(ROOT)))
        json.dump(dict(sha256=frozen,binary=str(binary),binary_sha256=digest(binary),order=order,build_command=command,
                       timeout_seconds=30,address_space_bytes=1073741824,platform=platform.platform(),
                       compiler=subprocess.check_output(['rustc','--version','--verbose'],cwd=ROOT,text=True)),mf,indent=2)
        mf.write('\n');mf.flush();os.fsync(mf.fileno())
        for cell in order:
            assert all(digest(ROOT/p)==h for p,h in frozen.items())
            start=time.monotonic();cmd=[str(binary),'--ignored','--exact','registered_completion_case','--nocapture']
            proc=subprocess.Popen(cmd,cwd=ROOT,env={**os.environ,'CHR_GRAPH_CASE':str(cell['index'])},stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,preexec_fn=bound,start_new_session=True)
            try:stdout,stderr=proc.communicate(timeout=30);status=proc.returncode
            except subprocess.TimeoutExpired:
                os.killpg(proc.pid,signal.SIGKILL);stdout,stderr=proc.communicate();status='timeout'
            row=dict(**cell,command=cmd,exit=status,stdout=stdout,stderr=stderr,wall_seconds=time.monotonic()-start)
            rf.write(json.dumps(row,sort_keys=True)+'\n');rf.flush();os.fsync(rf.fileno())
            if status!=0:raise RuntimeError(f'unsuccessful case retained: {cell}')
        assert all(digest(ROOT/p)==h for p,h in frozen.items())
    print('128 source completion children recorded')
if __name__=='__main__':main()
