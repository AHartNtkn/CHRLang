"""Registered paired core-placement challenge, using the frozen timing binary."""
import hashlib,itertools,json,math,os,random,resource,statistics,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s02-multihead-affinity'
BINARY=ROOT/'target/s02-multihead-lifecycle/time'
MODES=['local-scan','tuples','partial','scan','indexed','special-scan','special-indexed']
FAMILIES=['sparse','broad','nested','cold','dense','three']
def bound(cpu):
 os.sched_setaffinity(0,{cpu});assert os.sched_getaffinity(0)=={cpu}
 resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(30,30))
def main():
 OUT.mkdir(exist_ok=False)
 original=json.loads((ROOT/'docs/experiments/results/s02-multihead-lifecycle/freeze.json').read_text())
 assert hashlib.sha256(BINARY.read_bytes()).hexdigest()==original['binaries']['time']
 assert {0,8}<=os.sched_getaffinity(0)
 env={'allowed_cpus':sorted(os.sched_getaffinity(0)),'cpu_details':subprocess.check_output(['lscpu','-e=CPU,CORE,MAXMHZ,MINMHZ'],text=True),'load_start':Path('/proc/loadavg').read_text()}
 (OUT/'environment.json').write_text(json.dumps(env,indent=2)+'\n')
 blocks=list(itertools.product(range(17),[0,8],FAMILIES));rng=random.Random(7282);rng.shuffle(blocks)
 jobs=[]
 for rep,cpu,family in blocks:
  modes=MODES.copy();rng.shuffle(modes)
  jobs.extend((rep,cpu,family,mode) for mode in modes)
 (OUT/'order.json').write_text(json.dumps(jobs)+'\n')
 with (OUT/'runs.jsonl').open('w') as log:
  for i,(rep,cpu,family,mode) in enumerate(jobs):
   cmd=[str(BINARY),mode,family,'64','4']
   try:
    p=subprocess.run(cmd,cwd=ROOT,text=True,capture_output=True,timeout=45,preexec_fn=lambda:bound(cpu))
    receipt=dict(rep=rep,cpu=cpu,family=family,mode=mode,command=cmd,code=p.returncode,stdout=p.stdout,stderr=p.stderr)
   except subprocess.TimeoutExpired as e:receipt=dict(rep=rep,cpu=cpu,family=family,mode=mode,timeout=True,stdout=str(e.stdout),stderr=str(e.stderr))
   log.write(json.dumps(receipt)+'\n');log.flush()
   assert receipt.get('code')==0 and not receipt['stderr'],(i,receipt)
   x=json.loads(receipt['stdout']);assert (x['mode'],x['family'],x['width'],x['reuse'])==(mode,family,64,4)
   assert all('memory' not in p['measurement'] for p in x['phases'])
   if (i+1)%168==0:print(f'{i+1}/1428 pinned runs',flush=True)
 assert hashlib.sha256(BINARY.read_bytes()).hexdigest()==original['binaries']['time']
 inputs=[Path(__file__),ROOT/'docs/experiments/registrations/S02-multihead-affinity.md']
 (OUT/'freeze.json').write_text(json.dumps(dict(binary_sha256=original['binaries']['time'],sources={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs}),indent=2)+'\n')
 (OUT/'complete.json').write_text(json.dumps({'processes':1428,'load_end':Path('/proc/loadavg').read_text()})+'\n')
if __name__=='__main__':main()
