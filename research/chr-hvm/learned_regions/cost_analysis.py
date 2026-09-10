"""Descriptive registered contrasts and phase attribution from frozen observations."""
from pathlib import Path
import collections,hashlib,itertools,json,statistics as st
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s06-learning-cost-pilot'
def main():
    valid=json.loads((OUT/'validation.json').read_text())
    for f,h in valid['hashes'].items():assert hashlib.sha256((OUT/f).read_bytes()).hexdigest()==h
    manifest=json.loads((OUT/'manifest.json').read_text())
    for f,h in manifest['hashes'].items():assert hashlib.sha256((ROOT/f).read_bytes()).hexdigest()==h,f
    configs=manifest['configurations'];rows=[json.loads(l) for l in (OUT/'runs.jsonl').read_text().splitlines()]
    assert [[r['config'],r['cpu'],r['repetition']] for r in rows]==manifest['order']
    assert len(rows)==972 and sum(r['repetition']>=0 for r in rows)==810
    by=collections.defaultdict(list)
    for r in rows:
        if r['repetition']>=0:by[tuple(configs[r['config']]),r['cpu']].append(r)
    assert len(by)==162
    for rs in by.values():assert sorted(r['repetition'] for r in rs)==list(range(5))
    def screen(config,metric):
        directions=[];cpus=[]
        for cpu in [0,1]:
            a=[r[metric] for r in by[config,cpu]];b=[r[metric] for r in by[('recompute',*config[1:]),cpu]]
            ratio=st.median(a)/st.median(b)
            d='gain' if ratio<=.9 and max(a)<min(b) else 'loss' if ratio>=1.1 and min(a)>max(b) else 'unresolved'
            directions.append(d);cpus.append(dict(cpu=cpu,ratio=ratio,candidate_range=[min(a),max(a)],recompute_range=[min(b),max(b)]))
        return dict(direction=directions[0] if directions[0]==directions[1] else 'unresolved',cpus=cpus)
    comparisons=[]
    for config in map(tuple,configs):
        if config[0]=='recompute':continue
        a=by[config,0]+by[config,1];b=by[('recompute',*config[1:]),0]+by[('recompute',*config[1:]),1]
        def phases(rs):
            names={p['name'] for p in rs[0]['row']['phases']}
            return {name:st.median(sum(p['ns'] for p in r['row']['phases'] if p['name']==name) for r in rs) for name in sorted(names)}
        comparisons.append(dict(config=config,total=screen(config,'total_ns'),service=screen(config,'service_ns'),candidate_total_median=st.median(r['total_ns'] for r in a),recompute_total_median=st.median(r['total_ns'] for r in b),candidate_phases=phases(a),recompute_phases=phases(b)))
    output=dict(comparisons=comparisons,counts={m:dict(collections.Counter(c['total']['direction'] for c in comparisons if c['config'][0]==m)) for m in ['eager','covered']})
    (OUT/'analysis.json').write_text(json.dumps(output,indent=2)+'\n');print(output['counts'])
    for c in comparisons:
        print(c['config'],c['total']['direction'],round(c['candidate_total_median']/c['recompute_total_median'],3),'service',c['service']['direction'])
if __name__=='__main__':main()
