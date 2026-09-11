"""Audit frozen useful-union receipts against independently enumerated graph denotations."""
import hashlib,itertools,json,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s06-useful-union'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f=read(BASE/'freeze.json');assert sha(BASE/'sources.zip')==f['archive_sha256']
with zipfile.ZipFile(BASE/'sources.zip') as z:
    assert set(z.namelist())==set(f['sources'])
    for p,h in f['sources'].items():assert hashlib.sha256(z.read(p)).hexdigest()==h
all_rows=[]
for mode,b in f['binaries'].items():
    assert sha(Path(b['path']))==b['sha256']
    for repeat in [1,2]:
        r=read(BASE/f'{mode}-{repeat}.json');assert r['exit_code']==0 and not r['timeout'] and not r['stderr'];assert '1 passed; 0 failed' in r['stdout']
        rows=[]
        for line in r['stdout'].splitlines():
            if 'UNION,' not in line:continue
            family,*nums=line[line.index('UNION,'):].split(',')[1:]
            values=list(map(int,nums));assert len(values)==10
            rows.append(dict(zip(['family','width','branches','unique','raw','compiled_nodes','union_nodes','compile_budget','union_budget','branch_probes','edge_probes'],[family,*values])))
        assert [(r['width'],r['family']) for r in rows]==list(itertools.product([4,6,8],['overlap','disjoint','redundant','single']))
        all_rows.append(rows)
assert all(r==all_rows[0] for r in all_rows)
checks=0
for r in all_rows[0]:
    n=r['width'];cycle=[(i,(i+1)%n) for i in range(n)]
    if r['family']=='overlap':branches=[cycle[:i]+cycle[i+1:] for i in range(n)]
    elif r['family']=='disjoint':branches=[[e for e in itertools.combinations(range(4),2) if e!=skip]+[(i-1,i) for i in range(4,n)] for skip in [(0,1),(2,3)]]
    elif r['family']=='redundant':branches=[cycle]*n
    else:branches=[cycle]
    truth={};bp=ep=0
    for a in itertools.product(range(3),repeat=n):
        count=sum(all(a[i]!=a[j] for i,j in b) for b in branches)
        if count:truth[a]=count
        for b in branches:
            bp+=1;ok=True
            for i,j in b:
                ep+=1
                if a[i]==a[j]:ok=False;break
            if ok:break
        checks+=1
    assert r['branches']==len(branches) and r['unique']==len(truth) and r['raw']==sum(truth.values())
    assert (r['branch_probes'],r['edge_probes'])==(bp,ep)
print(json.dumps(dict(cases=12,caller_result_sets_per_repeat=60,reference_source_cases_per_repeat=20,complete_memberships_per_repeat=checks,control_membership_checks_per_repeat=checks*5,builds=2,repeats=2,exact_repeats=True,primary_timing=False,allocation_comparison=False,rows=all_rows[0]),indent=2))
