"""Independent coverage, ownership, freeze and phase-summed pilot audit."""
import collections,hashlib,itertools,json,random,statistics
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s02-multihead-lifecycle'
MODES=['local-scan','tuples','partial','scan','indexed','special-scan','special-indexed']
FAMILIES=['sparse','broad','nested','cold','dense','three']
CELLS=list(itertools.product(MODES,FAMILIES,[4,16,64],[1,4]))
def main():
 freeze=json.loads((OUT/'freeze.json').read_text())
 for path,h in freeze['sources'].items():assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==h,path
 for name,h in freeze['binaries'].items():assert hashlib.sha256((ROOT/'target/s02-multihead-lifecycle'/name).read_bytes()).hexdigest()==h,name
 groups={};pairs=0
 for kind,reps,seed in [('meter',2,7280),('time',5,7281)]:
  expected=list(itertools.product(range(reps),CELLS));random.Random(seed).shuffle(expected)
  assert json.loads((OUT/(kind+'-order.json')).read_text())==json.loads(json.dumps(expected))
  assert len(list((OUT/kind).glob('*.json')))==len(expected)
  by=collections.defaultdict(list)
  for i,(rep,key) in enumerate(expected):
   r=json.loads((OUT/kind/f'{i:04}.json').read_text());assert r['code']==0 and not r['stderr']
   assert r['command'][1:]==list(map(str,key))
   x=json.loads(r['stdout']);assert tuple(x[k] for k in ['mode','family','width','reuse'])==key
   phases=x['phases'];order=['source-build','prepare']+['input','setup','execute','observe','engine-drop','answer-drop','input-drop']*key[3]+['cancel-input','cancel-setup','cancel-advance','cancel-engine-drop','cancel-input-drop']*2+['prepared-drop','source-drop']
   assert [p['phase'] for p in phases]==order
   assert all(isinstance(p['measurement']['ns'],int) and p['measurement']['ns']>=0 for p in phases)
   if kind=='meter':
    mem=[p['measurement']['memory'] for p in phases]
    assert mem[0]['live_start']==mem[-1]['live_end']
    assert all(a['live_end']==b['live_start'] for a,b in zip(mem,mem[1:]))
    assert all(m['peak_live']>=max(m['live_start'],m['live_end']) for m in mem)
    prepared=mem[1]['live_end']
    for p in phases:
     if p['phase'] in ['input-drop','cancel-input-drop']:assert p['measurement']['memory']['live_end']==prepared
   else:assert all('memory' not in p['measurement'] for p in phases)
   by[key].append(x)
  groups[kind]=by
 rows=[]
 for key in CELLS:
  meter=groups['meter'][key];timing=groups['time'][key]
  assert len(meter)==2 and len(timing)==5
  assert [p['measurement']['memory'] for p in meter[0]['phases']]==[p['measurement']['memory'] for p in meter[1]['phases']]
  assert len({x['state_bytes'] for x in meter+timing})==1
  pairs+=1
  totals=[];parts=collections.defaultdict(list)
  for x in timing:
   phases=[p for p in x['phases'] if not p['phase'].startswith('cancel-')]
   totals.append(sum(p['measurement']['ns'] for p in phases))
   per=collections.Counter()
   for p in phases:per[p['phase']]+=p['measurement']['ns']
   for name,value in per.items():parts[name].append(value)
  memory=[p['measurement']['memory'] for p in meter[0]['phases'] if not p['phase'].startswith('cancel-')]
  rows.append(dict(zip(['mode','family','width','reuse'],key))|dict(min_ns=min(totals),median_ns=statistics.median(totals),max_ns=max(totals),requested_bytes=sum(m['requested_bytes'] for m in memory),peak_growth=max(m['peak_live'] for m in memory)-memory[0]['live_start'],phase_medians_ns={k:statistics.median(v) for k,v in parts.items()}))
 indexed={(r['mode'],r['family'],r['width'],r['reuse']):r for r in rows}
 comparisons=[]
 for row in rows:
  if row['mode']=='scan':continue
  b=indexed[('scan',row['family'],row['width'],row['reuse'])]
  classification='gain' if row['max_ns']<.9*b['min_ns'] else 'loss' if row['min_ns']>1.1*b['max_ns'] else 'unresolved'
  comparisons.append({k:row[k] for k in ['mode','family','width','reuse']}|dict(classification=classification,median_ratio=row['median_ns']/b['median_ns']))
 result=dict(cells=252,exact_allocation_pairs=pairs,timing_processes=1260,owners='pass',frozen_sources='pass',rows=rows,scan_comparisons=comparisons)
 (OUT/'audit.json').write_text(json.dumps(result,indent=2)+'\n')
 print('252 cells,252 exact allocation pairs,1260 ordinary timings, owners and frozen sources pass.')
 for mode in MODES[0:3]+MODES[4:]:print(mode,dict(collections.Counter(r['classification'] for r in comparisons if r['mode']==mode)))
 for family in FAMILIES:print(family,[(r['mode'],round(r['median_ns']/1e6,3),r['requested_bytes']) for r in rows if r['family']==family and r['width']==64 and r['reuse']==4])
if __name__=='__main__':main()
