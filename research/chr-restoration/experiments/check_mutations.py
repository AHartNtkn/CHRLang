#!/usr/bin/env python3
"""Diagnostic source mutations; run with no concurrent cargo/source editing.

Each mutation skips one kind of inverse edit. Release source tests must fail
at runtime. The exact original source is restored even if a run fails.
"""
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT / 'research/chr-restoration/src/lib.rs'
OUT = ROOT / 'docs/experiments/results/s04-source-gate'
original = SOURCE.read_bytes()
needle = '    fn apply(&self, s: &mut State, forward: bool) {\n'
mutants = {
    'binding': 'Self::Binding(..)',
    'resource': 'Self::Live(..)',
    'history': 'Self::History(..)',
    'pending': 'Self::Push(..) | Self::Pop(..)',
}
records = []
try:
    assert original.decode().count(needle) == 1
    for name, variant in mutants.items():
        replacement = needle + f'        if !forward && matches!(self, {variant}) {{ return; }}\n'
        mutated = original.decode().replace(needle, replacement)
        SOURCE.write_text(mutated)
        command = ['cargo', 'test', '-p', 'chr-restoration', '--release', '--test', 'source']
        result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, timeout=60)
        output = (result.stdout + result.stderr).rstrip() + '\n'
        (OUT / f'mutation-{name}.log').write_text(output)
        killed = result.returncode != 0 and 'test result: FAILED.' in output and 'could not compile' not in output
        records.append({'mutation': name, 'omitted_inverse': variant, 'command': command,
                        'exit_code': result.returncode, 'runtime_failure': killed,
                        'mutated_source_sha256': hashlib.sha256(mutated.encode()).hexdigest()})
        print(name, 'detected' if killed else 'NOT DETECTED', flush=True)
        assert killed, output
finally:
    SOURCE.write_bytes(original)
    (OUT / 'mutations.json').write_text(json.dumps({
        'source_sha256': hashlib.sha256(original).hexdigest(),
        'restored_exactly': SOURCE.read_bytes() == original,
        'records': records,
    }, indent=2) + '\n')
