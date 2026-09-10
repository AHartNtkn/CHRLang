"""Audit complete raw ownership receipts and summarize paired allocation contrasts."""
import hashlib
import itertools
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
BASE=ROOT/'docs/experiments/results/s03-dependency-ownership'
def read(p):return json.loads(p.read_text())
def clean(v):
    if isinstance(v,list):return [clean(x) for x in v]
    if isinstance(v,dict):return {k:clean(x) for k,x in v.items() if k!='ns'}
    return v
def result(path):
    rows=[json.loads(line) for line in path.read_text().splitlines() if line.startswith('{')]
    values=[r for r in rows if r.get('event')=='result'];assert len(values)==1,path
    return values[0]
freeze=read(BASE/'freeze.json')
for p,sha in freeze['sources'].items():
    current=(ROOT/p).read_bytes()
    if p=='research/chr-direct-conditional/examples/dependency_ownership.rs':
        original=(BASE/'before/dependency_ownership.rs').read_bytes()
        annotation=b'#[allow(clippy::assertions_on_constants)] // Reject instrumented measurements at runtime.\n'
        assert current.replace(annotation,b'')==original
        assert current.count(annotation)==1
        current=original
    assert hashlib.sha256(current).hexdigest()==sha,p
for binary in freeze['binaries'].values():assert hashlib.sha256((ROOT/binary['path']).read_bytes()).hexdigest()==binary['sha256']
for kind in ['meter','ordinary']:
    artifacts=[json.loads(line) for line in (BASE/f'build-{kind}.jsonl').read_text().splitlines()]
    for target in ['chr_compiled','chr_direct_choice','chr_observe','chr_persistent']:
        artifact=next(a for a in artifacts if a.get('target',{}).get('name')==target)
        assert artifact['features']==[],(kind,target,artifact['features'])
rows=read(BASE/'results.json');assert len(rows)==960
modes=['current','current-miss','birth','birth-miss','dependencies','dependencies-miss','scan','indexed']
expected=set(itertools.product(['plain','known-hit','known-miss','delayed-hit','delayed-miss'],[8,128],[False,True],modes,['immediate','window','all'],[False,True]))
assert {(r['family'],r['size'],r['reverse'],r['mode'],r['retention'],r['cancel']) for r in rows}==expected
assert len(list((BASE/'runs').glob('*.log')))==2880
for r in rows:
    index=r['index'];pair=[result(BASE/'runs'/f'{index}-meter-{i}.log') for i in range(2)]
    assert clean(pair[0])==clean(pair[1])==clean(r['result']),index
    ordinary=result(BASE/'runs'/f'{index}-ordinary-0.log')
    assert pair[0]['meter'] and not ordinary['meter']
    assert pair[0]['endpoints']==ordinary['endpoints']
    assert len(pair[0]['endpoints'])==4
    assert all(e['complete'] for i,e in enumerate(pair[0]['endpoints']) if not r['cancel'] or i%2)
    memories=[p['reading']['memory'] for p in pair[0]['phases']]
    baseline=memories[0]['live_start']
    assert memories[-1]['live_end']==baseline
    assert sum(m['requested_bytes'] for m in memories)==r['requested_bytes']
    assert max(m['peak_live'] for m in memories)-baseline==r['peak_excess']
def key(r):return tuple(r[k] for k in ['family','size','reverse','retention'])
complete={(key(r),r['mode']):r for r in rows if not r['cancel']}
comparisons=[]
for mode in ['current','birth','dependencies']:
    for control in [mode,'scan','indexed']:
        candidate=mode+'-miss';counts={field:{'lower':0,'equal':0,'higher':0} for field in ['requested_bytes','peak_excess']}
        for r in rows:
            if r['cancel'] or r['mode']!=candidate:continue
            other=complete[key(r),control]
            for field in counts:
                counts[field]['lower' if r[field]<other[field] else 'higher' if r[field]>other[field] else 'equal']+=1
        comparisons.append({'candidate':candidate,'control':control,**counts})
witness=[{k:r[k] for k in ['mode','requested_bytes','peak_excess']} for r in rows if not r['cancel'] and r['family']=='delayed-miss' and r['size']==128 and not r['reverse'] and r['retention']=='all']
print(json.dumps({'raw_processes':2880,'cells':960,'source_hashes':len(freeze['sources']),'exact_repeats':True,'ordinary_endpoints':True,'final_restoration':True,'comparisons':comparisons,'delayed_miss_witness':witness,'primary_timing':False},indent=2))
