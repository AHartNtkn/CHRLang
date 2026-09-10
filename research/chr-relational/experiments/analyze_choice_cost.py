"""Reconstruct the frozen pilot and report descriptive, placement-specific contrasts."""
from pathlib import Path
import collections,hashlib,json,statistics
from choice_cost import RAW,ROOT,CASES,ENGINES,verify
median=statistics.median
verify()
schedule=json.loads((RAW/'schedule.json').read_text())
assert hashlib.sha256((RAW/'sizes.json').read_bytes()).hexdigest()==schedule['sizes_sha256']
sizes=json.loads((RAW/'sizes.json').read_text())
results={}
for cpu,rep,i,e in schedule['jobs']:
    d=json.loads((RAW/f'confirm-{cpu}-{rep}-{i}-{e}.json').read_text())
    assert d['returncode']==0 and d['cpu']==cpu
    assert d['args']==[e,*CASES[i],sizes[f'{i}-{e}']]
    r=json.loads(d['stdout']);assert sum(r['phases_ns'])==r['total_ns'] and r['batches']==sizes[f'{i}-{e}']
    r={k:([v/r['batches'] for v in x] if isinstance(x,list) else x/r['batches']) for k,x in r.items() if k!='batches'}
    results[cpu,rep,i,e]=r
assert len(results)==5120
contrasts=[]
for cpu in [0,8]:
    for i,case in enumerate(CASES):
        for e in ['scan','indexed','context']:
            a=[results[cpu,rep,i,'local']['total_ns'] for rep in range(5)]
            b=[results[cpu,rep,i,e]['total_ns'] for rep in range(5)]
            ratios=[x/y for x,y in zip(a,b)]
            signal='lower-range' if max(a)<0.9*min(b) else 'higher-range' if min(a)>1.1*max(b) else 'overlap'
            contrasts.append(dict(cpu=cpu,case=i,source=case,control=e,paired_median_ratio=median(ratios),ratio_range=[min(ratios),max(ratios)],signal=signal,local_median_ns=median(a),control_median_ns=median(b),local_phases_ns=[median(results[cpu,rep,i,'local']['phases_ns'][j] for rep in range(5)) for j in range(7)],control_phases_ns=[median(results[cpu,rep,i,e]['phases_ns'][j] for rep in range(5)) for j in range(7)]))
summary={}
for e in ['scan','indexed','context']:
    rows=[x for x in contrasts if x['control']==e]
    summary[e]=dict(collections.Counter(x['signal'] for x in rows))
for i,case in enumerate(CASES):
    for e in ENGINES:
        a=json.loads((RAW/f'diagnostic-{i}-{e}-0.json').read_text());b=json.loads((RAW/f'diagnostic-{i}-{e}-1.json').read_text())
        assert a['returncode']==b['returncode']==0 and a['stdout']==b['stdout']
        assert json.loads((RAW/f'size-{i}-{e}.json').read_text())['returncode']==0
for cpu in [0,8]:
 for rep in range(5):
  for e in ENGINES:assert json.loads((RAW/f'cancel-{cpu}-{rep}-{e}.json').read_text())['returncode']==0
(RAW/'contrasts.json').write_text(json.dumps(contrasts,indent=2)+'\n')
audit=dict(primary_processes=5120,sizing_processes=512,diagnostic_processes=1024,cancellation_processes=40,scenarios=128,placement_comparisons=256,summary=summary,interpretation='Descriptive five-block pilot ranges, not confidence intervals or architectural selection.')
(RAW/'audit.json').write_text(json.dumps(audit,indent=2)+'\n');print(json.dumps(audit))
