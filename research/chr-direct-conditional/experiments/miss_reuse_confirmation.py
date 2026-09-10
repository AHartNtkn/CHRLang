"""Registered within-turn miss reuse: frozen source checks and causal work replay."""
import hashlib
import json
import os
from pathlib import Path
import resource
import subprocess

ROOT = Path.cwd()
BASE = ROOT / 'docs/experiments/results/s03-miss-reuse'
OUT = BASE / 'confirmation'
OUT.mkdir(exist_ok=False)
def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

executables = []
for mode in ['on', 'off']:
    for line in (BASE / f'build-{mode}.jsonl').read_text().splitlines():
        row = json.loads(line)
        if row.get('executable') and row.get('profile', {}).get('test'):
            path = Path(row['executable'])
            executables.append(dict(mode=mode, name=row['target']['name'], path=str(path.relative_to(ROOT)), sha256=digest(path)))
paths = subprocess.check_output(['git', 'ls-files', 'crates', 'research', 'Cargo.toml', 'Cargo.lock'], text=True).splitlines()
paths = [p for p in paths if p.endswith(('.rs', '.toml', '.lock'))]
paths += ['research/chr-direct-conditional/experiments/miss_reuse_confirmation.py', 'docs/experiments/registrations/S03-miss-reuse.md']
sources = {p: digest(ROOT / p) for p in paths}
(OUT / 'freeze.json').write_text(json.dumps(dict(sources=sources, executables=executables), indent=2) + '\n')
def bounds():
    resource.setrlimit(resource.RLIMIT_AS, (1024**3, 1024**3))
    resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
results = []
rows = {'off': [], 'on': []}
misses = {'off': [], 'on': []}
for exe in executables:
    is_work = exe['name'] == 'dependency_work'
    for memo in (['off', 'on'] if is_work else ['off']):
        for repeat in range(2 if is_work else 1):
            assert digest(ROOT / exe['path']) == exe['sha256']
            run = subprocess.run([str(ROOT / exe['path']), '--test-threads=1', '--nocapture'], env=dict(os.environ, MISS_REUSE=memo), capture_output=True, text=True, timeout=60, preexec_fn=bounds)
            log = f"{exe['mode']}-{exe['name']}-{memo}-{repeat}.log"
            (OUT / log).write_text(run.stdout + run.stderr)
            results.append(dict(mode=exe['mode'], name=exe['name'], memo=memo, repeat=repeat, returncode=run.returncode, log=log))
            (OUT / 'results.json').write_text(json.dumps(results, indent=2) + '\n')
            print(log, run.returncode, flush=True)
            if is_work:
                rows[memo].append([line.split(',', 1)[1] for line in run.stdout.splitlines() if line.startswith('DEPENDENCY,')])
                misses[memo].append([line.split(',', 1)[1] for line in run.stdout.splitlines() if line.startswith('MISS,')])
assert all(r['returncode'] == 0 for r in results)
for mode in ['off', 'on']:
    assert len(rows[mode]) == 2 and len(rows[mode][0]) == 180 and rows[mode][0] == rows[mode][1]
    assert misses[mode][0] == misses[mode][1] and len(misses[mode][0]) == 180
    (OUT / f'rows-{mode}.csv').write_text('kind,size,reverse,reuse,query,answers,ticks,force,match,validation_passes,validation_entries\n' + '\n'.join(rows[mode][0]) + '\n')
    (OUT / f'misses-{mode}.csv').write_text('kind,size,reverse,reuse,query,lookups,hits,inserts\n' + '\n'.join(misses[mode][0]) + '\n')
baseline = ROOT / 'docs/experiments/results/s03-dependency-work/qualified/rows.csv'
assert baseline.read_text().splitlines()[1:] == rows['off'][0]
assert all(digest(ROOT / p) == sha for p, sha in sources.items())
print('All rows repeat; disabled control exactly reproduces previous evidence.')
