"""Compile emitted call descriptors and qualify shared source/ownership gates."""
from pathlib import Path
import itertools,json,shutil,subprocess,sys,time
sys.dont_write_bytecode=True
import call_compiled_cost as previous
import call_trace_ownership as own
ROOT=own.ROOT
RAW=ROOT/'docs/experiments/results/s05-call-generated-entry'
PROJECT=ROOT/'target/s05-call-generated-project'
MODES=['generated','generated-sealed']
CELLS=[(mode,*c[1:])for mode in MODES for c in previous.CELLS if c[0]=='trace']
def binary(kind):return ROOT/f'target/s05-call-generated-{kind}/release/call-generated-artifact'
def invoke(cmd,path,timeout=45,bounded=True):
    assert not path.exists()
    start=time.perf_counter_ns()
    try:
        p=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=timeout,preexec_fn=own.bounds if bounded else None)
        r=dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr,wall_ns=time.perf_counter_ns()-start)
    except subprocess.TimeoutExpired as e:
        r=dict(command=cmd,timeout=True,stdout=(e.stdout or b'').decode(),stderr=(e.stderr or b'').decode())
    path.write_text(json.dumps(r,indent=2)+'\n');assert r.get('returncode')==0,path
    return r

def audit():
    freeze=json.loads((RAW/'freeze.json').read_text())
    for path,h in freeze['sha256'].items():
        actual=ROOT/path if path.startswith('target/s05-call-generated-') and '/release/' in path else RAW/'registered'/path
        assert own.sha(actual)==h,path
    for kind in ['primary','meter','metrics']:
        b=json.loads((RAW/f'build-{kind}.json').read_text());assert b['returncode']==0
        r=json.loads((RAW/f'gate-{kind}.json').read_text());assert r['returncode']==0
        assert 'complete_queries=768 cancellation_restarts=2304 retained_outputs_checked=true' in r['stderr']
    prior=json.loads((ROOT/'docs/experiments/results/s05-call-compiled-cost/analysis.json').read_text())['comparisons']
    expected={tuple(x['scenario']):x['trace_memory']['consumer']for x in prior}
    mems={};observations={}
    for kind,reps in [('meter',2),('primary',1)]:
        for k in range(reps):
            for i,c in enumerate(CELLS):
                r=json.loads((RAW/f'{kind}-{k}-{i:03}.json').read_text())
                assert r['command']==[str(binary(kind)),*map(str,c)]
                h,mem=own.parse(r);obs=(h['counts'],h['retained'])
                if c[1:] in observations:assert observations[c[1:]]==obs
                observations[c[1:]]=obs
                if kind=='meter':
                    assert mem is not None and mem['consumer']==expected[c[1:]]
                    if c in mems:assert mems[c]==mem
                    mems[c]=mem
                else:assert mem is None
    assert len(mems)==288
    return [dict(cell=list(c),memory=mems[c])for c in CELLS]

if __name__=='__main__':
    if sys.argv[1]=='run':
        RAW.mkdir(exist_ok=False)
        for kind,features in [('primary',[]),('meter',['--features','alloc-meter']),('metrics',['--features','metrics'])]:
            invoke(['cargo','build','--release','--offline','--jobs','2','--manifest-path',str(PROJECT/'Cargo.toml'),'--target-dir',str(ROOT/f'target/s05-call-generated-{kind}'),*features],RAW/f'build-{kind}.json',120,False)
            invoke([str(binary(kind)),'gate'],RAW/f'gate-{kind}.json')
            print(kind,'artifact and 768-query gate pass',flush=True)
        paths=[ROOT/p for p in subprocess.check_output(['git','ls-files','research','crates'],cwd=ROOT,text=True).splitlines()if p.endswith('.rs')or p.endswith('Cargo.toml')]
        paths += [ROOT/'Cargo.toml',ROOT/'Cargo.lock',PROJECT/'Cargo.toml',PROJECT/'Cargo.lock',PROJECT/'src/main.rs',PROJECT/'src/generated.rs',Path(__file__),Path(own.__file__),Path(previous.__file__),ROOT/'research/chr-reuse/examples/call_generated.rs',ROOT/'docs/experiments/registrations/S05-call-generated-entry.md']
        for p in paths:
            dst=RAW/'registered'/p.relative_to(ROOT);dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,dst)
        paths += [binary(k)for k in ['primary','meter','metrics']]
        (RAW/'freeze.json').write_text(json.dumps(dict(sha256={str(p.relative_to(ROOT)):own.sha(p)for p in paths},binary_bytes={k:binary(k).stat().st_size for k in ['primary','meter','metrics']}),indent=2)+'\n')
        for kind,reps in [('meter',2),('primary',1)]:
            for k in range(reps):
                for i,c in enumerate(CELLS):
                    invoke([str(binary(kind)),*map(str,c)],RAW/f'{kind}-{k}-{i:03}.json')
                print(kind,k,'288 lifecycle processes pass',flush=True)
        (RAW/'analysis.json').write_text(json.dumps(audit(),indent=2)+'\n')
    else:
        assert audit()==json.loads((RAW/'analysis.json').read_text())
        print('Three compiled gates and 864 ownership/source processes verified.')
