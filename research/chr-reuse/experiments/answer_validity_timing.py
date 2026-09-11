"""Paired ordinary lifecycle time with exact historical ownership revalidation."""
import gzip,hashlib,itertools,json,os,random,resource,shutil,statistics,subprocess,sys,zipfile
from pathlib import Path
sys.dont_write_bytecode=True
from validity_callers import normalized
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s08-answer-validity-timing';CPU=min(os.sched_getaffinity(0));VARIANTS=[('dependencies',False),('dependencies',True),('templates',False),('templates',True),('direct',False)]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
 os.sched_setaffinity(0,{CPU});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def invoke(b,case,sessions):
 r=subprocess.run([b]+[str(v).lower() for v in case]+[str(sessions)],capture_output=True,text=True,timeout=60,preexec_fn=limits);assert r.returncode==0,r.stderr
 rows=[json.loads(x) for x in r.stdout.splitlines()];assert len(rows)==sessions and all(d['validated'] and not d['profiled'] for d in rows);return rows
def audit_entries():
 old={}
 for cache,folder in [(False,'s08-validity-callers'),(True,'s08-answer-validity')]:
  for r in map(json.loads,gzip.open(ROOT/f'docs/experiments/results/{folder}/runs.jsonl.gz','rt')):
   j=r['job']
   if not j['profile']:old[cache,j['context'],tuple(j['case'])]=(normalized(r['result']),r['result']['answers'])
 entries=[json.loads(x) for x in gzip.open(OUT/'entries.jsonl.gz','rt')];assert len(entries)==384
 for r in entries:assert (normalized(r['result']),r['result']['answers'])==old[r['cache'],r['context'],tuple(r['case'])]
def audit():
 f=json.loads((OUT/'freeze.json').read_text());assert sha(OUT/'sources.zip')==f['source_hash']
 for b in f['builds'].values():assert sha(Path(b['binary']))==b['sha256']
 audit_entries()
 rows=[json.loads(x) for x in gzip.open(OUT/'runs.jsonl.gz','rt')];assert len(rows)==3200 and [r['job'] for r in rows]==f['jobs'];cells={}
 for r in rows:
  j=r['job'];assert len(r['sessions'])==5;totals=[]
  for d in r['sessions']:
   assert d['validated'] and not d['profiled'] and len(d['phases'])==9
   assert [p['phase'] for p in d['phases']]==['prepare','setup','execute','producer_drop','setup','execute','producer_drop','prepared_drop','consumer_drop']
   assert all(p['heap'] is None for p in d['phases']);totals.append(sum(p['ns'] for p in d['phases']))
  cell=cells.setdefault((tuple(j['group']),j['variant']),{});assert j['block'] not in cell;cell[j['block']]=statistics.mean(totals)
 assert len(cells)==320 and all(set(c)==set(range(10)) for c in cells.values())
 warm=[json.loads(x) for x in gzip.open(OUT/'warmups.jsonl.gz','rt')];assert len(warm)==320 and all(len(r['sessions'])==5 and all(d['validated'] for d in r['sessions']) for r in warm)
 contrasts=[]
 for group in sorted({k[0] for k in cells}):
  for a,b in itertools.combinations(range(5),2):
   ratios=[cells[group,b][i]/cells[group,a][i] for i in range(10)];med=statistics.median(ratios)
   contrasts.append(dict(group=group,reference=VARIANTS[a],candidate=VARIANTS[b],median=med,min=min(ratios),max=max(ratios),verdict='gain' if med<=.9 and max(ratios)<1 else 'loss' if med>=1.1 and min(ratios)>1 else 'unresolved'))
 (OUT/'analysis.json').write_text(json.dumps(contrasts,indent=2)+'\n');print('Audited384 exact ownership entries,3200 processes,16000 sessions,640 contrasts')
def run():
 OUT.mkdir(parents=True,exist_ok=True);assert not (OUT/'freeze.json').exists();builds={}
 for context,cache,meter in itertools.product(['lookup','seeking'],[False,True],[False,True]):
  features='chr-direct-choice/completed-traversal'+(',chr-direct-choice/seek-context' if context=='seeking' else '')+(',chr-direct-choice/answer-validity' if cache else '')+(',alloc-meter' if meter else '')
  cmd=['cargo','build','-p','chr-reuse','--release','--no-default-features','--features',features,'--example','validity_sources'];r=subprocess.run(cmd,capture_output=True,text=True);name=f'{context}-{cache}-{meter}';(OUT/f'build-{name}.log').write_text(r.stderr);assert r.returncode==0
  b=ROOT/f'target/answer-validity-time-{name}';shutil.copy2(ROOT/'target/release/examples/validity_sources',b);builds[name]=dict(binary=str(b),sha256=sha(b),command=cmd);print('built',name,flush=True)
 sources=list(itertools.product(['repeated','distinct'],[False,True],[False,True],[8,32],[False,True]));groups=[(c,*s) for c,s in itertools.product(['lookup','seeking'],sources)];jobs=[];rng=random.Random(7412)
 for block in range(10):
  order=list(enumerate(groups));rng.shuffle(order)
  for i,g in order:
   for v in [(i+block+j)%5 for j in range(5)]:jobs.append(dict(group=g,variant=v,block=block))
 paths=subprocess.check_output(['git','ls-files','research/chr-reuse','research/chr-direct-choice','research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax','Cargo.toml','Cargo.lock'],text=True).splitlines()+[str(Path(__file__).relative_to(ROOT)),'docs/experiments/registrations/S08-answer-validity-timing.md']
 with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
  for p in sorted(set(paths)):z.write(ROOT/p,p)
 (OUT/'freeze.json').write_text(json.dumps(dict(builds=builds,jobs=jobs,cpu=CPU,source_hash=sha(OUT/'sources.zip'),rustc=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2)+'\n')
 with gzip.open(OUT/'entries.jsonl.gz','wt') as out:
  for context,cache,mode,source in itertools.product(['lookup','seeking'],[False,True],['dependencies','templates','direct'],sources):
   case=(mode,*source);d=invoke(builds[f'{context}-{cache}-True']['binary'],case,1)[0];out.write(json.dumps(dict(context=context,cache=cache,case=case,result=d))+'\n')
 audit_entries()
 print('384 ownership entries validated',flush=True)
 with gzip.open(OUT/'warmups.jsonl.gz','wt') as out:
  for group in groups:
   for mode,cache in VARIANTS:out.write(json.dumps(dict(group=group,mode=mode,cache=cache,sessions=invoke(builds[f'{group[0]}-{cache}-False']['binary'],[mode,*group[1:]],5)))+'\n')
 with gzip.open(OUT/'runs.jsonl.gz','wt') as out:
  for i,j in enumerate(jobs):
   g=j['group'];mode,cache=VARIANTS[j['variant']];out.write(json.dumps(dict(job=j,sessions=invoke(builds[f'{g[0]}-{cache}-False']['binary'],[mode,*g[1:]],5)))+'\n');out.flush()
   if (i+1)%320==0:print('block',(i+1)//320,'complete',flush=True)
 audit()
if __name__=='__main__':audit() if '--audit' in sys.argv else run()
