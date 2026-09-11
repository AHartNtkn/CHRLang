"""Independent reconstruction of registered ownership and work receipts."""
import collections,hashlib,itertools,json,re,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s03-post-ownership'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f=read(BASE/'freeze.json');assert sha(BASE/'sources.zip')==f['source_archive_sha256']
with zipfile.ZipFile(BASE/'sources.zip') as archive:
    assert set(archive.namelist())==set(f['sources'])
    for path,digest in f['sources'].items():assert hashlib.sha256(archive.read(path)).hexdigest()==digest,path
for binary in f['binaries'].values():assert sha(Path(binary['path']))==binary['sha256']
assert sha(BASE/'cells.json')==f['cells_hash']
cells=read(BASE/'cells.json');families=['post-input','post-output','post-forward','post-miss','post-duplicate','post-template'];modes=['birth','birth-miss','birth-miss-template','scan','indexed','sealed']
assert cells==[list(x) for x in itertools.product(families,[8,32],[False,True],modes,['immediate','window','all'],[False,True])]
rows=[json.loads(x) for x in (BASE/'results.jsonl').read_text().splitlines()];assert len(rows)==2592 and len(list((BASE/'runs').glob('*.json')))==2592
self_check=read(BASE/'meter-check.json');assert self_check['exit_code']==0 and 'meter-check passed' in self_check['stdout']
expected_phases=[('source',0),('prepare',0)]+[(p,q) for q in range(4) for p in ['input','setup','execute_observe','engine_drop','consumer']]+[('prepared_drop',0),('consumer_drop',0)]
def normalize(x,base):
    if isinstance(x,list):return [normalize(v,base) for v in x]
    if isinstance(x,dict):return {k:(v-base if k in ['live_start','live_end','peak_live'] else normalize(v,base)) for k,v in x.items() if k!='ns'}
    return x
summaries=[]
for i,cell in enumerate(cells):
    family,size,reverse,mode,retention,cancel=cell;vs=[]
    for offset,(kind,rep) in enumerate([('meter',0),('meter',1),('ordinary',0)]):
        row=rows[i*3+offset];assert (row['index'],row['kind'],row['rep'])==(i,kind,rep)
        raw=read(BASE/'runs'/f'{i}-{kind}-{rep}.json');assert raw['exit_code']==0 and not raw['timeout'] and not raw['stderr']
        assert raw['command']==[f['binaries'][kind]['path'],mode,family,str(size),str(reverse).lower(),retention,str(cancel).lower()]
        result=next(json.loads(x) for x in raw['stdout'].splitlines() if x.startswith('{') and json.loads(x).get('event')=='result');assert result==row['result']
        assert result['meter']==(kind=='meter')
        ends=result['endpoints'];assert [e['complete'] for e in ends]==([False,True,False,True] if cancel else [True]*4)
        assert [e['answers'] for e in ends]==([0,1,0,1] if cancel else [1]*4)
        assert all(e['first_ns'] is None for e in ends)
        assert [(p['phase'],p['query']) for p in result['phases']]==expected_phases
        if kind=='meter':
            ms=[p['reading']['memory'] for p in result['phases']];base=ms[0]['live_start']
            assert ms[-1]['live_end']==base and all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]))
            vs.append(normalize(result,base))
        else:assert all(p['reading']['memory'] is None for p in result['phases'])
        if offset:assert ends==rows[i*3]['result']['endpoints']
    assert vs[0]==vs[1],i
    phases=vs[0]['phases'];ms=[p['reading']['memory'] for p in phases]
    summaries.append(dict(index=i,family=family,size=size,reverse=reverse,mode=mode,retention=retention,cancel=cancel,requested_bytes=sum(m['requested_bytes'] for m in ms),peak_excess=max(m['peak_live'] for m in ms),phases=[dict(phase=p['phase'],query=p['query'],**p['reading']['memory']) for p in phases],ticks=[e['ticks'] for e in vs[0]['endpoints']]))
work=[]
for rep in [1,2]:
    raw=read(BASE/f'work-{rep}.json');assert raw['exit_code']==0 and not raw['timeout'] and not raw['stderr']
    values=[line.split(',')[1:] for line in re.findall(r'POST_WORK,[^\n]+',raw['stdout'])]
    assert len(values)==288 and all(len(v)==16 for v in values)
    work.append(values)
assert work[0]==work[1]
expected_keys=[(k,str(n),str(r).lower(),m,str(q)) for k,n,r,m,q in itertools.product(families,[8,32],[False,True],modes[:3],range(4))]
assert [tuple(v[:5]) for v in work[0]]==expected_keys
work_records=[]
for row in work[0]:
    kind,size,reverse,mode,q=row[:5];numbers=list(map(int,row[5:]));ticks,force,match,validation,candidates,posts,binders,hits,nodes,templates,followed=numbers
    assert posts==int(size)*(2 if kind=='post-duplicate' else 1)
    assert binders==(0 if kind in ['post-input','post-forward'] else int(size))
    if kind=='post-template' and mode.endswith('-template'):assert hits>=int(size)-1
    work_records.append(dict(family=kind,size=int(size),reverse=reverse=='true',mode=mode,query=int(q),ticks=ticks,force=force,match=match,validation=validation,candidates=candidates,posts=posts,binders=binders,template_hits=hits,nodes=nodes,templates=templates,followed_calls=followed))
print(json.dumps(dict(configurations=864,workload_processes=2592,exact_allocation_pairs=864,ordinary_endpoints_equal=True,final_live_restored=True,work_rows=288,work_repeats_equal=True,primary_timing=False,summary=summaries,work=work_records),indent=2))
