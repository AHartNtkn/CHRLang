"""Frozen exploratory total-cost pilot with separate ordinary/meter/profile builds."""
import hashlib,itertools,json,os,random,resource,subprocess,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];B=ROOT/'docs/experiments/results/s02-read-cost'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
 os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_CPU,(60,60));resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
def main():
 assert not (B/'freeze.json').exists(),'already frozen';assert 0 in os.sched_getaffinity(0)
 binaries={}
 for kind,features in [('ordinary',[]),('meter',['alloc-meter']),('profile',['deduction-profile'])]:
  cmd=['cargo','build','-p','chr-relational','--release','--example','s02_deduction','--target-dir',f'target/s02-read-cost-checked-{kind}','--message-format=json']
  if features:cmd+=['--features',','.join(features)]
  r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=300);(B/f'build-{kind}.jsonl').write_text(r.stdout);(B/f'build-{kind}.log').write_text(r.stderr);assert r.returncode==0,r.stderr
  binary=Path(next(json.loads(x)['executable'] for x in r.stdout.splitlines() if json.loads(x).get('executable')));binaries[kind]={'path':str(binary),'sha256':sha(binary)}
 modes=['contextual','shared','relevant','persistent-shared','persistent-relevant','validated','persistent-validated','scan','indexed']
 families=[f'near-{k}-{n}' for k,n in itertools.product(['unique','repeated','mixed'],[8,32])]
 scenarios=[(f,d,q,None) for f,d,q in itertools.product(families,[1,8],[1,4])]+[(f,16,q,None) for f,q in itertools.product(['single','changed'],[1,4])]
 scenarios += [(f,8,4,1) for f in families]+[('changed',16,4,1)]
 configs=[{'scenario':i,'mode':m} for i,s in enumerate(scenarios) for m in modes+(['lowered'] if not s[0].startswith('near') else [])]
 assert len(configs)==320
 order=[]
 for c in configs:
  for rep in range(2):order.append(dict(c,build='meter',rep=rep,warmup=False))
  if c['mode'] in modes[:7]:
   for rep in range(2):order.append(dict(c,build='profile',rep=rep,warmup=False))
 rng=random.Random(86401)
 for rep in range(-1,5):
  ids=list(range(28));rng.shuffle(ids)
  for i in ids:
   cs=[c for c in configs if c['scenario']==i];rng.shuffle(cs)
   for c in cs:order.append(dict(c,build='ordinary',rep=rep,warmup=rep<0))
 order += [dict(c,build='ordinary',rep=0,warmup=False) for c in configs if c['scenario']>=28]
 assert len(order)==2730
 (B/'scenarios.json').write_text(json.dumps(scenarios,indent=2)+'\n');(B/'order.json').write_text(json.dumps(order,indent=2)+'\n')
 files=[ROOT/p for p in subprocess.check_output(['git','ls-files','crates','research','Cargo.toml','Cargo.lock'],cwd=ROOT,text=True).splitlines() if p.endswith(('.rs','.toml','.lock'))]
 files += [Path(__file__),ROOT/'research/chr-relational/examples/support/read_near_source.rs',ROOT/'docs/experiments/registrations/S02-read-cost.md']
 (B/'freeze.json').write_text(json.dumps({'sources':{str(p.relative_to(ROOT)):sha(p) for p in files},'binaries':binaries,'order_hash':sha(B/'order.json'),'scenarios_hash':sha(B/'scenarios.json'),'cpu':0,'affinity':sorted(os.sched_getaffinity(0))},indent=2)+'\n')
 def invoke(kind,args,path):
  r=subprocess.run([binaries[kind]['path'],*map(str,args)],cwd=ROOT,env=dict(os.environ,DEDUCTION_RETAIN='all'),capture_output=True,text=True,timeout=60,preexec_fn=limits)
  path.write_text(json.dumps({'build':kind,'args':args,'exit_code':r.returncode,'stdout':r.stdout,'stderr':r.stderr})+'\n');assert r.returncode==0,(path,r.stderr)
  return r.stdout
 for kind in ['meter','profile']:invoke(kind,['meter-check'],B/f'meter-check-{kind}.json')
 clocks=[json.loads(invoke('ordinary',['clock-check'],B/f'clock-{i}.json')) for i in range(5)]
 (B/'clocks.json').write_text(json.dumps(clocks,indent=2)+'\n')
 (B/'runs').mkdir();start=time.monotonic()
 with (B/'results.jsonl').open('x') as output:
  for i,item in enumerate(order):
   assert time.monotonic()-start<1800,'campaign bound'
   f,d,q,cancel=scenarios[item['scenario']];args=[item['mode'],f,d,q,1,0]+([] if cancel is None else [cancel])
   text=invoke(item['build'],args,B/'runs'/f'{i}.json');result=[json.loads(l) for l in text.splitlines() if l.startswith('{')][-1]
   assert result['event']=='result' and not result['counters'];assert len(result['samples'])==q
   assert [s['complete'] for s in result['samples']]==([True]*q if cancel is None else [False,True,False,True])
   output.write(json.dumps({'index':i,**item,'result':result})+'\n');output.flush()
   if (i+1)%128==0:print(f'completed {i+1}/{len(order)}',flush=True)
 (B/'campaign.json').write_text(json.dumps({'workload_processes':len(order),'clock_controls':5,'meter_self_checks':2,'seconds':time.monotonic()-start},indent=2)+'\n')
 print('campaign complete')
if __name__=='__main__':main()
