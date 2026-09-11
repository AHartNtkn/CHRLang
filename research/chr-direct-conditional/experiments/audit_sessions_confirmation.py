"""Apply the registered twenty simultaneous paired-median intervals."""
import sys,gzip,hashlib,itertools,json,math,statistics
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s08-prepared-sessions-confirm'
if sys.argv[1:]==['replication']: BASE=ROOT/'docs/experiments/results/s08-prepared-sessions-replication'
CONFIGS=[('birth-miss',True),('native-scan',False),('native-scan',True)]
def main():
    freeze=json.loads((BASE/'freeze.json').read_text());jobs=freeze['jobs'];rows=[json.loads(s) for s in gzip.open(BASE/'runs.jsonl.gz','rt')];assert len(rows)==len(jobs)==1560
    for p,h in freeze['sources'].items():assert hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h,p
    assert hashlib.sha256(Path(freeze['binary']['path']).read_bytes()).hexdigest()==freeze['binary']['sha256']
    values={}
    for r,j in zip(rows,jobs):
        assert r['job']==j and r['raw']['exit_code']==0
        v=json.loads(r['raw']['stdout'].splitlines()[-1]);assert not v['meter'] and not v['rss']
        assert len(v['endpoints'])==128 and all(e['complete'] and e['answers']==8 and e['first_ns'] is not None for e in v['endpoints'])
        assert len(v['phases'])==7*(1 if j['case'][2] else 128)+7*128+1
        if j['rep']<0:continue
        values[(json.dumps(j['case']),j['rep'])]=sum(p['reading']['ns'] for p in v['phases'])
    clock=json.loads((ROOT/'docs/experiments/results/s08-prepared-sessions-repair/clock-check.json').read_text());p99=json.loads(clock['stdout'])['p99'];floor=10*2*p99*(14*128+1)
    contrasts=[]
    for family,consumer in itertools.product(['common','independent'],['immediate','all']):
        for config in CONFIGS:contrasts.append(('reuse/rebuild',[family,consumer,True,list(config)],[family,consumer,False,list(config)]))
        for config in CONFIGS[1:]:contrasts.append(('demand/native',[family,consumer,True,list(CONFIGS[0])],[family,consumer,True,list(config)]))
    results=[]
    for name,a,b in contrasts:
        av=[values[(json.dumps(a),i)] for i in range(64)];bv=[values[(json.dumps(b),i)] for i in range(64)];ratios=sorted(x/y for x,y in zip(av,bv));lo,hi=ratios[18],ratios[45]
        status='gain' if hi<.9 else 'loss' if lo>1.1 else 'within10%' if lo>=.9 and hi<=1.1 else 'uncertain magnitude'
        if min(statistics.median(av),statistics.median(bv))<=floor:status='signal-limited'
        results.append(dict(contrast=name,numerator=a,denominator=b,median_ratio=statistics.median(ratios),interval=[lo,hi],two_rank_sensitivity=[ratios[16],ratios[47]],status=status,numerator_median_ns=statistics.median(av),denominator_median_ns=statistics.median(bv)))
    error=20*2*sum(math.comb(64,k) for k in range(19))/2**64
    out=dict(processes=len(rows),primary=1536,warmup=24,error_bound=error,signal_floor_ns=floor,results=results)
    (BASE/'audit.json').write_text(json.dumps(out,indent=2));print(json.dumps(out,indent=2))
if __name__=='__main__':main()
