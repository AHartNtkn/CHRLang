"""Compare deterministic allocation diagnostics, never the recorded clocks."""
from pathlib import Path
import hashlib
import json

root=Path(__file__).resolve().parents[3]
raw=root/'docs/experiments/results/s05-finite-reuse-ownership'
audit=json.loads((raw/'audit.json').read_text())
assert hashlib.sha256((raw/'runs.jsonl').read_bytes()).hexdigest()==audit['runs_sha256']
rows={tuple(r['config']):r for r in audit['results']}
comparisons=[]
for config,row in rows.items():
    if config[0]!='reuse' or config[5]!='none':continue
    for control in ['recompute','eager','covered']:
        other=rows[(control,*config[1:])]
        comparisons.append({'config':config,'control':control,'requested':row['requested'],'control_requested':other['requested'],'ratio':row['requested']/other['requested'],'peak':row['peak'],'control_peak':other['peak'],'retention_drop':row['retention_drop'],'control_retention_drop':other['retention_drop']})
summary={mode:{name:sum((r['requested']<r['control_requested'] if name=='lower' else r['requested']>r['control_requested'] if name=='higher' else r['requested']==r['control_requested']) for r in comparisons if r['control']==mode) for name in ['lower','equal','higher']} for mode in ['recompute','eager','covered']}
(raw/'analysis.json').write_text(json.dumps({'metric':'requested heap bytes; not time or RSS','counts':summary,'comparisons':comparisons},indent=2)+'\n')
print(json.dumps(summary,indent=2))
for key in [('reuse',484,1,4,16,'none',64),('reuse',511,2,4,16,'none',64),('reuse',0,1,4,16,'none',64),('reuse',484,1,1,16,'none',64),('reuse',484,1,4,1,'none',0)]:
    print('scenario',key[1:])
    for mode in ['recompute','eager','covered','reuse']:
        r=rows[(mode,*key[1:])]
        print(mode,{k:r[k] for k in ['answers','regions','results','requested','peak','retention_drop']})
attribution={}
for mode in ['recompute','covered','reuse']:
    r=rows[(mode,0,1,4,16,'none',64)]
    attribution[mode]=[p for p in r['phases'] if p['name'] in ['query_setup','finite_service'] and p['query'] in [0,1,2,3,4]]
(raw/'failure-attribution.json').write_text(json.dumps(attribution,indent=2)+'\n')
