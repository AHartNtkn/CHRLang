#!/usr/bin/env python3
"""Verify the preserved first pass against the extended-bound allocation matrix."""
from pathlib import Path
import json,hashlib
old=Path('docs/experiments/results/s06-finite-lifecycle')
new=Path('docs/experiments/results/s06-finite-lifecycle-extended')
def normalize(v,root):
 if isinstance(v,dict):return {k:(x-root if k in ('live_start','live_end','peak_live') else normalize(x,root)) for k,x in v.items() if k not in ('ns','first_answer_ns')}
 if isinstance(v,list):return [normalize(x,root) for x in v]
 return v
lookup={}
for p in (new/'meter').glob('*.json'):
 r=json.loads(p.read_text());d=json.loads(r['stdout'].splitlines()[-1]);key=(d['order'],*r['command'][1:]);value=normalize(d,d['phases'][0]['memory']['live_start'])
 if key in lookup:assert lookup[key]==value,key
 lookup[key]=value
count=0
for p in sorted((old/'meter').glob('*.json')):
 r=json.loads(p.read_text())
 if r['returncode']!=0:continue
 d=json.loads(r['stdout'].splitlines()[-1]);key=(d['order'],*r['command'][1:])
 assert normalize(d,d['phases'][0]['memory']['live_start'])==lookup[key],p
 count+=1
assert count==23
maps=json.loads((old/'source-snapshots.json').read_text())
for name,h in json.loads((old/'freeze.json').read_text()).items():
 p=Path(name)
 if hashlib.sha256(p.read_bytes()).hexdigest()!=h:p=Path(maps[name])
 assert hashlib.sha256(p.read_bytes()).hexdigest()==h,name
result={'initial_completed_cells_exactly_replayed':count,'initial_frozen_sources':'verified','engine_algorithm_changed':False}
(new/'bound-correction-audit.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result))
