"""Audit discovery/collector factorial costs without conflating result contracts."""
import collections,hashlib,itertools,json,statistics,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s06-unique-output'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f=read(BASE/'freeze.json');assert sha(BASE/'sources.zip')==f['archive_sha256'];assert sha(BASE/'order.json')==f['order_sha256']
with zipfile.ZipFile(BASE/'sources.zip')as z:
 assert set(z.namelist())==set(f['sources'])
 for p,h in f['sources'].items():assert hashlib.sha256(z.read(p)).hexdigest()==h
for b in f['binaries'].values():assert sha(Path(b['path']))==b['sha256']
clock=max(json.loads(read(BASE/f'clock-{i}.json')['stdout'])['median_ns']for i in range(5))
rows=[json.loads(l)for l in(BASE/'results.jsonl').read_text().splitlines()];order=read(BASE/'order.json');assert len(rows)==len(order)==1408;assert len(list((BASE/'runs').glob('*.json')))==1408;cells=collections.defaultdict(list)
for i,(r,j)in enumerate(zip(rows,order)):
 assert r['index']==i and all(r[k]==v for k,v in j.items());raw=read(BASE/'runs'/f'{i}.json');assert raw['exit_code']==0 and not raw['timeout'] and not raw['stderr'];assert [json.loads(l)for l in raw['stdout'].splitlines()]==r['rows'];assert raw['command']==[f['binaries'][j['build']]['path'],*map(str,j['args'])]
 mode,family,n,q,endpoint,retention,collector=j['args'];meta,*ps=r['rows'];assert meta==dict(mode=mode,family=family,width=n,queries=q,full=endpoint=='full',retention=retention,collector=collector,meter=j['build']=='meter')
 assert [p['phase']for p in ps]==['source','prepare','source-drop']+['request','execute-observe','request-drop','consumer']*q+['prepared-drop','consumer-drop'];v=dict(repeat=j['repeat'])
 if meta['meter']:
  ms=[p['memory']for p in ps];base=ms[0]['live_start'];assert ms[-1]['live_end']==base;assert all(a['live_end']==b['live_start']for a,b in zip(ms,ms[1:]));assert all(p['ns']is None for p in ps)
  v.update(normalized=[{k:x-base if k in ['live_start','live_end','peak_live']else x for k,x in m.items()}for m in ms],traffic=sum(m['requested_bytes']for m in ms),peak=max(m['peak_live']for m in ms)-base,retained=ps[-2]['memory']['live_end']-base,phase_traffic={label:sum(p['memory']['requested_bytes']for p in ps if p['phase']==label)for label in set(p['phase']for p in ps)})
 else:
  assert all(p['memory']is None for p in ps);v.update(ns=sum(p['ns']for p in ps),floor=100*clock*len(ps))
 cells[j['build'],*j['args']].append(v)
expected=set(itertools.product(['union','unique','dedup','reduced'],['overlap','disjoint','redundant','single'],[8],[1,64],['member','full'],['immediate','window','all'],['set','ordered']))
assert {k[1:]for k in cells if k[0]=='meter'}==expected
summary=[]
for args in sorted(expected):
 rs=cells['meter',*args];assert len(rs)==2 and rs[0]['normalized']==rs[1]['normalized'];mode,family,n,q,endpoint,retention,collector=args
 if endpoint=='member':assert rs[0]['normalized']==cells['meter',mode,family,n,q,endpoint,retention,'set'][0]['normalized']
 v=dict(zip(['mode','family','width','queries','endpoint','retention','collector'],args));v.update({k:rs[0][k]for k in ['traffic','peak','retained','phase_traffic']})
 if retention=='all':
  ts=sorted(cells['time',*args],key=lambda x:x['repeat']);assert len(ts)==5;v.update(ns=[r['ns']for r in ts],median_ns=statistics.median(r['ns']for r in ts),insufficient_signal=any(r['ns']<r['floor']for r in ts))
 summary.append(v)
lookup={(r['mode'],r['family'],r['queries'],r['endpoint'],r['retention'],r['collector']):r for r in summary}
for r in summary:
 if r['mode']!='union':continue
 u=lookup['unique',r['family'],r['queries'],r['endpoint'],r['retention'],r['collector']]
 assert r['phase_traffic']['execute-observe']==u['phase_traffic']['execute-observe']
 assert r['retained']==u['retained']
print(json.dumps(dict(matrix_processes=1408,meter_processes=768,timing_processes=640,allocation_cells=384,timing_cells=128,exact_allocation_repeats=True,member_collector_allocations_identical=True,union_unique_execution_traffic_and_retained_output_identical=True,clock_median_max_ns=clock,exploratory=True,summary=summary),indent=2))
