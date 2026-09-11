"""Independent reconstruction of clock qualification and ownership continuity."""
import hashlib,json,statistics
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];B=ROOT/'docs/experiments/results/s06-formula-clock'
def read(p):return json.loads(p.read_text())
f=read(B/'freeze.json')
for p,h in f['sources'].items():assert any(q.exists() and hashlib.sha256(q.read_bytes()).hexdigest()==h for q in [ROOT/p,B/'source-snapshot'/p]),p
for x in f['binaries'].values():assert hashlib.sha256((ROOT/x['path']).read_bytes()).hexdigest()==x['sha256']
def receipt(label):
 r=read(B/'runs'/f'{label}.json');assert r['exit_code']==0 and not r['stderr'];return r,[json.loads(s) for s in r['stdout'].splitlines()]
cases=read(B/'cases.json');sizing=read(B/'sizing.json');clocks=read(B/'clocks.json');assert len(cases)==len(sizing)==144
assert len(list((B/'runs').glob('*.json')))==1445
for i,c in enumerate(clocks):assert receipt(f'clock-{i}')[1]==[c] and c['min']<=c['median']<=c['p99']<=c['max']
threshold=200*max(c['median'] for c in clocks)
old=ROOT/'docs/experiments/results/s06-simplified-ownership';index={tuple(a):i for i,a in enumerate(read(old/'configurations.json'))}
def memory(rows):
 ps=[r for r in rows if 'phase' in r];base=ps[0]['memory']['live_start']
 for a,b in zip(ps,ps[1:]):assert a['memory']['live_end']==b['memory']['live_start']
 assert ps[-1]['memory']['live_end']==base
 return [(r['phase'],{k:v-base if k in ['live_start','live_end','peak_live'] else v for k,v in r['memory'].items()}) for r in ps]
for i,args in enumerate(cases):
 prior=[json.loads(x) for x in read(old/f'runs/{index[tuple(args)]}-0.json')['stdout'].splitlines()];expected=memory(prior)
 for mode in ['meter','phase']:
  for rep in range(2):
   raw,rows=receipt(f'{i}-{mode}-{rep}');assert raw['args']==args and rows[0]==prior[0] and memory(rows)==expected
   assert all((p['ns'] is None) if mode=='meter' else isinstance(p['ns'],int) for p in rows[1:])
 raw,ordinary=receipt(f'{i}-ordinary');assert raw['args']==args and ordinary[0]==dict(prior[0],meter=False)
 assert all(p['memory'] is None and p['ns'] is None for p in ordinary[1:])
 times=[]
 for rep in range(5):
  raw,rows=receipt(f'{i}-session-{rep}');assert raw['args']==args and len(rows)==2 and rows[0]==ordinary[0];times.append(rows[1]['session_ns'])
 assert sizing[i]=={'args':args,'session_ns':times,'qualified':min(times)>=threshold}
scenarios=sorted({tuple(r['args'][1:]) for r in sizing})
qualified=[list(s) for s in scenarios if all(r['qualified'] for r in sizing if tuple(r['args'][1:])==s)]
assert len(qualified)==17
print(json.dumps({'workload_processes':1440,'clock_processes':5,'meter_self_checks':1,'signal_threshold_ns':threshold,'qualified_cells':sum(r['qualified'] for r in sizing),'qualified_scenarios':qualified,'median_within_cell_max_min':statistics.median(max(r['session_ns'])/min(r['session_ns']) for r in sizing),'largest_within_cell_max_min':max(max(r['session_ns'])/min(r['session_ns']) for r in sizing),'primary_confirmation':False},indent=2))
