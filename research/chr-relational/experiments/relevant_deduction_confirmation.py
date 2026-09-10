"""Freeze and confirm the registered reachable-read deduction source gate."""
import hashlib
import json
from pathlib import Path
import resource
import subprocess
ROOT=Path.cwd()
BASE=ROOT/'docs/experiments/results/s02-relevant-deductions'
OUT=BASE/'confirmation'
OUT.mkdir(exist_ok=False)
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
executables=[]
for mode in ['on','off']:
    for line in (BASE/f'build-{mode}.jsonl').read_text().splitlines():
        r=json.loads(line)
        if r.get('executable') and r.get('profile',{}).get('test'):
            p=Path(r['executable']);executables.append(dict(mode=mode,name=r['target']['name'],path=str(p.relative_to(ROOT)),sha256=digest(p)))
paths=subprocess.check_output(['git','ls-files','crates','research','Cargo.toml','Cargo.lock'],text=True).splitlines()
paths=[p for p in paths if p.endswith(('.rs','.toml','.lock'))]
paths+=['research/chr-relational/tests/relevant_deductions.rs','research/chr-relational/experiments/relevant_deduction_confirmation.py','docs/experiments/registrations/S02-relevant-deductions.md']
sources={p:digest(ROOT/p) for p in paths}
(OUT/'freeze.json').write_text(json.dumps(dict(sources=sources,executables=executables),indent=2)+'\n')
def bounds():
    resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
results=[]
rows=[]
for e in executables:
    for repeat in range(2 if e['name']=='relevant_deductions' else 1):
        assert digest(ROOT/e['path'])==e['sha256']
        r=subprocess.run([str(ROOT/e['path']),'--test-threads=1','--nocapture'],capture_output=True,text=True,timeout=60,preexec_fn=bounds)
        log=f"{e['mode']}-{e['name']}-{repeat}.log"
        (OUT/log).write_text(r.stdout+r.stderr)
        results.append(dict(mode=e['mode'],name=e['name'],repeat=repeat,returncode=r.returncode,log=log))
        (OUT/'results.json').write_text(json.dumps(results,indent=2)+'\n')
        print(log,r.returncode,flush=True)
        if e['mode']=='on' and e['name']=='relevant_deductions':
            rows.append([x[x.index('READ_REUSE,'):] for x in r.stdout.splitlines() if 'READ_REUSE,' in x])
assert all(r['returncode']==0 for r in results)
assert len(rows)==2 and len(rows[0])==144 and rows[0]==rows[1]
assert all(digest(ROOT/p)==sha for p,sha in sources.items())
(OUT/'reuse.csv').write_text('family,depth,resource,reverse,mode,sampled_hits\n'+'\n'.join(x.split(',',1)[1] for x in rows[0])+'\n')
