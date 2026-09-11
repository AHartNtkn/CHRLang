"""Derive readable complete-cost and phase summaries without altering raw evidence."""
import collections,csv,gzip,json,statistics
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s10-post-continuation-cost'
def main():
    a=json.loads((BASE/'audit.json').read_text());groups=collections.defaultdict(list)
    with gzip.open(BASE/'runs.jsonl.gz','rt') as f:
        for line in f:
            r=json.loads(line);j=r['job']
            if j['stage']!='primary':continue
            case=j['case']
            if case[1:]!=[3,16,True,False,'all'] or j['mode'] not in ['native-scan','birth-miss','conditional']:continue
            result=next(json.loads(l) for l in r['raw']['stdout'].splitlines() if l.startswith('{') and json.loads(l).get('event')=='result')
            values=collections.Counter()
            for p in result['phases']:
                stage='preparation' if p['phase'] in ['source','count_prepare','static_prepare','value_prepare','engine_prepare','source_drop'] else 'query_setup' if p['phase'] in ['input','count_query','static_query','setup'] else 'execution_observation' if p['phase']=='execute_observe' else 'consumer_disposal'
                values[stage]+=p['reading']['ns']
            groups[(*case,j['mode'],j['counted'])].append(values)
    out=[dict(key=k,median_group_ns={stage:statistics.median(v[stage] for v in rows) for stage in ['preparation','query_setup','execution_observation','consumer_disposal']}) for k,rows in sorted(groups.items())]
    (BASE/'phase-summary.json').write_text(json.dumps(out,indent=2)+'\n')
    with (BASE/'summary.csv').open('w') as f:
        w=csv.writer(f,lineterminator="\n");w.writerow(['family','choices','depth','history','reverse','consumer','mode','counted','median_ns','min_ns','max_ns','requested_bytes','peak_excess','signal_limited'])
        for r in a['summary']:w.writerow([*r['key'],r['median_ns'],r['min_ns'],r['max_ns'],r['bytes'],r['peak'],r['sensitive']])
    for r in out:print(r)
if __name__=='__main__':main()
