"""Audit all registered sizing outcomes without treating censoring as success."""
import hashlib
import json
from collections import Counter
from run_maintained_sizing import ROOT,OUT,PREFIX
from audit_maintained_gate import canonical

def main():
    m=json.loads((OUT/f'{PREFIX}-manifest.json').read_text())
    rows=[json.loads(s) for s in (OUT/f'{PREFIX}.jsonl').read_text().splitlines()]
    cells=[dict(consuming=c,n=n,branches=b,rounds=r,mode=mode,quantum=8,seed=0)
           for c in (False,True) for n,b,r in ((4,1,1),(16,2,4),(64,1,1),(64,8,4))
           for mode in ('prefix','full','selective')]
    assert len(rows)==24 and m['order']==cells and [r['cell'] for r in rows]==cells
    assert all(hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h for p,h in m['sha256'].items())
    matched={};outcomes=Counter()
    for row in rows:
        c=row['cell']
        if row['exit']!=0:
            if row['exit']=='timeout':outcomes['wall_bound']+=1
            elif 'registered instrumented-work bound reached' in row['stderr']:outcomes['work_bound']+=1
            elif 'MemoryError' in row['stderr']:outcomes['memory_bound']+=1
            else:outcomes['unclassified_error']+=1
            continue
        assert 'result' in row and not row['stderr']
        r=row['result'];assert r['exhausted'] and r['failed']==0 and r['raw']==len(r['answers'])==c['branches']
        assert not r['transitions'] and sum(r['counts'].values())<=10000000
        n,b,rounds=c['n'],c['branches'],c['rounds']
        expected_size=n*rounds+2+(2*n-2 if c['consuming'] else 4*n+2)
        assert all(len(a['residual'])==expected_size for a in r['answers'])
        key=(c['consuming'],n,b,rounds);value=(r['steps'],sorted(canonical(a) for a in r['answers']))
        if key in matched:assert value==matched[key]
        else:matched[key]=value
        outcomes['complete_checked']+=1
    result=dict(recording_audit_passed=True,outcomes=dict(outcomes),all_successful_matched_observations_agree=True,
                semantic_failure_free=not outcomes['unclassified_error'],censored_questions_remain_open=True)
    with (OUT/f'{PREFIX}-audit.json').open('x') as stream:json.dump(result,stream,indent=2);stream.write('\n')
    print(json.dumps(result))

if __name__=='__main__':main()
