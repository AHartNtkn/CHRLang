"""Paired ordinary timing with exact repaired ownership qualification."""
import sys
sys.dont_write_bytecode=True
import gzip,itertools,json,random,shutil,statistics,subprocess,zipfile
from recognition_ownership import ROOT,CPU,sha,limits,normalized,MODES
OUT=ROOT/'docs/experiments/results/s05-recognition-stride-timing'
OLD=ROOT/'docs/experiments/results/s05-recognition-stride-exact-ownership'
def invoke(binary,case):
 r=subprocess.run([str(ROOT/binary)]+[str(x).lower() for x in case],capture_output=True,text=True,timeout=60,preexec_fn=limits);assert r.returncode==0,(case,r.stderr);return list(map(json.loads,r.stdout.splitlines()))
def audit_entries():
 old={tuple(r['job']['case']):normalized(r['rows']) for r in map(json.loads,gzip.open(OLD/'runs.jsonl.gz','rt'))}
 rows=list(map(json.loads,gzip.open(OUT/'entries.jsonl.gz','rt')));assert len(rows)==2048;seen=set()
 for r in rows:
  k=tuple(r['case']);assert k not in seen;seen.add(k);assert normalized(r['rows'])==old[k],k
 assert seen==set(old)
 return old
def audit():
 f=json.loads((OUT/'freeze.json').read_text());assert sha(OUT/'sources.zip')==f['source_hash'];assert sha(ROOT/f['binary'])==f['binary_hash'];assert sha(ROOT/f['meter'])==f['meter_hash'];entries=audit_entries()
 rows=list(map(json.loads,gzip.open(OUT/'runs.jsonl.gz','rt')));assert len(rows)==20480 and [r['job'] for r in rows]==f['jobs'];cells={}
 for r in rows:
  j=r['job'];h,*p=r['rows'];case=(MODES[j['variant']],*j['group']);old=entries[case];assert h['counts']==old[0]['counts'] and h['retained']==old[0]['retained'];assert h['requested_bytes'] is None and h['consumer_bytes'] is None and h['unreleased_bytes'] is None
  assert [(x['phase'],x['query']) for x in p]==[(x['phase'],x['query']) for x in old[1:]];assert all(x['memory'] is None and x['ns']>=0 for x in p)
  cell=cells.setdefault((tuple(j['group']),j['variant']),{});assert j['block'] not in cell;cell[j['block']]=sum(x['ns'] for x in p)
 assert len(cells)==2048 and all(set(c)==set(range(10)) for c in cells.values())
 floor=100*max(x['p99_ns'] for x in f['clocks']);result=[]
 for g in sorted({k[0] for k in cells}):
  for a,b in itertools.combinations(range(8),2):
   ratios=[cells[g,b][i]/cells[g,a][i] for i in range(10)];m=statistics.median(ratios);qualified=min(statistics.median(cells[g,a].values()),statistics.median(cells[g,b].values()))>=floor
   result.append(dict(group=g,reference=MODES[a],candidate=MODES[b],median=m,min=min(ratios),max=max(ratios),clock_qualified=qualified,verdict='gain' if qualified and m<=.9 and max(ratios)<1 else 'loss' if qualified and m>=1.1 and min(ratios)>1 else 'unresolved'))
 (OUT/'analysis.json').write_text(json.dumps(result,indent=2)+'\n');print('Audited2048 entry checks,20480 sessions,7168 contrasts; clock floor ns',floor)
def run():
 OUT.mkdir(parents=True,exist_ok=True);assert not (OUT/'freeze.json').exists();old=json.loads((OLD/'freeze.json').read_text());assert sha(ROOT/old['binary'])==old['binary_hash']
 cmd=['cargo','build','-p','chr-reuse','--release','--no-default-features','--example','inert_ownership'];r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True);(OUT/'build.log').write_text(r.stderr);assert r.returncode==0,r.stderr
 b=ROOT/'target/recognition-timing';shutil.copy2(ROOT/'target/release/examples/inert_ownership',b)
 clocks=[invoke(str(b.relative_to(ROOT)),['clock-check'])[0] for _ in range(3)]
 groups=list(itertools.product(range(8),[4,32],[1,4],['0','all'],[0,1],[False,True]));rng=random.Random(7508);jobs=[]
 for block in range(10):
  order=list(enumerate(groups));rng.shuffle(order)
  for i,g in order:
   for v in [(i+block+j)%8 for j in range(8)]:jobs.append(dict(group=g,variant=v,block=block))
 paths=subprocess.check_output(['git','ls-files','research/chr-reuse','research/chr-direct-choice','research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax','Cargo.toml','Cargo.lock'],text=True).splitlines()+[str(__file__).removeprefix(str(ROOT)+'/'),'docs/experiments/registrations/S05-recognition-stride-timing.md']
 with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
  for p in sorted(set(paths)):z.write(ROOT/p,p)
 (OUT/'freeze.json').write_text(json.dumps(dict(command=cmd,binary=str(b.relative_to(ROOT)),binary_hash=sha(b),meter=old['binary'],meter_hash=old['binary_hash'],source_hash=sha(OUT/'sources.zip'),jobs=jobs,clocks=clocks,cpu=CPU,rustc=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2)+'\n')
 with gzip.open(OUT/'entries.jsonl.gz','wt') as out:
  for m,g in itertools.product(MODES,groups):out.write(json.dumps(dict(case=[m,*g],rows=invoke(old['binary'],[m,*g])))+'\n')
 audit_entries();print('2048 exact ownership entries passed before timing',flush=True)
 with gzip.open(OUT/'runs.jsonl.gz','wt') as out:
  for i,j in enumerate(jobs):
   out.write(json.dumps(dict(job=j,rows=invoke(str(b.relative_to(ROOT)),[MODES[j['variant']],*j['group']])))+'\n');out.flush()
   if (i+1)%2048==0:print('block',(i+1)//2048,'complete',flush=True)
 audit()
if __name__=='__main__':audit() if '--audit' in sys.argv else run()
