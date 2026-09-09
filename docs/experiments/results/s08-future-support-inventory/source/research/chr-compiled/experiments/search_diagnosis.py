#!/usr/bin/env python3
"""Registered deterministic diagnostics; no timing comparisons."""
import hashlib,itertools,json,os,resource,subprocess,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/R03-search-diagnosis'
BIN=Path('/tmp/chr-t033/release/examples/search_diagnosis')
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(2*1024**3,)*2)
    resource.setrlimit(resource.RLIMIT_CPU,(20,20))
    resource.setrlimit(resource.RLIMIT_CORE,(0,0))
    os.sched_setaffinity(0,{min(os.sched_getaffinity(0))})
def main():
    files=[*sorted((ROOT/'research/chr-compiled').rglob('*.rs')),*sorted((ROOT/'research/chr-persistent').rglob('*.rs')),Path(__file__),ROOT/'docs/experiments/registrations/R03-search-diagnosis.md']
    freeze={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in files}
    freeze['binary']=hashlib.sha256(BIN.read_bytes()).hexdigest()
    (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    started=time.monotonic()
    with (OUT/'raw.jsonl').open('x') as out:
        for family,n,policy,access,execution in itertools.product(['chain','contradiction','weak'],[4,6,8],['global','active'],['scan','indexed'],['generated','generic']):
            previous=None
            for rep in range(2):
                assert time.monotonic()-started<600
                cmd=[str(BIN),family,str(n),policy,access,execution]
                result=subprocess.run(cmd,text=True,capture_output=True,timeout=30,preexec_fn=limits)
                record=dict(family=family,size=n,policy=policy,access=access,execution=execution,rep=rep,returncode=result.returncode,stdout=result.stdout,stderr=result.stderr)
                out.write(json.dumps(record)+'\n');out.flush()
                assert result.returncode==0,record
                if previous is not None:assert result.stdout==previous,record
                previous=result.stdout
            print(family,n,policy,access,execution,previous.strip(),flush=True)
if __name__=='__main__':main()
