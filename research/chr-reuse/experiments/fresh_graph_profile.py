"""Refresh graph cost attribution using the existing complete lifecycle runners."""
import collections,gzip,hashlib,itertools,json,os,random,re,resource,shutil,subprocess,sys,zipfile
from pathlib import Path
sys.dont_write_bytecode=True
from audit_continuing import lifecycle
from audit_traversal_lifecycle import norm
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s08-fresh-graph-profile';CPU=min(os.sched_getaffinity(0))
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
 os.sched_setaffinity(0,{CPU});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def args(c):
 context,source,mode,res,keep=c
 return [mode,str(res).lower(),'128' if source=='continuing' else '1',keep]+(['4','false','false'] if source=='continuing' else ['false'])
def validate(c,raw):
 assert raw['exit_code']==0,raw['stderr'];d=json.loads(raw['stdout']);assert d['validated']
 if c[1]=='continuing':lifecycle(raw,128,c[4])
 else:assert collections.Counter(x['phase'] for x in d['records'])==dict(prepare=1,setup=2,produce=2,consumer=2,exhaustion=2,producer_drop=2,prepared_drop=1,consumer_drop=1)
 return d
def audit():
 f=json.loads((OUT/'freeze.json').read_text());assert sha(OUT/'sources.zip')==f['source_hash']
 for b in f['builds'].values():assert sha(Path(b['binary']))==b['sha256']
 rows=[json.loads(x) for x in gzip.open(OUT/'runs.jsonl.gz','rt')];assert len(rows)==144 and [r['job'] for r in rows]==f['jobs'];cells={};ownership=[]
 for r in rows:
  c,kind,rep=r['job'];d=validate(c,r['raw']);assert d['meter']==(kind=='meter')
  if kind=='meter':
   n=norm(d);key=tuple(c)
   if key in cells:assert cells[key]==n
   else:ownership.append(dict(case=c,requested=sum(x['heap']['requested_bytes'] for x in n),peak=max(x['heap']['peak_live'] for x in n),phase_bytes={p:sum(x['heap']['requested_bytes'] for x in n if x['phase']==p) for p in {x['phase'] for x in n}}))
   cells[key]=n
 assert len(cells)==48
 profiles=json.loads((OUT/'profiles.json').read_text());assert len(profiles)==8;summary=[]
 for r in profiles:
  stem=r['stem'];data=gzip.decompress((OUT/(stem+'.perf.data.gz')).read_bytes());assert hashlib.sha256(data).hexdigest()==r['sha256']
  outputs=[json.loads(x) for x in gzip.open(OUT/(stem+'-outputs.jsonl.gz'),'rt')];assert len(outputs)==r['reps']
  for d in outputs:validate(r['case'],dict(exit_code=0,stdout=json.dumps(d),stderr=''))
  report=(OUT/(stem+'-self.txt')).read_text();assert '# Total Lost Samples: 0' in report
  samples=int(re.search(r'\((\d+) samples\)',(OUT/(stem+'-perf.log')).read_text())[1]);assert samples>=500,(stem,samples)
  summary.append(dict(stem=stem,samples=samples,processes=r['reps']))
 (OUT/'analysis.json').write_text(json.dumps(dict(qualification_processes=144,allocation_pairs=48,ownership=ownership,profiles=summary),indent=2)+'\n');print('Audited 144 qualification processes, 48 allocation pairs and 1200 profiled lifecycles')
def run():
 OUT.mkdir(parents=True,exist_ok=True);assert not (OUT/'freeze.json').exists();builds={}
 for context,kind in itertools.product(['lookup','seeking'],['plain','meter']):
  features='chr-direct-choice/completed-traversal'+(',chr-direct-choice/seek-context' if context=='seeking' else '')+(',alloc-meter' if kind=='meter' else '')
  cmd=['cargo','build','-p','chr-reuse','--release','--no-default-features','--features',features,'--example','continuing_lifecycle','--example','flat_lifecycle']
  r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True);(OUT/f'build-{context}-{kind}.log').write_text(r.stderr);assert r.returncode==0
  for source in ['continuing','flat']:
   b=ROOT/f'target/fresh-graph-{context}-{kind}-{source}';shutil.copy2(ROOT/f'target/release/examples/{source}_lifecycle',b);builds[f'{context}-{kind}-{source}']=dict(binary=str(b),sha256=sha(b),command=cmd)
  print('built',context,kind,flush=True)
 cases=list(itertools.product(['lookup','seeking'],['continuing','flat'],['dependencies','templates','direct'],[False,True],['0','all']));jobs=[(c,k,r) for c in cases for k,r in [('plain',0),('meter',0),('meter',1)]];random.Random(7410).shuffle(jobs)
 paths=subprocess.check_output(['git','ls-files','research/chr-reuse','research/chr-direct-choice','research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax','Cargo.toml','Cargo.lock'],text=True).splitlines()+[str(Path(__file__).relative_to(ROOT)),'docs/experiments/registrations/S08-fresh-graph-profile.md']
 with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
  for p in sorted(set(paths)):z.write(ROOT/p,p)
 (OUT/'freeze.json').write_text(json.dumps(dict(builds=builds,jobs=jobs,cpu=CPU,source_hash=sha(OUT/'sources.zip'),rustc=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2)+'\n')
 with gzip.open(OUT/'runs.jsonl.gz','wt') as out:
  for job in jobs:
   c,k,rep=job;b=builds[f'{c[0]}-{k}-{c[1]}']['binary'];r=subprocess.run([b]+args(c),capture_output=True,text=True,timeout=60,preexec_fn=limits);raw=dict(exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr);out.write(json.dumps(dict(job=job,raw=raw))+'\n');out.flush();validate(c,raw)
 print('144 qualification runs complete',flush=True);profiles=[]
 for context,mode,res in itertools.product(['lookup','seeking'],['dependencies','templates'],[False,True]):
  c=[context,'continuing',mode,res,'0'];reps=100 if mode=='dependencies' else 200;stem=f'{context}-{mode}-{res}';data=Path('/tmp')/f'fresh-graph-{stem}.data';b=builds[f'{context}-plain-continuing']['binary']
  script='for ((i=0;i<$1;i++)); do "$2" "${@:3}" || exit; done'
  cmd=['taskset','-c',str(CPU),'perf','record','-e','cycles:u','-F','999','--call-graph','dwarf','-o',str(data),'--','bash','-c',script,'profile',str(reps),b]+args(c)
  r=subprocess.run(cmd,capture_output=True,text=True,timeout=60,preexec_fn=limits);(OUT/(stem+'-perf.log')).write_text(r.stderr)
  with gzip.open(OUT/(stem+'-outputs.jsonl.gz'),'wt') as f:f.write(r.stdout)
  assert r.returncode==0,r.stderr
  for kind,extra in [('self',['--no-children']),('inclusive',['--children'])]:
   report=subprocess.check_output(['perf','report','-i',str(data),'--stdio','--sort','symbol','--percent-limit','0.5','--call-graph','none',*extra],text=True);(OUT/(stem+'-'+kind+'.txt')).write_text(report)
  with gzip.open(OUT/(stem+'.perf.data.gz'),'wb') as f:f.write(data.read_bytes())
  profiles.append(dict(stem=stem,case=c,reps=reps,command=cmd,sha256=sha(data)));(OUT/'profiles.json').write_text(json.dumps(profiles,indent=2)+'\n');print(stem,'profile complete',flush=True)
 audit()
if __name__=='__main__':audit() if '--audit' in sys.argv else run()
