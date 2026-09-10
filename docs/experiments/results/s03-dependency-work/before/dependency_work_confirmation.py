"""Freeze and run the registered dependency work screen; never overwrite receipts."""
import hashlib
import json
from pathlib import Path
import resource
import subprocess

ROOT = Path.cwd()
BASE = ROOT / 'docs/experiments/results/s03-dependency-work'
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
paths += ['research/chr-direct-conditional/tests/dependency_work.rs', 'research/chr-direct-conditional/experiments/dependency_work_confirmation.py', 'docs/experiments/registrations/S03-dependency-work.md']
sources = {p: digest(ROOT / p) for p in paths}
(OUT / 'freeze.json').write_text(json.dumps(dict(sources=sources, executables=executables), indent=2) + '\n')
def bounds():
    resource.setrlimit(resource.RLIMIT_AS, (1024**3, 1024**3))
    resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
results = []
rows = []
for exe in executables:
    for repeat in range(2 if exe['name'] == 'dependency_work' else 1):
        assert digest(ROOT / exe['path']) == exe['sha256']
        run = subprocess.run([str(ROOT / exe['path']), '--test-threads=1', '--nocapture'], capture_output=True, text=True, timeout=60, preexec_fn=bounds)
        log = f"{exe['mode']}-{exe['name']}-{repeat}.log"
        (OUT / log).write_text(run.stdout + run.stderr)
        results.append(dict(mode=exe['mode'], name=exe['name'], repeat=repeat, returncode=run.returncode, log=log))
        (OUT / 'results.json').write_text(json.dumps(results, indent=2) + '\n')
        print(log, run.returncode, flush=True)
        if exe['name'] == 'dependency_work':
            rows.append([line for line in run.stdout.splitlines() if line.startswith('DEPENDENCY,')])
assert all(r['returncode'] == 0 for r in results)
assert len(rows) == 2 and len(rows[0]) == 180 and rows[0] == rows[1]
assert all(digest(ROOT / p) == sha for p, sha in sources.items())
(OUT / 'rows.csv').write_text('kind,size,reverse,reuse,query,answers,ticks,force,match,validation_passes,validation_entries\n' + '\n'.join(line.split(',', 1)[1] for line in rows[0]) + '\n')
