from pathlib import Path
import json
from collections import Counter
root=Path.cwd()
base=root/'docs/experiments/results'
old=json.loads((base/'s06-learning-ownership/audit.json').read_text())['results']
new=json.loads((base/'s06-learning-setup/audit.json').read_text())['results']
key=lambda r: tuple(r[k] for k in ['mode','mask','weight','capacity','followups'])
a={key(r):r for r in old};b={key(r):r for r in new}
assert a.keys()==b.keys()
changes=[]; counts=Counter()
for k,r in b.items():
    prior=a[k]
    if r['mode']!='eager': assert r==prior,(prior,r)
    else:
        delta=r['requested']-prior['requested']
        counts['lower' if delta<0 else 'higher' if delta>0 else 'same']+=1
        if delta:
            assert r['mask']==0 and r['capacity']==4 and r['followups'] in (4,16)
            changes.append({'key':k,'before':prior['requested'],'after':r['requested']})
assert counts=={'same':68,'lower':4}
for kind in ['off','default']:
    log=(base/f's06-learning-setup/gate-{kind}.log').read_text()
    assert 'FAILED' not in log and 'error:' not in log
    assert sorted(int(line.split()[3]) for line in log.splitlines() if line.startswith('test result: ok.'))==[1,2,9,10,11]
assert 'Limit("partitions")' in (base/'s06-learning-setup/red.log').read_text()
assert 'Finished' in (base/'s06-learning-setup/clippy.log').read_text()
result={'eager_allocation_changes':dict(counts),'changes':changes,'unchanged_controls':144,'tests_per_build':33}
(base/'s06-learning-setup/comparison.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result))
