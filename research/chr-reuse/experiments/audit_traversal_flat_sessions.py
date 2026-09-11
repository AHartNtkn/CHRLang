"""Verify the single-alternative repeated-session comparisons."""
import collections
import gzip
import hashlib
import json
import statistics
import sys
import zipfile
sys.dont_write_bytecode=True
from audit_traversal_lifecycle import ROOT, norm
OUT=ROOT/'docs/experiments/results/s08-traversal-flat-sessions'


def main():
    f=json.loads((OUT/'freeze.json').read_text());sha=lambda b:hashlib.sha256(b).hexdigest()
    assert sha((OUT/'sources.zip').read_bytes())==f['archive_sha256']
    with zipfile.ZipFile(OUT/'sources.zip') as z:
        for p,h in f['sources'].items():assert sha(z.read(p))==h
    with gzip.open(OUT/'runs.jsonl.gz','rt') as stream:runs=[json.loads(s) for s in stream]
    assert len(runs)==220 and [r['job'] for r in runs]==f['jobs']
    cells=collections.defaultdict(dict)
    for r in runs:
        j,c=r['job'],r['job']['case'];assert r['raw']['exit_code']==0
        rows=[json.loads(s) for s in r['raw']['stdout'].splitlines()];assert len(rows)==50
        assert all(d['validated'] for d in rows)
        for d in rows:
            assert collections.Counter(x['phase'] for x in d['records'])==dict(prepare=1,setup=2,produce=2,consumer=2,exhaustion=2,producer_drop=2,prepared_drop=1,consumer_drop=1)
        cells[c['mode'],c['enabled'],c['resource'],c['keep']][j['kind'],j['rep']]=rows
    totals={}
    for key,data in cells.items():
        assert set(data)=={('time',i) for i in range(9)}|{('meter',i) for i in range(2)}
        heaps=[norm(s) for i in range(2) for s in data['meter',i]]
        assert all(h==heaps[0] for h in heaps),key
        totals[key]=[sum(x['reading']['ns'] for s in data['time',i] for x in s['records']) for i in range(9)]
    comparisons=[]
    for (mode,enabled,resource,keep),times in sorted(totals.items()):
        if mode=='direct':continue
        controls=[('direct',('direct',False,resource,keep))]
        if enabled:controls.append(('uncached',(mode,False,resource,keep)))
        for control,key in controls:
            ratios=[a/b for a,b in zip(times,totals[key])];m=statistics.median(ratios)
            disposition='lower' if m<.9 and max(ratios)<1 else 'higher' if m>1.1 and min(ratios)>1 else 'unresolved'
            comparisons.append(dict(mode=mode,enabled=enabled,resource=resource,keep=keep,control=control,median=m,minimum=min(ratios),maximum=max(ratios),disposition=disposition))
    result=dict(processes=220,complete_lifecycles=11000,exact_metered_lifecycles=2000,owner_restoration=True,comparisons=comparisons)
    (OUT/'analysis.json').write_text(json.dumps(result,indent=2)+'\n')
    print('Verified 220 processes, 11,000 complete lifecycles, 2,000 exact metered lifecycles')
    for control in ['direct','uncached']:
        for mode in ['dependencies','templates']:
            rs=[r for r in comparisons if r['control']==control and r['mode']==mode]
            print(mode,control,dict(collections.Counter(r['disposition'] for r in rs)))
    for r in comparisons:print(r)


if __name__=='__main__':main()
