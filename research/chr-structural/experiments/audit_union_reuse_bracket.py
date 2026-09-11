"""Verify the independently registered bracket and its prior byte predictions."""
import collections,hashlib,itertools,json,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s06-union-reuse/bracket'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f=read(BASE/'freeze.json');assert sha(BASE/'sources.zip')==f['archive_sha256'];assert sha(Path(f['binary']['path']))==f['binary']['sha256'];assert sha(BASE/'order.json')==f['order_sha256']
with zipfile.ZipFile(BASE/'sources.zip')as z:
 assert set(z.namelist())==set(f['sources'])
 for p,h in f['sources'].items():assert hashlib.sha256(z.read(p)).hexdigest()==h
order=read(BASE/'order.json');assert len(order)==72;assert len(list((BASE/'runs').glob('*.json')))==72
cells=collections.defaultdict(list)
for i,j in enumerate(order):
 raw=read(BASE/'runs'/f'{i}.json');assert raw['exit_code']==0 and not raw['stderr'];rows=[json.loads(l)for l in raw['stdout'].splitlines()];meta,*phases=rows;mode,family,n,q,endpoint,retention=j['args'];assert meta==dict(mode=mode,family=family,width=n,queries=q,full=True,retention=retention,meter=True)
 assert raw['command']==[f['binary']['path'],*map(str,j['args'])]
 assert [p['phase']for p in phases]==['source','prepare','source-drop']+['request','execute-observe','request-drop','consumer']*q+['prepared-drop','consumer-drop'];assert all(p['ns']is None for p in phases)
 ms=[p['memory']for p in phases];base=ms[0]['live_start'];assert ms[-1]['live_end']==base;assert all(a['live_end']==b['live_start']for a,b in zip(ms,ms[1:]));norm=[{k:v-base if k in ['live_start','live_end','peak_live']else v for k,v in m.items()}for m in ms];cells[tuple(j['args'])].append(dict(traffic=sum(m['requested_bytes']for m in ms),peak=max(m['peak_live']for m in ms)-base,normalized=norm))
assert set(cells)==set((m,'overlap',8,q,'full',c)for m,q,c in itertools.product(['union','separate','reduced','names','explicit','dedup'],[12,13],['immediate','window','all']))
summary=[]
for args,rs in cells.items():
 assert len(rs)==2 and rs[0]==rs[1];summary.append(dict(args=args,traffic=rs[0]['traffic'],peak=rs[0]['peak']))
for q,u,d in [(12,1221112,1205336),(13,1268368,1289512)]:
 for c in ['immediate','window','all']:
  assert cells['union','overlap',8,q,'full',c][0]['traffic']==u
  assert cells['dedup','overlap',8,q,'full',c][0]['traffic']==d
print(json.dumps(dict(processes=72,cells=36,exact_repeats=True,predictions_match=True,measured_traffic_crossing=[12,13],timing=False,summary=summary),indent=2))
