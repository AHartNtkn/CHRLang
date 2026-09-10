"""Build, freeze, and run the independent finite-denotation gate with hard bounds."""
import hashlib
import json
import os
from pathlib import Path
import resource
import subprocess

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s06-reusable-diagram'

def limits():
    resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
    resource.setrlimit(resource.RLIMIT_AS, (1024**3, 1024**3))


def main():
    if (OUT / 'freeze.json').exists():
        raise RuntimeError('confirmation already frozen; use a new registered output location')
    binaries = []
    for profile in ['debug', 'release']:
        command = ['cargo', 'test', '-p', 'chr-structural', '--test', 'reusable_diagram', '--no-run', '--message-format=json']
        if profile == 'release':
            command.append('--release')
        built = subprocess.run(command, cwd=ROOT, text=True, capture_output=True, timeout=300)
        (OUT / f'build-{profile}.jsonl').write_text(built.stdout)
        (OUT / f'build-{profile}.log').write_text(built.stderr)
        built.check_returncode()
        artifacts = [json.loads(line) for line in built.stdout.splitlines() if line.startswith('{')]
        paths = [a['executable'] for a in artifacts if a.get('reason') == 'compiler-artifact' and a.get('executable') and a['target']['name'] == 'reusable_diagram']
        assert len(paths) == 1
        path = Path(paths[0])
        binaries.append({'profile': profile, 'path': str(path.relative_to(ROOT)), 'sha256': hashlib.sha256(path.read_bytes()).hexdigest()})
    paths = list((ROOT / 'research/chr-structural/src').glob('*.rs')) + [ROOT / p for p in [
        'research/chr-structural/tests/reusable_diagram.rs',
        'research/chr-structural/experiments/reusable_diagram_gate.py',
        'research/chr-structural/Cargo.toml', 'Cargo.lock', 'Cargo.toml',
        'docs/experiments/registrations/S06-reusable-diagram.md']]
    freeze = {'sources': {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}, 'binaries': binaries,
              'rustc': subprocess.check_output(['rustc', '-Vv'], text=True), 'platform': list(os.uname())}
    (OUT / 'freeze.json').write_text(json.dumps(freeze, indent=2)+'\n')
    results = []
    for binary in binaries:
        run = subprocess.run([str(ROOT / binary['path']), '--nocapture'], cwd=ROOT, text=True, capture_output=True, timeout=60, preexec_fn=limits)
        (OUT / f"{binary['profile']}-confirmation.log").write_text(run.stdout + run.stderr)
        run.check_returncode()
        assert '2 passed; 0 failed' in run.stdout
        checks = int(run.stdout.split('independent_denotation_checks=')[1].splitlines()[0])
        results.append({'profile': binary['profile'], 'checks': checks, 'exit_code': run.returncode})
    assert results[0]['checks'] == results[1]['checks']
    (OUT / 'audit.json').write_text(json.dumps(results, indent=2)+'\n')
    print(json.dumps(results))

if __name__ == '__main__':
    main()
