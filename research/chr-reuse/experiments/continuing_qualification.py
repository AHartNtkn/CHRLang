"""Bounded correctness/opportunity qualification; no comparative cost claims."""
from pathlib import Path
import hashlib,itertools,json,resource,subprocess,time
ROOT=Path(__file__).resolve().parents[3]
RAW=ROOT/'docs/experiments/results/s08-continuing-qualification'
BIN=ROOT/'target/s08-continuing-qualification/release/examples/continuing_resource_gate'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def bounds():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
    resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def decode(s):return s.decode() if isinstance(s,bytes) else s or ''
if __name__=='__main__':
    configs=list(itertools.product(['dependencies','templates','dependencies-reclaim','templates-reclaim'],['pure','resource']))
    paths=[BIN,Path(__file__),ROOT/'research/chr-reuse/examples/continuing_resource_gate.rs',ROOT/'research/chr-reuse/tests/consumer_pressure.rs',ROOT/'research/chr-reuse/examples/support/stream_run.rs',ROOT/'docs/experiments/registrations/S08-continuing-qualification.md']
    paths+=list((ROOT/'research/chr-direct-choice/src').rglob('*.rs'))
    manifest=RAW/'manifest.json';assert not manifest.exists()
    manifest.write_text(json.dumps(dict(configs=configs,sha256={str(p.relative_to(ROOT)):digest(p) for p in paths}),indent=2)+'\n')
    for i,config in enumerate(configs):
        cmd=[str(BIN),*config]
        try:
            p=subprocess.run(cmd,capture_output=True,text=True,timeout=60,preexec_fn=bounds)
            r=dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
        except subprocess.TimeoutExpired as e:r=dict(command=cmd,timeout=True,stdout=decode(e.stdout),stderr=decode(e.stderr))
        (RAW/f'gate-{i}.json').write_text(json.dumps(r,indent=2)+'\n')
        print(config,r,flush=True)
        if r.get('returncode')!=0:raise SystemExit('Gate obstruction; further cells have not run.')
