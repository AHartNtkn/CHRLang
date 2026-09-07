"""Audit matrix, exact replay, independent observations, and current frozen inputs."""
import hashlib
import json
from run_maintained_gate import ROOT,OUT,PREFIX

def canonical(answer):
    return (json.dumps(answer['outputs']),tuple(sorted(json.dumps(t) for t in answer['residual'])))

def main():
    manifest=json.loads((OUT/f'{PREFIX}-manifest.json').read_text())
    rows=[json.loads(s) for s in (OUT/f'{PREFIX}.jsonl').read_text().splitlines()]
    cells=[dict(kind='store',family=f,mode=m) for f in range(4) for m in ('full','selective')]
    cells += [dict(kind='source',consuming=c,n=4,branches=b,rounds=1,mode=m,quantum=q)
              for c in (False,True) for b in (1,2) for m in ('prefix','full','selective') for q in (1,8)]
    cells=[dict(c,seed=0) for c in cells]+[dict(kind='source',consuming=True,n=4,branches=b,rounds=1,mode='selective',quantum=q,seed=seed) for seed in (1,2) for b in (1,2) for q in (1,8)]
    order=[dict(phase=p,cell=c) for p in ('gate','replay') for c in cells]
    assert len(rows)==80 and manifest['order']==order
    assert [dict(phase=r['phase'],cell=r['cell']) for r in rows]==order
    assert all(hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h for p,h in manifest['sha256'].items())
    assert all(r['exit']==0 and not r['stderr'] and 'result' in r and r['wall_seconds']<30 for r in rows)
    assert [r['result'] for r in rows[:40]]==[r['result'] for r in rows[40:]]
    observations={}
    for row in rows[:40]:
        c=row['cell'];r=row['result']
        if c['kind']=='store':
            stages=r['stages'];assert len(stages)==(4,3,2,2)[c['family']]
            value=[(s['stage'],s['contexts'],s['matches']) for s in stages]
            key=('store',c['family'])
        else:
            assert r['raw']==c['branches']==len(r['answers']) and r['failed']==0 and r['exhausted']
            assert r['transitions'] and r['steps']<100000 and sum(r['counts'].values())<=10000000
            assert all(len(a['outputs'])==4 and len(a['residual'])==(12 if c['consuming'] else 24) for a in r['answers'])
            value=(r['steps'],sorted(canonical(a) for a in r['answers']))
            key=('source',c['consuming'],c['branches'])
        if key in observations:assert observations[key]==value,(key,c)
        else:observations[key]=value
    sensitive={'maintain.invalidation_visit','maintain.descendant_index_visit'}
    seed_baselines={}
    for row in rows[:40]:
        c=row['cell']
        if c['kind']!='source' or c['mode']!='selective' or not c['consuming']:continue
        r=row['result'];value={k:v for k,v in r.items() if k!='counts'}
        value['counts']={k:v for k,v in r['counts'].items() if k not in sensitive}
        key=(c['branches'],c['quantum'])
        if key in seed_baselines:assert seed_baselines[key]==value
        else:seed_baselines[key]=value
    result=dict(passed=True,isolated_children=80,source_cells_per_pass=24,additional_seed_cells_per_pass=8,supported_context_projections_per_pass=88,
                exact_replay=True,all_control_and_quantum_observations_agree=True,
                max_child_wall_seconds=max(r['wall_seconds'] for r in rows))
    with (OUT/f'{PREFIX}-audit.json').open('x') as stream:json.dump(result,stream,indent=2);stream.write('\n')
    print(json.dumps(result))

if __name__=='__main__':main()
