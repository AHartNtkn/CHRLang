"""Prospectively registered controlled scale/reuse extension."""
import hashlib,itertools,json,os,random,resource,subprocess,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s02-multihead-scale'
BINARY=ROOT/'target/s02-multihead-lifecycle/time'
MODES=['local-scan','tuples','partial','scan','indexed','special-scan','special-indexed']
FAMILIES=['sparse','broad','nested','cold','dense','three']
PAIRS=[(4,1),(4,4),(16,1),(16,4),(64,1)]
def bound(cpu):
 os.sched_setaffinity(0,{cpu});assert os.sched_getaffinity(0)=={cpu}
 resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(30,30))
def main():
 original=json.loads((ROOT/'docs/experiments/results/s02-multihead-lifecycle/freeze.json').read_text())
 assert hashlib.sha256(BINARY.read_bytes()).hexdigest()==original['binaries']['time']
 assert {0,8}<=os.sched_getaffinity(0)
 OUT.mkdir(exist_ok=False)
 paths=[BINARY,Path(__file__),ROOT/'research/chr-relational/experiments/audit_multihead_scale.py',ROOT/'docs/experiments/registrations/S02-multihead-scale.md']
 hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
 (OUT/'freeze.json').write_text(json.dumps(hashes,indent=2)+'\n')
 (OUT/'environment.json').write_text(json.dumps(dict(allowed_cpus=sorted(os.sched_getaffinity(0)),cpu_details=subprocess.check_output(['lscpu','-e=CPU,CORE,MAXMHZ,MINMHZ'],text=True),load_start=Path('/proc/loadavg').read_text()),indent=2)+'\n')
 blocks=list(itertools.product(range(21),[0,8],FAMILIES,PAIRS));rng=random.Random(7283);rng.shuffle(blocks);jobs=[]
 for rep,cpu,family,(width,reuse) in blocks:
  modes=MODES.copy();rng.shuffle(modes)
  jobs.extend((rep,cpu,family,width,reuse,mode) for mode in modes)
 (OUT/'order.json').write_text(json.dumps(jobs)+'\n');start=time.monotonic()
 with (OUT/'runs.jsonl').open('w') as log:
  for i,(rep,cpu,family,width,reuse,mode) in enumerate(jobs):
   assert time.monotonic()-start<1800,'driver wall bound'
   cmd=[str(BINARY),mode,family,str(width),str(reuse)]
   receipt=dict(rep=rep,cpu=cpu,family=family,width=width,reuse=reuse,mode=mode,command=cmd)
   try:
    p=subprocess.run(cmd,cwd=ROOT,text=True,capture_output=True,timeout=45,preexec_fn=lambda:bound(cpu))
    receipt.update(code=p.returncode,stdout=p.stdout,stderr=p.stderr)
   except subprocess.TimeoutExpired as e:receipt.update(timeout=True,stdout=str(e.stdout),stderr=str(e.stderr))
   log.write(json.dumps(receipt)+'\n');log.flush()
   assert receipt.get('code')==0 and not receipt['stderr'],(i,receipt)
   x=json.loads(receipt['stdout']);assert (x['mode'],x['family'],x['width'],x['reuse'])==(mode,family,width,reuse)
   assert all('memory' not in p['measurement'] for p in x['phases'])
   if (i+1)%630==0:print(f'{i+1}/{len(jobs)} controlled runs',flush=True)
 for path,h in hashes.items():assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==h
 (OUT/'complete.json').write_text(json.dumps(dict(processes=len(jobs),elapsed_seconds=time.monotonic()-start,load_end=Path('/proc/loadavg').read_text()))+'\n')
if __name__=='__main__':main()
