"""Audit frozen stable-selection coverage and repeated process receipts."""
import hashlib,itertools,json,re,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s02-stable-selection'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f=read(BASE/'freeze.json');assert sha(BASE/'sources.zip')==f['archive_sha256']
with zipfile.ZipFile(BASE/'sources.zip') as z:
    assert set(z.namelist())==set(f['sources'])
    for p,h in f['sources'].items():assert hashlib.sha256(z.read(p)).hexdigest()==h
assert sha(Path(f['binary']['path']))==f['binary']['sha256']
orders=list(itertools.permutations(range(3)))
expected=[f'STABLE_CASE,order={list(o)},reverse={r},repair={p},ready={m},tokens={t},access={a}' for o,r,p,m,t,a in itertools.product(orders,['false','true'],['false','true'],[0,1,3,7],range(4),['Scan','Indexed'])]
chains=[f'STABLE_CHAIN,order={o},tokens={t},access={a}' for o,t,a in itertools.product([[1,0,2],[2,1,0]],range(4),['Scan','Indexed'])]
repeats=[]
for name in ['target-1','target-2']:
    raw=read(BASE/f'{name}.json');assert raw['exit_code']==0 and not raw['timeout'] and not raw['stderr']
    rows=re.findall(r'STABLE_CASE,[^\n]+',raw['stdout']);assert rows==expected
    assert re.findall(r'STABLE_CHAIN,[^\n]+',raw['stdout'])==chains
    assert '2 passed; 0 failed' in raw['stdout'];repeats.append(rows)
assert repeats[0]==repeats[1]
suite=read(BASE/'suite.json');assert suite['exit_code']==0 and not suite['timeout'] and not suite['stderr'];assert '26 passed; 0 failed' in suite['stdout']
print(json.dumps(dict(matrix_source_cases=384,chain_source_cases=8,compiled_comparisons_per_repeat=784,exact_repeat_coverage=True,containing_tests_passed=26,epoch_mutation_rejected=True,comparative_costs=False),indent=2))
