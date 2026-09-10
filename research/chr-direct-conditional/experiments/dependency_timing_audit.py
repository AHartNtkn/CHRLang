"""Audit every pinned block and apply the prospectively fixed separation rule."""
import collections,hashlib,json,statistics
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s03-dependency-timing'
def read(path):return json.loads(path.read_text())
freeze=read(BASE/'freeze.json')
for path,sha in freeze['sources'].items():assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==sha,path
assert hashlib.sha256((ROOT/freeze['binary']['path']).read_bytes()).hexdigest()==freeze['binary']['sha256']
for name in ['order','scenarios']:assert hashlib.sha256((BASE/f'{name}.json').read_bytes()).hexdigest()==freeze[f'{name}_sha256']
order=read(BASE/'order.json');scenarios=read(BASE/'scenarios.json');records=[json.loads(x) for x in (BASE/'results.jsonl').read_text().splitlines()]
assert len(records)==len(order)==5600
assert len(list((BASE/'runs').glob('*.log')))==5600
old=read(ROOT/'docs/experiments/results/s03-dependency-ownership/results.json')
expected={(r['family'],r['size'],r['reverse'],r['retention'],r['mode']):r['result']['endpoints'] for r in old if not r['cancel']}
positions=collections.defaultdict(collections.Counter);blocks={};warmups=0
for i,(r,item) in enumerate(zip(records,order)):
 assert r['index']==i and all(r[k]==v for k,v in item.items())
 raw=[json.loads(x) for x in (BASE/'runs'/f'{i}.log').read_text().splitlines() if x.startswith('{')]
 actual=next(x for x in raw if x.get('event')=='result');assert actual==r['result'] and not actual['meter']
 case=scenarios[r['scenario']]
 endpoints=[{k:v for k,v in x.items() if k!='first_ns'} for x in actual['endpoints']]
 assert endpoints==expected[case['family'],case['size'],case['reverse'],case['retention'],r['mode']]
 assert len(actual['phases'])==24 and all(x['complete'] for x in actual['endpoints'])
 assert sum(p['reading']['ns'] for p in actual['phases'])==r['session_ns']
 assert [x['first_ns'] for x in actual['endpoints']]==r['first_ns']
 for query,ns in enumerate(r['first_ns']):
  assert 0<=ns<=next(p['reading']['ns'] for p in actual['phases'] if p['query']==query and p['phase']=='execute_observe')
 cold=r['first_ns'][0]+sum(p['reading']['ns'] for p in actual['phases'] if p['phase'] in ['source','prepare'] or p['phase'] in ['input','setup'] and p['query']==0)
 assert cold==r['cold_first_ns']
 if item['warmup']:warmups+=1;continue
 key=(r['cpu'],r['scenario'],r['block']);group=blocks.setdefault(key,{})
 assert r['mode'] not in group
 positions[r['cpu'],r['scenario'],r['mode']][len(group)]+=1
 group[r['mode']]=r
assert warmups==224 and len(blocks)==2*14*24 and all(len(b)==8 for b in blocks.values())
assert all(set(p)==set(range(8)) and set(p.values())=={3} for p in positions.values())
modes=['current','current-miss','birth','birth-miss','dependencies','dependencies-miss','scan','indexed']
pairs=[(m+'-miss',m) for m in ['current','birth','dependencies']]+[(m,c) for m in modes[:6] for c in ['scan','indexed']]
comparisons=[]
for case in range(14):
 for candidate,control in pairs:
  cpus=[]
  for cpu in [0,2]:
   ratios=[blocks[cpu,case,block][candidate]['session_ns']/blocks[cpu,case,block][control]['session_ns'] for block in range(24)]
   cpus.append({'cpu':cpu,'median_ratio':statistics.median(ratios),'min_ratio':min(ratios),'max_ratio':max(ratios)})
  disposition='gain' if all(c['max_ratio']<0.9 for c in cpus) else 'loss' if all(c['min_ratio']>1.1 for c in cpus) else 'unresolved'
  comparisons.append({'scenario':case,**scenarios[case],'candidate':candidate,'control':control,'disposition':disposition,'cpus':cpus})
counts={}
for candidate,control in pairs:
 counts[candidate+'/'+control]=dict(collections.Counter(c['disposition'] for c in comparisons if c['candidate']==candidate and c['control']==control))
# Reproduce the exploratory phase attribution without excluding any run.
cells=collections.defaultdict(list)
for r in records:
 if not r['warmup']:cells[r['cpu'],r['scenario'],r['mode']].append(r)
excursions=[]
for (cpu,case,mode),rows in cells.items():
 r=max(rows,key=lambda x:x['session_ns'])
 phase=max(r['result']['phases'],key=lambda p:p['reading']['ns'])
 phase_key=(phase['phase'],phase['query'])
 matching=[p['reading']['ns'] for row in rows for p in row['result']['phases'] if (p['phase'],p['query'])==phase_key]
 excursions.append({'cpu':cpu,'scenario':case,'mode':mode,'index':r['index'],'session_ns':r['session_ns'],'max_over_median':r['session_ns']/statistics.median(row['session_ns'] for row in rows),'largest_phase':list(phase_key),'phase_ns':phase['reading']['ns'],'median_matching_phase_ns':statistics.median(matching)})
excursions.sort(key=lambda x:x['max_over_median'],reverse=True)
assert excursions[:8]==read(BASE/'excursion-analysis.json')
print(json.dumps({'primary_processes':5376,'warmups':224,'independent_endpoints':True,'balanced_positions':True,'outliers_excluded':0,'counts':counts,'comparisons':comparisons},indent=2))
