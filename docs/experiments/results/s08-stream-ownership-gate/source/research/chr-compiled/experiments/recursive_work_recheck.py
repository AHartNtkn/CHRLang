#!/usr/bin/env python3
"""Verify the runtime configuration-guard cleanup leaves all work evidence identical."""
from pathlib import Path
import json,hashlib,subprocess
import recursive_access_confirmation as comparison
OUT=comparison.OUT/'work-recheck'
BIN=Path('/tmp/chr-recursive-access-work-guard')
def main():
    OUT.mkdir(exist_ok=False)
    source=comparison.ROOT/'research/chr-compiled/examples/recursive_work.rs'
    freeze=dict(source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),binaries={})
    (OUT/'recursive_work.rs').write_bytes(source.read_bytes())
    for build in ['off','on']:
        p=BIN/build/'work';freeze['binaries'][str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
    (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    for i,c in enumerate(comparison.CONFIGS):
        for j,(b,m) in enumerate(comparison.BUILDS):
            index=i*10+j;command=comparison.cmd(b,m,c,'work');command[0]=str(BIN/b/'work')
            p=subprocess.run(command,capture_output=True,text=True,timeout=60,preexec_fn=comparison.pilot.limits)
            raw=dict(command=command,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
            (OUT/f'work-{index:03}.json').write_text(json.dumps(raw,indent=2)+'\n')
            assert p.returncode==0
            before=json.loads((comparison.OUT/f'work-{index:03}.json').read_text())
            assert json.loads(before['stdout'])==json.loads(p.stdout)
    (OUT/'validation.json').write_text(json.dumps(dict(processes=800,work_payloads_identical=True,change='runtime feature guard uses conditional panic instead of constant assert; no runtime or timing-runner change'),indent=2)+'\n')
    print('All 800 work payloads exactly match the registered comparison.')
if __name__=='__main__':main()
