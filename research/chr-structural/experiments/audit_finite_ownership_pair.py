#!/usr/bin/env python3
"""Check the isolated prepared-owner correction against the preserved initial gate."""
import hashlib
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OLD=ROOT/'docs/experiments/results/s06-finite-cost-ownership-initial'
NEW=ROOT/'docs/experiments/results/s06-finite-cost-ownership-gate'
FIELDS=['mode','family','width','retain','cancel','rep']

def records(folder):
    rows=[json.loads(line) for line in (folder/'runs.jsonl').open()]
    result={tuple(r[k] for k in FIELDS):r for r in rows}
    assert len(result)==len(rows)==720
    return result

def normalized(r):
    baseline=r['rows'][0]['memory']['live_start']
    result=[]
    for row in r['rows']:
        m=row['memory'].copy()
        for k in ['live_start','live_end','peak_live']:m[k]-=baseline
        result.append(dict(phase=row['phase'],query=row['query'],**m))
    return result

for folder in [OLD,NEW]:
    for line in (folder/'freeze.sha256').read_text().splitlines():
        digest,name=line.split('  ',1);p=Path(name)
        if folder==OLD and p.is_relative_to(ROOT):p=OLD/'source-files'/p.relative_to(ROOT)
        assert hashlib.sha256(p.read_bytes()).hexdigest()==digest,(folder,name)
old,new=records(OLD),records(NEW)
assert old.keys()==new.keys()
unchanged=0;corrected=[]
for k,r in new.items():
    before,after=normalized(old[k]),normalized(r)
    if k[0]!='lowered':
        assert before==after,k
        unchanged+=1
    else:
        # One small mode string is the only requested heap kept by this control.
        assert after[0]['live_end']==len('lowered'),k
        assert before[0]['live_end']>after[0]['live_end'],k
        # Same cumulative traffic: ownership changes when grammar disposal occurs.
        assert sum(m['requested_bytes'] for m in before)==sum(m['requested_bytes'] for m in after),k
        if k[-1]==0:
            corrected.append(dict(zip(FIELDS[:-1],k[:-1]),prepared_before=before[0]['live_end'],prepared_after=after[0]['live_end'],peak_before=max(m['peak_live'] for m in before),peak_after=max(m['peak_live'] for m in after)))
receipt=dict(unchanged_control_runs=unchanged,corrected_lowered_cells=len(corrected),combined_processes=1440,combined_exact_replays=720,corrections=corrected)
(NEW/'paired-audit.json').write_text(json.dumps(receipt,indent=2)+'\n')
print({k:v for k,v in receipt.items() if k!='corrections'})
