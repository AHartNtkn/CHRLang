import argparse
import json
import os
from pathlib import Path
import random
import subprocess
import sys
from unification_workloads import cases

p = argparse.ArgumentParser()
p.add_argument('output', type=Path)
p.add_argument('--seed', type=int, required=True)
a = p.parse_args()
jobs = [(c['id'], mode) for c in cases() for mode in ['map', 'table', 'encoded', 'net']]
random.Random(a.seed).shuffle(jobs)
with a.output.open('w') as out:
    for case, mode in jobs:
        base = dict(id=case, mode=mode)
        try:
            run = subprocess.run([sys.executable, str(Path(__file__).with_name('measure_unification.py')),
                                  case, mode], capture_output=True, text=True, timeout=60,
                                 env=dict(os.environ, PYTHONHASHSEED='0', PYTHONDONTWRITEBYTECODE='1'))
            row = json.loads(run.stdout) if run.returncode == 0 else dict(base, exit_code=run.returncode, error=run.stderr)
        except subprocess.TimeoutExpired:
            row = dict(base, timeout_seconds=60)
        out.write(json.dumps(row, sort_keys=True) + '\n')
        out.flush()
