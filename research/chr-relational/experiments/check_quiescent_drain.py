"""Reject unbounded drain and draining across pending source effects."""
import hashlib
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s02-quiescent-drain'
SOURCE = ROOT / 'research/chr-relational/src/execute.rs'


def main():
    original = SOURCE.read_bytes()
    source = original.decode()
    mutations = {
        'unbounded': ('while used < budget && state.store.step() {', 'while state.store.step() {'),
        'pending-effects': ('if let Some(effect) = state.pending.pop() {', '''if reads.is_some() {
            while used < budget && state.store.step() { used += 1; }
        }
        if let Some(effect) = state.pending.pop() {'''),
    }
    receipts = []
    try:
        for name, (before, after) in mutations.items():
            assert source.count(before) == 1
            SOURCE.write_text(source.replace(before, after))
            command = ['cargo', 'test', '-p', 'chr-relational', '--lib',
                       'bounded_drain_preserves_priority_and_avoids_repeated_idle_advances', '--', '--nocapture']
            result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, timeout=60)
            log = result.stdout + result.stderr
            (OUT / f'mutation-{name}.log').write_text(log.rstrip() + '\n')
            assert result.returncode == 101 and 'test result: FAILED' in log and 'error[E' not in log
            expected = 'deduction budget exceeded' if name == 'unbounded' else 'do not drain across pending source failure'
            assert expected in log
            receipts.append(dict(mutation=name, command=command, exit_code=result.returncode, assertion=expected))
    finally:
        SOURCE.write_bytes(original)
    (OUT / 'mutation-audit.json').write_text(json.dumps(dict(
        source_restored_sha256=hashlib.sha256(original).hexdigest(), rejected=receipts), indent=2) + '\n')
    print('Both scheduling mutations rejected; source restored exactly')


if __name__ == '__main__':
    main()
