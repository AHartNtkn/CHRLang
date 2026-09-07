"""Audit the independently registered grid, replay and frozen source identity."""
import hashlib
import itertools
import json
from pathlib import Path
from run_gate import ROOT, OUT, PREFIX

manifest=json.loads((OUT/f'{PREFIX}-manifest.json').read_text())
rows=[json.loads(s) for s in (OUT/f'{PREFIX}.jsonl').read_text().splitlines()]
expected_order=[(phase,offset) for phase in ['gate','replay'] for offset in range(0,432,4)]
assert [(r['phase'],r['offset']) for r in rows]==expected_order
assert [(r['phase'],r['offset']) for r in manifest['order']]==expected_order
assert all(hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h for p,h in manifest['sha256'].items())
recipes=list(itertools.product(range(12),range(12),range(3)))
for row in rows:
    assert row['exit']==0 and not row['stderr'] and 'parse_error' not in row
    result=row['result'];offset=row['offset']
    assert result['recipes']==[list(x) for x in recipes[offset:offset+4]]
    assert len(result['expected'])==len(result['actual'])==4
    assert result['expected']==result['actual']
    assert result['bad_support']==sum(1<<i for i,a in enumerate(result['expected']) if a is None)
    assert result['stats']['rounds']<=1000
    assert row['wall_seconds']<30
assert [r['result'] for r in rows[:108]]==[r['result'] for r in rows[108:]]
answers=[a for row in rows[:108] for a in row['result']['actual']]
result={'pass':True,'recipes':len(answers),'batches':108,'isolated_children_including_replay':216,
        'success':sum(a is not None for a in answers),'failure':sum(a is None for a in answers),
        'exact_result_and_counter_replay':True,
        'max_universe':max(r['result']['universe'] for r in rows),
        'max_facts':max(r['result']['facts'] for r in rows),
        'max_rounds':max(r['result']['stats']['rounds'] for r in rows),
        'max_joins':max(r['result']['stats']['joins'] for r in rows),
        'max_child_wall_seconds':max(r['wall_seconds'] for r in rows)}
with (OUT/f'{PREFIX}-audit.json').open('x') as stream:
    stream.write(json.dumps(result,indent=2)+'\n')
print(json.dumps(result))
