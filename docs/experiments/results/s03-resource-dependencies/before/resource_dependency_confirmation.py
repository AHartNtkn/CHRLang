import hashlib,json,os,pathlib,resource,subprocess,time
root=pathlib.Path.cwd(); out=root/'docs/experiments/results/s03-resource-dependencies'
def digest(p): return hashlib.sha256(p.read_bytes()).hexdigest()
executables=[]
for mode in ['on','off','kernel']:
 for line in (out/f'build-{mode}.jsonl').read_text().splitlines():
  r=json.loads(line)
  if r.get('executable') and r.get('profile',{}).get('test'):
   p=pathlib.Path(r['executable'])
   executables.append({'mode':mode,'name':r['target']['name'],'path':str(p.relative_to(root)),'sha256':digest(p)})
assert any(r['name']=='resource_dependencies' for r in executables)
paths=subprocess.check_output(['git','ls-files','crates','research','Cargo.toml','Cargo.lock'],text=True).splitlines()
paths=[p for p in paths if p.endswith(('.rs','.toml','.lock'))]
paths+=['research/chr-direct-conditional/tests/resource_dependencies.rs','docs/experiments/registrations/S03-resource-dependencies.md','research/chr-direct-conditional/experiments/resource_dependency_confirmation.py']
manifest={'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'sources':{p:digest(root/p) for p in paths},'executables':executables,'scope':'Source and regression correctness only; no comparative timing.'}
(out/'freeze.json').write_text(json.dumps(manifest,indent=2)+'\n')
def bounds():
 resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
results=[]
for e in executables:
 for repeat in range(2 if e['name']=='resource_dependencies' else 1):
  p=root/e['path']; assert digest(p)==e['sha256']
  start=time.monotonic()
  try:
   r=subprocess.run([str(p),'--test-threads=1','--nocapture'],capture_output=True,text=True,timeout=60,preexec_fn=bounds)
   code=r.returncode; output=r.stdout+r.stderr
  except subprocess.TimeoutExpired as error:
   code='timeout';output=str(error)
  log=f"confirmation-{e['mode']}-{e['name']}-{repeat}.log"
  (out/log).write_text(output)
  results.append({'mode':e['mode'],'name':e['name'],'repeat':repeat,'returncode':code,'elapsed_seconds':time.monotonic()-start,'log':log})
  (out/'confirmation.json').write_text(json.dumps(results,indent=2)+'\n')
  print(e['mode'],e['name'],repeat,code,flush=True)
assert all(r['returncode']==0 for r in results),results
assert all(digest(root/p)==sha for p,sha in manifest['sources'].items())
