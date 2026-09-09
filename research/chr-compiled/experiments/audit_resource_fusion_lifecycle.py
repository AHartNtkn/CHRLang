#!/usr/bin/env python3
import collections,hashlib,itertools,json,pathlib,random,statistics
ROOT=pathlib.Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s06-resource-fusion-lifecycle'
MODES=['original-scan','original-special','original-indexed','fused-scan','fused-special','fused-indexed'];FAMILIES=['plain','choices','duplicates','shared','spare']
CONFIGS=list(itertools.product(MODES,FAMILIES,[0,1,4],[1,4]))
def read(path):
 r=json.loads(path.read_text());assert r['returncode']==0 and not r['stderr'],path
 x=json.loads(r['stdout']);key=tuple(x[k] for k in ['mode','family','firings','reuse']);assert r['command'][1:]==list(map(str,key))
 names=['inference']
 for i in range(x['reuse']):names+=['input','certify']+(['prepare'] if i==0 else [])+['setup','execute','observe','completed-drop','search-drop','answers-drop','input-drop']
 names+=['cancel-input','cancel-certify','cancel-setup','cancel-unit','cancel-search-drop','cancel-input-drop','prepared-drop','inference-drop']
 assert [p['phase'] for p in x['phases']]==names
 if x['meter']:
  m=[p['memory'] for p in x['phases']];base=m[0]['live_start'];inf=m[0]['live_end'];prep=next(p['memory'] for p in x['phases'] if p['phase']=='prepare');owner=inf+prep['live_end']-prep['live_start']
  assert all(a['live_end']==b['live_start'] for a,b in zip(m,m[1:])) and m[-1]['live_end']==base
  for p in x['phases']:
   q=p['memory'];assert q['peak_live']>=max(q['live_start'],q['live_end'])
   if p['phase'] in ['input-drop','cancel-input-drop']:assert q['live_end']==owner
   if p['phase']=='prepared-drop':assert q['live_end']==inf
 else:assert all(p['memory'] is None for p in x['phases'])
 return key,x
def groups(kind,reps,seed):
 order=[(rep,c) for rep in range(reps) for c in CONFIGS];random.Random(seed).shuffle(order);assert json.loads((OUT/f'{kind}-order.json').read_text())==json.loads(json.dumps(order))
 groups=collections.defaultdict(list)
 for i,(rep,c) in enumerate(order):
  key,x=read(OUT/kind/f'{i:03}-r{rep}.json');assert key==c;groups[key].append(x)
 assert set(groups)==set(CONFIGS);return groups
def allocation_audit():
 g=groups('meter',2,7306);rows=[]
 for key,pair in sorted(g.items()):
  assert [{p['phase']:None} for p in pair[0]['phases']]==[{p['phase']:None} for p in pair[1]['phases']]
  assert [p['memory'] for p in pair[0]['phases']]==[p['memory'] for p in pair[1]['phases']],key
  ps=[p for p in pair[0]['phases'] if not p['phase'].startswith('cancel-')];base=ps[0]['memory']['live_start'];byphase=collections.Counter()
  for p in ps:byphase[p['phase']]+=p['memory']['requested_bytes']
  rows.append(dict(zip(['mode','family','firings','reuse'],key))|{'traffic':sum(byphase.values()),'peak_growth':max(p['memory']['peak_live'] for p in ps)-base,'phase_bytes':dict(byphase)})
 pre=[read(p)[0] for p in (OUT/'preflight').glob('*.json')];assert len(pre)==90 and set(pre)=={c for c in CONFIGS if c[-1]==1}
 (OUT/'allocation-audit.json').write_text(json.dumps({'runs':360,'preflight':90,'exact_pairs':180,'rows':rows},indent=2)+'\n')
 return rows
def main():
 rows=allocation_audit();g=groups('time',5,7307);timings=[];distorted=[]
 for key,xs in sorted(g.items()):
  values=[];cpus=[]
  for i,x in enumerate(xs):
   ps=[p for p in x['phases'] if not p['phase'].startswith('cancel-')];ns=sum(p['ns'] for p in ps);cpu=sum(p['cpu_ns'] for p in ps);values.append(ns);cpus.append(cpu)
   if ns>cpu*1.2:distorted.append([*key,i,ns,cpu])
  timings.append(dict(zip(['mode','family','firings','reuse'],key))|{'median_ns':statistics.median(values),'range_ns':[min(values),max(values)],'cpu_median_ns':statistics.median(cpus),'cpu_range_ns':[min(cpus),max(cpus)]})
 freeze=json.loads((OUT/'freeze.json').read_text())
 for p,h in freeze['sources'].items():assert hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h,p
 for k,h in freeze['binaries'].items():assert hashlib.sha256((ROOT/'target/resource-fusion-lifecycle'/k).read_bytes()).hexdigest()==h,k
 (OUT/'timing-audit.json').write_text(json.dumps({'runs':900,'wall_over_cpu_1_2':distorted,'rows':timings},indent=2)+'\n')
 print('1350 processes; 180 exact allocation pairs; all disposal checks; scheduling flags',len(distorted))
 for family in FAMILIES:
  print(family,[(r['mode'],r['traffic'],next(t['median_ns'] for t in timings if all(t[k]==r[k] for k in ['mode','family','firings','reuse']))) for r in rows if r['family']==family and r['firings']==4 and r['reuse']==4])
if __name__=='__main__':main()
