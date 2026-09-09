"""Validate retained matching at the common source artifact boundary."""
import json
import os
import subprocess
import artifact_boundary as gate
from artifact_sizing import validate

ROOT = gate.ROOT
OUT = ROOT / 'docs/experiments/results/s01-retained-artifact-gate'
TARGET = ROOT / 'target/s01-retained-artifact-gate'


def main():
    assert not (OUT / 'runtime-build.json').exists(), 'Preserve existing run receipts'
    OUT.mkdir(exist_ok=True)
    TARGET.mkdir(parents=True, exist_ok=True)
    gate.OUT = OUT
    env = os.environ.copy()
    env['CARGO_TARGET_DIR'] = str(TARGET / 'runtime')
    env['RUSTFLAGS'] = '-Cembed-bitcode=yes'
    gate.run('runtime-build', ['cargo', 'build', '--offline', '--release', '-p', 'chr-compiled',
        '--no-default-features', '--features', 'experiment', '--lib', '--bin', 'chr-access-emit'], 4 << 30, env)
    release = TARGET / 'runtime/release'
    shared = {str(p.relative_to(ROOT)): gate.digest(p) for p in sorted((release / 'deps').glob('*.rlib'))}
    shared[str((release / 'libchr_compiled.rlib').relative_to(ROOT))] = gate.digest(release / 'libchr_compiled.rlib')
    modes = ['generic', 'planned', 'specialized', 'native', 'native-generic-repair', 'native-planned',
             'native-specialized', 'retained-indexed', 'retained-eager', 'retained-subscribed']
    artifacts = {}
    checked = 0
    for family in ['generic', 'subscription']:
        source = TARGET / (family + '.rs')
        gate.run('emit-' + family, [release / 'chr-access-emit', family, source], 1 << 30)
        for opt in ['off', 'thin']:
            binary = TARGET / (family + '-' + opt)
            gate.run('compile-' + binary.name, ['rustc', '--edition', '2024', '-C', 'opt-level=3',
                '-C', 'codegen-units=1', '-C', 'lto=' + opt, source, '--extern',
                'chr_compiled=' + str(release / 'libchr_compiled.rlib'), '-L',
                'dependency=' + str(release / 'deps'), '-o', binary], 4 << 30)
            artifacts[binary.name] = dict(source_sha256=gate.digest(source), binary_sha256=gate.digest(binary))
    for opt in ['off', 'thin']:
        for mode in modes:
            for size in [0, 8, 32]:
                binary = TARGET / (('subscription' if mode.startswith('native') else 'generic') + '-' + opt)
                result = gate.run(f'run-{opt}-{mode}-{size}', [binary, mode, 'subscription', size, 4], 1 << 30)
                validate(result, 'subscription', mode, size, 4)
                checked += 4
        for name, binary, mode, family, reason in [
            ('unsupported', 'generic', 'retained-eager', 'chain', 'requires subscription'),
            ('native-mode', 'subscription', 'retained-eager', 'subscription', 'execution mode disagree'),
            ('source-mismatch', 'subscription', 'native', 'chain', 'does not match supplied rules')]:
            result = gate.run(name + '-' + opt, [TARGET / (binary + '-' + opt), mode, family, 0, 1], 1 << 30, expected=2)
            assert not result['stdout'] and reason in result['stderr']
        print('validated', opt, flush=True)
    assert checked == 240
    assert all(gate.digest(ROOT / p) == h for p, h in shared.items())
    summary = dict(parent_commit=subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
                   complete_queries=checked, processes=60, rejection_checks=6, shared_rlibs=shared,
                   artifacts=artifacts, rustc=subprocess.check_output(['rustc', '-Vv'], text=True),
                   interpretation='Retained source/control correctness; no comparative timing')
    (OUT / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')


if __name__ == '__main__':
    main()
