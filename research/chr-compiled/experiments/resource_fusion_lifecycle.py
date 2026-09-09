#!/usr/bin/env python3
import hashlib,itertools,json,pathlib,random,resource,subprocess
ROOT=pathlib.Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s06-resource-fusion-lifecycle';BINS=ROOT/'target/resource-fusion-lifecycle'
MODES=['original-scan','original-special','original-indexed','fused-scan','fused-special','fused-indexed'];FAMILIES=['plain','choices','duplicates','shared','spare']
CONFIGS=list(itertools.product(MODES,FAMILIES,[0,1,4],[1,4]))
def bounds():resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
def run(kind,args,path):
 command=[str(BINS/kind),*map(str,args)]
 try:
  p=subprocess.run(command,cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=bounds);r={'command':command,'returncode':p.returncode,'stdout':p.stdout,'stderr':p.stderr}
 except subprocess.TimeoutExpired as e:r={'command':command,'timeout':60,'stdout':str(e.stdout),'stderr':str(e.stderr)}
 path.write_text(json.dumps(r,indent=2)+'\n');assert r.get('returncode')==0,path
 return json.loads(r['stdout'])
def main():
 OUT.mkdir(exist_ok=False)
 files=['research/chr-compiled/src/resource_fusion.rs','research/chr-compiled/examples/resource_fusion_cost.rs','research/chr-compiled/examples/support/resource_fusion_source.rs','research/chr-compiled/experiments/meter.rs','docs/experiments/registrations/S06-resource-fusion-lifecycle.md']
 freeze={'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'binaries':{k:hashlib.sha256((BINS/k).read_bytes()).hexdigest() for k in ['meter','time']},'sources':{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in files}}
 (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
 (OUT/'preflight').mkdir()
 for i,c in enumerate(c for c in CONFIGS if c[-1]==1):run('meter',c,OUT/'preflight'/f'{i:03}.json')
 print('90 preflight processes passed',flush=True)
 for kind,reps,seed in [('meter',2,7306),('time',5,7307)]:
  jobs=[(rep,c) for rep in range(reps) for c in CONFIGS];random.Random(seed).shuffle(jobs)
  (OUT/kind).mkdir();(OUT/f'{kind}-order.json').write_text(json.dumps(jobs)+'\n')
  for i,(rep,c) in enumerate(jobs):
   run(kind,c,OUT/kind/f'{i:03}-r{rep}.json')
   if (i+1)%90==0:print(f'{kind}: {i+1}/{len(jobs)} passed',flush=True)
  if kind=='meter':
   from audit_resource_fusion_lifecycle import allocation_audit
   allocation_audit();print('180 exact allocation pairs and all owners pass; starting timing',flush=True)
if __name__=='__main__':main()
