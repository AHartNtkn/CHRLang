"""Reconstruct CPU attribution and the registered wall-only policy contrasts."""
import collections,hashlib,json,statistics
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];B=ROOT/'docs/experiments/results/s04-adaptive-cpu'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f=read(B/'freeze.json')
for p,h in f['sources'].items():assert sha(ROOT/p)==h,p
for b in f['binaries'].values():assert sha(ROOT/b['path'])==b['sha256']
for n in ['order','scenarios']:assert sha(B/f'{n}.json')==f[f'{n}_sha256']
order=read(B/'order.json');scenarios=read(B/'scenarios.json');records=[json.loads(x) for x in (B/'results.jsonl').read_text().splitlines()]
assert len(records)==len(order)==3264 and len(list((B/'runs').glob('*.json')))==3264
clocks=read(B/'clocks.json')
assert len(clocks)==5
for i,c in enumerate(clocks):
 assert c==json.loads((B/f'clock-{i}.log').read_text())
 assert c['sleep_wall']>=15_000_000 and c['sleep_cpu']*4<c['sleep_wall'] and 0<c['busy_cpu']*10<=c['busy_wall']*11
cpu_floor=max(c['cpu_median'] for c in clocks);wall_floor=max(c['wall_median'] for c in clocks)
blocks={};cells=collections.defaultdict(list);positions=collections.defaultdict(collections.Counter);endpoints={};warmups=0
for i,(r,item) in enumerate(zip(records,order)):
 assert r['index']==i and all(r[k]==v for k,v in item.items())
 raw=read(B/f'runs/{i}.json');args=[r['mode'],*scenarios[r['scenario']]];assert raw['args']==args and raw['exit_code']==0 and not raw['stderr']
 data=[json.loads(x) for x in raw['stdout'].splitlines()];meta=data[0];assert not meta['metered']
 counts=next(x['service_counts'] for x in data if 'service_counts' in x)
 expected_count=1 if args[-1] else 16
 assert meta['counts']==[expected_count]*args[3] and meta['retained']==expected_count*args[3]
 endpoint=[meta['counts'],meta['retained'],meta['engine_bytes'],counts];assert r['endpoint']==endpoint
 assert endpoints.setdefault((r['scenario'],r['mode']),endpoint)==endpoint
 phases=[x for x in data if 'phase' in x];cpus={(x['cpu_phase'],x['query']):x['cpu_ns'] for x in data if 'cpu_phase' in x}
 assert sum(x['ns'] for x in phases)==r['total_ns']==meta['total_ns']
 service=[x for x in phases if x['phase']=='first' or (x['phase']=='remaining' and not args[-1])]
 assert r['service_intervals']==len(service) and r['service_wall_ns']==sum(x['ns'] for x in service)
 if r['build']=='cpu':
  assert len(cpus)==len(phases) and all(x>=0 for x in cpus.values())
  assert r['service_cpu_ns']==sum(cpus[x['phase'],x['query']] for x in service)
 else:assert not cpus and r['service_cpu_ns'] is None
 if r['warmup']:warmups+=1;continue
 key=(r['cpu'],r['scenario'],r['block'],r['build']);group=blocks.setdefault(key,{})
 assert r['mode'] not in group;positions[r['cpu'],r['scenario'],r['build'],r['mode']][len(group)]+=1;group[r['mode']]=r
 cells[r['cpu'],r['scenario'],r['mode'],r['build']].append(r)
assert warmups==192 and len(blocks)==2*6*16*2 and all(len(b)==8 for b in blocks.values())
assert all(set(c)==set(range(8)) and set(c.values())=={2} for c in positions.values())
attribution=[];excursions=[]
for (cpu,case,mode,kind),rows in cells.items():
 if kind!='cpu':continue
 walls=[r['service_wall_ns'] for r in rows];cpus=[r['service_cpu_ns'] for r in rows];mw=statistics.median(walls);mc=statistics.median(cpus)
 intervals=rows[0]['service_intervals'];qualified=min(walls)>=100*intervals*wall_floor and min(cpus)>=100*intervals*cpu_floor
 material=[r for r in rows if r['service_wall_ns']-r['service_cpu_ns']>max(0.05*r['service_wall_ns'],10*intervals*cpu_floor)]
 wall_only=[r for r in rows if r['service_wall_ns']>1.5*mw and r['service_cpu_ns']<=1.1*mc]
 plain=[r['service_wall_ns'] for r in cells[cpu,case,mode,'wall']]
 attribution.append({'cpu':cpu,'scenario':case,'mode':mode,'qualified':qualified,'service_intervals':intervals,'wall_median_ns':mw,'cpu_median_ns':mc,'wall_max_min':max(walls)/min(walls),'cpu_max_min':max(cpus)/min(cpus),'material_elapsed_excess':len(material),'elapsed_only_excursions':len(wall_only),'instrumented_over_plain_median':mw/statistics.median(plain)})
 for r in rows:excursions.append({'index':r['index'],'cpu':cpu,'scenario':case,'mode':mode,'wall_ns':r['service_wall_ns'],'cpu_ns':r['service_cpu_ns'],'wall_over_median':r['service_wall_ns']/mw,'cpu_over_median':r['service_cpu_ns']/mc,'excess_ns':r['service_wall_ns']-r['service_cpu_ns']})
pairs=[(a,b) for a in ['backoff1','backoff8'] for b in ['copy','reunion','eager','fixed1','fixed8']]+[('scheduled','eager')]
comparisons=[]
for case in range(6):
 for candidate,control in pairs:
  cpu_results=[]
  for cpu in [0,2]:
   ratios=[blocks[cpu,case,block,'wall'][candidate]['total_ns']/blocks[cpu,case,block,'wall'][control]['total_ns'] for block in range(16)]
   cpu_results.append({'cpu':cpu,'min_ratio':min(ratios),'median_ratio':statistics.median(ratios),'max_ratio':max(ratios)})
  disposition='gain' if all(c['max_ratio']<0.9 for c in cpu_results) else 'loss' if all(c['min_ratio']>1.1 for c in cpu_results) else 'unresolved'
  comparisons.append({'scenario':case,'candidate':candidate,'control':control,'disposition':disposition,'cpus':cpu_results})
counts={a+'/'+b:dict(collections.Counter(r['disposition'] for r in comparisons if r['candidate']==a and r['control']==b)) for a,b in pairs}
excursions.sort(key=lambda r:r['wall_over_median'],reverse=True)
# Post-registration descriptive attribution: retain all phases of the largest excursions.
# This does not change the registered sample selection or policy decision rule.
for excursion in excursions[:12]:
 raw=read(B/f"runs/{excursion['index']}.json")
 data=[json.loads(x) for x in raw['stdout'].splitlines()]
 cpu={(r['cpu_phase'],r['query']):r['cpu_ns'] for r in data if 'cpu_phase' in r}
 phases=[dict(r,cpu_ns=cpu[r['phase'],r['query']],excess_ns=r['ns']-cpu[r['phase'],r['query']]) for r in data if 'phase' in r]
 excursion['phase_excesses']=sorted(phases,key=lambda r:r['excess_ns'],reverse=True)
print(json.dumps({'primary_diagnostic_processes':3072,'warmups':warmups,'clock_controls':5,'max_empty_cpu_median_ns':cpu_floor,'max_empty_wall_median_ns':wall_floor,'qualified_cpu_cells':sum(r['qualified'] for r in attribution),'instrumented_service_samples':1536,'material_elapsed_excess_samples':sum(r['material_elapsed_excess'] for r in attribution),'elapsed_only_excursions':sum(r['elapsed_only_excursions'] for r in attribution),'median_cpu_spread':statistics.median(r['cpu_max_min'] for r in attribution),'median_wall_spread':statistics.median(r['wall_max_min'] for r in attribution),'perturbation_median_ratio':statistics.median(r['instrumented_over_plain_median'] for r in attribution),'counts':counts,'cells':attribution,'comparisons':comparisons,'largest_excursions':excursions[:12]},indent=2))
