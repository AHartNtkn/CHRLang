"""Run the registered exploratory equality/guard matrix in fresh processes."""
import argparse
import json
from pathlib import Path
import subprocess
import sys

parser=argparse.ArgumentParser()
parser.add_argument('manifest',type=Path)
parser.add_argument('output',type=Path)
args=parser.parse_args()
cases=['U01-clash','U02-self','U03-alias','U04-occurs','U05-conditional-occurs','U06-union-cycle','G01-suspended','G02-recheck']
with args.output.open('w') as output:
    for case in cases:
        try:
            run=subprocess.run([sys.executable,str(Path(__file__).with_name('probe.py')),str(args.manifest),case],capture_output=True,text=True,timeout=60)
            row=run.stdout.strip() if run.returncode==0 else json.dumps({'case':case,'error':run.stderr})
        except subprocess.TimeoutExpired:
            row=json.dumps({'case':case,'timeout_seconds':60})
        output.write(row+'\n');output.flush()
        print(row,flush=True)
