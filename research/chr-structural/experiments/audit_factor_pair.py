#!/usr/bin/env python3
"""Audit the allocation-only factoring attribution against the frozen prior owner gate."""
import hashlib,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OLD=ROOT/'docs/experiments/results/s06-finite-cost-ownership-gate'
NEW=ROOT/'docs/experiments/results/s06-overlap-factor-attribution'
FIELDS=['mode','family','width','retain','cancel','rep']
def records(folder):
    rows=[json.loads(x) for x in (folder/'runs.jsonl').open()]
    result={tuple(r[k] for k in FIELDS):r for r in rows}
    assert len(result)==len(rows)==720
    return result

def normalized(r):
    base=r['rows'][0]['memory']['live_start'];out=[]
    for x in r['rows']:
        m=x['memory'].copy()
        for field in ['live_start','live_end','peak_live']:m[field]-=base
        out.append(dict(phase=x['phase'],query=x['query'],**m))
    return out
for folder in [OLD,NEW]:
    files=['freeze.sha256']+(['driver.sha256'] if folder==NEW else [])
    for file in files:
        for line in (folder/file).read_text().splitlines():
            digest,name=line.split('  ',1);p=Path(name)
            data=subprocess.check_output(['git','show','12f1a409e:'+str(p.relative_to(ROOT))],cwd=ROOT) if folder==OLD and p.is_relative_to(ROOT) else p.read_bytes()
            assert hashlib.sha256(data).hexdigest()==digest,name
old,new=records(OLD),records(NEW);assert old.keys()==new.keys()
unchanged=0;contrasts=[]
for k,r in new.items():
    a,b=normalized(old[k]),normalized(r)
    assert old[k]['answers']==r['answers']
    if k[0]!='reduced':assert a==b,k;unchanged+=1
    elif k[-1]==0:
        item=dict(zip(FIELDS[:-1],k[:-1]));item.update(before_bytes=sum(x['requested_bytes'] for x in a),after_bytes=sum(x['requested_bytes'] for x in b),before_peak=max(x['peak_live'] for x in a),after_peak=max(x['peak_live'] for x in b),before_prepare=a[0]['requested_bytes'],after_prepare=b[0]['requested_bytes'])
        contrasts.append(item)
assert unchanged==576
receipt=dict(unchanged_control_runs=unchanged,reduced_cells=len(contrasts),contrasts=contrasts)
(NEW/'paired-audit.json').write_text(json.dumps(receipt,indent=2)+'\n')
print({k:v for k,v in receipt.items() if k!='contrasts'})
for r in contrasts:
    if r['width']==6 and not r['cancel'] and not r['retain']:print(r)
