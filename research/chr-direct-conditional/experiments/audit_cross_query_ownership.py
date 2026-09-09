#!/usr/bin/env python3
"""Reconstruct the registered matrix and verify owners across consumer policies."""
import hashlib
import itertools
import json
from pathlib import Path
import random

root = Path('docs/experiments/results/s08-cross-query-ownership')
configs = [(m,f,n,1,k,0,t) for m,f,n,k,t in itertools.product(
    ['conditional','inferred','scan','resumable'], ['aliases','distinct'], [16,64], ['0','4','all'], [0,1])]
configs += [(m,'aliases',64,1,k,1,0) for m,k in itertools.product(['conditional','inferred','scan','resumable'],['0','4','all'])]
jobs = [(r,*c) for r in range(2) for c in configs]
random.Random(7860).shuffle(jobs)
assert json.loads((root/'order.json').read_text()) == [list(j) for j in jobs]
assert len(list(root.glob('run-*.json'))) == 216
for path,h in json.loads((root/'freeze.json').read_text()).items():
    assert hashlib.sha256(Path(path).read_bytes()).hexdigest() == h,path
records={}
rows=[]
for i,(_,m,f,n,resource,keep,cancel,fail) in enumerate(jobs):
    r = json.loads((root/f'run-{i:03}.json').read_text())
    assert r['returncode'] == 0
    assert r['command'][1:] == list(map(str,[m,f,n,resource,keep,cancel,fail]))
    d = json.loads(r['stdout'].splitlines()[-1])
    assert (d['mode'],d['family'],d['depth'],d['resource'],d['keep'],d['cancel'],d['fail_tail'],d['queries']) == (m,f,n,True,keep,bool(cancel),bool(fail),8)
    assert d['meter'] is True and d['counters'] is False
    key=(m,f,n,keep,cancel,fail)
    if key in records:
        assert records[key] == d
        continue
    records[key]=d
    xs=d['snapshots'];assert xs[0]['label']=='prepared'
    prepared=xs[0]['memory']['live_end']; total=0
    expected_labels=['prepared']
    for q in range(8):
        count=4 if cancel and q==0 else n+q+1-fail
        expected_labels += ['setup']+['delivery']*sum(x<=count for x in [1,4,16,64])+['cancelled' if cancel and q==0 else 'exhausted','engine-disposed']
        group=[s for s in xs if s['query']==q and s['label']!='prepared']
        before=group[-2];after=group[-1]
        assert before['answers']==after['answers']==count
        expected_retained=total if keep=='all' else min(total,int(keep))
        assert group[0]['retained']==expected_retained
        total+=count
        expected_retained=total if keep=='all' else min(total,int(keep))
        assert before['retained']==after['retained']==expected_retained
        if keep=='0':assert after['memory']['live_end']==prepared
        engine_bytes=before['memory']['live_end']-after['memory']['live_end'];assert engine_bytes>=0
        answer_bytes=after['memory']['live_end']-prepared;assert answer_bytes>=0
        rows.append(dict(mode=m,family=f,depth=n,keep=keep,cancel=cancel,fail_tail=fail,query=q,answers=count,retained=expected_retained,engine_bytes=engine_bytes,answer_bytes=answer_bytes))
    expected_labels+=['consumer-released']
    assert [s['label'] for s in xs]==expected_labels
    assert xs[-1]['memory']['live_end']==prepared
    assert d['restored']['live_end']==d['restored']['live_start']==xs[0]['baseline']
# Every exhausted or cancelled engine's remaining bytes are independent of how
# the external consumer keeps previous answers. This is a cross-run owner check.
groups={}
for row in rows:
    key=tuple(row[k] for k in ['mode','family','depth','cancel','fail_tail','query'])
    groups.setdefault(key,[]).append(row['engine_bytes'])
assert len(groups)==288
assert all(len(values)==3 and len(set(values))==1 for values in groups.values())
output_groups={}
for row in rows:
    key=tuple(row[k] for k in ['family','depth','keep','cancel','fail_tail','query'])
    output_groups.setdefault(key,[]).append(row['answer_bytes'])
assert len(output_groups)==216
assert all(len(values)==4 and len(set(values))==1 for values in output_groups.values())
(root/'summary.json').write_text(json.dumps(rows,indent=2)+'\n')
(root/'audit.json').write_text(json.dumps(dict(processes=216,exact_replays=108,query_owner_groups=288,output_owner_groups=216,equal_output_bytes=True,consumer_independent_engine_bytes=True,root_restoration=True,source_hashes=True,order=True,timing='not_run'),indent=2)+'\n')
print('216 processes; 108 exact replays; 288 consumer-independent engine owner groups; all final baselines restored')
for row in rows:
    if row['family']=='aliases' and row['depth']==64 and not row['cancel'] and not row['fail_tail'] and row['query']==7:
        print(row)
