"""Verify fixture coverage and frozen receipts independently of the timed runner."""
import hashlib,json,re,statistics,zipfile,collections
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s08-context-selection-repair'
def main():
    f=json.loads((OUT/'freeze.json').read_text());sha=lambda b:hashlib.sha256(b).hexdigest()
    assert sha((OUT/'sources.zip').read_bytes())==f['archive_sha256']
    with zipfile.ZipFile(OUT/'sources.zip') as z:
        for p,h in f['sources'].items():assert sha(z.read(p))==h
    truth=json.loads((OUT/'truth.json').read_text());assert truth['exit_code']==0 and 'CONTEXT_TRUTH,cases=531441' in truth['stdout'] and '1 passed; 0 failed' in truth['stdout']
    raw=json.loads((OUT/'operations.json').read_text());assert raw['exit_code']==0
    expected=set()
    for n in [0,1,2,8,16,32,63,64,65,127,256,257]:
        for k in {0,1,2,max(0,n//4-1),n//4,n//4+1,n//2,n}:
            if k>n:continue
            for keys in {tuple(range(k)),tuple(range(n-k,n)),tuple(0 if k<=1 else i*(n-1)//(k-1) for i in range(k))}:
                for outcome in (['match'] if k==0 else ['match','first-conflict','last-conflict','missing']):expected.add((n,k,keys,outcome))
    cases={}
    for i,n,k,keys,outcome in re.findall(r'SELECT_CASE,id=(\d+),n=(\d+),k=(\d+),keys=(\[[^\]]*\]),outcome=([^\n]+)',raw['stdout']):
        assert int(i) not in cases
        cases[int(i)]=(int(n),int(k),tuple(json.loads(keys)),outcome)
    assert set(cases.values())==expected and len(cases)==len(expected)
    assert set(cases)==set(range(len(expected)))
    times=collections.defaultdict(dict)
    for i,rep,method,ns in re.findall(r'SELECT_TIME,id=(\d+),rep=(\d+),method=(\d+),ns=(\d+)',raw['stdout']):
        key=(int(rep),int(method));assert key not in times[int(i)];times[int(i)][key]=int(ns)
    assert set(times)==set(cases)
    rows=[]
    for i,c in cases.items():
        assert set(times[i])=={(r,m) for r in range(9) for m in range(3)}
        for method,control in [(1,0),(2,0),(2,1)]:
            ratios=[times[i][r,method]/times[i][r,control] for r in range(9)]
            med=statistics.median(ratios)
            rows.append(dict(id=i,case=c,method=method,control=control,median=med,minimum=min(ratios),maximum=max(ratios),disposition='lower' if med<.9 and max(ratios)<1 else 'higher' if med>1.1 and min(ratios)>1 else 'unresolved'))
    (OUT/'analysis.json').write_text(json.dumps(dict(cases=len(cases),native_rows=len(cases)*27,truth_pairs=531441,comparisons=rows),indent=2)+'\n')
    print('Verified',len(cases),'cases;',len(cases)*27,'native rows')
    a=[r for r in rows if r['method']==2 and r['control']==0]
    print('selection/lookup',dict(collections.Counter(r['disposition'] for r in a)))
    for r in sorted(a,key=lambda r:r['median'],reverse=True)[:5]:print(r['id'],r['case'][0:2],r['case'][3],r['median'],r['disposition'])
if __name__=='__main__':main()
