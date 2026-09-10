#!/usr/bin/env python3
import hashlib,itertools,json
from pathlib import Path
root=Path('docs/experiments/results/s08-support-order-gate')
snapshot_file=root/'source-snapshots.json'
snapshots=json.loads(snapshot_file.read_text()) if snapshot_file.exists() else {}
for file,digest in json.loads((root/'freeze.json').read_text()).items():
 path=Path(file)
 if hashlib.sha256(path.read_bytes()).hexdigest()!=digest:
  path=Path(snapshots[file])
 assert hashlib.sha256(path.read_bytes()).hexdigest()==digest,file
rows=json.loads((root/'summary.json').read_text());assert len(rows)==36
for order,policy,family,n in itertools.product(['ascending','ascending-general','descending-general'],['ordinary','combined'],['aliases','distinct'],[0,16,64]):
    receipt=json.loads((root/f'{order}-{policy}-{family}-{n}.json').read_text());assert receipt['returncode']==0
    assert receipt['command'][1:]==['inferred',family,str(n)]
    d=json.loads(receipt['stdout']);assert d['family']==family and d['depth']==n
    r=next(r for r in rows if (r['order'],r['policy'],r['family'],r['depth'])==(order,policy,family,n))
    assert (r['ticks'],r['nodes'],r['jobs'],r['frames'])==(d['ticks'],d['nodes'],len(d['trace']),sum(t[4] for t in d['trace']))
    assert all(t[5]!=2**64-1 for t in d['trace'])
for policy,family,n in itertools.product(['ordinary','combined'],['aliases','distinct'],[0,16,64]):
    a=next(r for r in rows if (r['order'],r['policy'],r['family'],r['depth'])==('ascending',policy,family,n))
    b=next(r for r in rows if (r['order'],r['policy'],r['family'],r['depth'])==('ascending-general',policy,family,n))
    assert all(a[k]==b[k] for k in ['nodes','jobs','frames'])
(root/'audit.json').write_text(json.dumps(dict(processes=36,frozen_inputs=True,summary_rows=True,completed_jobs=True,ascending_observation_controls=12,timing='not_run',allocation='not_measured'),indent=2)+'\n')
print('36 processes;12 isolated ascending observation controls;frozen source and raw summaries pass')
