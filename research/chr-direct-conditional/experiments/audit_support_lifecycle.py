#!/usr/bin/env python3
import hashlib,itertools,json,random,statistics
from pathlib import Path
root=Path('docs/experiments/results/s08-support-lifecycle')
controls=[('ordinary','inferred'),('identities','inferred'),('cache','inferred'),('combined','inferred'),('ordinary','scan'),('ordinary','resumable')]
cells=[(p,m,f,n,k,0,r,0) for (p,m),f,n,k,r in itertools.product(controls,['aliases','distinct'],[0,64],['0','all'],[1,4])]
cells += [(p,m,f,64,'4',0,4,0) for (p,m),f in itertools.product(controls,['aliases','distinct'])]
cells += [(p,m,'aliases',64,k,0,4,1) for (p,m),k in itertools.product(controls,['0','4'])]
cells += [(p,m,'aliases',64,k,1,4,0) for (p,m),k in itertools.product(controls,['0','4','all'])]
for path,h in json.loads((root/'freeze.json').read_text()).items():
    data=Path(path).read_bytes()
    if hashlib.sha256(data).hexdigest()!=h:
        mapping=json.loads((root/'source-snapshots.json').read_text());data=Path(mapping[path]).read_bytes()
    assert hashlib.sha256(data).hexdigest()==h,path
records={};owner_groups={};answer_groups={}
for kind,reps,seed in [('meter',2,7870),('time',5,7871)]:
    jobs=[(rep,*c) for rep in range(reps) for c in cells];random.Random(seed).shuffle(jobs)
    assert json.loads((root/f'{kind}-order.json').read_text())==[list(j) for j in jobs]
    assert len(list((root/kind).glob('*.json')))==len(jobs)
    for i,(rep,*c) in enumerate(jobs):
        policy,mode,family,n,keep,cancel,reuse,fail=c
        receipt=json.loads((root/kind/f'{i:03}.json').read_text());assert receipt['returncode']==0
        assert receipt['command'][1:]==[mode,family,str(n),'4','8','1',keep,str(cancel),str(reuse),str(fail)]
        d=json.loads(receipt['stdout'].splitlines()[-1]);assert not d['counters'] and d['meter']==(kind=='meter')
        assert [d[k] for k in ['policy','mode','family','depth','keep','cancel','reuse','fail_tail']]==[policy,mode,family,n,keep,bool(cancel),reuse,bool(fail)]
        assert len(d['samples'])==reuse
        phases=[d['source_build'],d['preparation']]
        total_answers=0
        for q,sample in enumerate(d['samples']):
            expected=4 if cancel and q==0 else n+q+1-fail
            assert sample['answers']==expected and sample['depth']==n+q
            assert sample['complete']==(not(cancel and q==0))
            assert (sample['first_answer_ns'] is not None)==(expected>0)
            phases.extend(sample[k] for k in ['input_build','setup','execute_observe','engine_drop'])
            total_answers+=expected
        phases.extend([d['consumer_drop'],d['prepared_drop']])
        key=tuple(c);records.setdefault(key,{'meter':[],'time':[]})
        if kind=='time':
            assert d['snapshots']==[] and all(p['memory'] is None for p in phases)
            records[key]['time'].append(sum(p['ns'] for p in phases));continue
        ms=[p['memory'] for p in phases]
        assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]))
        assert ms[-2]['live_end']==d['preparation']['memory']['live_end']
        assert ms[-1]['live_end']==ms[0]['live_start']
        prepared=d['preparation']['memory']['live_end'];total=0
        for q in range(reuse):
            xs=[s for s in d['snapshots'] if s['query']==q]
            count=d['samples'][q]['answers'];expected_labels=['setup']+['delivery']*sum(x<=count for x in [1,4,16,64])+['cancelled' if cancel and q==0 else 'exhausted','engine-disposed']
            assert [s['label'] for s in xs]==expected_labels
            assert xs[0]['retained']==(total if keep=='all' else min(total,int(keep)))
            total+=count
            assert xs[-1]['retained']==xs[-2]['retained']==(total if keep=='all' else min(total,int(keep)))
            assert xs[-1]['answers']==xs[-2]['answers']==count
            if keep=='0':assert xs[-1]['live']==prepared
            if rep==0:
                og=(policy,mode,family,n,cancel,reuse,fail,q)
                owner_groups.setdefault(og,[]).append(xs[-2]['live']-xs[-1]['live'])
                ag=(family,n,keep,cancel,reuse,fail,q)
                answer_groups.setdefault(ag,[]).append(xs[-1]['live']-prepared)
        records[key]['meter'].append((ms,d['snapshots']))
assert all(len(v)>=2 and len(set(v))==1 for v in owner_groups.values())
assert all(len(v)==6 and len(set(v))==1 for v in answer_groups.values())
rows=[]
for key,rec in records.items():
    assert rec['meter'][0]==rec['meter'][1]
    ms=rec['meter'][0][0];policy,mode,family,n,keep,cancel,reuse,fail=key
    rows.append(dict(policy=policy,mode=mode,family=family,depth=n,keep=keep,cancel=cancel,reuse=reuse,fail_tail=fail,requested_bytes=sum(m['requested_bytes'] for m in ms),peak_growth=max(m['peak_live'] for m in ms)-ms[0]['live_start'],median_ns=statistics.median(rec['time']),min_ns=min(rec['time']),max_ns=max(rec['time'])))
(root/'summary.json').write_text(json.dumps(rows,indent=2)+'\n')
(root/'audit.json').write_text(json.dumps(dict(processes=966,cells=138,exact_allocation_pairs=138,phase_continuity=True,owner_groups=len(owner_groups),equal_answer_groups=len(answer_groups),frozen_inputs=True,registered_orders=True,timing='five-sample exploratory'),indent=2)+'\n')
print('966 processes;138 exact allocation pairs;all phase/owner/order/hash checks pass')
for r in rows:
    if r['family']=='aliases' and r['depth']==64 and r['keep']=='0' and r['reuse']==4 and not r['fail_tail'] and not r['cancel']:print(r)
