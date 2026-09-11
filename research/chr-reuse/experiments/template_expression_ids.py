"""Challenge requested ownership with two observer-buffer layouts."""
import gzip,hashlib,itertools,json,os,resource,shutil,subprocess,sys,zipfile
from pathlib import Path
sys.dont_write_bytecode=True
from validity_callers import normalized
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s08-template-expression-ids';CPU=min(os.sched_getaffinity(0))
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
 os.sched_setaffinity(0,{CPU});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def audit():
 f=json.loads((OUT/'freeze.json').read_text());assert sha(OUT/'sources.zip')==f['source_hash']
 for b in f['builds'].values():assert sha(Path(b['binary']))==b['sha256']
 rows=list(map(json.loads,gzip.open(OUT/'runs.jsonl.gz','rt')));assert len(rows)==768 and [r['job'] for r in rows]==f['jobs'];cells={};analysis=[]
 for r in rows:
  j,d=r['job'],r['result'];assert d['validated'] and not d['profiled'];key=(j['context'],j['cache'],tuple(j['case']));n=normalized(d)
  if key in cells:assert cells[key]==(n,d['answers']),key
  else:analysis.append(dict(context=j['context'],cache=j['cache'],case=j['case'],answers=d['answers'],requested=sum(p['requested_bytes'] for p in n),peak=max(p['peak_live'] for p in n)))
  cells[key]=(n,d['answers'])
 assert len(cells)==384;(OUT/'analysis.json').write_text(json.dumps(analysis,indent=2)+'\n');print('768 source processes,384 exact observer-layout pairs; all owners restored')
def run():
 OUT.mkdir(parents=True,exist_ok=True);assert not (OUT/'freeze.json').exists();builds={}
 for context,cache in itertools.product(['lookup','seeking'],[False,True]):
  features='alloc-meter,chr-direct-choice/completed-traversal'+(',chr-direct-choice/seek-context' if context=='seeking' else '')+(',chr-direct-choice/answer-validity' if cache else '')
  cmd=['cargo','build','-p','chr-reuse','--release','--no-default-features','--features',features,'--example','validity_sources'];r=subprocess.run(cmd,capture_output=True,text=True);name=f'{context}-{cache}';(OUT/f'build-{name}.log').write_text(r.stderr);assert r.returncode==0
  b=ROOT/f'target/expression-ids-{name}';shutil.copy2(ROOT/'target/release/examples/validity_sources',b);builds[name]=dict(binary=str(b),sha256=sha(b),command=cmd);print('built',name,flush=True)
 cases=list(itertools.product(['dependencies','templates','direct'],['repeated','distinct'],[False,True],[False,True],[8,32],[False,True]));jobs=[dict(context=c,cache=k,case=s,capacity=n) for c,k,s,n in itertools.product(['lookup','seeking'],[False,True],cases,[9,73])]
 paths=subprocess.check_output(['git','ls-files','research/chr-reuse','research/chr-direct-choice','research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax','Cargo.toml','Cargo.lock'],text=True).splitlines()+[str(Path(__file__).relative_to(ROOT)),'docs/experiments/registrations/S08-template-expression-ids.md']
 with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
  for p in sorted(set(paths)):z.write(ROOT/p,p)
 (OUT/'freeze.json').write_text(json.dumps(dict(builds=builds,jobs=jobs,cpu=CPU,source_hash=sha(OUT/'sources.zip'),rustc=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2)+'\n')
 with gzip.open(OUT/'runs.jsonl.gz','wt') as out:
  for i,j in enumerate(jobs):
   cmd=[builds[f"{j['context']}-{j['cache']}"]['binary']]+[str(v).lower() for v in j['case']]+['1',str(j['capacity'])];r=subprocess.run(cmd,capture_output=True,text=True,timeout=60,preexec_fn=limits);assert r.returncode==0,r.stderr;out.write(json.dumps(dict(job=j,result=json.loads(r.stdout)))+'\n');out.flush()
   if (i+1)%192==0:print(i+1,'complete',flush=True)
 audit()
if __name__=='__main__':audit() if '--audit' in sys.argv else run()
