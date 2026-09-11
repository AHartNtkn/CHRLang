"""Exact generated post endpoints, diagnostic repetitions and native work checks."""
import hashlib,itertools,json,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s03-post-controls'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f=read(BASE/'freeze.json');assert sha(BASE/'sources.zip')==f['archive_sha256']
with zipfile.ZipFile(BASE/'sources.zip') as z:
    assert set(z.namelist())==set(f['sources'])
    for p,h in f['sources'].items():assert hashlib.sha256(z.read(p)).hexdigest()==h
families=['post-input','post-output','post-forward','post-miss','post-duplicate','post-template']
expected=list(itertools.product(families,[1,8,32],['false','true'],['a','b'],range(3),['Global','Active'],['Scan','Indexed']));rows={}
for build,b in f['binaries'].items():
    assert sha(Path(b['path']))==b['sha256'];repeats=[]
    for rep in [1,2]:
        r=read(BASE/f'{build}-{rep}.json');assert r['exit_code']==0 and not r['timeout'] and not r['stderr'];assert '1 passed; 0 failed' in r['stdout'] and 'POST_CONTROL_COMPLETE,864' in r['stdout'];parsed=[]
        for line in r['stdout'].splitlines():
            if 'POST_CONTROL,' not in line:continue
            family,n,reverse,value,mode,policy,access,*work=line[line.index('POST_CONTROL,'):].split(',')[1:];key=(family,int(n),reverse,value,int(mode),policy,access);work=list(map(int,work));assert len(work)==8;parsed.append((key,work))
            if build=='plain':assert work==[0]*8
            else:
                assert work[-1]==(int(n)*(1 if family=='post-miss' else 3 if family in ['post-forward','post-duplicate'] else 2)+1)
                if int(mode)>0:assert work[2]==0
                if int(mode)==2:assert work[4:7]==[0,0,0]
        assert [k for k,v in parsed]==expected;repeats.append(parsed)
    assert repeats[0]==repeats[1];rows[build]=dict(repeats[0])
for family,n,rev,val,pol,acc in itertools.product(families,[1,8,32],['false','true'],['a','b'],['Global','Active'],['Scan','Indexed']):
    group=[rows['work'][(family,n,rev,val,m,pol,acc)][:2] for m in range(3)];assert group[0]==group[1]==group[2]
selected=[]
for fam,rev,mode,pol,acc in itertools.product(families,['false','true'],[0,2],['Global','Active'],['Scan','Indexed']):selected.append(dict(family=fam,size=32,reverse=rev=='true',mode=mode,policy=pol,access=acc,work=rows['work'][(fam,32,rev,'a',mode,pol,acc)]))
print(json.dumps(dict(source_configurations=72,executions_per_run=864,runs=4,exact_repeats=True,all_policies_match_independent_answers=True,generated_traces_match_generic=True,generated_matching_counts_equal_generic=True,counters_off_zero=True,timing_measured=False,allocation_measured=False,work_columns=['structural','candidates','ast','keys','slot_copies','pool_entries','key_templates','applications'],size32=selected),indent=2))
