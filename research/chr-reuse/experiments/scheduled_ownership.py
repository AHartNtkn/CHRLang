"""Registered requested ownership of scheduled and contracted templates."""
import gzip,itertools,json,random,shutil,subprocess,sys,zipfile
from validity_callers import ROOT,CPU,sha,limits,normalized
OUT=ROOT/'docs/experiments/results/s06-scheduled-ownership'
MODES=['dependencies','templates-zero','templates','direct','sealed','carriers']
def audit():
 f=json.loads((OUT/'freeze.json').read_text());assert sha(OUT/'sources.zip')==f['source_hash']
 for b in f['builds'].values():assert sha(ROOT/b['binary'])==b['sha256']
 rows=list(map(json.loads,gzip.open(OUT/'runs.jsonl.gz','rt')));assert len(rows)==2304 and [r['job'] for r in rows]==f['jobs'];cells={}
 for r in rows:
  j,d=r['job'],r['result'];assert d['validated'] and not d['profiled'];h=normalized(d);key=(j['scheduled'],tuple(j['case']));value=(h,d['answers'])
  if key in cells:assert cells[key]==value,key
  cells[key]=value
 assert len(cells)==1152
 summary=[]
 for (scheduled,case),(h,answers) in sorted(cells.items()):
  if case[0] in ['direct','sealed','carriers']:assert (h,answers)==cells[not scheduled,case]
  assert answers==cells[not scheduled,case][1]
  summary.append(dict(scheduled=scheduled,case=case,answers=answers,requested=sum(p['requested_bytes'] for p in h),peak=max(p['peak_live'] for p in h),phases=h))
 (OUT/'analysis.json').write_text(json.dumps(summary,indent=2)+'\n');print('Audited 2304 processes,1152 exact layout pairs and 288 cross-build explicit-control pairs')
def run():
 OUT.mkdir(parents=True,exist_ok=True);assert not (OUT/'freeze.json').exists();builds={}
 for scheduled in [False,True]:
  features='alloc-meter,carrier-contraction,chr-direct-choice/completed-traversal,chr-direct-choice/seek-context'+(',chr-direct-choice/scheduled-templates' if scheduled else '')
  cmd=['cargo','build','-p','chr-reuse','--release','--no-default-features','--features',features,'--example','validity_sources'];r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True);(OUT/f'build-{scheduled}.log').write_text(r.stderr);assert r.returncode==0,r.stderr
  b=ROOT/f'target/scheduled-ownership-{scheduled}';shutil.copy2(ROOT/'target/release/examples/validity_sources',b);builds[str(scheduled)]=dict(binary=str(b.relative_to(ROOT)),sha256=sha(b),command=cmd);print('built',scheduled,flush=True)
 sources=list(itertools.product(['chain','repeated','distinct'],[False,True],[False,True],[8,32],[False,True],[False,True]))
 jobs=[dict(scheduled=s,case=[m,*c],capacity=cap) for s,m,c,cap in itertools.product([False,True],MODES,sources,[9,73])];assert len(jobs)==2304;random.Random(7306).shuffle(jobs)
 paths=subprocess.check_output(['git','ls-files','research/chr-reuse','research/chr-direct-choice','research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax','Cargo.toml','Cargo.lock'],text=True).splitlines()+[str(__file__).removeprefix(str(ROOT)+'/'),'docs/experiments/registrations/S06-scheduled-ownership.md']
 with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
  for p in sorted(set(paths)):z.write(ROOT/p,p)
 (OUT/'freeze.json').write_text(json.dumps(dict(builds=builds,jobs=jobs,cpu=CPU,source_hash=sha(OUT/'sources.zip'),rustc=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2)+'\n')
 with gzip.open(OUT/'runs.jsonl.gz','wt') as out:
  for i,j in enumerate(jobs):
   c=j['case'];cmd=[str(ROOT/builds[str(j['scheduled'])]['binary'])]+[str(v).lower() for v in c[:-1]]+['1',str(j['capacity']),str(c[-1]).lower()]
   r=subprocess.run(cmd,capture_output=True,text=True,timeout=60,preexec_fn=limits)
   if r.returncode:(OUT/'failure.json').write_text(json.dumps(dict(job=j,stderr=r.stderr),indent=2)+'\n')
   assert r.returncode==0,(j,r.stderr);d=json.loads(r.stdout);out.write(json.dumps(dict(job=j,result=d))+'\n');out.flush()
   if (i+1)%288==0:print(i+1,'complete',flush=True)
 audit()
if __name__=='__main__':audit() if '--audit' in sys.argv else run()
