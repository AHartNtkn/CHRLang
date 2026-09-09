"""Interrupted-query ownership and task-owned artifact filesystem lifetime."""
import json
import os
import subprocess
import time
import artifact_boundary as gate

ROOT = gate.ROOT
OUT = ROOT / 'docs/experiments/results/s01-cancellation-artifact-lifetime'
TARGET = ROOT / 'target/s01-cancellation-artifact-lifetime'


def validate(result, build, family, mode, size, budget):
    rows = [json.loads(line) for line in result['stdout'].splitlines()]
    assert len(rows) == 5 and result['exit_code'] == 0
    header = rows[-1]
    assert header['mode'] == mode and header['family'] == family and header['queries'] == 4
    assert header['cancelled_queries'] == 2 and header['counters'] is False
    assert header['allocator'] == ('ordinary' if build == 'ordinary' else 'requested-meter')
    for index, row in enumerate(rows[:-1]):
        assert row['query'] == index and row['size'] == size + index % 2
        phases = ['setup_ns', 'execute_ns', 'engine_drop_ns']
        if index % 2 == 0:
            assert row['cancelled'] is True and 'validated' not in row
            if budget == 0:
                assert row['exhausted_before_cancel'] is False
        else:
            assert row['validated'] is True and 'cancelled' not in row
            phases += ['observation_ns', 'answer_drop_ns']
        assert row['query_ns'] == sum(row[key] for key in phases)
    assert header['lifecycle_ns'] == sum(row['query_ns'] for row in rows[:-1]) + sum(header[k] for k in ['source_ns', 'prepare_ns', 'prepared_drop_ns'])
    if build == 'metered':
        assert header['prepared_restored'] is True
        assert all(row['query_restored'] is True for row in rows[:-1])
    return rows


def main():
    assert not OUT.exists(), 'Preserve existing receipts'
    OUT.mkdir(parents=True)
    TARGET.mkdir(parents=True, exist_ok=True)
    gate.OUT = OUT
    modes = ['generic', 'planned', 'specialized', 'native', 'native-generic-repair', 'native-planned', 'native-specialized']
    artifacts = {}
    owned = []
    checked = finished_before_cancel = 0
    for build, feature in [('ordinary', 'experiment'), ('metered', 'alloc-meter')]:
        env = os.environ.copy()
        env['CARGO_TARGET_DIR'] = str(TARGET / build)
        gate.run('build-' + build, ['cargo', 'build', '--offline', '--release', '-p', 'chr-compiled', '--no-default-features', '--features', feature, '--lib', '--bin', 'chr-access-emit'], 4 << 30, env)
        release = TARGET / build / 'release'
        for family in ['generic', 'payload', 'subscription']:
            source = TARGET / (family + '-' + build + '.rs')
            binary = TARGET / (family + '-' + build)
            gate.run('emit-' + binary.name, [release / 'chr-access-emit', family, source], 1 << 30)
            gate.run('compile-' + binary.name, ['rustc', '--edition', '2024', '-C', 'opt-level=3', '-C', 'codegen-units=1', source, '--extern', 'chr_compiled=' + str(release / 'libchr_compiled.rlib'), '-L', 'dependency=' + str(release / 'deps'), '-o', binary], 4 << 30)
            artifacts[binary.name] = dict(source_sha256=gate.digest(source), binary_sha256=gate.digest(binary))
            owned.extend([source, binary])
        for family in ['payload', 'subscription']:
            for mode in modes + (['retained-indexed', 'retained-eager', 'retained-subscribed'] if family == 'subscription' else []):
                for size in [0, 16]:
                    for budget in [0, 3]:
                        traffic = []
                        for repetition in range(2):
                            binary = TARGET / ((family if mode.startswith('native') else 'generic') + '-' + build)
                            result = gate.run(f'run-{build}-{family}-{mode}-{size}-{budget}-{repetition}', [binary, mode, family, size, 4, budget], 1 << 30)
                            rows = validate(result, build, family, mode, size, budget)
                            finished_before_cancel += sum(row.get('exhausted_before_cancel', False) for row in rows[:-1])
                            if build == 'metered':
                                traffic.append([row['memory'] for row in rows])
                            checked += 2
                        if build == 'metered':
                            assert traffic[0] == traffic[1], (family, mode, size, budget)
            print('validated', build, family, flush=True)
    assert checked == 544
    # All subprocess calls above have returned; only these newly created artifact files are owned here.
    disposals = []
    for path in owned:
        size = path.stat().st_size
        digest = gate.digest(path)
        start = time.perf_counter_ns()
        path.unlink()
        elapsed = time.perf_counter_ns() - start
        assert not path.exists()
        disposals.append(dict(path=str(path.relative_to(ROOT)), bytes=size, sha256=digest, unlink_ns=elapsed, absent=True))
    (OUT / 'artifact-disposal.json').write_text(json.dumps(disposals, indent=2) + '\n')
    summary = dict(parent_commit=subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
                   processes=272, completed_queries=checked, interrupted_lifetimes=checked,
                   exhausted_before_cancel=finished_before_cancel, metered_replay_cells=68,
                   artifacts=artifacts, artifact_files_disposed=len(disposals),
                   interpretation='Cancellation ownership and filesystem syscall accounting; no runtime ranking')
    (OUT / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')


if __name__ == '__main__':
    main()
