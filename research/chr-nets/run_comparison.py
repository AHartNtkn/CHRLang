import argparse
import itertools
import json
import os
from pathlib import Path
import random
import subprocess
import sys

p = argparse.ArgumentParser()
p.add_argument('output', type=Path)
p.add_argument('--seed', type=int, required=True)
a = p.parse_args()
jobs = list(itertools.product([0, 1, 4, 16, 64], [0, 8, 64],
                              ['first', 'last', 'absent'], ['net', 'borrowed', 'copied']))
random.Random(a.seed).shuffle(jobs)
with a.output.open('w') as out:
    for v, depth, position, mode in jobs:
        base = dict(v=v, depth=depth, position=position, mode=mode)
        try:
            run = subprocess.run([sys.executable, str(Path(__file__).with_name('measure.py')),
                                  str(v), str(depth), position, mode], text=True,
                                 capture_output=True, timeout=60,
                                 env=dict(os.environ, PYTHONDONTWRITEBYTECODE='1', PYTHONHASHSEED='0'))
            row = json.loads(run.stdout) if run.returncode == 0 else dict(
                base, error=run.stderr, exit_code=run.returncode)
        except subprocess.TimeoutExpired:
            row = dict(base, timeout_seconds=60)
        out.write(json.dumps(row, sort_keys=True) + '\n')
        out.flush()
