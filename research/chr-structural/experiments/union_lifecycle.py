"""Freeze and run registered allocation/timing sizing for substantive logical union."""
import hashlib,itertools,json,os,random,resource,subprocess,time,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s06-union-lifecycle';CPU=min(os.sched_getaffinity(0))
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
 os.sched_setaffinity(0,{CPU});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def run(binary,args,path):
 cmd=[binary,*map(str,args)]
 try:
  r=subprocess.run(cmd,capture_output=True,text=True,cwd=ROOT,timeout=60,preexec_fn=limits)
  raw=dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr,timeout=False)
 except subprocess.TimeoutExpired as e:
  dec=lambda x:x.decode() if isinstance(x,bytes) else (x or '')
  raw=dict(command=cmd,exit_code=None,stdout=dec(e.stdout),stderr=dec(e.stderr),timeout=True)
 path.write_text(json.dumps(raw)+'\n');assert raw['exit_code']==0 and not raw['timeout'],path
 return [json.loads(l) for l in raw['stdout'].splitlines()]
def main():
 assert not (BASE/'freeze.json').exists()
 bins={}
 for kind,features in [('meter',['--features','alloc-meter']),('time',[])]:
  cmd=['cargo','build','-p','chr-structural','--example','union_lifecycle','--release','--no-default-features',*features,'--target-dir',f'target/s06-union-lifecycle-{kind}','--message-format=json']
  r=subprocess.run(cmd,capture_output=True,text=True,cwd=ROOT,timeout=300);(BASE/f'build-{kind}.json').write_text(json.dumps(dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr))+'\n');assert r.returncode==0,r.stderr
  p=Path(next(json.loads(l)['executable'] for l in r.stdout.splitlines() if json.loads(l).get('executable')));bins[kind]=dict(path=str(p),sha256=sha(p))
 modes=['union','separate','reduced','names','explicit','dedup'];families=['overlap','disjoint','redundant','single']
 for i,(mode,family) in enumerate(itertools.product(modes,families)):
  run(bins['meter']['path'],[mode,family,4,8,'full','window'],BASE/f'smoke-{i}.json')
 paths=subprocess.check_output(['git','ls-files','crates','research','Cargo.toml','Cargo.lock'],cwd=ROOT,text=True).splitlines();paths=[p for p in paths if p.endswith(('.rs','.toml','.lock'))]
 paths+=['research/chr-structural/examples/union_lifecycle.rs','research/chr-structural/examples/support/useful_union_source.rs','research/chr-structural/experiments/union_lifecycle.py','docs/experiments/registrations/S06-union-lifecycle.md']
 hashes={p:sha(ROOT/p) for p in sorted(set(paths))}
 with zipfile.ZipFile(BASE/'sources.zip','w',compression=zipfile.ZIP_DEFLATED) as z:
  for p in hashes:z.write(ROOT/p,p)
 cells=list(itertools.product(modes,families,[4,8],[1,8],['member','full'],['immediate','window','all']))
 order=[]
 for kind,reps,seed in [('meter',2,76021),('time',5,76022)]:
  jobs=[dict(build=kind,repeat=r,args=list(c)) for r in range(reps) for c in cells if kind=='meter' or c[-1]=='all'];random.Random(seed).shuffle(jobs);order+=jobs
 (BASE/'order.json').write_text(json.dumps(order)+'\n')
 (BASE/'freeze.json').write_text(json.dumps(dict(sources=hashes,archive_sha256=sha(BASE/'sources.zip'),binaries=bins,cpu=CPU,order_sha256=sha(BASE/'order.json'),commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()),indent=2)+'\n')
 for i in range(5):run(bins['time']['path'],['clock-check'],BASE/f'clock-{i}.json')
 (BASE/'runs').mkdir(exist_ok=True);start=time.monotonic()
 with (BASE/'results.jsonl').open('w') as out:
  for i,job in enumerate(order):
   assert time.monotonic()-start<1800
   rows=run(bins[job['build']]['path'],job['args'],BASE/'runs'/f'{i}.json')
   out.write(json.dumps(dict(index=i,**job,rows=rows))+'\n');out.flush()
   if i%200==0:print(i,'/',len(order),flush=True)
 assert all(sha(ROOT/p)==h for p,h in hashes.items())
 print('completed',len(order),flush=True)
if __name__=='__main__':main()
