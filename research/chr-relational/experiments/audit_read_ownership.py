"""Independently reconstruct registered allocation rows and matched prior controls."""
import collections
import hashlib
import itertools
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
B = ROOT / 'docs/experiments/results/s02-read-ownership'
def read(p): return json.loads(p.read_text())
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def strip(x, semantic=False):
    if isinstance(x, list): return [strip(v, semantic) for v in x]
    if isinstance(x, dict): return {k:strip(v, semantic) for k,v in x.items() if k not in (['ns','first_answer_ns','memory','meter'] if semantic else ['ns','first_answer_ns'])}
    return x
f = read(B/'freeze.json')
for path, expected in f['sources'].items(): assert sha(ROOT/path)==expected,path
for value in f['binaries'].values(): assert sha(ROOT/value['path'])==value['sha256']
modes=['contextual','shared','relevant','persistent-shared','persistent-relevant','validated','persistent-validated','scan','lowered']
configs=[(*row,None) for row in itertools.product(modes,['single','shared','distinct','changed'],[0,16],[1,4],[0,1],['immediate','all'])]
configs += [(m,f,16,4,1,h,1) for m,f,h in itertools.product(modes,['single','shared','distinct','changed'],['immediate','all'])]
records=read(B/'results.json');assert len(records)==len(configs)==648
assert len(list((B/'runs').glob('*.json')))==1872
rows={};prior=read(ROOT/'docs/experiments/results/s02-relevant-ownership/results.json')
def key(r): return tuple(r[k] for k in ['mode','family','depth','queries','tokens','retention','cancel'])
old={key(r):r for r in prior};prior_differences=[]
for index,(r,c) in enumerate(zip(records,configs)):
    assert r['index']==index and key(r)==c
    args=[*c[:5],0]+([] if c[-1] is None else [c[-1]])
    raw=[]
    for build,repeat in [('meter',0),('meter',1)]+([] if c[-1] else [('ordinary',None)]):
        suffix=f'{index}-{build}'+('' if repeat is None else f'-{repeat}')
        receipt=read(B/'runs'/f'{suffix}.json')
        assert receipt['exit_code']==0 and not receipt['stderr'] and receipt['args']==args and receipt['retention']==c[5]
        assert (B/'runs'/f'{suffix}.log').read_text()==receipt['stdout']
        v=[json.loads(l) for l in receipt['stdout'].splitlines() if l.startswith('{')][-1]
        assert v['event']=='result' and v['meter']==(build=='meter') and not v['counters']
        assert v['mode']==c[0] and v['family']==c[1] and v['resource']==bool(c[4]) and v['retained']==(c[5]=='all')
        assert len(v['samples'])==c[3]
        assert [s['complete'] for s in v['samples']]==([True]*c[3] if c[-1] is None else [False,True,False,True])
        if raw: assert strip(v,build=='ordinary')==strip(raw[0],build=='ordinary')
        raw.append(v)
    v=raw[0];assert r['result']==v
    phases=[v['source_build'],v['preparation']]
    for sample in v['samples']:
        phases.extend(sample[k] for k in ['input_build','setup','execute_observe','engine_drop','answer_drop','answer_hold'] if sample[k] is not None)
    phases.extend([v['prepared_drop'],v['consumer_drop']])
    mem=[p['memory'] for p in phases];base=mem[0]['live_start']
    assert all(a['live_end']==b['live_start'] for a,b in zip(mem,mem[1:])),index
    assert mem[-1]['live_end']==base
    calculated={'requested_bytes':sum(p['requested_bytes'] for p in mem),'allocation_calls':sum(p['allocation_calls'] for p in mem),'peak_excess':max(p['peak_live'] for p in mem)-base}
    assert all(r[k]==n for k,n in calculated.items())
    rows[c]=r
    if c in old and strip(r['result'])!=strip(old[c]['result']):prior_differences.append({'config':c,'new':calculated,'prior':{k:old[c][k] for k in calculated}})
# Normalize only the pre-existing live baseline and the absent/null diagnostic field.
# argv path storage is outside the measured ownership interval.
def normalize(v):
    base=v['source_build']['memory']['live_start']
    def walk(x):
        if isinstance(x,list):return [walk(y) for y in x]
        if isinstance(x,dict):return {k:(y-base if k in ['live_start','live_end','peak_live'] else walk(y)) for k,y in x.items() if k not in ['ns','first_answer_ns'] and not (k=='attribution' and y is None)}
        return x
    return walk(v)
normalized_differences=[c for c in rows if c in old and normalize(rows[c]['result'])!=normalize(old[c]['result'])]
baseline_shifts=sorted({rows[c]['result']['source_build']['memory']['live_start']-old[c]['result']['source_build']['memory']['live_start'] for c in rows if c in old})
comparisons=[]
for candidate in ['validated','persistent-validated']:
    controls=['relevant' if candidate=='validated' else 'persistent-relevant','contextual','shared' if candidate=='validated' else 'persistent-shared','scan','lowered']
    for control in controls:
        cases=[]
        for c,r in rows.items():
            if c[0]!=candidate:continue
            other=rows[(control,*c[1:])]
            cases.append({'scenario':c[1:],'requested_bytes':r['requested_bytes'],'control_bytes':other['requested_bytes'],'peak_excess':r['peak_excess'],'control_peak':other['peak_excess']})
        counts={}
        for metric,field in [('traffic','requested_bytes'),('peak','peak_excess')]:
            control_field='control_bytes' if metric=='traffic' else 'control_peak'
            counts[metric]=dict(collections.Counter('lower' if r[field]<r[control_field] else 'higher' if r[field]>r[control_field] else 'equal' for r in cases))
        comparisons.append({'candidate':candidate,'control':control,'counts':counts,'cases':cases})
print(json.dumps({'configurations':648,'meter_processes':1296,'ordinary_processes':576,'exact_allocation_pairs':648,'all_phase_continuity_and_final_release':True,'prior_controls_checked':len(old),'prior_control_raw_differences':len(prior_differences),'prior_baseline_shifts_bytes':baseline_shifts,'prior_normalized_control_differences':normalized_differences,'comparisons':comparisons},indent=2))
