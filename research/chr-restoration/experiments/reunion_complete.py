#!/usr/bin/env python3
"""Prospective complete lifecycle pilot; separate ordinary and allocation builds."""
import hashlib
import itertools
import json
import os
from pathlib import Path
import random
import resource
import statistics
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s04-reunion-complete'
MODES = ['copy', 'reunion', 'scan', 'indexed', 'specialized-scan',
         'specialized-indexed', 'short-copy', 'short-reunion',
         'short-specialized-scan', 'short-specialized-indexed', 'factored', 'indexed-cow']
CELLS = list(itertools.product(['plain', 'equal', 'late', 'payload'], [2, 4], [0, 12, 48], [1, 4], MODES))

def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

def sources():
    paths = [ROOT / 'Cargo.toml', ROOT / 'Cargo.lock', Path(__file__).resolve(),
             ROOT / 'docs/experiments/registrations/S04-reunion-complete-pilot.md',
             ROOT / 'research/chr-direct-conditional/tests/runtime_support/mod.rs']
    for name in ['crates/chr-syntax', 'research/chr-restoration', 'research/chr-factors',
                 'research/chr-compiled', 'research/chr-persistent', 'research/chr-observe']:
        paths.extend(p for p in (ROOT / name).rglob('*') if p.is_file() and (p.suffix == '.rs' or p.name == 'Cargo.toml'))
    return {str(p.relative_to(ROOT)): sha(p) for p in sorted(set(paths))}

def call(cmd, path, cpu=None, seconds=120):
    if path.exists():
        r = json.loads(path.read_text())
        assert r['command'] == cmd and r['exit_code'] == 0, path
        return r
    def bound():
        resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
        os.sched_setaffinity(0, {cpu})
    try:
        p = subprocess.run(cmd, cwd=ROOT, text=True, capture_output=True, timeout=seconds,
                           preexec_fn=bound if cpu is not None else None)
        r = dict(command=cmd, exit_code=p.returncode, stdout=p.stdout, stderr=p.stderr)
    except subprocess.TimeoutExpired as e:
        r = dict(command=cmd, exit_code=None, cutoff='wall', stdout=(e.stdout or b'').decode(), stderr=(e.stderr or b'').decode())
    path.write_text(json.dumps(r, indent=2) + '\n')
    assert r['exit_code'] == 0, path
    return r

def build():
    assert not (OUT / 'freeze.json').exists()
    bins = {}
    for name, features in [('ordinary', ''), ('ordinary-cow', 'arena-cow'),
                           ('meter', 'alloc-meter'), ('meter-cow', 'alloc-meter,arena-cow')]:
        target = ROOT / 'target/s04-reunion-complete/measured' / name
        feature_args = ['--features', features] if features else []
        call(['cargo', 'build', '-p', 'chr-restoration', '--release', '--example',
              'reunion_complete_cost', '--target-dir', str(target)] + feature_args, OUT / f'build-{name}.json')
        binary = target / 'release/examples/reunion_complete_cost'
        bins[name] = dict(path=str(binary), sha256=sha(binary))
        feature = subprocess.check_output(['cargo', 'tree', '-p', 'chr-restoration', '-e', 'features',
                                          '--edges', 'normal,build,dev'] + feature_args, cwd=ROOT, text=True)
        assert not any('feature "' + x + '"' in feature for x in ['metrics', 'kernel-metrics', 'compiled-work', 'replay-diagnostic'])
        (OUT / f'features-{name}.txt').write_text(feature)
        print('built', name, flush=True)
    freeze = dict(sources=sources(), binaries=bins, cpu=min(os.sched_getaffinity(0)),
                  affinity=sorted(os.sched_getaffinity(0)), toolchain=subprocess.check_output(['rustc', '--version'], text=True),
                  head=subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True),
                  machine=subprocess.check_output(['uname', '-a'], text=True))
    (OUT / 'freeze.json').write_text(json.dumps(freeze, indent=2) + '\n')

def validate(r, cell, kind):
    f, owners, depth, reuse, mode = cell
    d = json.loads(r['stdout'])
    assert (d['family'], d['owners'], d['depth'], d['reuse'], d['mode']) == (f, owners, depth, reuse, mode.removesuffix('-cow'))
    assert d['cow'] == mode.endswith('-cow') and d['meter'] == (kind == 'meter')
    phases = d['phases']
    query = ['input', 'lower', 'setup', 'lowered-drop', 'execute', 'engine-drop', 'answers-drop', 'input-drop']
    assert [p['phase'] for p in phases] == ['prepare'] + query * reuse + ['cancel-' + ('first' if q == 'execute' else q) for q in query] + ['prepared-drop']
    assert len(d['first_ns']) == reuse + 1
    executions = [p for p in phases if p['phase'] in ['execute', 'cancel-first']]
    assert all(0 < first <= p['ns'] for first, p in zip(d['first_ns'], executions))
    if kind == 'meter':
        mem = [p['memory'] for p in phases]
        assert all(a['live_end'] == b['live_start'] for a, b in zip(mem, mem[1:]))
        assert all(m['peak_live'] >= max(m['live_start'], m['live_end']) for m in mem)
        assert mem[-1]['live_end'] == mem[0]['live_start']
        assert all(p['memory']['live_end'] == mem[0]['live_end'] for p in phases if p['phase'] in ['input-drop', 'cancel-input-drop'])
    else:
        assert all(p['memory'] is None for p in phases)
    return d

def run():
    freeze = json.loads((OUT / 'freeze.json').read_text())
    assert freeze['sources'] == sources()
    assert all(sha(Path(b['path'])) == b['sha256'] for b in freeze['binaries'].values())
    rng = random.Random(20260911)
    results = {}
    for kind, reps in [('meter', 2), ('ordinary', 5)]:
        for rep in range(reps):
            cells = CELLS.copy(); rng.shuffle(cells)
            for index, cell in enumerate(cells):
                f, owners, depth, reuse, mode = cell
                key = '-'.join(map(str, cell))
                name = kind + ('-cow' if mode.endswith('-cow') else '')
                cmd = [freeze['binaries'][name]['path'], mode.removesuffix('-cow'), f, str(owners), str(depth), str(reuse)]
                receipt = call(cmd, OUT / f'{kind}-{rep}-{key}.json', freeze['cpu'], 60)
                d = validate(receipt, cell, kind)
                results.setdefault((kind, cell), []).append(d)
                if kind == 'meter' and rep == 1:
                    assert [p['memory'] for p in d['phases']] == [p['memory'] for p in results[(kind, cell)][0]['phases']], key
                if (index + 1) % 144 == 0:
                    print(kind, rep + 1, index + 1, '/', len(cells), flush=True)
        if kind == 'meter':
            print('allocation gate: 576 exact pairs and complete disposal passed', flush=True)
    summary = []
    for cell in CELLS:
        samples = results[('ordinary', cell)]
        times = [sum(p['ns'] for p in d['phases'] if not p['phase'].startswith('cancel-')) for d in samples]
        m = results[('meter', cell)][0]
        full = [p for p in m['phases'] if not p['phase'].startswith('cancel-')]
        summary.append(dict(cell=cell, total_ns=dict(median=statistics.median(times), minimum=min(times), maximum=max(times)),
            phase_medians_ns={name: statistics.median(sum(p['ns'] for p in d['phases'] if p['phase'] == name) for d in samples) for name in dict.fromkeys(p['phase'] for p in samples[0]['phases'])},
            first_ns_medians=[statistics.median(d['first_ns'][i] for d in samples) for i in range(cell[3] + 1)],
            requested_bytes=sum(p['memory']['requested_bytes'] for p in full),
            peak_growth=max(p['memory']['peak_live'] for p in full) - full[0]['memory']['live_start'],
            prepared_live=full[0]['memory']['live_end'] - full[0]['memory']['live_start']))
    (OUT / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
    (OUT / 'audit.json').write_text(json.dumps(dict(successful_processes=4032, ordinary=2880, allocation=1152,
        exact_allocation_pairs=576, complete_answer_checks=True, disposal_and_continuity=True), indent=2) + '\n')
    print('audit: 4032 successful processes', flush=True)

if __name__ == '__main__':
    OUT.mkdir(parents=True, exist_ok=True)
    {'build': build, 'run': run}[sys.argv[1]]()
