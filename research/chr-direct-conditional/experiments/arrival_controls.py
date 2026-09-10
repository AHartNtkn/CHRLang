#!/usr/bin/env python3
"""Bounded source/work gate; no timing comparisons."""
import hashlib,json,resource,subprocess,re
from pathlib import Path
R=Path('docs/experiments/results/s10-arrival-controls')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
 resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
R.mkdir(exist_ok=False)
files=[Path(__file__),Path('Cargo.lock'),Path('Cargo.toml'),Path('docs/experiments/registrations/S10-arrival-controls.md')]
for d in ['research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','crates/chr-syntax']:
 files.extend(Path(d).rglob('*.rs'));files.append(Path(d)/'Cargo.toml')
freeze={str(p):sha(p) for p in files}
for kind in ['work','off']:
 cmd=['cargo','build','--release','-p','chr-direct-conditional','--example','arrival_controls','--no-default-features','--features','experiment'+(',metrics' if kind=='work' else '')]
 run=subprocess.run(cmd,text=True,capture_output=True);(R/f'{kind}-build.json').write_text(json.dumps(dict(command=cmd,returncode=run.returncode,stdout=run.stdout,stderr=run.stderr)));assert run.returncode==0
 binary=Path(f'target/arrival-controls-{kind}').resolve();binary.write_bytes(Path('target/release/examples/arrival_controls').read_bytes());binary.chmod(0o755);freeze[str(binary)]=sha(binary)
(R/'freeze.json').write_text(json.dumps(freeze,indent=2))
(R/'toolchain.txt').write_text(subprocess.check_output(['rustc','-Vv'],text=True))
outputs=[]
for label in ['work1','work2','off']:
 cmd=[str(Path('target/arrival-controls-'+('off' if label=='off' else 'work')).resolve())]
 run=subprocess.run(cmd,text=True,capture_output=True,timeout=75,preexec_fn=limits)
 (R/f'{label}.json').write_text(json.dumps(dict(command=cmd,returncode=run.returncode,stdout=run.stdout,stderr=run.stderr)));assert run.returncode==0
 outputs.append(run.stdout)
assert outputs[0]==outputs[1]
def endpoints(s):return re.sub(r'source: \d+, applications: \d+, dispatch: \d+, specialized: \d+', 'COUNTERS',s)
assert endpoints(outputs[0])==endpoints(outputs[2])
rows=[line for line in outputs[0].splitlines() if line.startswith('work ')]
assert len(rows)==376,len(rows) #64 arrival*4 +48 stream*2 +6 probes*4
assert len([x for x in outputs[0].splitlines() if x.startswith('reused ')])==4
assert all(sha(Path(p))==h for p,h in freeze.items())
(R/'completion.json').write_text(json.dumps(dict(regular_sources=112,resource_alias_probes=6,reused_queries=4,work_rows_per_run=376,exact_diagnostic_runs=2,counter_free_endpoint_agreement=True))+'\n')
print('112 source configurations,6 resource/alias probes,4 reused queries: exact diagnostic replay and counter-free endpoint agreement pass')
