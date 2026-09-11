"""Verify frozen context gates and summarize paired ordinary operation costs."""
import hashlib,json,re,statistics,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
def main():
    for name,methods in [('inclusion',2),('seeking',3)]:
        out=ROOT/f'docs/experiments/results/s08-context-{name}'
        f=json.loads((out/'freeze.json').read_text())
        sha=lambda b:hashlib.sha256(b).hexdigest()
        assert sha((out/'sources.zip').read_bytes())==f['archive_sha256']
        with zipfile.ZipFile(out/'sources.zip') as z:
            for p,h in f['sources'].items():assert sha(z.read(p))==h
        work=None
        for alg in ['lookup','ordered','seeking'][:methods]:
            for rep in range(2):
                r=json.loads((out/f'{alg}-{rep}.json').read_text());assert r['exit_code']==0
                assert 'CONTEXT_TRUTH,cases=531441\nok' in r['stdout']
                assert '0 failed;' in r['stdout']
                rows=re.findall(r'CONTEXT_WORK,[^\n]+',r['stdout']);assert len(rows)==51
                if work is None:work=rows
                assert rows==work
        r=json.loads((out/'operations.json').read_text());assert r['exit_code']==0
        rows=[dict(part.split('=') for part in line.split(',')[1:]) for line in re.findall(r'CONTEXT_TIME,[^\n]+',r['stdout'])]
        assert len(rows)==51*9*methods
        cells={}
        for row in rows:
            key=tuple(row[k] for k in ['size','pattern','outcome'])
            k=(int(row['rep']),int(row['method']) if 'method' in row else int(row['ordered']=='true'))
            assert k not in cells.setdefault(key,{})
            cells[key][k]=int(row['ns'])
        assert len(cells)==51
        comparisons=[]
        for key,data in sorted(cells.items()):
            assert set(data)=={(r,m) for r in range(9) for m in range(methods)}
            for m in range(1,methods):
                ratios=[data[r,m]/data[r,0] for r in range(9)]
                med=statistics.median(ratios)
                comparisons.append(dict(case=key,method=m,median=med,minimum=min(ratios),maximum=max(ratios),disposition='lower' if med<.9 and max(ratios)<1 else 'higher' if med>1.1 and min(ratios)>1 else 'unresolved'))
        (out/'analysis.json').write_text(json.dumps(dict(truth_pairs_per_build=729**2,work_cases=51,native_rows=len(rows),comparisons=comparisons),indent=2)+'\n')
        print(name,'archive, repeated truth/work and',len(rows),'native rows verified')
if __name__=='__main__':main()
