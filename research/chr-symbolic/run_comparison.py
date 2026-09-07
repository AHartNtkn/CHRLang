"""Frozen E11 first matched matrix. Sequential fresh processes; preserve failures."""
import argparse
import json
from pathlib import Path
import random
import subprocess
import sys
import time

parser=argparse.ArgumentParser()
parser.add_argument('manifest',type=Path)
parser.add_argument('output',type=Path)
parser.add_argument('--case',action='append',default=[])
args=parser.parse_args()
bounds={r['case']:r['bounds'] for r in map(json.loads,Path('docs/experiments/results/E11-derived-bounds.jsonl').read_text().splitlines())}
cases=['U05-conditional-occurs','H05-conditional-history','app-add-forward','app-add-decompose','app-type-synthesis-prefix','carry-k1-w0-n0']
if args.case:cases=args.case
jobs=[(case,engine,repeat) for case in cases for engine in ['direct','terms'] for repeat in range(3)]
random.Random(1101).shuffle(jobs)
with args.output.open('w') as output:
    for case,engine,repeat in jobs:
        start=time.monotonic()
        try:
            child=subprocess.run([sys.executable,str(Path(__file__).with_name('compare_probe.py')),str(args.manifest),case,engine,json.dumps(bounds[case])],capture_output=True,text=True,timeout=60)
            if child.returncode:row=dict(case=case,engine=engine,error=child.stderr,exit_code=child.returncode)
            else:row=json.loads(child.stdout)
        except subprocess.TimeoutExpired:row=dict(case=case,engine=engine,timeout_seconds=60)
        row.update(repeat=repeat,process_seconds=time.monotonic()-start)
        output.write(json.dumps(row)+'\n');output.flush()
        print(json.dumps(row),flush=True)
