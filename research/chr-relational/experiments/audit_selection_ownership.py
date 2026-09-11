"""Reconstruct stable-selection ownership and work from frozen raw receipts."""
import hashlib,itertools,json,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s02-selection-ownership'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def norm(x,base):
    if isinstance(x,list):return [norm(v,base) for v in x]
    if isinstance(x,dict):return {k:(v-base if k in ['live_start','live_end','peak_live'] else norm(v,base)) for k,v in x.items() if k not in ['ns','cpu_ns']}
    return x
f=read(BASE/'freeze.json');assert sha(BASE/'sources.zip')==f['archive_sha256']
with zipfile.ZipFile(BASE/'sources.zip') as z:
    assert set(z.namelist())==set(f['sources'])
    for p,h in f['sources'].items():assert hashlib.sha256(z.read(p)).hexdigest()==h
for b in f['binaries'].values():assert sha(Path(b['path']))==b['sha256']
assert sha(BASE/'cells.json')==f['cells_sha256']
cells=read(BASE/'cells.json');assert cells==[list(c) for c in itertools.product(['flat','sparse','chain','competition'],[2,6],['local','local-filtered','scan','indexed','sealed','chr-scan','chr-indexed','chr-sealed'],['immediate','all'],['complete','cancel'])]
assert len(cells)==256 and len(list((BASE/'runs').glob('*.json')))==896
phases=[('source',0),('prepare',0)]+[(p,i) for i in range(4) for p in ['input','encode','setup','execute','observe','engine_drop','input_drop','consumer']]+[('prepared_drop',0),('source_drop',0),('consumer_drop',0)]
summaries=[];work_summaries=[]
for i,(family,n,mode,retention,stop) in enumerate(cells):
    allocations=[];ends=None;works=[]
    for kind,rep in [('meter',0),('meter',1),('ordinary',0)]+([('work',0),('work',1)] if retention=='all' and stop=='complete' else []):
        raw=read(BASE/'runs'/f'{i}-{kind}-{rep}.json');assert raw['exit_code']==0 and not raw['timeout'] and not raw['stderr'],(i,kind,rep)
        assert raw['command']==[f['binaries'][kind]['path'],'selection',mode,family,str(n),retention,stop]
        events=[json.loads(x) for x in raw['stdout'].splitlines()];assert [e['event'] for e in events]==['start','result'];r=events[1]
        assert r['meter']==(kind=='meter') and r['work_enabled']==(kind=='work') and r['chr']==mode.startswith('chr-')
        assert [(p['phase'],p['query']) for p in r['phases']]==phases
        assert [e['cancelled'] for e in r['endpoints']]==([False]*4 if stop=='complete' else [True,False,True,False])
        assert all(e['exhausted'] and e['observed'] for e in r['endpoints'] if not e['cancelled'])
        assert all(not e['observed'] for e in r['endpoints'] if e['cancelled'])
        if ends is None:ends=r['endpoints']
        else:assert ends==r['endpoints']
        if kind=='meter':
            ms=[p['reading']['heap'] for p in r['phases']];base=ms[0]['live_start'];assert ms[-1]['live_end']==base
            assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]))
            allocations.append(norm(r,base));assert r['work']==[]
        elif kind=='work':assert len(r['work'])==4;works.append(r['work'])
        else:assert r['work']==[] and all('heap' not in p['reading'] for p in r['phases'])
    assert allocations[0]==allocations[1],i
    rows=allocations[0]['phases'];ms=[p['reading']['heap'] for p in rows]
    summaries.append(dict(index=i,family=family,size=n,mode=mode,retention=retention,stop=stop,requested_bytes=sum(m['requested_bytes'] for m in ms),peak_excess=max(m['peak_live'] for m in ms),phases=[dict(phase=p['phase'],query=p['query'],**p['reading']['heap']) for p in rows]))
    if works:assert works[0]==works[1];work_summaries.append(dict(index=i,family=family,size=n,mode=mode,queries=works[0]))
check=read(BASE/'meter-check.json');assert check['exit_code']==0 and 'meter-check passed' in check['stdout']
smoke=read(BASE/'existing-smoke.json');assert smoke['exit_code']==0 and '147 independent source comparisons pass' in smoke['stdout']
trace_freeze=read(BASE/'trace-freeze.json');assert sha(Path(trace_freeze['binary']['path']))==trace_freeze['binary']['sha256'];assert sha(BASE/trace_freeze['source_snapshot'])==trace_freeze['source']['sha256'];assert sha(BASE/'freeze.json')==trace_freeze['parent_freeze_sha256']
traces=[]
for rep in range(2):
    raw=read(BASE/f'trace-{rep}.json');assert raw['exit_code']==0 and not raw['stderr'];assert raw['command']==[trace_freeze['binary']['path']]
    values=[json.loads(x) for x in raw['stdout'].splitlines()];assert [(x['family'],x['size'],x['value']) for x in values]==list(itertools.product(['flat','sparse','chain','competition'],[2,6],['a','b']))
    for value in values:
        assert sum(value['rules'].values())==value['applications']
        for w in work_summaries:
            if (w['family'],w['size'])==(value['family'],value['size']) and w['mode'].startswith('chr'):
                assert w['queries'][0 if value['value']=='a' else 1]['applications']==value['applications']
    traces.append(values)
assert traces[0]==traces[1]
print(json.dumps(dict(configurations=256,processes=896,exact_allocation_pairs=256,exact_work_pairs=64,ordinary_endpoints_equal=True,final_live_restored=True,primary_timing=False,independent_rule_traces=traces[0],summary=summaries,work=work_summaries),indent=2))
