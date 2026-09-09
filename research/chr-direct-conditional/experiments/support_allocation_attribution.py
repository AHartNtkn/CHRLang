#!/usr/bin/env python3
import hashlib
import itertools
import json
from pathlib import Path
import resource
import subprocess

out=Path('docs/experiments/results/s08-support-allocation-attribution')
binary=Path('target/s08-support-attribution/stream').resolve()

def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(120,120))

def main():
    out.mkdir(exist_ok=False)
    files=[Path(__file__),Path('Cargo.lock'),Path('docs/experiments/registrations/S08-support-allocation-attribution.md')]
    for directory in ['research/chr-direct-conditional','research/chr-reuse','research/chr-compiled','crates/chr-syntax']:
        files.extend(Path(directory).rglob('*.rs'));files.append(Path(directory)/'Cargo.toml')
    freeze={str(p):digest(p) for p in files+[binary]}
    (out/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    (out/'toolchain.txt').write_text(subprocess.check_output(['rustc','-Vv'],text=True))
    cells=list(itertools.product(['conditional','inferred'],['aliases','distinct'],[16,64]))
    prior={}
    for rep in range(2):
        for i,c in enumerate(cells):
            run=subprocess.run([str(binary),*map(str,c)],capture_output=True,text=True,timeout=150,preexec_fn=limits)
            (out/f'{rep}-{i}.json').write_text(json.dumps(dict(command=[str(binary),*map(str,c)],returncode=run.returncode,stdout=run.stdout,stderr=run.stderr),indent=2)+'\n')
            assert run.returncode==0,(rep,c)
            d=json.loads(run.stdout.splitlines()[-1]);
            if rep:assert prior[c]==d,c
            else:prior[c]=d
    assert all(digest(Path(p))==h for p,h in freeze.items())
    # Re-read all raw receipts independently of the in-memory replay comparisons.
    summary=[]
    for i,c in enumerate(cells):
        a=json.loads(json.loads((out/f'0-{i}.json').read_text())['stdout'].splitlines()[-1])
        b=json.loads(json.loads((out/f'1-{i}.json').read_text())['stdout'].splitlines()[-1]);assert a==b
        assert (a['mode'],a['family'],a['depth'])==c
        assert sum(s[1] for s in a['stages'])==a['execution']['requested_bytes']
        assert sum(s[2] for s in a['stages'])==a['execution']['allocation_calls']
        assert a['restored']['live_start']==a['restored']['live_end']
        expected=[x for x in [1,4,16,64] if x<=c[2]+1]+[c[2]+1]
        assert [s['answers'] for s in a['snapshots']]==expected
        prior=json.loads(Path('docs/experiments/results/s08-stream-allocation-attribution/summary.json').read_text())
        old=next(d for d in prior if (d['mode'],d['family'],d['depth'])==c)
        assert a['stages']==old['stages']
        assert a['snapshots']==old['snapshots']
        assert all(sum(domain)<=stage[1] for domain,stage in zip(a['support_bytes'],a['stages']))
        summary.append(a)
    (out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    (out/'audit.json').write_text(json.dumps(dict(processes=16,exact_pairs=8,stage_sums=True,prior_stage_and_owner_equality=True,domain_bounds=True,owner_baselines=True,hashes=True,peak_interpretation='nested meter resets: no peak claim',timing='not_run'),indent=2)+'\n')
    for d in summary:
        print(d['mode'],d['family'],d['depth'],[s[1] for s in d['stages']],d['support_bytes'],flush=True)

if __name__=='__main__':main()
