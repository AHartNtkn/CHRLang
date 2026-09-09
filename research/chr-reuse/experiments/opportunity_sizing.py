import hashlib,itertools,json,random,shutil,subprocess,sys,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'research/chr-direct-conditional/experiments'))
import derivation_sizing as runner
from shared_template_analysis import phases,primary
OUT=ROOT/'docs/experiments/results/s05-reuse-opportunity-sizing'
BIN=Path('/tmp/chr-opportunity-021343a1a')
MODES=['direct','exact','alpha','live','scan','sealed','dependencies','templates','compact-exact','compact-alpha','compact-live','lowered']
CONFIGS=[(f,n,2,r,False,w,p) for f,n,r,w,p in itertools.product(['exact','rename','history','history-early','distinct'],[0,32],[False,True],[1,4,16],[0,32])]
def run(kind,index,mode,c,cancel=None):
 binary=BIN/('meter' if kind in ['allocation','cancel-meter'] else 'ordinary')
 command=[str(binary),mode]+[str(int(x)) if isinstance(x,bool) else str(x) for x in c]
 if cancel is not None:command.append(str(cancel))
 path=OUT/f'{kind}-{index:04}.json';assert not path.exists();start=time.monotonic()
 try:
  p=subprocess.run(command,capture_output=True,text=True,timeout=60,preexec_fn=runner.limits)
  raw={'command':command,'returncode':p.returncode,'stdout':p.stdout,'stderr':p.stderr,'wall_seconds':time.monotonic()-start}
 except subprocess.TimeoutExpired as e:
  path.write_text(json.dumps({'command':command,'timeout':True,'stdout':str(e.stdout),'stderr':str(e.stderr)},indent=2)+'\n');raise
 path.write_text(json.dumps(raw,indent=2)+'\n');assert p.returncode==0,path
 v=json.loads(p.stdout.splitlines()[-1]);assert not v['counters'];assert v['meter']==(binary.name=='meter');assert v['alternatives']==c[5] and v['payload_depth']==c[6]
 assert all(x['complete']==(cancel is None or i%2==1) for i,x in enumerate(v['samples']))
 if cancel is None:assert all(x['answers']==c[5] for x in v['samples'])
 return v
def main():
 OUT.mkdir(exist_ok=False)
 paths=['Cargo.toml','Cargo.lock','docs/experiments/registrations/S05-reuse-opportunity-sizing.md',str(Path(__file__).relative_to(ROOT))]
 for d in ['crates/chr-syntax','research/chr-reuse','research/chr-persistent','research/chr-compiled','research/chr-observe','research/chr-direct-choice','research/chr-direct-conditional']:
  paths += [str(p.relative_to(ROOT)) for p in (ROOT/d).rglob('*') if p.is_file() and p.suffix in ['.rs','.toml','.py']]
 f={'commit':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'cpu':runner.CPU,'configs':CONFIGS,'modes':MODES,'sources':{},'binaries':{}}
 for p in sorted(set(paths)):
  dst=OUT/'source'/p;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(ROOT/p,dst);f['sources'][p]=hashlib.sha256(dst.read_bytes()).hexdigest()
 for n in ['ordinary','meter']:
  p=BIN/n;f['binaries'][str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
 (OUT/'freeze.json').write_text(json.dumps(f,indent=2)+'\n')
 check=subprocess.run([str(BIN/'meter'),'meter-check'],capture_output=True,text=True,timeout=60,preexec_fn=runner.limits);(OUT/'meter-check.log').write_text(check.stdout+check.stderr);assert check.returncode==0
 saved={}
 for rep in range(2):
  for i,c in enumerate(CONFIGS):
   for j,m in enumerate(MODES):
    index=12*i+j;v=run('allocation',rep*1440+index,m,c);record=runner.allocation_records(v)
    if rep==0:saved[index]=record
    else:assert saved[index]==record
  print(f'allocation {rep+1}/2',flush=True)
 order=list(range(1440));random.Random(7831).shuffle(order)
 for index in order:
  i,j=divmod(index,12);run('ordinary',index,MODES[j],CONFIGS[i])
 for j,m in enumerate(MODES):
  for kind in ['cancel','cancel-meter']:run(kind,j,m,('history-early',16,2,True,False,16,32),1)
 results=[]
 def read(kind,index):return json.loads(json.loads((OUT/f'{kind}-{index:04}.json').read_text())['stdout'].splitlines()[-1])
 for i,c in enumerate(CONFIGS):
  row={'config':c,'modes':{}}
  for j,m in enumerate(MODES):
   v,a=read('ordinary',i*12+j),read('allocation',i*12+j);ps=list(phases(a).values());all_ps=ps+[a['source_build']]+[x['input_build'] for x in a['samples']]
   row['modes'][m]={'primary_ms':primary(v)/1e6,'inclusive_ms':(primary(v)+v['source_build']['ns']+sum(x['input_build']['ns'] for x in v['samples']))/1e6,'requested_bytes':sum(x['memory']['requested_bytes'] for x in ps),'peak_growth':max(x['memory']['peak_live'] for x in all_ps)-a['source_build']['memory']['live_start'],'phase_ms':{k:p['ns']/1e6 for k,p in phases(v).items()}}
  results.append(row)
 (OUT/'summary.json').write_text(json.dumps({'exploratory':True,'results':results},indent=2)+'\n');print('4344 successful processes; 1440 exact allocation replays',flush=True)
if __name__=='__main__':main()
