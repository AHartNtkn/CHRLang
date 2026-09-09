"""Validate phase allocation ownership separately from ordinary timing."""
import json
import os
import subprocess
import artifact_boundary as gate
from artifact_sizing import validate

ROOT = gate.ROOT
OUT = ROOT / 'docs/experiments/results/s01-artifact-allocation-gate'
TARGET = ROOT / 'target/s01-artifact-allocation-gate'


def main():
    assert not OUT.exists(), 'Preserve existing receipts'
    OUT.mkdir(parents=True)
    TARGET.mkdir(parents=True, exist_ok=True)
    gate.OUT = OUT
    modes = ['generic', 'planned', 'specialized', 'native', 'native-generic-repair', 'native-planned', 'native-specialized']
    artifacts = {}
    checked = 0
    for build, feature in [('ordinary', 'experiment'), ('metered', 'alloc-meter')]:
        env = os.environ.copy()
        env['CARGO_TARGET_DIR'] = str(TARGET / build)
        gate.run('build-' + build, ['cargo', 'build', '--offline', '--release', '-p', 'chr-compiled',
                 '--no-default-features', '--features', feature, '--lib', '--bin', 'chr-access-emit'], 4 << 30, env)
        release = TARGET / build / 'release'
        for family in ['generic', 'payload', 'subscription']:
            source = TARGET / (family + '-' + build + '.rs')
            binary = TARGET / (family + '-' + build)
            gate.run('emit-' + binary.name, [release / 'chr-access-emit', family, source], 1 << 30)
            gate.run('compile-' + binary.name, ['rustc', '--edition', '2024', '-C', 'opt-level=3', '-C', 'codegen-units=1', source,
                '--extern', 'chr_compiled=' + str(release / 'libchr_compiled.rlib'), '-L', 'dependency=' + str(release / 'deps'), '-o', binary], 4 << 30)
            artifacts[binary.name] = dict(source_sha256=gate.digest(source), binary_sha256=gate.digest(binary))
        for family in ['payload', 'subscription']:
            for mode in modes + (['retained-indexed', 'retained-eager', 'retained-subscribed'] if family == 'subscription' else []):
                for size in [0, 16]:
                    traffic = []
                    for repetition in range(2):
                        binary = TARGET / ((family if mode.startswith('native') else 'generic') + '-' + build)
                        result = gate.run(f'run-{build}-{family}-{mode}-{size}-{repetition}', [binary, mode, family, size, 4], 1 << 30)
                        rows = [json.loads(line) for line in result['stdout'].splitlines()]
                        assert rows[-1]['allocator'] == ('ordinary' if build == 'ordinary' else 'requested-meter')
                        if build == 'metered':
                            assert rows[-1]['prepared_restored'] is True
                            assert all(row['query_restored'] is True for row in rows[:-1])
                            for row in rows:
                                for phase in row['memory'].values():
                                    assert phase['peak_live'] >= max(phase['live_start'], phase['live_end'])
                                    assert all(type(value) is int and value >= 0 for value in phase.values())
                            traffic.append([row['memory'] for row in rows])
                        else:
                            assert all('memory' not in row for row in rows)
                        # Reuse the timing-sum validator after separately verifying the real allocator label.
                        rows[-1]['allocator'] = 'ordinary'
                        sanitized = dict(result, stdout='\n'.join(json.dumps(row) for row in rows))
                        validate(sanitized, family, mode, size, 4)
                        checked += 4
                    if build == 'metered':
                        assert traffic[0] == traffic[1], (family, mode, size, 'allocation replay differs')
            print('validated', build, family, flush=True)
    assert checked == 544
    result = dict(parent_commit=subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
                  processes=136, complete_queries=checked, metered_replay_cells=34, artifacts=artifacts,
                  interpretation='Allocation accounting gate; no metered timing ranking')
    (OUT / 'summary.json').write_text(json.dumps(result, indent=2) + '\n')


if __name__ == '__main__':
    main()
