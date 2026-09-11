"""Registered recognition-stride requested ownership, using the existing runner."""
import sys
sys.dont_write_bytecode=True
import gzip,itertools,json,random,shutil,subprocess,zipfile
from validity_callers import ROOT,CPU,sha,limits
from audit_inert_ownership import normalized
EXACT='--exact-output' in sys.argv
OUT=ROOT/('docs/experiments/results/s05-recognition-stride-exact-ownership' if EXACT else 'docs/experiments/results/s05-recognition-stride-ownership')
MODES=['memo','memo4','memo16','separate','direct','scan','indexed','sealed']
def audit():
 f=json.loads((OUT/'freeze.json').read_text());assert sha(OUT/'sources.zip')==f['source_hash'];assert sha(ROOT/f['binary'])==f['binary_hash']
 rows=list(map(json.loads,gzip.open(OUT/'runs.jsonl.gz','rt')));assert len(rows)==4096 and [r['job'] for r in rows]==f['jobs'];cells={}
 for r in rows:
  j=r['job'];case=j['case'];mode,family,depth,reuse,keep,cancel,distinct=case;data=normalized(r['rows']);h,*ph=data
  count=1 if cancel or family in [5,6] else 2
  assert h['counts']==[count]*reuse and h['retained']==(0 if keep=='0' else count*reuse) and h['unreleased_bytes']==0
  phases=[('source',0),('prepare',0),('source_dispose',0)]
  for q in range(reuse):
   phases += [('input',q),('setup',q)]+[x for _ in range(count) for x in [('service_observe',q),('consume',q)]]
   if not cancel:phases.append(('service_observe',q))
   phases += [('engine_dispose',q),('input_dispose',q)]
  phases += [('prepared_dispose',reuse),('consumer_dispose',reuse)]
  assert [(p['phase'],p['query']) for p in ph]==phases and all(p['ns'] is None for p in ph)
  assert ph[0]['memory']['live_start']==ph[-1]['memory']['live_end']==0
  assert all(a['memory']['live_end']==b['memory']['live_start'] for a,b in zip(ph,ph[1:]))
  assert h['requested_bytes']==sum(p['memory']['requested_bytes'] for p in ph)
  assert h['consumer_bytes']==ph[-2]['memory']['live_end']
  key=tuple(case)
  if key in cells:assert cells[key]==data,key
  cells[key]=data
 assert len(cells)==2048
 summary=[]
 for case,data in sorted(cells.items()):
  h,*ph=data;assert h['consumer_bytes']==cells[('direct',*case[1:])][0]['consumer_bytes'];summary.append(dict(case=case,requested=h['requested_bytes'],peak=max(p['memory']['peak_live'] for p in ph),consumer=h['consumer_bytes']))
 (OUT/'analysis.json').write_text(json.dumps(summary,indent=2)+'\n');print('Audited4096 processes,2048 exact pairs,256 scenarios with equal consumer ownership')
def run():
 OUT.mkdir(parents=True,exist_ok=True);assert not (OUT/'freeze.json').exists()
 cmd=['cargo','build','-p','chr-reuse','--release','--no-default-features','--features','alloc-meter','--example','inert_ownership'];r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True);(OUT/'build.log').write_text(r.stderr);assert r.returncode==0,r.stderr
 b=ROOT/('target/recognition-exact-ownership' if EXACT else 'target/recognition-ownership');shutil.copy2(ROOT/'target/release/examples/inert_ownership',b)
 sources=list(itertools.product(range(8),[4,32],[1,4],['0','all'],[0,1],[False,True]));jobs=[dict(case=[m,*s],rep=r) for m,s,r in itertools.product(MODES,sources,range(2))];assert len(jobs)==4096;random.Random(7507).shuffle(jobs)
 paths=subprocess.check_output(['git','ls-files','research/chr-reuse','research/chr-direct-choice','research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax','Cargo.toml','Cargo.lock'],text=True).splitlines()+[str(__file__).removeprefix(str(ROOT)+'/'),'docs/experiments/registrations/S05-recognition-stride-ownership.md']
 if EXACT:paths += ['docs/experiments/registrations/S05-recognition-stride-exact-ownership.md']
 with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
  for p in sorted(set(paths)):z.write(ROOT/p,p)
 (OUT/'freeze.json').write_text(json.dumps(dict(command=cmd,binary=str(b.relative_to(ROOT)),binary_hash=sha(b),source_hash=sha(OUT/'sources.zip'),jobs=jobs,cpu=CPU,rustc=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2)+'\n')
 with gzip.open(OUT/'runs.jsonl.gz','wt') as out:
  for i,j in enumerate(jobs):
   r=subprocess.run([str(b)]+[str(v).lower() for v in j['case']],capture_output=True,text=True,timeout=60,preexec_fn=limits)
   if r.returncode:(OUT/'failure.json').write_text(json.dumps(dict(job=j,stderr=r.stderr),indent=2)+'\n')
   assert r.returncode==0,(j,r.stderr);out.write(json.dumps(dict(job=j,rows=list(map(json.loads,r.stdout.splitlines()))))+'\n');out.flush()
   if (i+1)%512==0:print(i+1,'complete',flush=True)
 audit()
if __name__=='__main__':audit() if '--audit' in sys.argv else run()
