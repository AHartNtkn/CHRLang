"""Reconstruct all registered phases, allocation repeats and exploratory timing contrasts."""
import collections,hashlib,json,statistics,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s06-union-lifecycle'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f=read(BASE/'freeze.json');assert sha(BASE/'sources.zip')==f['archive_sha256'];assert sha(BASE/'order.json')==f['order_sha256']
with zipfile.ZipFile(BASE/'sources.zip') as z:
 assert set(z.namelist())==set(f['sources'])
 for p,h in f['sources'].items():assert hashlib.sha256(z.read(p)).hexdigest()==h,p
for b in f['binaries'].values():assert sha(Path(b['path']))==b['sha256']
clocks=[]
for i in range(5):
 r=read(BASE/f'clock-{i}.json');assert r['exit_code']==0;clocks.append(json.loads(r['stdout'])['median_ns'])
clock=max(clocks);order=read(BASE/'order.json');rows=[json.loads(l) for l in (BASE/'results.jsonl').read_text().splitlines()];assert len(rows)==len(order)==2112;assert len(list((BASE/'runs').glob('*.json')))==2112
cells=collections.defaultdict(list)
for i,(r,j)in enumerate(zip(rows,order)):
 assert r['index']==i and all(r[k]==v for k,v in j.items());raw=read(BASE/'runs'/f'{i}.json');assert raw['exit_code']==0 and not raw['timeout'] and not raw['stderr'];assert [json.loads(l) for l in raw['stdout'].splitlines()]==r['rows']
 mode,family,n,q,endpoint,retention=j['args'];meta,*phases=r['rows'];assert meta==dict(mode=mode,family=family,width=n,queries=q,full=endpoint=='full',retention=retention,meter=j['build']=='meter');assert [p['phase'] for p in phases]==['source','prepare','source-drop']+['request','execute-observe','request-drop','consumer']*q+['prepared-drop','consumer-drop']
 entry=dict(repeat=j['repeat']);
 if meta['meter']:
  ms=[p['memory'] for p in phases];base=ms[0]['live_start'];assert ms[-1]['live_end']==base;assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]));assert all(p['ns'] is None for p in phases)
  entry.update(traffic=sum(m['requested_bytes'] for m in ms),peak=max(m['peak_live'] for m in ms)-base,normalized=[{k:v-base if k in ['live_start','live_end','peak_live'] else v for k,v in m.items()} for m in ms],phase_traffic={key:sum(p['memory']['requested_bytes'] for p in phases if p['phase']==key) for key in set(p['phase'] for p in phases)})
 else:
  assert all(p['memory'] is None for p in phases);entry.update(ns=sum(p['ns'] for p in phases),floor=100*clock*len(phases))
 cells[j['build'],*j['args']].append(entry)
summary=[]
for k,rs in cells.items():
 if k[0]!='meter':continue
 assert len(rs)==2 and rs[0]['normalized']==rs[1]['normalized'];_,mode,family,n,q,endpoint,retention=k
 result=dict(mode=mode,family=family,width=n,queries=q,endpoint=endpoint,retention=retention,traffic=rs[0]['traffic'],peak=rs[0]['peak'],phase_traffic=rs[0]['phase_traffic'])
 if retention=='all':
  ts=sorted(cells['time',*k[1:]],key=lambda r:r['repeat']);assert len(ts)==5;ns=[r['ns'] for r in ts];result.update(ns=ns,median_ns=statistics.median(ns),insufficient_signal=any(r['ns']<r['floor'] for r in ts))
 summary.append(result)
assert len(summary)==576
contrasts=[]
lookup={(r['mode'],r['family'],r['width'],r['queries'],r['endpoint'],r['retention']):r for r in summary}
for r in summary:
 if r['mode']!='union' or r['retention']!='all':continue
 for other in ['separate','reduced','names','explicit','dedup']:
  c=lookup[other,r['family'],r['width'],r['queries'],r['endpoint'],'all'];ratios=[a/b for a,b in zip(r['ns'],c['ns'])];contrasts.append(dict(family=r['family'],width=r['width'],queries=r['queries'],endpoint=r['endpoint'],control=other,traffic_ratio=r['traffic']/c['traffic'],peak_ratio=r['peak']/c['peak'],median_time_ratio=r['median_ns']/c['median_ns'],repeat_ratio_range=[min(ratios),max(ratios)],insufficient_signal=r['insufficient_signal'] or c['insufficient_signal']))
print(json.dumps(dict(matrix_processes=2112,meter_processes=1152,timing_processes=960,allocation_cells=576,timing_cells=192,exact_allocation_repeats=True,clock_median_max_ns=clock,exploratory=True,summary=summary,contrasts=contrasts),indent=2))
