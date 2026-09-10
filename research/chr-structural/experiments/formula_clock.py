"""Qualify clock signal, counter-free semantics and unchanged allocation phases."""
import hashlib,itertools,json,random,resource,subprocess,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s06-formula-clock'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
 resource.setrlimit(resource.RLIMIT_CPU,(60,60));resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
def run(path,args,label):
 p=subprocess.run([str(path),*map(str,args)],capture_output=True,text=True,timeout=60,preexec_fn=limits)
 with (OUT/'runs'/f'{label}.json').open('x') as f:json.dump({'args':args,'exit_code':p.returncode,'stdout':p.stdout,'stderr':p.stderr},f)
 p.check_returncode();return [json.loads(x) for x in p.stdout.splitlines()]
def normalize(rows):
 phases=[r for r in rows if 'phase' in r];baseline=phases[0]['memory']['live_start']
 return [{'phase':r['phase'],'memory':{k:v-baseline if k in ['live_start','live_end','peak_live'] else v for k,v in r['memory'].items()}} for r in phases]
def main():
 assert not (OUT/'freeze.json').exists(),'already frozen'
 binaries={}
 for mode,features in [('meter','alloc-meter'),('phase','alloc-meter,phase-clock'),('ordinary',''),('session','session-clock')]:
  cmd=['cargo','build','-p','chr-structural','--release','--no-default-features','--example','diagram_ownership','--message-format=json','--target-dir',str(ROOT/f'target/s06-formula-clock-{mode}')]
  if features:cmd+=['--features',features]
  p=subprocess.run(cmd,cwd=ROOT,text=True,capture_output=True,timeout=300);(OUT/f'build-{mode}.jsonl').write_text(p.stdout);(OUT/f'build-{mode}.log').write_text(p.stderr);p.check_returncode()
  binaries[mode]=Path(next(json.loads(x)['executable'] for x in p.stdout.splitlines() if json.loads(x).get('executable') and json.loads(x)['target']['name']=='diagram_ownership'))
 files=[Path(__file__),ROOT/'research/chr-structural/examples/diagram_ownership.rs',ROOT/'research/chr-structural/Cargo.toml',ROOT/'research/chr-compiled/experiments/meter.rs',ROOT/'docs/experiments/registrations/S06-formula-clock.md',ROOT/'Cargo.lock',ROOT/'Cargo.toml']
 for directory in ['research/chr-structural/src','crates/chr-syntax/src']:files+=list((ROOT/directory).rglob('*.rs'))
 freeze={'sources':{str(p.relative_to(ROOT)):sha(p) for p in files},'binaries':{k:{'path':str(p.relative_to(ROOT)),'sha256':sha(p)} for k,p in binaries.items()},'rustc':subprocess.check_output(['rustc','-Vv'],text=True)}
 (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n');(OUT/'runs').mkdir()
 p=subprocess.run([str(binaries['meter']),'meter-check'],capture_output=True,text=True,timeout=60,preexec_fn=limits);(OUT/'meter-check.log').write_text(p.stdout+p.stderr);p.check_returncode()
 clocks=[run(binaries['session'],['clock-check'],f'clock-{i}')[0] for i in range(5)]
 (OUT/'clocks.json').write_text(json.dumps(clocks,indent=2)+'\n')
 cases=[(mode,family,n,3,q,'member' if n==3 else 'full','all') for mode,family,n,q in itertools.product(['reduced','diagram','names','projected','symbolic','explicit'],['free','star','clique','overlap'],[3,6],[1,16,128])]
 random.Random(73061).shuffle(cases);(OUT/'cases.json').write_text(json.dumps(cases,indent=2)+'\n')
 oldcases=json.loads((ROOT/'docs/experiments/results/s06-simplified-ownership/configurations.json').read_text());index={tuple(args):i for i,args in enumerate(oldcases)}
 start=time.monotonic();results=[]
 for i,args in enumerate(cases):
  assert time.monotonic()-start<1800,'campaign bound'
  measured=[run(binaries[m],args,f'{i}-{m}-{rep}') for m in ['meter','phase'] for rep in range(2)]
  old=json.loads((ROOT/f'docs/experiments/results/s06-simplified-ownership/runs/{index[args]}-0.json').read_text());prior=[json.loads(x) for x in old['stdout'].splitlines()]
  expected=normalize(prior)
  assert all(normalize(r)==expected for r in measured),(i,args,'allocation changed')
  assert all(r[0]==prior[0] for r in measured)
  assert all(p['ns'] is not None and p['ns']>=0 for r in measured[2:] for p in r[1:])
  ordinary=run(binaries['ordinary'],args,f'{i}-ordinary')
  assert ordinary[0]==dict(prior[0],meter=False) and all(p['memory'] is None and p['ns'] is None for p in ordinary[1:])
  timings=[]
  for rep in range(5):
   r=run(binaries['session'],args,f'{i}-session-{rep}');assert len(r)==2 and r[0]==ordinary[0] and r[1]['session_ns']>0
   timings.append(r[1]['session_ns'])
  results.append({'args':args,'session_ns':timings})
  if (i+1)%24==0:print(f'qualified {i+1}/{len(cases)}',flush=True)
 threshold=200*max(c['median'] for c in clocks)
 for r in results:r['qualified']=min(r['session_ns'])>=threshold
 (OUT/'sizing.json').write_text(json.dumps(results,indent=2)+'\n')
 audit={'cells':len(cases),'workload_processes':len(cases)*10,'clock_processes':5,'empty_medians_ns':[c['median'] for c in clocks],'signal_threshold_ns':threshold,'qualified_cells':sum(r['qualified'] for r in results),'allocation_matches':True,'primary_confirmation':False,'elapsed_seconds':time.monotonic()-start}
 (OUT/'audit.json').write_text(json.dumps(audit,indent=2)+'\n');print(json.dumps(audit))
if __name__=='__main__':main()
