"""Independently audit coverage and agreement from the selection receipts."""
import hashlib,itertools,json,re,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s02-request-priority'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f=read(BASE/'freeze.json');assert sha(BASE/'sources.zip')==f['archive_sha256']
with zipfile.ZipFile(BASE/'sources.zip') as z:
    assert set(z.namelist())==set(f['sources'])
    for p,h in f['sources'].items():assert hashlib.sha256(z.read(p)).hexdigest()==h
assert sha(Path(f['binary']['path']))==f['binary']['sha256']
expected=list(itertools.product(range(2),range(2),['None','Some(0)','Some(1)'],range(3),['false','true'],['Scan','Indexed']))
pattern=r'REQUEST_PRIORITY,request=(\d),descriptor=(\d),repair=(None|Some\([01]\)),tokens=(\d),request_priority=(false|true),access=(Scan|Indexed),agrees=(true|false)'
repeats=[]
for name in ['target-1','target-2']:
    raw=read(BASE/f'{name}.json');assert raw['exit_code']==0 and not raw['timeout'] and not raw['stderr']
    rows=re.findall(pattern,raw['stdout']);assert len(rows)==144
    keys=[(int(a),int(b),c,int(d),e,f) for a,b,c,d,e,f,g in rows];assert keys==expected
    for a,b,c,d,e,f,g in rows:
        winner=(1-int(c[5])) if c!='None' else int(a if e=='true' else b)
        assert (g=='true')==(int(d)!=1 or winner==int(a))
    repeats.append(rows)
assert repeats[0]==repeats[1]
suite=read(BASE/'suite.json');assert suite['exit_code']==0 and not suite['timeout'] and not suite['stderr'];assert '24 passed; 0 failed' in suite['stdout']
counts={mode:sum(e==mode and g=='false' for a,b,c,d,e,f,g in repeats[0]) for mode in ['false','true']}
assert counts=={'false':12,'true':8}
print(json.dumps(dict(source_configurations=36,encoding_configurations=72,compiled_comparisons_per_repeat=144,exact_repeats=True,containing_tests_passed=24,disagreements=counts,no_repair_request_priority_disagreements=sum(c=='None' and e=='true' and g=='false' for a,b,c,d,e,f,g in repeats[0]),timing_comparison=False),indent=2))
