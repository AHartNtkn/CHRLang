"""Audit every pinned timing receipt and apply predeclared comparisons."""
import collections,hashlib,json,statistics
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];B=ROOT/'docs/experiments/results/s06-formula-timing';Q=ROOT/'docs/experiments/results/s06-formula-clock'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f=read(B/'freeze.json')
for p,h in f['sources'].items():assert sha(ROOT/p)==h,p
assert sha(ROOT/f['binary']['path'])==f['binary']['sha256']
for name in ['order','scenarios']:assert sha(B/f'{name}.json')==f[f'{name}_sha256']
order=read(B/'order.json');scenarios=read(B/'scenarios.json');records=[json.loads(x) for x in (B/'results.jsonl').read_text().splitlines()]
assert len(records)==len(order)==5100 and len(list((B/'runs').glob('*.json')))==5100
expected={}
for i,args in enumerate(read(Q/'cases.json')):expected[tuple(args)]=json.loads(read(Q/f'runs/{i}-ordinary.json')['stdout'].splitlines()[0])
blocks={};positions=collections.defaultdict(collections.Counter);cells=collections.defaultdict(list);warmups=0
for i,(r,item) in enumerate(zip(records,order)):
 assert r['index']==i and all(r[k]==v for k,v in item.items())
 raw=read(B/f'runs/{i}.json');args=[item['mode'],*scenarios[item['scenario']]]
 assert raw['args']==args and raw['exit_code']==0 and not raw['stderr']
 lines=[json.loads(x) for x in raw['stdout'].splitlines()];assert lines==[expected[tuple(args)],{'session_ns':r['session_ns']}] and r['endpoint']==lines[0]
 if r['warmup']:warmups+=1;continue
 key=(r['cpu'],r['scenario'],r['block']);group=blocks.setdefault(key,{})
 assert r['mode'] not in group;positions[r['cpu'],r['scenario'],r['mode']][len(group)]+=1;group[r['mode']]=r
 cells[r['cpu'],r['scenario'],r['mode']].append(r)
assert warmups==204 and len(blocks)==2*17*24 and all(len(b)==6 for b in blocks.values())
assert all(set(c)==set(range(6)) and set(c.values())=={4} for c in positions.values())
pairs=[('reduced',m) for m in ['diagram','names','projected','symbolic','explicit']]+[('diagram',m) for m in ['names','projected','symbolic','explicit']]
comparisons=[]
for case in range(17):
 for candidate,control in pairs:
  cpus=[];signal=True
  for cpu in [0,2]:
   ratios=[blocks[cpu,case,b][candidate]['session_ns']/blocks[cpu,case,b][control]['session_ns'] for b in range(24)]
   values={m:[r['session_ns'] for r in cells[cpu,case,m]] for m in [candidate,control]}
   signal &= all(min(ns)>=4200 for ns in values.values())
   cpus.append({'cpu':cpu,'min_ratio':min(ratios),'median_ratio':statistics.median(ratios),'max_ratio':max(ratios),'candidate_median_ns':statistics.median(values[candidate]),'control_median_ns':statistics.median(values[control])})
  disposition='insufficient-clock-signal' if not signal else 'gain' if all(x['max_ratio']<0.9 for x in cpus) else 'loss' if all(x['min_ratio']>1.1 for x in cpus) else 'unresolved'
  comparisons.append({'scenario':case,'source':scenarios[case],'candidate':candidate,'control':control,'disposition':disposition,'cpus':cpus})
counts={a+'/'+b:dict(collections.Counter(r['disposition'] for r in comparisons if r['candidate']==a and r['control']==b)) for a,b in pairs}
excursions=[]
for (cpu,case,mode),rows in cells.items():
 median=statistics.median(r['session_ns'] for r in rows);r=max(rows,key=lambda r:r['session_ns'])
 excursions.append({'cpu':cpu,'scenario':case,'mode':mode,'index':r['index'],'session_ns':r['session_ns'],'median_ns':median,'max_over_median':r['session_ns']/median})
excursions.sort(key=lambda x:x['max_over_median'],reverse=True)
print(json.dumps({'primary_processes':4896,'warmups':warmups,'balanced_positions':True,'outliers_excluded':0,'counts':counts,'comparisons':comparisons,'largest_excursions':excursions[:8]},indent=2))
