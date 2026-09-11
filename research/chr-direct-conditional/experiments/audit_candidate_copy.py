"""Reconstruct copy attribution from raw receipts and the frozen parent."""
import hashlib,itertools,json,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s03-candidate-copy';PARENT=ROOT/'docs/experiments/results/s03-post-ownership'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def norm(x,base):
    if isinstance(x,list):return [norm(v,base) for v in x]
    if isinstance(x,dict):return {k:(v-base if k in ['live_start','live_end','peak_live'] else norm(v,base)) for k,v in x.items() if k!='ns'}
    return x
f=read(BASE/'freeze.json');assert sha(BASE/'sources.zip')==f['archive_sha256']
with zipfile.ZipFile(BASE/'sources.zip') as archive:
    assert set(archive.namelist())==set(f['sources'])
    for path,digest in f['sources'].items():assert hashlib.sha256(archive.read(path)).hexdigest()==digest,path
assert sha(Path(f['binary']['path']))==f['binary']['sha256']
assert sha(BASE/'cells.json')==f['cells_sha256']
for p,digest in f['parent'].items():assert sha(PARENT/p)==digest
parent_cells=read(PARENT/'cells.json');parent_rows=[json.loads(x) for x in (PARENT/'results.jsonl').read_text().splitlines()]
cells=read(BASE/'cells.json')
assert cells==[dict(parent_index=i,cell=c) for i,c in enumerate(parent_cells) if c[3] in ['birth','birth-miss','birth-miss-template'] and c[4]=='all']
assert len(cells)==144 and len(list((BASE/'runs').glob('*.json')))==288
check=read(BASE/'meter-check.json');assert check['exit_code']==0 and 'meter-check passed' in check['stdout']
summary=[]
for i,row in enumerate(cells):
    family,size,reverse,mode,retention,cancel=row['cell']
    parent=parent_rows[3*row['parent_index']]['result']
    assert parent_rows[3*row['parent_index']]['kind']=='meter'
    baseline=parent['phases'][0]['reading']['memory']['live_start']
    profiles=[]
    for rep in range(2):
        raw=read(BASE/'runs'/f'{i}-{rep}.json')
        assert raw['command']==[f['binary']['path'],mode,family,str(size),str(reverse).lower(),retention,str(cancel).lower()]
        assert raw['exit_code']==0 and not raw['timeout'] and not raw['stderr'],(i,rep)
        events=[json.loads(x) for x in raw['stdout'].splitlines() if x.startswith('{')]
        assert [e['event'] for e in events]==['start','result','candidate-profile']
        result=events[1];base=result['phases'][0]['reading']['memory']['live_start']
        assert norm(result,base)==norm(parent,baseline),('parent mismatch',i,rep)
        assert result['phases'][-1]['reading']['memory']['live_end']==base
        profiles.append(events[2]['rows'])
    assert profiles[0]==profiles[1],i
    profile=profiles[0];assert [p['phase'] for p in profile]==['arguments','environment','selection']
    assert profile[0]['scopes']==profile[1]['scopes']>=profile[2]['scopes']
    assert all(all(isinstance(v,int) and v>=0 for k,v in p.items() if k!='phase') for p in profile)
    phases=parent['phases'];total=sum(p['reading']['memory']['requested_bytes'] for p in phases)
    execution=sum(p['reading']['memory']['requested_bytes'] for p in phases if p['phase']=='execute_observe')
    copies=sum(p['requested_bytes'] for p in profile);assert copies<=execution
    controls={}
    for control in ['scan','indexed','sealed']:
        cell=[family,size,reverse,control,retention,cancel]
        r=parent_rows[3*parent_cells.index(cell)]['result']
        controls[control]=sum(p['reading']['memory']['requested_bytes'] for p in r['phases'])
    summary.append(dict(index=i,cell=row['cell'],requested_bytes=total,execution_bytes=execution,copy_bytes=copies,optimistic_remaining_bytes=total-copies,controls=controls,profile=profile))
print(json.dumps(dict(configurations=144,processes=288,parent_allocation_and_endpoint_agreement=True,exact_profile_repeats=True,final_live_restored=True,primary_timing=False,summary=summary),indent=2))
