"""Registered counter-free timing plus paired allocation gate; no engine changes."""
import hashlib,itertools,json,os,random,resource,shutil,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s02-multihead-lifecycle'
BUILD=ROOT/'target/s02-multihead-lifecycle'
MODES=['local-scan','tuples','partial','scan','indexed','special-scan','special-indexed']
FAMILIES=['sparse','broad','nested','cold','dense','three']
CELLS=list(itertools.product(MODES,FAMILIES,[4,16,64],[1,4]))
def bounds():
 resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(30,30))
def execute(command,path):
 try:
  p=subprocess.run(list(map(str,command)),cwd=ROOT,text=True,capture_output=True,timeout=45,preexec_fn=bounds)
  receipt=dict(command=list(map(str,command)),code=p.returncode,stdout=p.stdout,stderr=p.stderr)
 except subprocess.TimeoutExpired as e:receipt=dict(command=list(map(str,command)),timeout=True,stdout=str(e.stdout),stderr=str(e.stderr))
 path.write_text(json.dumps(receipt)+'\n');assert receipt.get('code')==0,path
 return json.loads(receipt['stdout'])
def build(kind,features):
 cmd=['cargo','build','--release','-p','chr-relational','--no-default-features','--test','multihead_lifecycle','--message-format=json']
 if features:cmd+=['--features',features]
 p=subprocess.run(cmd,cwd=ROOT,text=True,capture_output=True,timeout=120)
 (OUT/(kind+'-build.jsonl')).write_text(p.stdout);(OUT/(kind+'-build.log')).write_text(p.stderr);assert p.returncode==0
 artifacts=[json.loads(l) for l in p.stdout.splitlines()]
 source=[a['executable'] for a in artifacts if a.get('reason')=='compiler-artifact' and a.get('target',{}).get('name')=='multihead_lifecycle' and a.get('executable')]
 assert len(source)==1
 binary=BUILD/kind;shutil.copy2(source[0],binary)
 return binary

def allocation(x):
 phases=x['phases'];memory=[p['measurement']['memory'] for p in phases]
 assert all(a['live_end']==b['live_start'] for a,b in zip(memory,memory[1:]))
 assert memory[0]['live_start']==memory[-1]['live_end']
 prep=next(p['measurement']['memory']['live_end'] for p in phases if p['phase']=='prepare')
 for p in phases:
  m=p['measurement']['memory'];assert m['peak_live']>=max(m['live_start'],m['live_end'])
  if p['phase'] in ['input-drop','cancel-input-drop']:assert m['live_end']==prep
 return [{k:v for k,v in p.items() if k!='ns'} for p in memory]
def normalized(m,base):return {k:(v-base if k in ['live_start','live_end','peak_live'] else v) for k,v in m.items()}
def old_comparison(x,old):
 phases=[p for p in x['phases'] if p['phase'] not in ['source-build','source-drop']]
 assert [p['phase'] for p in phases]==[p['phase'] for p in old['phases']]
 new_base=phases[0]['measurement']['memory']['live_start'];old_base=old['phases'][0]['memory']['live_start']
 assert [normalized(p['measurement']['memory'],new_base) for p in phases]==[normalized(p['memory'],old_base) for p in old['phases']],(x['mode'],x['family'],x['width'],x['reuse'])
def main():
 OUT.mkdir(exist_ok=False);BUILD.mkdir(exist_ok=False)
 meter=build('meter','alloc-meter');time=build('time','')
 for name,binary in [('meter',meter),('time',time)]:
  p=subprocess.run([str(binary)],text=True,capture_output=True,timeout=45);assert p.returncode==0 and '42 independent' in p.stdout
  (OUT/(name+'-smoke.log')).write_text(p.stdout+p.stderr)
  execute([binary,'clock-check'],OUT/(name+'-clock.json'))
 # Freeze all implementation sources used by the adapters, not only the runner.
 inputs=[ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'research/chr-relational/Cargo.toml',Path(__file__).resolve(),ROOT/'docs/experiments/registrations/S02-multihead-lifecycle.md']
 for folder in ['research/chr-relational/tests','research/chr-relational/src','research/chr-compiled/src','research/chr-compiled/experiments','research/chr-persistent/src','research/chr-observe/src','research/chr-direct-conditional/tests/runtime_support','crates/chr-syntax/src']:
  inputs+=list((ROOT/folder).rglob('*.rs'))
 hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(inputs))}
 (OUT/'freeze.json').write_text(json.dumps(dict(sources=hashes,binaries={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in [meter,time]},rustc=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2)+'\n')
 old={}
 for p in (ROOT/'docs/experiments/results/s02-multihead-ownership/runs').glob('*.json'):
  x=json.loads(json.loads(p.read_text())['stdout']);old[tuple(x[k] for k in ['mode','family','width','reuse'])]=x
 assert len(old)==252
 (OUT/'meter').mkdir();jobs=list(itertools.product(range(2),CELLS));random.Random(7280).shuffle(jobs)
 (OUT/'meter-order.json').write_text(json.dumps(jobs)+'\n');seen={}
 for i,(rep,cell) in enumerate(jobs):
  x=execute([meter,*cell],OUT/'meter'/f'{i:04}.json');m=allocation(x);old_comparison(x,old[cell])
  if cell in seen:assert m==seen[cell],cell
  seen[cell]=m
  if (i+1)%84==0:print(f'allocation {i+1}/504',flush=True)
 assert len(seen)==252
 (OUT/'allocation-audit.json').write_text(json.dumps(dict(cells=252,exact_pairs=252,old_comparisons=504,ownership='pass'))+'\n')
 (OUT/'time').mkdir();jobs=list(itertools.product(range(5),CELLS));random.Random(7281).shuffle(jobs)
 (OUT/'time-order.json').write_text(json.dumps(jobs)+'\n')
 for i,(rep,cell) in enumerate(jobs):
  x=execute([time,*cell],OUT/'time'/f'{i:04}.json');assert all('memory' not in p['measurement'] for p in x['phases'])
  if (i+1)%126==0:print(f'timing {i+1}/1260',flush=True)
 for p,h in hashes.items():assert hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h,p
 (OUT/'complete.json').write_text(json.dumps(dict(allocation=504,timing=1260,cells=252))+'\n')
if __name__=='__main__':main()
