"""Audit completed-traversal source freezes, repeated work and parent controls."""
import hashlib
import json
import zipfile
from continuing_lifecycle_entry import ROOT, BASE, sha

OUT=ROOT/'docs/experiments/results/s08-completed-traversal'

def read(p):return json.loads(p.read_text())
def data(x):
    assert x['raw']['exit_code']==0,(x['job'],x['raw']['stderr'])
    return [json.loads(line) for line in x['raw']['stdout'].splitlines()]

def main():
    f=read(OUT/'freeze.json');assert sha(OUT/'sources.zip')==f['archive_sha256']
    with zipfile.ZipFile(OUT/'sources.zip') as z:
        for p,h in f['sources'].items():assert hashlib.sha256(z.read(p)).hexdigest()==h
    for b in f['binaries'].values():assert sha(ROOT/b['path'])==b['sha256']
    runs=[read(OUT/f'run-{i}.json') for i in range(40)]
    assert [x['job'] for x in runs]==f['jobs']
    cells={}
    for x in runs:
        j=x['job'];key=tuple(j[k] for k in ['mode','resource','marker','demand','enabled']);d=data(x)
        assert d[-1]['answers']==j['demand']
        if key in cells:assert cells[key]==d
        cells[key]=d
    parents=[read(BASE/f'graph-{i}.json') for i in range(20)]
    for p in parents:
        key=tuple(p['job'][k] for k in ['mode','resource','marker','demand'])+(False,)
        assert cells[key]==data(p),key
    summary=[]
    for key,off in cells.items():
        if key[-1]:continue
        on=cells[key[:-1]+(True,)]
        assert len(off)==len(on)
        for a,b in zip(off,on):
            assert {k:v for k,v in a.items() if k not in ['force','validation']}=={k:v for k,v in b.items() if k not in ['force','validation']},key
        a,b=off[-1],on[-1]
        summary.append(dict(mode=key[0],resource=key[1],marker=key[2],demand=key[3],control=a,shared=b,force_ratio=b['force']/a['force'],validation_ratio=b['validation']/a['validation']))
    report=dict(processes=len(runs),exact_128_repeats=True,exact_parent_controls=True,nontraversal_work_and_graph_counts_unchanged=True,rows=summary)
    (OUT/'audit.json').write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps(report,indent=2))

if __name__=='__main__':main()
