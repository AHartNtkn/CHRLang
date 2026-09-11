"""Audit complete run matrix, frozen evidence and diagnostic contrasts."""
import hashlib,itertools,json,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
BASE=ROOT/'docs/experiments/results/s01-generated-prefix'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f=read(BASE/'freeze.json')
assert sha(BASE/'sources.zip')==f['archive_sha256']
with zipfile.ZipFile(BASE/'sources.zip') as z:
    assert set(z.namelist())==set(f['sources'])
    for p,h in f['sources'].items():assert hashlib.sha256(z.read(p)).hexdigest()==h,p
expected=list(itertools.product(['sparse','keyed','kill','miss'],[4,8],[0,8,32],[0,1],range(3),['Global','Active'],['Scan','Indexed']))
rows={}
for mode,b in f['binaries'].items():
    assert sha(Path(b['path']))==b['sha256']
    repeats=[]
    for rep in [1,2]:
        r=read(BASE/f'{mode}-{rep}.json')
        assert r['exit_code']==0 and not r['timeout'] and not r['stderr']
        assert '1 passed; 0 failed' in r['stdout'] and 'PREFIX_COMPLETE,576' in r['stdout']
        parsed=[]
        for l in r['stdout'].splitlines():
            if 'PREFIX,' not in l:continue
            family,n,depth,seed,impl,policy,access,*values=l[l.index('PREFIX,'):].split(',')[1:]
            key=(family,int(n),int(depth),int(seed),int(impl),policy,access)
            values=list(map(int,values));assert len(values)==8
            parsed.append((key,values))
            if mode=='plain':assert values==[0]*8,(key,values)
            else:
                assert values[-1]==(int(n) if family in ['sparse','keyed'] else (2*int(n)-1 if family=='kill' and policy=='Active' else int(n) if family=='kill' else 0)),key
                if int(impl)>0:assert values[2]==0
                if int(impl)==2:assert values[4:7]==[0,0,0]
        assert [k for k,v in parsed]==expected
        repeats.append(parsed)
    assert repeats[0]==repeats[1]
    rows[mode]=dict(repeats[0])
# Native emission must preserve actual structural and candidate work of each plan.
for family,n,depth,seed,policy,access in itertools.product(['sparse','keyed','kill','miss'],[4,8],[0,8,32],[0,1],['Global','Active'],['Scan','Indexed']):
    group=[rows['work'][(family,n,depth,seed,i,policy,access)][:2] for i in range(3)]
    assert group[0]==group[1]==group[2],(family,n,depth,seed,policy,access,group)
contrasts={}
for family,policy,access in itertools.product(['sparse','keyed','kill','miss'],['Global','Active'],['Scan','Indexed']):
    contrasts[f'{family}/{policy}/{access}']=[rows['work'][(family,8,d,0,0,policy,access)][0] for d in [0,8,32]]
print(json.dumps(dict(configurations_per_run=576,scalar_agreement_per_run=504,independent_active_kill_counterexamples_per_run=72,repetitions_per_build=2,builds=2,exact_repeats=True,matching_work_equal_across_execution_forms=True,generated_ast_visits=0,native_frame_pool_template_counts=0,width8_structural_tests=contrasts,timing_measured=False,allocation_measured=False),indent=2))
