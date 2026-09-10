"""Registered paired thread-CPU/elapsed attribution with static restoration controls."""
import hashlib,json,os,platform,random,resource,subprocess,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];B=ROOT/'docs/experiments/results/s04-adaptive-cpu'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 assert not (B/'freeze.json').exists(),'already frozen'
 modes=['copy','reunion','eager','scheduled','fixed1','fixed8','backoff1','backoff8']
 scenarios=[['late',0,4,'all',1],['late',4,4,'all',0],['history',4,1,'all',1],['history',4,4,'all',0],['plain',4,4,'all',0],['plain',0,1,'all',0]]
 binaries={}
 for kind in ['wall','cpu']:
  command=['cargo','build','-p','chr-restoration','--release','--example','adaptive_cost','--message-format=json','--target-dir',str(ROOT/f'target/s04-adaptive-cpu-{kind}')]
  if kind=='cpu':command+=['--features','cpu-clock']
  p=subprocess.run(command,cwd=ROOT,capture_output=True,text=True,timeout=300);(B/f'build-{kind}.jsonl').write_text(p.stdout);(B/f'build-{kind}.log').write_text(p.stderr);p.check_returncode()
  binaries[kind]=Path(next(json.loads(x)['executable'] for x in p.stdout.splitlines() if json.loads(x).get('executable') and json.loads(x)['target']['name']=='adaptive_cost'))
 files=[Path(__file__),ROOT/'research/chr-restoration/examples/adaptive_cost.rs',ROOT/'research/chr-restoration/examples/support/repeated_source.rs',ROOT/'research/chr-restoration/Cargo.toml',ROOT/'Cargo.lock',ROOT/'docs/experiments/registrations/S04-adaptive-cpu.md',ROOT/'research/chr-direct-conditional/tests/runtime_support/mod.rs']
 for directory in ['research/chr-restoration/src','research/chr-observe/src','crates/chr-syntax/src']:files+=list((ROOT/directory).rglob('*.rs'))
 affinity=[]
 for cpu in [0,2]:
  assert cpu in os.sched_getaffinity(0)
  p=Path(f'/sys/devices/system/cpu/cpu{cpu}/topology');affinity.append({'cpu':cpu,'package':(p/'physical_package_id').read_text().strip(),'core':(p/'core_id').read_text().strip()})
 assert len({(a['package'],a['core']) for a in affinity})==2
 rng=random.Random(771004);base={}
 for cpu in [0,2]:
  for case in range(6):a=modes.copy();rng.shuffle(a);base[cpu,case]=a
 order=[]
 for block in range(-1,16):
  cpus=[0,2];rng.shuffle(cpus)
  for cpu in cpus:
   cases=list(range(6));rng.shuffle(cases)
   for case in cases:
    a=base[cpu,case];shift=max(0,block)%8
    for mode in a[shift:]+a[:shift]:
     builds=['wall','cpu'];rng.shuffle(builds)
     for kind in builds:order.append({'cpu':cpu,'scenario':case,'block':block,'warmup':block<0,'mode':mode,'build':kind})
 (B/'order.json').write_text(json.dumps(order,indent=2)+'\n');(B/'scenarios.json').write_text(json.dumps(scenarios,indent=2)+'\n')
 freeze={'sources':{str(p.relative_to(ROOT)):sha(p) for p in files},'binaries':{k:{'path':str(p.relative_to(ROOT)),'sha256':sha(p)} for k,p in binaries.items()},'order_sha256':sha(B/'order.json'),'scenarios_sha256':sha(B/'scenarios.json'),'affinity':affinity,'platform':platform.platform(),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'clock_id_header':str(Path('/usr/include/x86_64-linux-gnu/bits/time.h'))}
 (B/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n');(B/'runs').mkdir()
 def limits(cpu):
  def apply():os.sched_setaffinity(0,{cpu});resource.setrlimit(resource.RLIMIT_CPU,(60,60));resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
  return apply
 clocks=[]
 for i in range(5):
  p=subprocess.run([str(binaries['cpu']),'clock-check'],capture_output=True,text=True,timeout=60,preexec_fn=limits(0));(B/f'clock-{i}.log').write_text(p.stdout+p.stderr);p.check_returncode();clocks.append(json.loads(p.stdout))
 (B/'clocks.json').write_text(json.dumps(clocks,indent=2)+'\n')
 start=time.monotonic();endpoints={}
 with (B/'results.jsonl').open('x') as output:
  for i,item in enumerate(order):
   assert time.monotonic()-start<1800,'campaign bound'
   args=[item['mode'],*scenarios[item['scenario']]]
   p=subprocess.run([str(binaries[item['build']]),*map(str,args)],capture_output=True,text=True,timeout=60,preexec_fn=limits(item['cpu']))
   (B/'runs'/f'{i}.json').write_text(json.dumps({'args':args,'exit_code':p.returncode,'stdout':p.stdout,'stderr':p.stderr})+'\n');p.check_returncode()
   data=[json.loads(x) for x in p.stdout.splitlines()];meta=data[0];assert not meta['metered']
   counts=next(r['service_counts'] for r in data if 'service_counts' in r);key=(item['scenario'],item['mode']);endpoint=(meta['counts'],meta['retained'],meta['engine_bytes'],counts)
   assert endpoints.setdefault(key,endpoint)==endpoint
   phases=[r for r in data if 'phase' in r];cpu_phases={(r['cpu_phase'],r['query']):r['cpu_ns'] for r in data if 'cpu_phase' in r}
   assert sum(r['ns'] for r in phases)==meta['total_ns']
   service=[r for r in phases if r['phase']=='first' or (r['phase']=='remaining' and not args[-1])]
   service_wall=sum(r['ns'] for r in service);service_cpu=sum(cpu_phases[r['phase'],r['query']] for r in service) if item['build']=='cpu' else None
   output.write(json.dumps({'index':i,**item,'endpoint':endpoint,'total_ns':meta['total_ns'],'service_wall_ns':service_wall,'service_cpu_ns':service_cpu,'service_intervals':len(service)})+'\n');output.flush()
   if (i+1)%192==0:print(f'completed {i+1}/{len(order)}',flush=True)
 (B/'audit.json').write_text(json.dumps({'processes':len(order),'primary_diagnostic':3072,'warmups':192,'clock_controls':5,'elapsed_seconds':time.monotonic()-start},indent=2)+'\n');print((B/'audit.json').read_text())
if __name__=='__main__':main()
