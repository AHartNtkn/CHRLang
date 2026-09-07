"""Freeze and run the registered borrowed-observation semantic matrix."""
import hashlib,json,os,platform,resource,signal,subprocess,sys,tarfile,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results'
PREFIX='E14-graph-gate-v1'
GROUPS=['targeted_templates','common_roots_different_environments','all_directed_graph_pairs']
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def bound():resource.setrlimit(resource.RLIMIT_AS,(1073741824,1073741824))
def main():
    paths=[OUT/f'{PREFIX}{suffix}' for suffix in ('-manifest.json','.jsonl','-inputs.tar.gz')]
    if any(p.exists() for p in paths):raise FileExistsError('evidence exists')
    build=subprocess.run(['cargo','test','-p','chr-observe','--test','graph_oracle','--no-run','--message-format=json'],cwd=ROOT,text=True,capture_output=True)
    if build.returncode:raise RuntimeError(build.stderr)
    artifacts=[json.loads(s) for s in build.stdout.splitlines() if s.startswith('{')]
    binary=Path(next(a['executable'] for a in artifacts if a.get('reason')=='compiler-artifact' and a.get('target',{}).get('name')=='graph_oracle' and a.get('executable')))
    inputs=[ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'docs/experiments/registrations/E14-graph-gate.md']
    for base in ('research/chr-observe','crates/chr-syntax','crates/chr-reference'):
        inputs += sorted((ROOT/base).rglob('*.rs'))+[ROOT/base/'Cargo.toml']
    inputs+=sorted((ROOT/'research/chr-observe/scripts').glob('*.py'))
    toolchain=ROOT/'rust-toolchain.toml'
    if toolchain.exists():inputs.append(toolchain)
    frozen={str(p.relative_to(ROOT)):digest(p) for p in inputs}
    order=[dict(phase=p,group=g) for p in ('gate','replay') for g in GROUPS]
    with paths[0].open('x') as mf,paths[1].open('x') as rf,tarfile.open(paths[2],'x:gz') as archive:
        for p in inputs:archive.add(p,arcname=str(p.relative_to(ROOT)))
        json.dump(dict(sha256=frozen,binary=str(binary),binary_sha256=digest(binary),order=order,
                       timeout_seconds=30,address_space_bytes=1073741824,platform=platform.platform(),
                       compiler=subprocess.check_output(['rustc','--version','--verbose'],cwd=ROOT,text=True),
                       build_command=['cargo','test','-p','chr-observe','--test','graph_oracle','--no-run','--message-format=json']),mf,indent=2)
        mf.write('\n');mf.flush();os.fsync(mf.fileno())
        for cell in order:
            assert all(digest(ROOT/p)==h for p,h in frozen.items())
            start=time.monotonic();command=[str(binary),'--exact',cell['group'],'--nocapture']
            proc=subprocess.Popen(command,cwd=ROOT,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,preexec_fn=bound,start_new_session=True)
            try:stdout,stderr=proc.communicate(timeout=30);status=proc.returncode
            except subprocess.TimeoutExpired:
                os.killpg(proc.pid,signal.SIGKILL);stdout,stderr=proc.communicate();status='timeout'
            row=dict(**cell,command=command,exit=status,stdout=stdout,stderr=stderr,wall_seconds=time.monotonic()-start)
            rf.write(json.dumps(row,sort_keys=True)+'\n');rf.flush();os.fsync(rf.fileno())
            if status!=0:raise RuntimeError(f'unsuccessful group retained: {cell}')
        assert all(digest(ROOT/p)==h for p,h in frozen.items())
    print('6 isolated groups recorded')
if __name__=='__main__':main()
