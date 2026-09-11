"""Quiet dependency/template work attribution, including the passive-marker control."""
import hashlib,itertools,json
from pathlib import Path
from continuing_lifecycle_entry import ROOT,BASE,invoke

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    assert not (BASE/'graph-freeze.json').exists()
    binary=ROOT/'target/s08-continuing-graph-diagnostic/release/examples/continuing_graph_attribution'
    source=ROOT/'research/chr-reuse/examples/continuing_graph_attribution.rs'
    jobs=[dict(mode=m,resource=r,marker=q,demand=128,rep=i) for m,r,q,i in itertools.product(['dependencies','templates'],[False,True],[False,True],range(2))]
    jobs += [dict(mode=m,resource=r,marker=True,demand=512,rep=0) for m,r in itertools.product(['dependencies','templates'],[False,True])]
    assert len(jobs)==20
    (BASE/'graph-freeze.json').write_text(json.dumps(dict(binary=dict(path=str(binary),sha256=sha(binary)),source_sha256=sha(source),registration_sha256=sha(ROOT/'docs/experiments/registrations/S08-continuing-graph-attribution.md'),runner_sha256=sha(Path(__file__)),jobs=jobs),indent=2))
    for i,j in enumerate(jobs):
        raw=invoke([str(binary)]+[str(j[k]).lower() for k in ['mode','resource','marker','demand']],120 if j['demand']==512 else 60)
        (BASE/f'graph-{i}.json').write_text(json.dumps(dict(job=j,raw=raw),indent=2));print(i,j,raw['exit_code'],flush=True)
if __name__=='__main__':main()
