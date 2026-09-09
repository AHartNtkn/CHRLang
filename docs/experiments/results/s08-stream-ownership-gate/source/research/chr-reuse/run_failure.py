"""Registered fresh-process failure-learning matrix, with preserved failed rows."""
import argparse,csv,io,json,random,subprocess
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('output',type=Path);p.add_argument('--seed',type=int,required=True);a=p.parse_args()
exe='target/release/examples/failure_probe'
cases=subprocess.check_output([exe,'--list'],text=True).splitlines()
jobs=[(case,mode) for case in cases for mode in ['Direct','Learn','Native']]
random.Random(a.seed).shuffle(jobs)
with a.output.open('w') as out:
 for case,mode in jobs:
  try:
   run=subprocess.run([exe,case,mode],capture_output=True,text=True,timeout=60)
   if run.returncode:row=dict(case=case,mode=mode,error=run.stderr,exit_code=run.returncode)
   else:
    row=list(csv.DictReader(io.StringIO(run.stdout),delimiter='\t'))[0]
    row={k:(v=='true' if k=='pass' else int(v) if k not in ['case','mode'] else v) for k,v in row.items()}
  except subprocess.TimeoutExpired:row=dict(case=case,mode=mode,timeout_seconds=60)
  out.write(json.dumps(row)+'\n');out.flush()
