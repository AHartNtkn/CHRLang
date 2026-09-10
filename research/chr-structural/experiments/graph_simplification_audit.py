"""Verify isolated graph/source confirmation artifacts against their frozen inputs."""
import hashlib,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
BASE=ROOT/'docs/experiments/results/s06-graph-simplification/isolated-confirmation'
f=json.loads((BASE/'freeze.json').read_text())
for p,h in f['sources'].items():assert hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h,p
for b in f['binaries']:assert hashlib.sha256((ROOT/b['path']).read_bytes()).hexdigest()==b['sha256'],b['path']
r=json.loads((BASE/'audit.json').read_text())
assert {(x['profile'],x['test']) for x in r}=={(p,t) for p in ['debug','release'] for t in ['graph_simplification','diagram_source']}
for row in r:
 assert row['exit_code']==0
 text=(BASE/f"{row['profile']}-{row['test']}.log").read_text()
 assert row['checks'] in text and '2 passed; 0 failed' in text
print(json.dumps({'confirmed_executables':len(r),'graph_projections_per_build':3072,'union_projections_per_build':128,'source_configurations_per_build':360,'candidate_set_checks_per_build':1800,'frozen_sources':len(f['sources'])},indent=2))
