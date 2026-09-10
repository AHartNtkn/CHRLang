"""Frozen, pinned, balanced confirmation of complete dependency session costs."""
import hashlib,json,os,random,resource,subprocess,time
from pathlib import Path
ROOT=Path.cwd();BASE=ROOT/'docs/experiments/results/s03-dependency-timing';BASE.mkdir(exist_ok=False);OUT=BASE/'runs';OUT.mkdir()
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
clock=json.loads((ROOT/'docs/experiments/results/s03-dependency-clock/freeze.json').read_text());binary=clock['binaries']['ordinary'];exe=ROOT/binary['path'];assert digest(exe)==binary['sha256']
cpus=[0,2];assert set(cpus)<=os.sched_getaffinity(0)
modes=['current','current-miss','birth','birth-miss','dependencies','dependencies-miss','scan','indexed']
scenarios=[{'family':family,'size':size,'reverse':False,'retention':'immediate'} for family in ['plain','known-hit','known-miss','delayed-hit','delayed-miss'] for size in [8,128]]
scenarios += [{'family':family,'size':128,'reverse':True,'retention':'immediate'} for family in ['delayed-hit','delayed-miss']]
scenarios += [{'family':family,'size':128,'reverse':False,'retention':'all'} for family in ['known-hit','delayed-miss']]
assert len(scenarios)==14
rng=random.Random(73042);permutations={}
for cpu in cpus:
 for scenario in range(14):
  order=modes.copy();rng.shuffle(order);permutations[cpu,scenario]=order
order=[]
for cpu in cpus:
 for i in range(14):
  for mode in permutations[cpu,i]:order.append({'warmup':True,'cpu':cpu,'scenario':i,'block':-1,'mode':mode})
for block in range(24):
 cpu_order=cpus.copy();rng.shuffle(cpu_order)
 for cpu in cpu_order:
  case_order=list(range(14));rng.shuffle(case_order)
  for i in case_order:
   base=permutations[cpu,i];shift=block%8;rotated=base[shift:]+base[:shift]
   for mode in rotated:order.append({'warmup':False,'cpu':cpu,'scenario':i,'block':block,'mode':mode})
assert len(order)==5600
(BASE/'order.json').write_text(json.dumps(order,indent=2)+'\n');(BASE/'scenarios.json').write_text(json.dumps(scenarios,indent=2)+'\n')
paths=dict(clock['sources']);paths.update({p:digest(ROOT/p) for p in ['research/chr-direct-conditional/experiments/dependency_timing.py','docs/experiments/registrations/S03-dependency-timing.md']})
assert all(digest(ROOT/p)==sha for p,sha in paths.items())
freeze={'sources':paths,'binary':binary,'cpus':cpus,'available_affinity':sorted(os.sched_getaffinity(0)),'order_sha256':digest(BASE/'order.json'),'scenarios_sha256':digest(BASE/'scenarios.json'),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'platform':os.uname()._asdict() if hasattr(os.uname(),'_asdict') else list(os.uname())}
(BASE/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
old=json.loads((ROOT/'docs/experiments/results/s03-dependency-ownership/results.json').read_text())
controls={(r['family'],r['size'],r['reverse'],r['retention'],r['mode']):r['result']['endpoints'] for r in old if not r['cancel']}
def endpoint(r):return [{k:v for k,v in e.items() if k!='first_ns'} for e in r['endpoints']]
start=time.monotonic()
with (BASE/'results.jsonl').open('x') as summaries:
 for index,item in enumerate(order):
  assert time.monotonic()-start<45*60,'campaign bound; partial results preserved'
  case=scenarios[item['scenario']];cpu=item['cpu'];mode=item['mode']
  assert digest(exe)==binary['sha256']
  def limits():
   os.sched_setaffinity(0,{cpu});resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
  args=[mode,case['family'],str(case['size']),str(case['reverse']).lower(),case['retention'],'false']
  process=subprocess.run([str(exe),*args],env=dict(os.environ,DEPENDENCY_FIRST_CLOCK='on'),capture_output=True,text=True,preexec_fn=limits,timeout=60)
  (OUT/f'{index}.log').write_text(process.stdout+process.stderr)
  assert process.returncode==0,(index,process.returncode,process.stderr)
  rows=[json.loads(x) for x in process.stdout.splitlines() if x.startswith('{')];r=next(row for row in rows if row.get('event')=='result')
  assert not r['meter'];assert endpoint(r)==controls[case['family'],case['size'],case['reverse'],case['retention'],mode]
  assert all(e['complete'] and e['first_ns'] is not None for e in r['endpoints'])
  assert len(r['phases'])==24
  total=sum(p['reading']['ns'] for p in r['phases'])
  first=[e['first_ns'] for e in r['endpoints']]
  for i,ns in enumerate(first):assert ns<=next(p['reading']['ns'] for p in r['phases'] if p['phase']=='execute_observe' and p['query']==i)
  cold=first[0]+sum(p['reading']['ns'] for p in r['phases'] if p['phase'] in ['source','prepare'] or p['phase'] in ['input','setup'] and p['query']==0)
  summaries.write(json.dumps({'index':index,**item,'session_ns':total,'first_ns':first,'cold_first_ns':cold,'result':r})+'\n');summaries.flush()
  if (index+1)%224==0:print('completed processes',index+1,flush=True)
assert all(digest(ROOT/p)==sha for p,sha in paths.items())
(BASE/'audit.json').write_text(json.dumps({'primary_processes':5376,'warmup_processes':224,'complete_endpoints':True,'pinned_cpus':cpus,'balanced_positions':True,'outliers_excluded':0,'elapsed_seconds':time.monotonic()-start},indent=2)+'\n')
print('All 5600 pinned session processes complete.',flush=True)
