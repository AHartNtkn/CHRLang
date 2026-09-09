"""Isolated uninstrumented/allocation pilot; no ranking from one observation."""
import hashlib,json,os,platform,resource,signal,subprocess,tarfile,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results';PREFIX='E14-graph-cost-pilot-v1'
def ids():return [f'{f}-{n}-{kind}' for f,n in [('dag',4),('dag',8),('unary',16),('unary',64),('symmetric',4),('symmetric',8),('retention',64),('retention',1024)] for kind in ('repeat','distinct')]
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def bound():resource.setrlimit(resource.RLIMIT_AS,(1073741824,1073741824))
def main():
    paths=[OUT/f'{PREFIX}{s}' for s in ('-manifest.json','.jsonl','-inputs.tar.gz')]
    if any(p.exists() for p in paths):raise FileExistsError('pilot exists')
    for gate in ('E14-graph-gate-v2','E14-graph-source-gate-v2'):assert json.loads((OUT/f'{gate}-audit.json').read_text())['passed']
    cmd=['cargo','build','-p','chr-persistent','--release','--example','graph_cost','--example','graph_memory']
    subprocess.run(cmd,cwd=ROOT,check=True)
    binaries={kind:ROOT/'target/release/examples'/name for kind,name in [('system','graph_cost'),('meter','graph_memory')]}
    inputs=[ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'docs/experiments/registrations/E14-graph-cost-pilot.md']
    for base in ('research/chr-persistent','research/chr-observe','research/chr-cases','crates/chr-syntax','crates/chr-reference','crates/chr-programs'):
        inputs+=sorted((ROOT/base).rglob('*.rs'))+[ROOT/base/'Cargo.toml']
    inputs+=sorted((ROOT/'research/chr-persistent/scripts').glob('*.py'))
    if (ROOT/'rust-toolchain.toml').exists():inputs.append(ROOT/'rust-toolchain.toml')
    frozen={str(p.relative_to(ROOT)):digest(p) for p in inputs}
    order=[dict(kind=kind,id=id,policy=policy) for kind in ('system','meter') for id in ids() for policy in ('eager_clone','eager_compare','eager_graph_compare','graph_compare')]
    with paths[0].open('x') as mf,paths[1].open('x') as rf,tarfile.open(paths[2],'x:gz') as archive:
        for p in inputs:archive.add(p,arcname=str(p.relative_to(ROOT)))
        json.dump(dict(sha256=frozen,binary_sha256={k:digest(p) for k,p in binaries.items()},order=order,build_command=cmd,
                       timeout_seconds=30,address_space_bytes=1073741824,platform=platform.platform(),
                       compiler=subprocess.check_output(['rustc','--version','--verbose'],cwd=ROOT,text=True)),mf,indent=2)
        mf.write('\n');mf.flush();os.fsync(mf.fileno())
        for cell in order:
            assert all(digest(ROOT/p)==h for p,h in frozen.items())
            start=time.monotonic();command=[str(binaries[cell['kind']]),cell['id'],cell['policy']]
            proc=subprocess.Popen(command,cwd=ROOT,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,preexec_fn=bound,start_new_session=True)
            try:stdout,stderr=proc.communicate(timeout=30);status=proc.returncode
            except subprocess.TimeoutExpired:
                os.killpg(proc.pid,signal.SIGKILL);stdout,stderr=proc.communicate();status='timeout'
            rf.write(json.dumps(dict(**cell,command=command,exit=status,stdout=stdout,stderr=stderr,wall_seconds=time.monotonic()-start),sort_keys=True)+'\n');rf.flush();os.fsync(rf.fileno())
            if status!=0:raise RuntimeError(f'unsuccessful child retained: {cell}')
        assert all(digest(ROOT/p)==h for p,h in frozen.items())
    print('128 observation lifecycle pilot children recorded')
if __name__=='__main__':main()
