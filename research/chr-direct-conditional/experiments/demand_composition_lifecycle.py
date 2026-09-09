#!/usr/bin/env python3
"""Registered isolated composition pilot; invocation: script OUT METER TIME."""
import hashlib
import itertools
import json
from pathlib import Path
import random
import resource
import subprocess
import sys

out = Path(sys.argv[1])
meter, timing = map(lambda x: Path(x).resolve(), sys.argv[2:4])
out.mkdir(exist_ok=False)
cells = list(itertools.product(
    [f'{lower}-{mode}' for lower in ['original', 'fused'] for mode in
     ['contextual', 'demand']],
    ['plain', 'choices', 'duplicates', 'shared', 'spare', 'history'], [0, 1, 3], [1, 4]))
sources = [Path('research/chr-direct-conditional/examples/demand_composition_cost.rs'),
           Path('research/chr-direct-conditional/tests/composition_support/mod.rs'),
           Path('research/chr-compiled/examples/support/resource_fusion_source.rs'),
           Path('research/chr-compiled/experiments/meter.rs'),
           Path('docs/experiments/registrations/S10-demand-lifecycle.md'), Path(__file__), Path('research/chr-relational/src/contextual.rs'), Path('research/chr-relational/src/contextual_execute.rs')]
def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()
freeze = {str(p): sha(p) for p in sources + [meter, timing]}
(out / 'freeze.json').write_text(json.dumps(freeze, indent=2))
def limits():
    resource.setrlimit(resource.RLIMIT_AS, (1024**3, 1024**3))
    resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
def run_group(kind, binary, repeats, seed):
    jobs = [(r, *cell) for r in range(repeats) for cell in cells]
    random.Random(seed).shuffle(jobs)
    (out / f'{kind}-order.json').write_text(json.dumps(jobs))
    directory = out / kind
    directory.mkdir()
    results = {}
    for i, (r, mode, family, n, reuse) in enumerate(jobs):
        path = directory / f'{r}-{mode}-{family}-{n}-{reuse}.json'
        p = subprocess.run([str(binary), mode, family, str(n), str(reuse)],
                           capture_output=True, text=True, timeout=75, preexec_fn=limits)
        if p.returncode:
            path.with_suffix('.error').write_text(p.stdout + p.stderr)
            raise RuntimeError(f'failed {path}: {p.returncode}')
        path.write_text(p.stdout)
        d = json.loads(p.stdout)
        assert (d['mode'], d['family'], d['firings'], d['reuse']) == (mode, family, n, reuse)
        assert d['meter'] == (kind == 'meter')
        phases = d['phases']
        assert len(phases) == 10 + reuse * 8
        if kind == 'meter':
            readings = [(p['phase'], p['memory']) for p in phases]
            assert readings[0][1]['live_start'] == readings[-1][1]['live_end']
            assert all(a[1]['live_end'] == b[1]['live_start'] for a, b in zip(readings, readings[1:]))
            key = (mode, family, n, reuse)
            if key in results:
                assert results[key] == readings, f'allocation replay {key}'
            results[key] = readings
        if (i + 1) % 120 == 0:
            print(kind, i + 1, '/', len(jobs), flush=True)
    return len(jobs)
count = run_group('meter', meter, 2, 7820)
(out / 'allocation-gate.json').write_text(json.dumps({'processes': count, 'exact_pairs': len(cells), 'ownership': 'pass'}))
count += run_group('time', timing, 5, 7821)
assert all(sha(Path(p)) == h for p, h in freeze.items())
(out / 'completion.json').write_text(json.dumps({'processes': count, 'cells': len(cells), 'timing': 'exploratory'}))
