"""Reconstruct lifecycle/attribution and prospectively specified exploratory contrasts."""
import collections,hashlib,json,statistics
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];B=ROOT/'docs/experiments/results/s02-read-cost'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f=read(B/'freeze.json')
for p,h in f['sources'].items():assert any(q.exists() and sha(q)==h for q in [ROOT/p,B/'source-snapshot'/p]),p
for b in f['binaries'].values():assert sha(Path(b['path']))==b['sha256']
assert sha(B/'order.json')==f['order_hash'] and sha(B/'scenarios.json')==f['scenarios_hash']
order=read(B/'order.json');scenarios=read(B/'scenarios.json');rows=[json.loads(l) for l in (B/'results.jsonl').read_text().splitlines()];assert len(rows)==len(order)==2730
assert len(list((B/'runs').glob('*.json')))==2730
clocks=read(B/'clocks.json');assert len(clocks)==5
for i,c in enumerate(clocks):
 r=read(B/f'clock-{i}.json');assert r['exit_code']==0 and json.loads(r['stdout'])==c
floor=max(c['median_ns'] for c in clocks)
def phases(v):
 p=[v['source_build'],v['preparation']]
 for s in v['samples']:p += [s[k] for k in ['input_build','setup','execute_observe','engine_drop','answer_drop','answer_hold'] if s[k] is not None]
 return p+[v['prepared_drop'],v['consumer_drop']]
def normal(v,semantic=False):
 base=0 if semantic else v['source_build']['memory']['live_start']
 def walk(x):
  if isinstance(x,list):return [walk(y) for y in x]
  if isinstance(x,dict):return {k:(y-base if k in ['live_start','live_end','peak_live'] else walk(y)) for k,y in x.items() if k not in ['ns','first_answer_ns','attribution'] and not (semantic and k in ['memory','meter'])}
  return x
 return walk(v)
groups=collections.defaultdict(list);counts=collections.Counter()
for i,(r,item) in enumerate(zip(rows,order)):
 assert r['index']==i and all(r[k]==v for k,v in item.items());v=r['result'];raw=read(B/'runs'/f'{i}.json');assert raw['exit_code']==0 and not raw['stderr']
 family,depth,q,cancel=scenarios[r['scenario']];args=[r['mode'],family,depth,q,1,0]+([] if cancel is None else [cancel]);assert raw['args']==args
 assert [json.loads(l) for l in raw['stdout'].splitlines() if l.startswith('{')][-1]==v
 assert v['mode']==r['mode'] and v['family']==family and v['retained'] and v['resource'] and not v['counters'] and v['meter']==(r['build']!='ordinary')
 assert len(v['samples'])==q and [s['complete'] for s in v['samples']]==([True]*q if cancel is None else [False,True,False,True])
 p=phases(v);r['total_ns']=sum(x['ns'] for x in p);r['clock_intervals']=len(p)+q
 if r['build']!='ordinary':
  mem=[x['memory'] for x in p];base=mem[0]['live_start'];assert mem[-1]['live_end']==base
  assert all(a['live_end']==b['live_start'] for a,b in zip(mem,mem[1:]))
  r['traffic']=sum(x['requested_bytes'] for x in mem);r['peak']=max(x['peak_live'] for x in mem)-base
 else:assert all(x['memory'] is None for x in p)
 counts[r['build']+('-warmup' if r['warmup'] else '')]+=1;groups[r['scenario'],r['mode'],r['build']].append(r)
summary=[]
for (case,mode,kind),rs in groups.items():
 if kind!='meter':continue
 assert len(rs)==2 and normal(rs[0]['result'])==normal(rs[1]['result'])
 ordinary=groups[case,mode,'ordinary'];assert len(ordinary)==(6 if case<28 else 1)
 for r in ordinary:assert normal(r['result'],True)==normal(rs[0]['result'],True)
 ps=groups.get((case,mode,'profile'),[])
 if ps:
  assert len(ps)==2 and ps[0]['result']['attribution']==ps[1]['result']['attribution']
  for p in ps:assert normal(p['result'])==normal(rs[0]['result'])
 times=[r['total_ns'] for r in ordinary if not r['warmup']]
 threshold=100*floor*ordinary[0]['clock_intervals']
 summary.append({'scenario':case,'mode':mode,'traffic':rs[0]['traffic'],'peak':rs[0]['peak'],'median_ns':statistics.median(times),'min_ns':min(times),'max_ns':max(times),'threshold_ns':threshold,'qualified':min(times)>threshold,'attribution':ps[0]['result']['attribution'] if ps else None})
assert counts=={'meter':640,'profile':490,'ordinary-warmup':256,'ordinary':1344}
lookup={(r['scenario'],r['mode']):r for r in summary};comparisons=[]
for case in range(28):
 for candidate in ['validated','persistent-validated']:
  controls=['relevant' if candidate=='validated' else 'persistent-relevant','contextual','shared' if candidate=='validated' else 'persistent-shared','scan','indexed']+(['lowered'] if case>=24 else [])
  for control in controls:
   a=lookup[case,candidate];b=lookup[case,control]
   aa={r['rep']:r['total_ns'] for r in groups[case,candidate,'ordinary'] if not r['warmup']};bb={r['rep']:r['total_ns'] for r in groups[case,control,'ordinary'] if not r['warmup']};assert set(aa)==set(bb)==set(range(5))
   ratios=[aa[i]/bb[i] for i in range(5)]
   status='insufficient-signal' if not(a['qualified'] and b['qualified']) else 'gain' if max(ratios)<.9 else 'loss' if min(ratios)>1.1 else 'unresolved'
   comparisons.append({'scenario':case,'candidate':candidate,'control':control,'status':status,'min_ratio':min(ratios),'median_ratio':statistics.median(ratios),'max_ratio':max(ratios)})
archives=[]
for name in ['s02-read-cost-initial','s02-read-cost-second']:
 archive=B.parent/name;af=read(archive/'freeze.json')
 for path,h in af['sources'].items():
  options=[ROOT/path,B/'source-snapshot'/path]+[B.parent/n/'source-snapshot'/Path(path).name for n in ['s02-read-cost-initial','s02-read-cost-second']]
  assert any(p.exists() and sha(p)==h for p in options),(name,path)
 for binary in af['binaries'].values():assert sha(Path(binary['path']))==binary['sha256']
 assert sha(archive/'order.json')==af['order_hash']==f['order_hash']
 assert sha(archive/'scenarios.json')==af['scenarios_hash']==f['scenarios_hash']
 old=[json.loads(l) for l in (archive/'results.jsonl').read_text().splitlines()];assert len(old)==len(rows)
 for previous,current in zip(old,rows):
  assert all(previous[k]==current[k] for k in ['index','scenario','mode','build','rep','warmup'])
  raw=read(archive/'runs'/f"{previous['index']}.json");assert raw['exit_code']==0 and not raw['stderr']
  assert [json.loads(l) for l in raw['stdout'].splitlines() if l.startswith('{')][-1]==previous['result']
  assert normal(previous['result'],previous['build']=='ordinary')==normal(current['result'],current['build']=='ordinary')
  assert previous['result']['attribution']==current['result']['attribution']
 old_comp=read(archive/'review-audit.json')['comparisons']
 archives.append({'directory':name,'workload_processes':len(old),'normalized_allocations_endpoints_and_profiles_equal':True,'dispositions':dict(collections.Counter(r['status'] for r in old_comp))})
print(json.dumps({'preserved_runs':archives,'processes':dict(counts),'configurations':320,'exact_allocation_pairs':320,'exact_profile_pairs':245,'profile_meter_ownership_matches':245,'clock_floor_ns':floor,'qualified_complete_configurations':sum(r['qualified'] for r in summary if r['scenario']<28),'dispositions':dict(collections.Counter(r['status'] for r in comparisons)),'summary':summary,'comparisons':comparisons},indent=2))
