"""Bounded semantic entry runner. No performance inference from test duration."""
from pathlib import Path
import hashlib
import json
import re
import resource
import subprocess

ROOT = Path(__file__).resolve().parents[3]
RAW = ROOT / 'docs/experiments/results/s06-names-entry'

def binaries(path):
    result = {}
    for line in path.read_text().splitlines():
        r = json.loads(line)
        if r.get('reason') == 'compiler-artifact' and r.get('executable') and r.get('profile', {}).get('test'):
            result[r['target']['name']] = r['executable']
    return result

def limits():
    resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
    resource.setrlimit(resource.RLIMIT_CPU, (60, 60))

def run(label, binary):
    cmd = [binary, '--nocapture']
    p = subprocess.run(cmd, capture_output=True, text=True, preexec_fn=limits, timeout=60)
    with (RAW / f'{label}.json').open('x') as f:
        json.dump(dict(command=cmd, returncode=p.returncode, stdout=p.stdout, stderr=p.stderr), f, indent=2)
    assert p.returncode == 0, (label, p.stdout, p.stderr)
    print(label, re.findall(r'test result:.*', p.stdout), flush=True)

if __name__ == '__main__':
    default = binaries(RAW / 'final-build.jsonl')
    off = binaries(RAW / 'final-off-build.jsonl')
    lambda_binary = re.search(r'Running tests/lambda.rs \(([^)]+)\)', (RAW / 'lambda.log').read_text())[1]
    files = [ROOT / 'Cargo.lock', Path(__file__), ROOT / 'crates/chr-programs/src/lib.rs',
             ROOT / 'research/chr-cases/tests/lambda.rs', ROOT / 'research/chr-structural/Cargo.toml']
    files += list((ROOT / 'research/chr-structural/src').glob('*.rs'))
    files += list((ROOT / 'research/chr-structural/tests').glob('*.rs'))
    files += list((ROOT / 'crates/chr-reference/src').glob('*.rs'))
    files += [ROOT / 'docs/experiments/registrations/S06-names-entry.md', ROOT / 'docs/experiments/registrations/S06-names-source-wakeup.md']
    all_binaries = {f'default-{k}':v for k,v in default.items()} | {f'off-{k}':v for k,v in off.items()} | {'lambda': str(ROOT / lambda_binary)}
    freeze = {'inputs': {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in files},
              'binaries': {k: {'path':v, 'sha256':hashlib.sha256(Path(v).read_bytes()).hexdigest()} for k,v in all_binaries.items()}}
    with (RAW / 'final-freeze.json').open('x') as f:
        json.dump(freeze, f, indent=2)
    for mode, bs in [('default', default), ('off', off)]:
        for rep in range(2):
            run(f'final-{mode}-{rep}', bs['names'])
    for name, binary in default.items():
        if name != 'names': run(f'regression-{name}', binary)
    run('regression-lambda', str(ROOT / lambda_binary))
