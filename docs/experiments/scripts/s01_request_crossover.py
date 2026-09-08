#!/usr/bin/env python3
"""Prospectively frozen S01 pilot; existing run output is never overwritten."""
import hashlib,json,os,platform,random,resource,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s01-request-crossover'
MODES=['direct','retained']
FAMILIES=['stable-selective','consume-dense']
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def binary(kind):return ROOT/f'target/s01-request-{kind}/release/examples/s01_request_crossover'
def limits():
 os.sched_setaffinity(0,{min(os.sched_getaffinity(0))})
 resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3))
def invoke(cmd):
 p=subprocess.run(list(map(str,cmd)),cwd=ROOT,text=True,capture_output=True,timeout=60,preexec_fn=limits)
 return dict(command=list(map(str,cmd)),exit_code=p.returncode,stdout=p.stdout,stderr=p.stderr)
def prepare():
 assert not (OUT/'freeze.json').exists()
 checks=[]
 for kind in ['time','memory']:
  r=invoke([binary(kind),'gate']);checks.append(r)
  (OUT/f'gate-{kind}.log').write_text((r['stdout']+r['stderr']).rstrip()+'\n')
  assert r['exit_code']==0,r
  cmd=['cargo','tree','-p','chr-compiled','--no-default-features','-e','features']+(['--features','alloc-meter'] if kind=='memory' else [])
  r=subprocess.run(cmd,cwd=ROOT,text=True,capture_output=True,check=True)
  (OUT/f'features-{kind}.log').write_text(r.stdout)
  assert 'feature "metrics"' not in r.stdout and 'feature "kernel-metrics"' not in r.stdout
 rng=random.Random(20260913);order=[]
 for kind,count in [('time',10),('memory',2)]:
  for repetition in range(count):
   cells=[dict(mode=m,family=f,n=n,rounds=r) for m in MODES for f in FAMILIES for n in [8,32] for r in [1,8,64]];rng.shuffle(cells)
   order.extend(dict(kind=kind,repetition=repetition,**c) for c in cells)
 (OUT/'order.json').write_text(json.dumps(order,indent=2)+'\n')
 paths=[p for p in subprocess.check_output(['git','ls-files'],cwd=ROOT,text=True).splitlines() if p.endswith(('.rs','.toml','.lock')) and not p.startswith(('.agents/','.codex/'))]
 paths+=['research/chr-compiled/examples/s01_request_crossover.rs','docs/experiments/scripts/s01_request_crossover.py','docs/experiments/registrations/S01-request-crossover.md']
 freeze=dict(head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),source_sha256={p:digest(ROOT/p) for p in sorted(set(paths))},binary_sha256={k:digest(binary(k)) for k in ['time','memory']},order_sha256=digest(OUT/'order.json'),compiler=subprocess.check_output(['rustc','-Vv'],cwd=ROOT,text=True),machine=platform.uname()._asdict(),cpu=min(os.sched_getaffinity(0)),checks=checks)
 (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
 print('Frozen 288 processes; all semantic and meter gates pass.',flush=True)
def run():
 freeze=json.loads((OUT/'freeze.json').read_text())
 for p,h in freeze['source_sha256'].items():assert digest(ROOT/p)==h,p
 for k,h in freeze['binary_sha256'].items():assert digest(binary(k))==h,k
 assert digest(OUT/'order.json')==freeze['order_sha256']
 assert min(os.sched_getaffinity(0))==freeze['cpu']
 with (OUT/'raw.jsonl').open('x') as f:
  for i,cell in enumerate(json.loads((OUT/'order.json').read_text())):
   try:r=invoke([binary(cell['kind']),cell['mode'],cell['family'],cell['n'],cell['rounds']])
   except subprocess.TimeoutExpired as e:r=dict(exit_code=None,timeout=True,stdout=str(e.stdout or ''),stderr=str(e.stderr or ''))
   record=dict(index=i,**cell,**r)
   if r['exit_code']==0:record['measurement']=json.loads(r['stdout'])
   f.write(json.dumps(record)+'\n');f.flush()
   assert r['exit_code']==0,record
   assert len(record['measurement']['queries'])==1
   assert record['measurement']['meter']==(cell['kind']=='memory')
   if (i+1)%20==0:print(f'{i+1}/288 complete',flush=True)
if __name__=='__main__':{'prepare':prepare,'run':run}[sys.argv[1]]()
