"""Run/replay E16's finite regional semantic matrix, with independent key audit."""
import csv
import hashlib
import json
import os
from pathlib import Path
import platform
import resource
import subprocess
import time
OUT=Path('docs/experiments/results')
PREFIX='E16-regions-gate-v2'
COMMAND=['target/release/examples/parallel_regions_gate']
if any((OUT/f'{PREFIX}{suffix}').exists() for suffix in ['-manifest.json','.tsv','-replay.tsv']):
    raise SystemExit('Evidence already exists; use a separately recorded retry prefix')
paths=[Path(COMMAND[0]),Path('Cargo.toml'),Path('Cargo.lock'),Path(__file__),Path('docs/experiments/registrations/E16-regions.md')]
for folder in ['research/chr-factors','research/chr-persistent','research/chr-cases','research/chr-observe','crates/chr-reference','crates/chr-syntax','crates/chr-programs']:
    paths += [p for p in Path(folder).rglob('*') if p.is_file() and p.suffix in ['.rs','.toml']]
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def bounds():resource.setrlimit(resource.RLIMIT_AS,(1073741824,1073741824))
manifest={'purpose':'regional semantic gate, no timing inference','created_utc':time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime()),
    'platform':platform.platform(),'affinity':sorted(os.sched_getaffinity(0)),
    'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'timeout_seconds':300,'rlimit_as_bytes':1073741824,
    'command':COMMAND,'sha256':{str(p):digest(p) for p in sorted(set(paths))}}
(OUT/f'{PREFIX}-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
for suffix in ['', '-replay']:
    with (OUT/f'{PREFIX}{suffix}.tsv').open('w') as stdout,(OUT/f'{PREFIX}{suffix}.stderr').open('w') as stderr:
        subprocess.run(COMMAND,stdout=stdout,stderr=stderr,timeout=300,preexec_fn=bounds,check=True)
    print(suffix or 'initial','pass',flush=True)
a,b=[list(csv.DictReader((OUT/f'{PREFIX}{s}.tsv').open(),delimiter='\t')) for s in ['', '-replay']]
original=list(csv.DictReader((OUT/'E08-factors.tsv').open(),delimiter='\t'))
cases={r['case'] for r in original};assert len(cases)==103
configs={('Baseline','1','0')}|{(m,q,'4') for m in ['Inline','Threads(1)','Threads(2)'] for q in ['1','8']}
expected={(c,m,q,k) for c in cases for m,q,k in configs}
def key(r):return (r['case'],r['mode'],r['quantum'],r['limit'])
assert len(a)==len(b)==len(expected)==721
assert {key(r) for r in a}=={key(r) for r in b}==expected
assert [key(r) for r in a]==[key(r) for r in b]
variable={'owner_buffered_peak','actual_source_steps','cancelled_requests','unserved_quantum_slots'}
for r in a+b:
    assert len(r)==28 and None not in r and all(v is not None for v in r.values())
    assert r['status']=='pass'
    assert int(r['actual_source_steps'])>=int(r['accepted_source_steps'])
    if r['mode']!='Baseline':
        assert int(r['issued'])==int(r['received'])
        assert int(r['issued'])-int(r['accepted'])==int(r['unaccepted_at_shutdown'])
        assert int(r['outstanding'])==int(r['buffered'])==0
        assert int(r['max_outstanding'])<=int(r['limit'])
for x,y in zip(a,b):
    assert {k:v for k,v in x.items() if k not in variable}=={k:v for k,v in y.items() if k not in variable}, key(x)
logical=['factors','answers','raw_products','owner_turns','source_turns','applications','products','product_jobs','duplicates','refutations','accepted_source_steps']
for case in cases:
    for q in ['1','8']:
        group=[r for r in a if r['case']==case and r['quantum']==q]
        assert len({tuple(r[k] for k in logical) for r in group})==1,(case,q)
assert all(digest(Path(p))==value for p,value in manifest['sha256'].items())
summary={'status':'pass','rows_per_run':721,'cases':103,'configurations_per_case':7,
    'independent_reference_cases':62,'replay':'all logical and deterministic fields exact',
    'variable_columns':sorted(variable),'changed_rows':{k:sum(x[k]!=y[k] for x,y in zip(a,b)) for k in sorted(variable)},
    'max_outstanding':max(int(r['max_outstanding']) for r in a),
    'max_unaccepted_at_shutdown':max(int(r['unaccepted_at_shutdown']) for r in a),
    'sha256':{f:digest(OUT/f) for f in [f'{PREFIX}.tsv',f'{PREFIX}-replay.tsv',f'{PREFIX}-manifest.json']}}
(OUT/f'{PREFIX}-audit.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
