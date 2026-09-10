"""Prospectively bounded paired allocation matrix with frozen inputs."""
from pathlib import Path
import gzip,hashlib,itertools,json,resource,subprocess
ROOT=Path(__file__).resolve().parents[3];RAW=ROOT/'docs/experiments/results/s05-inert-ownership/compact';BIN=ROOT/'target/s05-inert-owner-compact/release/examples/inert_ownership'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
 resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
if __name__=='__main__':
 RAW.mkdir(exist_ok=False)
 cells=list(itertools.product(['compact'],range(6),[0,4],[1,4],['0','1','all'],[0,1]));assert len(cells)==144
 files=[Path(__file__),BIN,ROOT/'research/chr-reuse/examples/inert_ownership.rs',ROOT/'research/chr-reuse/examples/support/inert_source.rs',ROOT/'research/chr-reuse/src/residuals.rs',ROOT/'research/chr-reuse/src/continuations.rs',ROOT/'research/chr-reuse/Cargo.toml',ROOT/'research/chr-compiled/experiments/meter.rs',ROOT/'research/chr-direct-conditional/tests/runtime_support/mod.rs',ROOT/'docs/experiments/registrations/S05-inert-ownership.md',ROOT/'docs/experiments/registrations/S05-inert-ownership-compact-control.md',ROOT/'Cargo.lock']
 files+=list((ROOT/'research/chr-persistent/src').glob('*.rs'))+list((ROOT/'research/chr-observe/src').rglob('*.rs'))
 with (RAW/'manifest.json').open('x') as f:json.dump(dict(cells=cells,binary=str(BIN.relative_to(ROOT)),sha256={str(p.relative_to(ROOT)):digest(p) for p in files}),f,indent=2)
 result=[]
 for i,cell in enumerate(cells):
  pair=[]
  for rep in range(2):
   cmd=[str(BIN),*map(str,cell)];p=subprocess.run(cmd,capture_output=True,text=True,timeout=60,preexec_fn=limits)
   receipt=dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
   path=RAW/f'run-{i:03}-{rep}.json.gz';assert not path.exists();path.write_bytes(gzip.compress((json.dumps(receipt)+'\n').encode(),mtime=0));assert p.returncode==0 and not p.stderr,receipt
   data=[json.loads(l) for l in p.stdout.splitlines()];head,*rows=data
   assert head['unreleased_bytes']==0
   assert head['counts']==[1 if cell[-1] or cell[1]==5 else 2]*cell[3]
   assert head['requested_bytes']==sum(r['memory']['requested_bytes'] for r in rows)
   base=rows[0]['memory']['live_start'];assert rows[-1]['memory']['live_end']==base
   assert head['consumer_bytes']==rows[-2]['memory']['live_end']-base
   if cell[4]=='0':
    assert head['consumer_bytes']==0
    assert all(r['memory']['live_end']==rows[2]['memory']['live_end'] for r in rows if r['phase']=='input_dispose')
   pair.append(data)
  assert pair[0]==pair[1],cell
  head,*rows=pair[0];base=rows[0]['memory']['live_start']
  result.append(dict(cell=cell,**head,peak_live_bytes=max(r['memory']['peak_live'] for r in rows)-base,phases=rows))
  if (i+1)%96==0:print(i+1,'/144 pairs verified',flush=True)
 for p,h in json.loads((RAW/'manifest.json').read_text())['sha256'].items():assert digest(ROOT/p)==h,p
 (RAW/'audit.json').write_text(json.dumps(dict(status='passed',processes=288,cells=result),indent=2)+'\n');print('Verified all pairs, independent endpoints and ownership conservation.')
