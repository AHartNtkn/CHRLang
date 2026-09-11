"""Verify repeated lifecycle precision and zero-execution sensitivity."""
import hashlib
import json
import statistics
import zipfile
from collections import defaultdict
from pathlib import Path
from audit_readiness_lifecycle import phases, normalized

ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s02-readiness-small-sessions'


def main():
    frozen=json.loads((OUT/'freeze.json').read_text())
    sha=lambda data:hashlib.sha256(data).hexdigest()
    assert sha((OUT/'sources.zip').read_bytes())==frozen['archive_sha256']
    with zipfile.ZipFile(OUT/'sources.zip') as z:
        for p,h in frozen['sources'].items():assert sha(z.read(p))==h
    rows=[json.loads(s) for s in (OUT/'samples.jsonl').read_text().splitlines()];assert len(rows)==88
    cells=defaultdict(dict)
    for row in rows:
        assert row['receipt']['exit_code']==0
        samples=[json.loads(s) for s in row['receipt']['stdout'].splitlines()]
        assert len(samples)==50 and all(s['validated'] for s in samples)
        cells[(row['shared'],row['outcome'],row['mode'])][row['flavor'],row['rep']]=samples
    assert len(cells)==8
    totals={}
    for key,data in cells.items():
        assert set(data)=={('ordinary',i) for i in range(9)}|{('meter',i) for i in range(2)}
        meter=[normalized(s) for i in range(2) for s in data['meter',i]]
        assert all(m==meter[0] for m in meter),key
        totals[key]=[sum(p['ns'] for s in data['ordinary',i] for p in phases(s)) for i in range(9)]
    cancelled = [normalized(cells['true','cancel',m]['meter',0][0]) for m in ['selective','batch8','batch256']]
    assert cancelled[0] == cancelled[1] == cancelled[2]
    assert cancelled[0]['advances'] == [0]
    ratios=[]
    for (shared,outcome,mode),times in sorted(totals.items()):
        if mode=='compiled':continue
        paired=[a/b for a,b in zip(times,totals[shared,outcome,'compiled'])]
        med=statistics.median(paired)
        disposition='lower' if med<.9 and max(paired)<1 else 'higher' if med>1.1 and min(paired)>1 else 'unresolved'
        ratios.append(dict(shared=shared,outcome=outcome,mode=mode,median=med,minimum=min(paired),maximum=max(paired),disposition=disposition))
    # Set the entire measured execution+observation interval to zero. This is an
    # intentionally optimistic bound for optimizing readiness alone, not a run.
    initial=[json.loads(s) for s in (ROOT/'docs/experiments/results/s02-readiness-lifecycle/samples.jsonl').read_text().splitlines()]
    witness={}
    for row in initial:
        if row['flavor']=='ordinary' and row['depth']==64 and row['shared']=='false' and row['outcome']=='fail' and row['count']==16:
            sample=json.loads(row['receipt']['stdout']);total=sum(p['ns'] for p in phases(sample))
            without_execution=total-sum(q['execution_observation']['ns'] for q in sample['queries'])
            witness[row['mode'],row['rep']]=(total,without_execution)
    sensitivity=[]
    for mode in ['selective','batch8','batch256']:
        paired=[witness[mode,i][1]/witness['compiled',i][0] for i in range(5)]
        sensitivity.append(dict(mode=mode,minimum=min(paired),median=statistics.median(paired),maximum=max(paired)))
    result=dict(processes=88,complete_sessions=4400,exact_meter_sessions=800,shared_cancel_identical_ownership_and_zero_advances=True,ratios=ratios,zero_execution_sensitivity=sensitivity)
    (OUT/'analysis.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(result,indent=2))


if __name__=='__main__':main()
