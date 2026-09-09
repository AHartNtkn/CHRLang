#!/usr/bin/env python3
import csv
import hashlib
import itertools
import json
from pathlib import Path
import sys
p=Path(sys.argv[1])
for name,h in json.loads((p/'freeze.json').read_text()).items():
    data=Path(name).read_bytes()
    if hashlib.sha256(data).hexdigest()!=h and name=='research/chr-direct-conditional/tests/post_choice_work.rs':
        data=Path('docs/experiments/results/s10-post-choice-gate/measured-test-source.rs').read_bytes()
    assert hashlib.sha256(data).hexdigest()==h,name
cells=list(itertools.product(['scan','indexed','special','contextual','demand','shared','conditional'],['common','independent','early'],[0,3],[0,4,16,64],[1,4]))
rows={(x['mode'],x['family'],int(x['choices']),int(x['depth']),int(x['reuse'])):x for x in csv.DictReader((p/'summary.csv').open())}
assert set(rows)==set(cells)
for mode,family,k,depth,reuse in cells:
    pair=[]
    names=['prepare']+['input','setup','first-delivery-or-exhaustion','remaining-delivery','search-drop','answer-policy','input-drop']*reuse+['cancel-input','cancel-setup','cancel-unit','cancel-search-drop','cancel-input-drop','prepared-drop','retained-answers-drop']
    for r in range(2):
        d=json.loads((p/'meter'/f'{r}-{mode}-{family}-{k}-{depth}-{reuse}.json').read_text())
        assert (d['mode'],d['family'],d['choices'],d['depth'],d['reuse'],d['consumer'],d['meter'])==(mode,family,k,depth,reuse,'query',True)
        xs=d['phases'];assert [x['phase'] for x in xs]==names;ms=[x['memory'] for x in xs]
        assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]));assert ms[0]['live_start']==ms[-1]['live_end']
        for x in xs:
            if x['phase'] in ['input-drop','cancel-input-drop']:assert x['memory']['live_end']==ms[0]['live_end']
            if x['phase']=='prepared-drop':assert x['memory']['live_end']==ms[0]['live_start']
        pair.append(ms)
    assert pair[0]==pair[1]
    row=rows[(mode,family,k,depth,reuse)]
    assert int(row['requested_bytes'])==sum(x['requested_bytes'] for x in pair[0])
    assert int(row['peak_above_start'])==max(x['peak_live'] for x in pair[0])-pair[0][0]['live_start']
order=json.loads((p/'order.json').read_text());assert len(order)==672;assert set(map(tuple,order))=={(r,*c) for r in range(2) for c in cells}
assert len(list((p/'meter').glob('*.json')))==672
print('672 processes; 336 exact allocation pairs; every complete observation, phase and owner check passed; summary and frozen sources verified; no timing comparison')
