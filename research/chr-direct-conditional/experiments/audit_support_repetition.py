#!/usr/bin/env python3
import hashlib,itertools,json
from pathlib import Path
root=Path('docs/experiments/results/s08-support-repetition')
mapping=json.loads((root/'source-snapshots.json').read_text())
for name,h in json.loads((root/'freeze.json').read_text()).items():
    data=Path(name).read_bytes()
    if hashlib.sha256(data).hexdigest()!=h:data=Path(mapping[name]).read_bytes()
    assert hashlib.sha256(data).hexdigest()==h,name
prior=json.loads((root.parent/'s08-stream-allocation-attribution/summary.json').read_text())
for i,c in enumerate(itertools.product(['conditional','inferred'],['aliases','distinct'],[0,1,16,64])):
    records=[]
    for rep in range(2):
        r=json.loads((root/f'{rep}-{i}.json').read_text());assert r['returncode']==0
        assert r['command'][1:]==list(map(str,c));records.append(json.loads(r['stdout']))
    a,b=records;assert a==b
    if c[2]>=16:
        p=next(r for r in prior if (r['mode'],r['family'],r['depth'])==c)
        assert a['ticks']==sum(s[0] for s in p['stages'])
    seen={}
    for op,x,y,cheap,frames,result in a['trace']:
        assert result!=2**64-1
        if cheap:assert frames==1
        key=(op,x,y)
        if key in seen:assert seen[key]==result
        seen[key]=result
print('32 successful processes;16 exact traces;8 prior tick controls;completed roots and frozen sources pass')
