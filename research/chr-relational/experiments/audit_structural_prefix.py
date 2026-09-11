"""Verify frozen source, complete test receipts, exact repeats and depth contrasts."""
import hashlib,itertools,json,re,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s01-structural-prefix'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f=read(BASE/'freeze.json');assert sha(BASE/'sources.zip')==f['archive_sha256']
with zipfile.ZipFile(BASE/'sources.zip') as z:
    assert set(z.namelist())==set(f['sources'])
    for p,h in f['sources'].items():assert hashlib.sha256(z.read(p)).hexdigest()==h,p
modes=['local-'+str(i) for i in range(4)]+['compiled-'+str(a).lower()+'-'+str(b).lower() for a,b in itertools.product([False,True],repeat=2)]
expected=list(itertools.product(['sparse','keyed','kill','miss'],[4,8],[0,8,32],[0,1],modes))
rows={}
for build,b in f['binaries'].items():
    assert sha(Path(b['path']))==b['sha256']
    repeats=[]
    for rep in [1,2]:
        r=read(BASE/f'{build}-{rep}.json');assert r['exit_code']==0 and not r['timeout'] and not r['stderr']
        assert '2 passed; 0 failed' in r['stdout']
        parsed=[]
        for line in r['stdout'].splitlines():
            if 'STRUCTURE,' not in line:continue
            fields=line[line.index('STRUCTURE,'):].split(',')[1:]
            family,n,depth,seed,mode,*work=fields
            parsed.append(((family,int(n),int(depth),int(seed),mode),list(map(int,work))))
        assert [k for k,w in parsed]==expected
        repeats.append(parsed)
    assert repeats[0]==repeats[1]
    rows[build]=dict(repeats[0])
for key,work in rows['work'].items():
    family,n,depth,seed,mode=key
    assert work==rows['work'][family,n,depth,1-seed,mode]
    if mode.startswith('local'):
        assert work==rows['plain'][key]
        assert work[-1]==(n if family in ['sparse','keyed','kill'] else 0)
        if mode=='local-3':assert work[5] <= n+n*n+1
    else:
        assert rows['plain'][key]==[0]*5
        assert work[-1]==0 # inferred specialization is ineligible for these rules
        assert work==rows['work'][family,n,depth,seed,mode.replace('true-','false-')]
slopes=[]
for family,n,mode in itertools.product(['sparse','keyed','kill','miss'],[4,8],modes):
    values=[rows['work'][family,n,d,0,mode][0] for d in [0,8,32]]
    slopes.append(dict(family=family,width=n,mode=mode,depth_0_8_32=values))
for n in [4,8]:
    local=lambda m,d:rows['work']['sparse',n,d,0,m][0]
    assert local('local-0',32)-local('local-0',0)>(local('local-3',32)-local('local-3',0))>0
    assert local('compiled-false-true',32)>local('compiled-false-true',0)
print(json.dumps(dict(source_configurations=48,rows_per_build=384,repeats_per_build=2,builds=2,complete_backend_answers_per_repeat=576,cancellation_reuse_pairs_per_repeat=384,exact_repeats=True,inferred_specialized_applications=0,timing_measured=False,allocation_measured=False,pattern_or_structural_tests=slopes),indent=2))
