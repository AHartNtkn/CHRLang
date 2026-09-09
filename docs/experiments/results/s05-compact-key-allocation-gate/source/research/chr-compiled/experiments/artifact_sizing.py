"""Bounded prospective lifecycle sizing. No comparative ranking."""
import hashlib
import json
import os
from pathlib import Path
import random
import resource
import subprocess
import time

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s01-lifecycle-sizing'
TARGET = ROOT / 'target/s01-link-optimization'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(name, command, limit):
    def bounded():
        resource.setrlimit(resource.RLIMIT_AS, (limit, limit))
    cpu = resource.getrusage(resource.RUSAGE_CHILDREN)
    start = time.monotonic()
    try:
        result = subprocess.run([str(x) for x in command], cwd=ROOT, capture_output=True,
                                text=True, timeout=60, preexec_fn=bounded)
        record = dict(exit_code=result.returncode, stdout=result.stdout, stderr=result.stderr)
    except subprocess.TimeoutExpired as error:
        def decode(value):
            return value.decode(errors='replace') if isinstance(value, bytes) else value or ''
        record = dict(cutoff=True, stdout=decode(error.stdout), stderr=decode(error.stderr))
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    record.update(command=[str(x) for x in command], elapsed_seconds=time.monotonic()-start,
                  child_cpu_seconds=after.ru_utime+after.ru_stime-cpu.ru_utime-cpu.ru_stime,
                  address_space_bytes=limit, timeout_seconds=60)
    (OUT / (name + '.json')).write_text(json.dumps(record, indent=2) + '\n')
    return record


def validate(record, family, mode, size, count):
    assert record.get('exit_code') == 0
    rows = [json.loads(line) for line in record['stdout'].splitlines()]
    assert len(rows) == count + 1
    for index, row in enumerate(rows[:-1]):
        assert row['validated'] is True and row['query'] == index
        assert row['size'] == size + index % 2
        keys = ['setup_ns', 'execute_ns', 'observation_ns', 'engine_drop_ns', 'answer_drop_ns']
        assert all(type(row[key]) is int and row[key] >= 0 for key in keys)
        assert row['query_ns'] == sum(row[key] for key in keys)
    header = rows[-1]
    assert header['mode'] == mode and header['family'] == family and header['queries'] == count
    assert header['counters'] is False and header['allocator'] == 'ordinary'
    assert header['lifecycle_ns'] == sum(row['query_ns'] for row in rows[:-1]) + sum(header[key] for key in ['source_ns', 'prepare_ns', 'prepared_drop_ns'])
    return rows


def main():
    assert not OUT.exists(), 'Preserve existing sizing receipts'
    OUT.mkdir(parents=True)
    freeze = json.loads((ROOT / 'docs/experiments/results/s01-link-optimization/summary.json').read_text())
    for path, hash_value in freeze['shared_rlibs'].items():
        assert digest(ROOT / path) == hash_value
    for name, artifact in freeze['artifacts'].items():
        assert digest(TARGET / name) == artifact['binary_sha256']
    emitter = ROOT / 'target/s01-artifact-boundary/runtime/release/chr-access-emit'
    source = TARGET / 'dispatch64.rs'
    emitted = run('emit-dispatch64', [emitter, 'dispatch64', source], 1 << 30)
    assert emitted.get('exit_code') == 0, emitted
    extra = {}
    for optimization in ['off', 'thin']:
        binary = TARGET / ('dispatch64-' + optimization)
        result = run('compile-' + binary.name, ['rustc', '--edition', '2024', '-C', 'opt-level=3',
            '-C', 'codegen-units=1', '-C', 'lto=' + optimization, source,
            '--extern', 'chr_compiled=' + str(TARGET / 'runtime/release/libchr_compiled.rlib'),
            '-L', 'dependency=' + str(TARGET / 'runtime/release/deps'), '-o', binary], 4 << 30)
        assert result.get('exit_code') == 0, result
        extra[binary.name] = dict(source_sha256=digest(source), binary_sha256=digest(binary),
                                 binary_bytes=binary.stat().st_size)
    cells = [(optimization, family, mode, size, count)
             for optimization in ['off', 'thin']
             for family in ['chain', 'payload', 'subscription', 'dispatch16', 'dispatch64']
             for mode in ['generic', 'planned', 'specialized', 'native', 'native-generic-repair', 'native-planned', 'native-specialized']
             for size in [8, 32] for count in [1, 16]]
    random.Random(7001).shuffle(cells)
    checked = 0
    failures = []
    phases = {}
    for index, (optimization, family, mode, size, count) in enumerate(cells):
        name = f'run-{optimization}-{family}-{mode}-{size}-{count}'
        binary = TARGET / ((family if mode.startswith('native') else 'generic') + '-' + optimization)
        result = run(name, [binary, mode, family, size, count], 1 << 30)
        try:
            rows = validate(result, family, mode, size, count)
        except (AssertionError, KeyError, ValueError) as error:
            failures.append(dict(name=name, reason=repr(error)))
        else:
            checked += count
            for row in rows:
                for key, value in row.items():
                    if key.endswith('_ns') and (key not in phases or value > phases[key]['ns']):
                        phases[key] = dict(ns=value, cell=name)
        if (index + 1) % 20 == 0:
            print(index + 1, 'processes;', checked, 'checked queries;', len(failures), 'failures', flush=True)
    for path, hash_value in freeze['shared_rlibs'].items():
        assert digest(ROOT / path) == hash_value
    summary = dict(cells=len(cells), complete_queries=checked, failures=failures, seed=7001,
                   extra_artifacts=extra, maximum_phases=phases,
                   parent_commit=subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
                   interpretation='Exploratory sizing only; no comparative ranking')
    (OUT / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
    print(json.dumps(summary), flush=True)


if __name__ == '__main__':
    main()
