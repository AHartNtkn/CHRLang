"""Reconstruct every allocation summary from isolated raw receipts."""
from pathlib import Path
import collections,hashlib,itertools,json
ROOT=Path(__file__).resolve().parents[3];RAW=ROOT/'docs/experiments/results/s06-simplified-ownership'
def read(path):return json.loads(path.read_text())
f=read(RAW/'freeze.json')
for p,h in f['sources'].items():assert hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h,p
for b in f['binaries'].values():assert hashlib.sha256((ROOT/b['path']).read_bytes()).hexdigest()==b['sha256']
configs=list(itertools.product(['reduced','diagram','names','projected','symbolic','explicit'],['free','star','clique','duplicate','overlap','union'],[3,6],[2,3],[1,16,128],['member','full'],['immediate','window','all']))
assert read(RAW/'configurations.json')==[list(c) for c in configs]
assert len(list((RAW/'runs').glob('*.json')))==7776
summary=[]
for index,args in enumerate(configs):
 records=[]
 for suffix in ['0','1','ordinary']:
  r=read(RAW/f'runs/{index}-{suffix}.json');assert r['args']==list(args) and r['returncode']==0 and not r['stderr']
  records.append([json.loads(line) for line in r['stdout'].splitlines()])
 a,b,c=records;assert a==b and a[0]['meter'] and not c[0]['meter']
 assert a[0]['queries']==args[4]
 assert {k:v for k,v in a[0].items() if k!='meter'}=={k:v for k,v in c[0].items() if k!='meter'}
 names=['prepare']+['request','transport','observe','query-dispose','consumer']*args[4]+['identity-dispose','prepare-dispose','consumer-dispose']
 assert [r['phase'] for r in a[1:]]==[r['phase'] for r in c[1:]]==names
 assert all(r['memory'] is None for r in c[1:])
 phases=a[1:];baseline=phases[0]['memory']['live_start']
 for left,right in zip(phases,phases[1:]):assert left['memory']['live_end']==right['memory']['live_start']
 assert phases[-1]['memory']['live_end']==baseline
 assert all(r['memory']['peak_live']>=max(r['memory']['live_start'],r['memory']['live_end']) for r in phases)
 summary.append({'args':list(args),'identity_records':a[0]['identity_records'],'requested_bytes':sum(r['memory']['requested_bytes'] for r in phases),'peak_excess':max(r['memory']['peak_live'] for r in phases)-baseline,'prepare_bytes':phases[0]['memory']['requested_bytes'],'prepare_live':phases[0]['memory']['live_end']-baseline,'observe_bytes':sum(r['memory']['requested_bytes'] for r in phases if r['phase']=='observe'),'transport_bytes':sum(r['memory']['requested_bytes'] for r in phases if r['phase']=='transport')})
assert summary==read(RAW/'summary.json')
old=read(ROOT/'docs/experiments/results/s06-diagram-ownership/summary.json')
assert [r for r in summary if r['args'][0]!='reduced']==old,'original controls changed'
by_args={tuple(r['args']):r for r in summary}
contrasts={}
for control in ['diagram','names','projected','symbolic','explicit']:
 counts={metric:collections.Counter() for metric in ['requested_bytes','peak_excess']}
 reuse={q:collections.Counter() for q in [1,16,128]}
 for r in summary:
  if r['args'][0]!='reduced':continue
  other=by_args[(control,*r['args'][1:])]
  for metric in counts:
   direction='lower' if r[metric]<other[metric] else 'higher' if r[metric]>other[metric] else 'equal'
   counts[metric][direction]+=1
   if metric=='requested_bytes':reuse[r['args'][4]][direction]+=1
 contrasts[control]={'overall':counts,'requested_by_query_count':reuse}
print(json.dumps({'configurations':len(configs),'metered_processes':5184,'ordinary_processes':2592,'phase_continuity':True,'exact_repeats':True,'final_restoration':True,'contrasts':contrasts},indent=2))
