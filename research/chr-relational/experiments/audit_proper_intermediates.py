"""Audit source qualification, exact work repeats and retained-entry predictions."""
import hashlib,itertools,json,re,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s01-proper-intermediates'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f=read(BASE/'freeze.json');assert sha(BASE/'sources.zip')==f['archive_sha256']
with zipfile.ZipFile(BASE/'sources.zip') as z:
    assert set(z.namelist())==set(f['sources'])
    for p,h in f['sources'].items():assert hashlib.sha256(z.read(p)).hexdigest()==h
assert sha(Path(f['binary']['path']))==f['binary']['sha256']
repeats=[]
for name in ['target-1','target-2']:
    raw=read(BASE/f'{name}.json');assert raw['exit_code']==0 and not raw['timeout'] and not raw['stderr'];assert '10 passed; 0 failed' in raw['stdout']
    assert raw['stdout'].count('INTERMEDIATE_SOURCE,')==76
    rows=re.findall(r'PROPER_JOIN,size=(\d+),mode=(\w+),heads=(\d+),facts=(\d+),intermediates=(\d+),peak=(\d+),firings=(\d+)',raw['stdout'])
    assert [(int(n),m) for n,m,*_ in rows]==list(itertools.product([2,4,8],['scan','partial','intermediate']))
    for n,m,h,fact,visits,peak,firings in rows:
        n,h,fact,visits,peak,firings=map(int,[n,h,fact,visits,peak,firings]);assert firings==n
        if m=='partial':assert peak==h==n+n*n+n**3
        elif m=='intermediate':assert peak==n+n*n;assert h==n+n*n+n*n*n*(n+1)//2-n*(n-1)//2;assert visits>0
        else:assert peak==visits==0
    repeats.append(rows)
assert repeats[0]==repeats[1]
p=read(BASE/'semantic-package.json');assert p['exit_code']==0;assert len(p['targets'])==16 and len(p['excluded_cost_clis'])==6
results=re.findall(r'test result: ok\. (\d+) passed; 0 failed',p['stdout']);assert len(results)==17
print(json.dumps(dict(source_configurations=76,new_mode_checks_per_repeat=152,work_rows=9,exact_repeats=True,semantic_targets=17,semantic_tests=sum(map(int,results)),primary_timing=False,allocation_comparison=False,work=[dict(size=int(n),mode=m,heads=int(h),fact_visits=int(fa),intermediate_visits=int(v),peak_entries=int(p),firings=int(f)) for n,m,h,fa,v,p,f in repeats[0]]),indent=2))
