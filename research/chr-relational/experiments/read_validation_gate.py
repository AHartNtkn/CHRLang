"""Run qualified read-validation tests under explicit runtime bounds and freeze inputs."""
import hashlib
import json
from pathlib import Path
import resource
import subprocess

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s02-read-validation'

def limits():
    resource.setrlimit(resource.RLIMIT_CPU, (120, 120))
    resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

receipts = []
for kind, features in [('primary', []), ('diagnostic', ['deduction-work,deduction-profile'])]:
    command = ['cargo', 'test', '-p', 'chr-relational', '--test', 'relevant_deductions', '--no-run', '--message-format=json', '--target-dir', 'target/s02-read-validation']
    if features:
        command += ['--features', features[0]]
    build = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, timeout=300)
    (OUT / f'bounded-build-{kind}.log').write_text(build.stderr)
    assert build.returncode == 0, build.stderr
    binary = next(json.loads(line)['executable'] for line in build.stdout.splitlines() if json.loads(line).get('executable'))
    run = subprocess.run([binary, '--nocapture'], cwd=ROOT, capture_output=True, text=True, timeout=120, preexec_fn=limits)
    (OUT / f'bounded-{kind}.log').write_text(run.stdout + run.stderr)
    assert run.returncode == 0, run.stderr
    receipts.append({'kind': kind, 'build_command': command, 'binary': binary, 'sha256': hashlib.sha256(Path(binary).read_bytes()).hexdigest(), 'exit_code': run.returncode, 'cpu_seconds_limit': 120, 'wall_seconds_limit': 120, 'address_space_limit': 1 << 30})
files = ['research/chr-relational/src/contextual.rs', 'research/chr-relational/src/contextual_execute.rs', 'research/chr-relational/tests/relevant_deductions.rs', 'research/chr-relational/examples/support/deduction_source.rs', 'research/chr-direct-conditional/tests/runtime_support/mod.rs', 'docs/experiments/registrations/S02-read-validation.md', 'research/chr-relational/experiments/read_validation_gate.py', 'research/chr-relational/experiments/read_validation_mutation.py']
(OUT / 'gate.json').write_text(json.dumps({'receipts': receipts, 'source_hashes': {p: hashlib.sha256((ROOT / p).read_bytes()).hexdigest() for p in files}}, indent=2) + '\n')
print('Both bounded builds preserve complete answers and diagnostic recognition checks.')
