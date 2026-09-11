"""Exact caller-work and complete requested-ownership attribution."""
import gzip,hashlib,itertools,json,os,random,resource,shutil,subprocess,sys,zipfile
from pathlib import Path
REUSE='--answer-validity' in sys.argv
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/('docs/experiments/results/s08-answer-validity' if REUSE else 'docs/experiments/results/s08-validity-callers');CPU=min(os.sched_getaffinity(0))
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
 os.sched_setaffinity(0,{CPU});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def normalized(d):
 p=d['phases'];assert [r['phase'] for r in p]==['prepare','setup','execute','producer_drop','setup','execute','producer_drop','prepared_drop','consumer_drop']
 root=p[0]['heap']['live_start'];h=[{k:v-root if k in ['live_start','live_end','peak_live'] else v for k,v in r['heap'].items()} for r in p]
 assert all(a['live_end']==b['live_start'] for a,b in zip(h,h[1:])) and h[-1]['live_end']==0
 return h
def audit():
 f=json.loads((OUT/'freeze.json').read_text());assert sha(OUT/'sources.zip')==f['source_hash']
 for b in f['builds'].values():assert sha(Path(b['binary']))==b['sha256']
 rows=[json.loads(s) for s in gzip.open(OUT/'runs.jsonl.gz','rt')];assert len(rows)==768 and [r['job'] for r in rows]==f['jobs'];cells={};summary=[]
 for r in rows:
  j,d=r['job'],r['result'];assert d['validated'] and d['profiled']==j['profile'];h=normalized(d);key=(j['context'],j['profile'],tuple(j['case']));value=(h,d['answers'],d['profile'])
  if key in cells:assert cells[key]==value,key
  cells[key]=value
 assert len(cells)==384
 for (context,profile,case),(h,answers,work) in cells.items():
  if not profile:continue
  control=cells[context,False,case];assert h==control[0] and answers==control[1]
  assert len(work)==12 and len({r['site'] for r in work})==12
  for r in work:
   assert 0<=r['accepted']<=r['calls'] and r['visited']<=r['support']
   assert r['requested_bytes']==r['allocation_calls']==r['deallocation_calls']==0
  if case[0]=='direct':assert all(r['calls']==0 for r in work)
  summary.append(dict(context=context,case=case,answers=answers,requested=sum(p['requested_bytes'] for p in h),peak=max(p['peak_live'] for p in h),profile=work))
 assert any(r['accepted']<r['calls'] for s in summary for r in s['profile'])
 if REUSE:
  old=json.loads((ROOT/'docs/experiments/results/s08-validity-callers/analysis.json').read_text())['rows'];old={(r['context'],tuple(r['case'])):r for r in old};contrasts=[]
  for r in summary:
   prior=old[r['context'],tuple(r['case'])];assert prior['answers']==r['answers']
   for a,b in zip(prior['profile'],r['profile']):
    if a['site']!='answer_residual':assert a==b,(r['case'],a,b)
   if r['case'][0]=='direct':assert {k:v for k,v in prior.items() if k!='case'}=={k:v for k,v in r.items() if k!='case'}
   contrasts.append(dict(context=r['context'],case=r['case'],control_requested=prior['requested'],candidate_requested=r['requested'],control_peak=prior['peak'],candidate_peak=r['peak'],control_residual=next(p for p in prior['profile'] if p['site']=='answer_residual'),candidate_residual=next(p for p in r['profile'] if p['site']=='answer_residual')))
  (OUT/'comparison.json').write_text(json.dumps(contrasts,indent=2)+'\n')
 (OUT/'analysis.json').write_text(json.dumps(dict(processes=768,exact_repetitions=384,diagnostic_control_pairs=192,rows=summary),indent=2)+'\n');print('Audited 768 processes, 384 exact repetitions and 192 diagnostic/control ownership pairs')
def run():
 OUT.mkdir(parents=True,exist_ok=True);assert not (OUT/'freeze.json').exists();builds={}
 for context,profile in itertools.product(['lookup','seeking'],[False,True]):
  features='alloc-meter,chr-direct-choice/completed-traversal'+(',chr-direct-choice/seek-context' if context=='seeking' else '')+(',validity-profile' if profile else '')+(',chr-direct-choice/answer-validity' if REUSE else '')
  cmd=['cargo','build','-p','chr-reuse','--release','--no-default-features','--features',features,'--example','validity_sources']
  r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True);(OUT/f'build-{context}-{profile}.log').write_text(r.stderr);assert r.returncode==0
  b=ROOT/f'target/{"answer-validity" if REUSE else "validity-callers"}-{context}-{profile}';shutil.copy2(ROOT/'target/release/examples/validity_sources',b);builds[f'{context}-{profile}']=dict(binary=str(b),sha256=sha(b),command=cmd);print('built',context,profile,flush=True)
 cases=list(itertools.product(['dependencies','templates','direct'],['repeated','distinct'],[False,True],[False,True],[8,32],[False,True]));assert len(cases)==96
 jobs=[dict(context=c,profile=p,case=case,rep=r) for c,p,case,r in itertools.product(['lookup','seeking'],[False,True],cases,range(2))];random.Random(7411).shuffle(jobs)
 paths=subprocess.check_output(['git','ls-files','research/chr-reuse','research/chr-direct-choice','research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax','Cargo.toml','Cargo.lock'],text=True).splitlines()+[str(Path(__file__).relative_to(ROOT)),'research/chr-reuse/examples/validity_sources.rs','research/chr-direct-choice/src/demand/validity_profile.rs','docs/experiments/registrations/S08-validity-callers.md']
 if REUSE: paths += ['docs/experiments/registrations/S08-answer-validity.md']
 with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
  for p in sorted(set(paths)):z.write(ROOT/p,p)
 (OUT/'freeze.json').write_text(json.dumps(dict(builds=builds,jobs=jobs,source_hash=sha(OUT/'sources.zip'),cpu=CPU,rustc=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2)+'\n')
 with gzip.open(OUT/'runs.jsonl.gz','wt') as out:
  for i,j in enumerate(jobs):
   cmd=[builds[f"{j['context']}-{j['profile']}"]['binary']]+[str(v).lower() for v in j['case']]
   r=subprocess.run(cmd,capture_output=True,text=True,timeout=60,preexec_fn=limits)
   if r.returncode:(OUT/'failure.json').write_text(json.dumps(dict(job=j,command=cmd,stderr=r.stderr),indent=2)+'\n')
   assert r.returncode==0,(j,r.stderr);d=json.loads(r.stdout);assert d['validated'];out.write(json.dumps(dict(job=j,result=d))+'\n');out.flush()
   if (i+1)%96==0:print(i+1,'complete',flush=True)
 audit()
if __name__=='__main__':audit() if '--audit' in sys.argv else run()
