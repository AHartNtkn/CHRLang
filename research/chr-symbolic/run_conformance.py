"""Run the registered exploratory equality/guard matrix in fresh processes."""
import argparse
import json
from pathlib import Path
import subprocess
import sys

parser=argparse.ArgumentParser()
parser.add_argument('manifest',type=Path)
parser.add_argument('output',type=Path)
parser.add_argument('--representation',choices=['heap','terms'],default='heap')
parser.add_argument('--case',action='append',default=[])
parser.add_argument('--bounds',type=Path)
parser.add_argument('--all',action='store_true',help='Use every case in the supplied registry manifest')
args=parser.parse_args()
cases=['U01-clash','U02-self','U03-alias','U04-occurs','U05-conditional-occurs','U06-union-cycle','G01-suspended','G02-recheck']
if args.case:cases=args.case
if args.all:cases=[json.loads(line)['id'] for line in args.manifest.read_text().splitlines()]
bounds={} if args.bounds is None else {r['case']:r['bounds'] for r in map(json.loads,args.bounds.read_text().splitlines())}
with args.output.open('w') as output:
    for case in cases:
        try:
            run=subprocess.run([sys.executable,str(Path(__file__).with_name('probe.py')),str(args.manifest),case,'--representation',args.representation,*[part for key,value in bounds.get(case,{}).items() for part in ['--'+key,str(value)]]],capture_output=True,text=True,timeout=60)
            row=run.stdout.strip() if run.stdout.strip() else json.dumps({'case':case,'error':run.stderr})
        except subprocess.TimeoutExpired as error:
            stderr=error.stderr or b''
            if isinstance(stderr,bytes):stderr=stderr.decode(errors='replace')
            phases=[json.loads(line)['phase'] for line in stderr.splitlines() if line.startswith('{"phase":')]
            row=json.dumps({'case':case,'timeout_seconds':60,'phase':phases[-1] if phases else 'unknown'})
        output.write(row+'\n');output.flush()
        print(row,flush=True)
