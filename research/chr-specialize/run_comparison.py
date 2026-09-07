"""Registered sequential specialization comparison with compilation reuse."""
import argparse,csv,io,json,random,subprocess
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('output',type=Path);p.add_argument('--seed',type=int,required=True);a=p.parse_args()
exe='target/release/examples/measure'
cases=subprocess.check_output([exe,'--list'],text=True).splitlines()
jobs=[(case,budget) for case in cases for budget in [0,1,2,4,8,16]]
random.Random(a.seed).shuffle(jobs)
with a.output.open('w') as out:
 for case,budget in jobs:
  try:
   run=subprocess.run([exe,case,str(budget)],capture_output=True,text=True,timeout=60)
   if run.returncode:rows=[dict(case=case,budget=budget,error=run.stderr,exit_code=run.returncode)]
   else:
    rows=[{k:(v=='true' if k=='pass' else v if k=='case' else int(v)) for k,v in row.items()} for row in csv.DictReader(io.StringIO(run.stdout),delimiter='\t')]
    assert len(rows)==3
  except subprocess.TimeoutExpired:rows=[dict(case=case,budget=budget,timeout_seconds=60)]
  for row in rows:out.write(json.dumps(row)+'\n')
  out.flush()
