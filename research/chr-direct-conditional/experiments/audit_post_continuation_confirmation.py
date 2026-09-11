"""Reconstruct all pre-registered paired median intervals and outcomes."""
import collections,gzip,hashlib,json,math,statistics
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];PARENT=ROOT/'docs/experiments/results/s10-post-continuation-cost';BASE=PARENT/'confirmation'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    f=json.loads((BASE/'freeze.json').read_text());assert sha(PARENT/'freeze.json')==f['parent_sha256'];assert sha(Path(f['binary']['path']))==f['binary']['sha256']
    for p,h in f['sources'].items():assert sha(ROOT/p)==h
    parent=json.loads((PARENT/'freeze.json').read_text())
    for p,h in parent['sources'].items():assert sha(ROOT/p)==h,p
    values={};endpoints={};n=0
    with gzip.open(BASE/'runs.jsonl.gz','rt') as inp:
        for line in inp:
            r=json.loads(line);n+=1;assert r['raw']['exit_code']==0
            value=next(json.loads(l) for l in r['raw']['stdout'].splitlines() if l.startswith('{') and json.loads(l).get('event')=='result');assert not value['meter'] and len(value['phases'])==36 and all(e['complete'] for e in value['endpoints'])
            key=(r['index'],r['side']);assert endpoints.setdefault(key,value['endpoints'])==value['endpoints']
            key=(r['index'],r['rep'],r['side']);assert key not in values;values[key]=sum(p['reading']['ns'] for p in value['phases'])
    assert n==5070 and len(values)==5070
    rows=[]
    for i,case in enumerate(f['cases']):
        ratios=[values[i,r,'left']/values[i,r,'right'] for r in range(64)];s=sorted(ratios);lo,hi=s[18],s[45]
        limited=any(values[i,r,side]<f['clock_floor_ns'] for r in range(64) for side in ['left','right'])
        verdict='signal-limited' if limited else 'gain' if hi<.9 else 'loss' if lo>1.1 else 'within-10%' if lo>=.9 and hi<=1.1 else 'unresolved magnitude'
        rows.append(dict(case=case,median_ratio=statistics.median(ratios),interval=[lo,hi],verdict=verdict,first_half=statistics.median(ratios[:32]),last_half=statistics.median(ratios[32:]),ratios=ratios))
    out=dict(processes=n,family_error_bound=78*sum(math.comb(64,i) for i in range(19))/2**64,counts=dict(collections.Counter(r['verdict'] for r in rows)),rows=rows)
    (BASE/'audit.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k!='rows'}))
    for r in rows:print(r['case'],r['interval'],r['verdict'])
if __name__=='__main__':main()
