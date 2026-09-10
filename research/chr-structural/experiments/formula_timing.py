"""Prospectively registered, balanced and pinned finite-formula lifecycle confirmation."""
import hashlib,json,os,random,resource,subprocess,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];B=ROOT/'docs/experiments/results/s06-formula-timing';Q=ROOT/'docs/experiments/results/s06-formula-clock'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 B.mkdir(exist_ok=True);assert not (B/'freeze.json').exists(),'already frozen'
 f=read(Q/'freeze.json');binary=f['binaries']['session']
 for p,h in f['sources'].items():assert sha(ROOT/p)==h,p
 assert sha(ROOT/binary['path'])==binary['sha256']
 assert {0,2}<=os.sched_getaffinity(0)
 scenarios=read(Q/'review-audit.json')['qualified_scenarios'];assert len(scenarios)==17
 modes=['reduced','diagram','names','projected','symbolic','explicit'];rng=random.Random(73062)
 base={}
 for cpu in [0,2]:
  for case in range(17):
   permutation=modes.copy();rng.shuffle(permutation);base[cpu,case]=permutation
 order=[]
 for block in range(-1,24):
  cpus=[0,2];rng.shuffle(cpus)
  for cpu in cpus:
   cases=list(range(17));rng.shuffle(cases)
   for case in cases:
    permutation=base[cpu,case];shift=max(block,0)%6;permutation=permutation[shift:]+permutation[:shift]
    for mode in permutation:order.append({'cpu':cpu,'scenario':case,'block':block,'warmup':block<0,'mode':mode})
 (B/'scenarios.json').write_text(json.dumps(scenarios,indent=2)+'\n');(B/'order.json').write_text(json.dumps(order,indent=2)+'\n')
 files={**f['sources'],str(Path(__file__).relative_to(ROOT)):sha(Path(__file__)),'docs/experiments/registrations/S06-formula-timing.md':sha(ROOT/'docs/experiments/registrations/S06-formula-timing.md')}
 (B/'freeze.json').write_text(json.dumps({'sources':files,'binary':binary,'order_sha256':sha(B/'order.json'),'scenarios_sha256':sha(B/'scenarios.json'),'affinity':read(Q/'affinity.json'),'rustc':f['rustc']},indent=2)+'\n');(B/'runs').mkdir()
 expected={}
 for i,args in enumerate(read(Q/'cases.json')):expected[tuple(args)]=json.loads(read(Q/f'runs/{i}-ordinary.json')['stdout'].splitlines()[0])
 start=time.monotonic()
 with (B/'results.jsonl').open('x') as out:
  for i,item in enumerate(order):
   assert time.monotonic()-start<1800,'campaign bound'
   def limits():
    os.sched_setaffinity(0,{item['cpu']});resource.setrlimit(resource.RLIMIT_CPU,(60,60));resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
   args=[item['mode'],*scenarios[item['scenario']]]
   p=subprocess.run([str(ROOT/binary['path']),*map(str,args)],capture_output=True,text=True,timeout=60,preexec_fn=limits)
   (B/'runs'/f'{i}.json').write_text(json.dumps({'args':args,'exit_code':p.returncode,'stdout':p.stdout,'stderr':p.stderr})+'\n');p.check_returncode()
   lines=[json.loads(x) for x in p.stdout.splitlines()];assert len(lines)==2 and lines[0]==expected[tuple(args)] and lines[1]['session_ns']>0
   out.write(json.dumps({'index':i,**item,'session_ns':lines[1]['session_ns'],'endpoint':lines[0]})+'\n');out.flush()
   if (i+1)%300==0:print(f'completed {i+1}/{len(order)}',flush=True)
 (B/'audit.json').write_text(json.dumps({'processes':len(order),'primary':4896,'warmups':204,'elapsed_seconds':time.monotonic()-start},indent=2)+'\n')
 print((B/'audit.json').read_text())
if __name__=='__main__':main()
