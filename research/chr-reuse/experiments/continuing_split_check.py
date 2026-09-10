"""Qualify the obstructed configuration with separately bounded replay."""
from pathlib import Path
import hashlib,importlib.util,json,resource,subprocess,sys
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3]
RAW=ROOT/'docs/experiments/results/s08-continuing-qualification'
BIN=ROOT/'target/s08-continuing-metered-probe/release/examples/continuing_ownership_split'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def bounds():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def decode(s):return s.decode() if isinstance(s,bytes) else s or ''
if __name__=='__main__':
    config=['dependencies',512,'all',4]
    paths=[BIN,Path(__file__),ROOT/'research/chr-reuse/examples/continuing_ownership_split.rs',ROOT/'research/chr-reuse/Cargo.toml',ROOT/'research/chr-reuse/examples/continuing_ownership.rs',ROOT/'research/chr-reuse/experiments/continuing_ownership.py']
    manifest=RAW/'split-manifest.json';assert not manifest.exists()
    manifest.write_text(json.dumps(dict(config=config,sha256={str(p.relative_to(ROOT)):digest(p) for p in paths}),indent=2)+'\n')
    spec=importlib.util.spec_from_file_location('ownership',paths[-1]);module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    for phase in ['validate','measure']:
        command=[str(BIN),*map(str,config),phase]
        try:
            p=subprocess.run(command,capture_output=True,text=True,timeout=60,preexec_fn=bounds)
            r=dict(command=command,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
        except subprocess.TimeoutExpired as e:r=dict(command=command,timeout=True,stdout=decode(e.stdout),stderr=decode(e.stderr))
        (RAW/f'split-{phase}.json').write_text(json.dumps(r,indent=2)+'\n');print(phase,r,flush=True)
        assert r.get('returncode')==0 and not r['stderr'],r
        row=json.loads(r['stdout'])
        if phase=='validate':assert row=={'validated':True}
        else:module.validate(row,config)
