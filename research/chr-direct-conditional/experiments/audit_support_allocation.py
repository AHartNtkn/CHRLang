#!/usr/bin/env python3
import hashlib
import itertools
import json
from pathlib import Path
root=Path('docs/experiments/results/s08-support-allocation-attribution')
for directory in ['s08-stream-allocation-attribution','s08-support-allocation-attribution']:
    p=root.parent/directory
    mapping=json.loads((p/'source-snapshots.json').read_text())
    for path,h in json.loads((p/'freeze.json').read_text()).items():
        data=Path(path).read_bytes()
        if hashlib.sha256(data).hexdigest()!=h:data=Path(mapping[path]).read_bytes()
        assert hashlib.sha256(data).hexdigest()==h,path
prior=json.loads((root.parent/'s08-stream-allocation-attribution/summary.json').read_text())
cells=list(itertools.product(['conditional','inferred'],['aliases','distinct'],[16,64]))
for i,c in enumerate(cells):
    readings=[]
    for rep in range(2):
        receipt=json.loads((root/f'{rep}-{i}.json').read_text());assert receipt['returncode']==0
        assert receipt['command'][1:]==list(map(str,c))
        readings.append(json.loads(receipt['stdout'].splitlines()[-1]))
    a,b=readings;assert a==b
    assert (a['mode'],a['family'],a['depth'])==c
    old=next(d for d in prior if (d['mode'],d['family'],d['depth'])==c)
    assert a['stages']==old['stages'] and a['snapshots']==old['snapshots']
    assert sum(s[1] for s in a['stages'])==a['execution']['requested_bytes']
    assert sum(s[2] for s in a['stages'])==a['execution']['allocation_calls']
    assert all(sum(x)<=s[1] for x,s in zip(a['support_bytes'],a['stages']))
    assert a['restored']['live_start']==a['restored']['live_end']
print('16 processes, 8 exact replays, 8 unchanged stage/work/owner controls; measured hashes and root restoration pass')
