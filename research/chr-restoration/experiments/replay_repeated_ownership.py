#!/usr/bin/env python3
"""Check that diagnostic-only instrumentation leaves allocation readings identical."""
import hashlib,json,pathlib,shutil
import repeated_ownership as driver
OUT=driver.OUT
replay=OUT/'instrumentation-replay'
replay.mkdir(exist_ok=False)
driver.BIN=driver.ROOT/'target/repeated-reunion-ownership/instrumented-off'
shutil.copy2(driver.ROOT/'target/release/examples/repeated_cost',driver.BIN)
seen=set()
for p in sorted((OUT/'runs').glob('*.json')):
    original=json.loads(p.read_text());args=original['command'][1:];key=tuple(args)
    if key in seen:continue
    seen.add(key)
    got=driver.run(args,replay/f'{len(seen):03}.json')
    assert got==json.loads(original['stdout']),key
sources=['research/chr-restoration/src/reunion.rs','research/chr-restoration/examples/repeated_work.rs']
result={'exact_replayed_cells':len(seen),'binary_sha256':hashlib.sha256(driver.BIN.read_bytes()).hexdigest(),'source_sha256':{p:hashlib.sha256((driver.ROOT/p).read_bytes()).hexdigest() for p in sources}}
assert len(seen)==108
(OUT/'instrumentation-audit.json').write_text(json.dumps(result,indent=2)+'\n')
print('108 instrumentation-off allocation cells match every original reading')
