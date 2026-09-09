#!/usr/bin/env python3
"""Paired frozen-binary calibration, independent of rebuilding the candidate."""
import hashlib
import json
import os
from pathlib import Path
import random
import resource
import statistics
import subprocess

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s02-repeated-alias-lifecycle'
MODES = ['endpoint', 'filtered', 'local-indexed', 'scan', 'indexed', 'special-scan', 'special-indexed']

def limit():
    resource.setrlimit(resource.RLIMIT_AS, (1024**3, 1024**3))
    os.sched_setaffinity(0, {0})

def main():
    assert not (OUT / 'calibration.jsonl').exists()
    builds = {}
    for tag, name in [('old', 's02-local-validation-attribution'), ('new', 's02-repeated-alias-lifecycle')]:
        manifest = json.loads((ROOT / 'docs/experiments/results' / name / 'manifest.json').read_text())
        build = manifest['builds']['time']
        binary = ROOT / build['binary']
        assert hashlib.sha256(binary.read_bytes()).hexdigest() == build['sha256']
        builds[tag] = binary
    rng = random.Random(7206)
    rows = []
    with (OUT / 'calibration.jsonl').open('x') as log:
        for rep in range(10):
            modes = MODES.copy()
            rng.shuffle(modes)
            for mode in modes:
                tags = ['old', 'new']
                rng.shuffle(tags)
                for tag in tags:
                    family = 'lowyield' if tag == 'old' else 'aliases-1'
                    command = [str(builds[tag]), mode, family, '128', '4', 'complete']
                    result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, check=True, timeout=60, preexec_fn=limit)
                    receipt = json.loads(result.stdout)
                    assert not receipt['metrics'] and not receipt['meter']
                    row = {'rep': rep, 'mode': mode, 'tag': tag, 'command': command, 'receipt': receipt}
                    log.write(json.dumps(row) + '\n')
                    log.flush()
                    rows.append(row)
    summary = []
    for mode in MODES:
        values = {(r['rep'], r['tag']): sum(p['ns'] for p in r['receipt']['phases']) for r in rows if r['mode'] == mode}
        ratios = [values[rep, 'new'] / values[rep, 'old'] for rep in range(10)]
        summary.append({'mode': mode, 'paired_new_old_ratio_median': statistics.median(ratios), 'paired_ratio_min': min(ratios), 'paired_ratio_max': max(ratios), 'old_ns_median': statistics.median(values[rep, 'old'] for rep in range(10)), 'new_ns_median': statistics.median(values[rep, 'new'] for rep in range(10))})
    (OUT / 'calibration-summary.json').write_text(json.dumps(summary, indent=2) + '\n')
    print(json.dumps(summary, indent=2))

if __name__ == '__main__':
    main()
