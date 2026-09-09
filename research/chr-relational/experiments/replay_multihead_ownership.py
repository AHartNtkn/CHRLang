#!/usr/bin/env python3
import hashlib,json,pathlib,shutil
import multihead_ownership as driver
out=driver.OUT/'runner-replay';out.mkdir(exist_ok=False)
build=[json.loads(x) for x in pathlib.Path('/tmp/chr-multihead-replay-build.jsonl').read_text().splitlines()]
source=[x['executable'] for x in build if x.get('reason')=='compiler-artifact' and x.get('target',{}).get('name')=='multihead_ownership' and x.get('executable')];assert len(source)==1
driver.BIN=driver.ROOT/'target/multihead-ownership/smoke-enabled';shutil.copy2(source[0],driver.BIN)
seen=set()
for p in sorted((driver.OUT/'runs').glob('*.json')):
    old=json.loads(p.read_text());args=old['command'][1:];key=tuple(args)
    if key in seen:continue
    seen.add(key);got=driver.run(args,out/f'{len(seen):03}.json');assert got==json.loads(old['stdout']),key
assert len(seen)==252
(driver.OUT/'runner-replay-audit.json').write_text(json.dumps({'exact_cells':252,'binary_sha256':hashlib.sha256(driver.BIN.read_bytes()).hexdigest(),'runner_sha256':hashlib.sha256((driver.ROOT/'research/chr-relational/tests/multihead_ownership.rs').read_bytes()).hexdigest()},indent=2)+'\n')
print('252 cells exactly replay after argument-free smoke integration')
