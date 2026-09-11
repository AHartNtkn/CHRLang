"""Freeze and execute the registered continuing resource stage probes."""
import hashlib,json
from pathlib import Path
from continuing_lifecycle_entry import ROOT,BASE,invoke

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    assert not (BASE/'diagnostic-freeze.json').exists()
    binary=ROOT/'target/s08-continuing-diagnostic/release/examples/continuing_resource_stages'
    source=ROOT/'research/chr-reuse/examples/continuing_resource_stages.rs'
    registration=ROOT/'docs/experiments/registrations/S08-continuing-resource-attribution.md'
    (BASE/'diagnostic-freeze.json').write_text(json.dumps(dict(binary=dict(path=str(binary),sha256=sha(binary)),source_sha256=sha(source),registration_sha256=sha(registration),runner_sha256=sha(Path(__file__)),parent_sha256=sha(BASE/'freeze.json'),cases=[[False,512],[True,512],[True,128],[True,128]]),indent=2))
    for i,(resource,target) in enumerate([(False,512),(True,512),(True,128),(True,128)]):
        raw=invoke([str(binary),str(resource).lower(),str(target)],120)
        (BASE/f'diagnostic-{i}.json').write_text(json.dumps(raw,indent=2));print(i,resource,target,raw['exit_code'],flush=True)
        # A bounded failure remains diagnostic evidence; finish independent probes.
if __name__=='__main__':main()
