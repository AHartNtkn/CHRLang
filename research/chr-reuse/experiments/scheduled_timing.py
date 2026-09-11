"""Paired complete-session timing, gated by exact scheduled ownership receipts."""
import sys
sys.dont_write_bytecode=True
import gzip,itertools,json,random,shutil,statistics,subprocess,zipfile
from validity_callers import ROOT,CPU,sha,limits,normalized
from scheduled_ownership import MODES
OUT=ROOT/'docs/experiments/results/s06-scheduled-timing'
OLD=ROOT/'docs/experiments/results/s06-scheduled-ownership'
VARIANTS=list(itertools.product([False,True],MODES))
PHASES=['prepare','setup','execute','producer_drop','setup','execute','producer_drop','prepared_drop','consumer_drop']
def invoke(binary,case,sessions):
 cmd=[str(ROOT/binary)]+[str(v).lower() for v in case[:-1]]+[str(sessions),'9',str(case[-1]).lower()]
 r=subprocess.run(cmd,capture_output=True,text=True,timeout=60,preexec_fn=limits);assert r.returncode==0,(cmd,r.stderr)
 rows=list(map(json.loads,r.stdout.splitlines()));assert len(rows)==sessions and all(d['validated'] and not d['profiled'] for d in rows);return rows
def audit_entries():
 old={(r['job']['scheduled'],tuple(r['job']['case'])):(normalized(r['result']),r['result']['answers']) for r in map(json.loads,gzip.open(OLD/'runs.jsonl.gz','rt'))}
 entries=list(map(json.loads,gzip.open(OUT/'entries.jsonl.gz','rt')));assert len(entries)==1152
 seen=set()
 for r in entries:
  k=(r['scheduled'],tuple(r['case']));assert k not in seen;seen.add(k);assert (normalized(r['result']),r['result']['answers'])==old[k],k
 assert seen==set(old)
def audit():
 f=json.loads((OUT/'freeze.json').read_text());assert sha(OUT/'sources.zip')==f['source_hash']
 for b in [*f['builds'].values(),*f['meters'].values()]:assert sha(ROOT/b['binary'])==b['sha256']
 audit_entries();rows=list(map(json.loads,gzip.open(OUT/'runs.jsonl.gz','rt')));assert len(rows)==11520 and [r['job'] for r in rows]==f['jobs'];cells={}
 for r in rows:
  j=r['job'];assert len(r['sessions'])==5;totals=[]
  for d in r['sessions']:
   assert d['validated'] and not d['profiled'];assert [p['phase'] for p in d['phases']]==PHASES;assert all(p['heap'] is None for p in d['phases']);totals.append(sum(p['ns'] for p in d['phases']))
  cell=cells.setdefault((tuple(j['group']),j['variant']),{});assert j['block'] not in cell;cell[j['block']]=statistics.mean(totals)
 assert len(cells)==1152 and all(set(c)==set(range(10)) for c in cells.values())
 warm=list(map(json.loads,gzip.open(OUT/'warmups.jsonl.gz','rt')));assert len(warm)==1152 and all(len(r['sessions'])==5 and all(d['validated'] for d in r['sessions']) for r in warm)
 contrasts=[]
 for g in sorted({k[0] for k in cells}):
  for a,b in itertools.combinations(range(12),2):
   ratios=[cells[g,b][i]/cells[g,a][i] for i in range(10)];m=statistics.median(ratios);contrasts.append(dict(group=g,reference=VARIANTS[a],candidate=VARIANTS[b],median=m,min=min(ratios),max=max(ratios),verdict='gain' if m<=.9 and max(ratios)<1 else 'loss' if m>=1.1 and min(ratios)>1 else 'unresolved'))
 (OUT/'analysis.json').write_text(json.dumps(contrasts,indent=2)+'\n');print('Audited1152 ownership entries,11520 processes,57600 sessions,6336 contrasts')
def run():
 OUT.mkdir(parents=True,exist_ok=True);assert not (OUT/'freeze.json').exists();builds={};meters=json.loads((OLD/'freeze.json').read_text())['builds']
 for b in meters.values():assert sha(ROOT/b['binary'])==b['sha256']
 for scheduled in [False,True]:
  features='carrier-contraction,chr-direct-choice/completed-traversal,chr-direct-choice/seek-context'+(',chr-direct-choice/scheduled-templates' if scheduled else '')
  cmd=['cargo','build','-p','chr-reuse','--release','--no-default-features','--features',features,'--example','validity_sources'];r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True);(OUT/f'build-{scheduled}.log').write_text(r.stderr);assert r.returncode==0,r.stderr
  b=ROOT/f'target/scheduled-timing-{scheduled}';shutil.copy2(ROOT/'target/release/examples/validity_sources',b);builds[str(scheduled)]=dict(binary=str(b.relative_to(ROOT)),sha256=sha(b),command=cmd);print('built',scheduled,flush=True)
 groups=list(itertools.product(['chain','repeated','distinct'],[False,True],[False,True],[8,32],[False,True],[False,True]));jobs=[];rng=random.Random(7307)
 for block in range(10):
  order=list(enumerate(groups));rng.shuffle(order)
  for i,g in order:
   for v in [(i+block+j)%12 for j in range(12)]:jobs.append(dict(group=g,variant=v,block=block))
 paths=subprocess.check_output(['git','ls-files','research/chr-reuse','research/chr-direct-choice','research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax','Cargo.toml','Cargo.lock'],text=True).splitlines()+[str(__file__).removeprefix(str(ROOT)+'/'),'docs/experiments/registrations/S06-scheduled-timing.md']
 with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
  for p in sorted(set(paths)):z.write(ROOT/p,p)
 (OUT/'freeze.json').write_text(json.dumps(dict(builds=builds,meters=meters,jobs=jobs,cpu=CPU,source_hash=sha(OUT/'sources.zip'),rustc=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2)+'\n')
 with gzip.open(OUT/'entries.jsonl.gz','wt') as out:
  for s,m,g in itertools.product([False,True],MODES,groups):out.write(json.dumps(dict(scheduled=s,case=[m,*g],result=invoke(meters[str(s)]['binary'],[m,*g],1)[0]))+'\n')
 audit_entries();print('1152 ownership entries validated before timing',flush=True)
 with gzip.open(OUT/'warmups.jsonl.gz','wt') as out:
  for g in groups:
   for s,m in VARIANTS:out.write(json.dumps(dict(group=g,scheduled=s,mode=m,sessions=invoke(builds[str(s)]['binary'],[m,*g],5)))+'\n')
 with gzip.open(OUT/'runs.jsonl.gz','wt') as out:
  for i,j in enumerate(jobs):
   s,m=VARIANTS[j['variant']];out.write(json.dumps(dict(job=j,sessions=invoke(builds[str(s)]['binary'],[m,*j['group']],5)))+'\n');out.flush()
   if (i+1)%1152==0:print('block',(i+1)//1152,'complete',flush=True)
 audit()
if __name__=='__main__':audit() if '--audit' in sys.argv else run()
