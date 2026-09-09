#!/usr/bin/env python3
import collections,hashlib,itertools,json,pathlib,random
ROOT=pathlib.Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s02-multihead-ownership'
MODES=['local-scan','tuples','partial','scan','indexed','special-scan','special-indexed'];FAMILIES=['sparse','broad','nested','cold','dense','three']
def read(path):
 r=json.loads(path.read_text());assert r['returncode']==0 and not r['stderr'],path
 x=json.loads(r['stdout']);key=tuple(x[k] for k in ['mode','family','width','reuse']);assert r['command'][1:]==list(map(str,key))
 phases=x['phases'];assert [p['phase'] for p in phases]==['prepare']+['input','setup','execute','observe','engine-drop','answer-drop','input-drop']*x['reuse']+['cancel-input','cancel-setup','cancel-advance','cancel-engine-drop','cancel-input-drop']*2+['prepared-drop']
 m=[p['memory'] for p in phases];base=m[0]['live_start'];prepared=m[0]['live_end']
 assert all(a['live_end']==b['live_start'] for a,b in zip(m,m[1:])) and m[-1]['live_end']==base
 for p in phases:
  q=p['memory'];assert q['peak_live']>=max(q['live_start'],q['live_end'])
  if p['phase'] in ['input-drop','cancel-input-drop']:assert q['live_end']==prepared
 normal=[p['memory'] for p in phases if not p['phase'].startswith('cancel-')]
 return key,x,{'traffic':sum(p['requested_bytes'] for p in normal),'peak_growth':max(p['peak_live'] for p in normal)-base,'prepared_live':prepared-base,'setup_bytes':sum(p['memory']['requested_bytes'] for p in phases if p['phase']=='setup'),'execution_bytes':sum(p['memory']['requested_bytes'] for p in phases if p['phase']=='execute'),'observation_bytes':sum(p['memory']['requested_bytes'] for p in phases if p['phase']=='observe')}
def main():
 configs=list(itertools.product(MODES,FAMILIES,[4,16,64]));order=[(rep,(*c,reuse)) for rep in range(2) for c in configs for reuse in [1,4]];random.Random(7204).shuffle(order)
 assert json.loads((OUT/'order.json').read_text())==json.loads(json.dumps(order))
 cells=collections.defaultdict(list)
 for i,(rep,c) in enumerate(order):
  key,x,summary=read(OUT/'runs'/f'{i:03}-r{rep}.json');assert key==c;cells[key].append((x,summary))
 assert len(cells)==252
 rows=[]
 for key,pair in sorted(cells.items()):
  assert len(pair)==2 and pair[0]==pair[1],key
  rows.append(dict(zip(['mode','family','width','reuse'],key))|pair[0][1])
 pre=[read(p)[0] for p in (OUT/'preflight').glob('*.json')];assert len(pre)==126 and set(pre)=={(*c,1) for c in configs}
 freeze=json.loads((OUT/'freeze.json').read_text());assert hashlib.sha256((ROOT/'target/multihead-ownership/runner').read_bytes()).hexdigest()==freeze['binary_sha256']
 archive={'research/chr-relational/tests/multihead_ownership.rs':OUT/'measured-source/runner.rs','docs/experiments/registrations/S02-multihead-ownership.md':OUT/'measured-source/registration.md'}
 for p,h in freeze['source_sha256'].items():assert hashlib.sha256(archive.get(p,ROOT/p).read_bytes()).hexdigest()==h,p
 result={'processes':504,'preflights':126,'exact_pairs':252,'rows':rows};(OUT/'audit.json').write_text(json.dumps(result,indent=2)+'\n')
 print('504 runs; 126 preflights; 252 exact pairs; all query/cancel/prepared baselines pass')
 for family in FAMILIES:
  print(family,[(r['mode'],r['traffic'],r['peak_growth']) for r in rows if r['width']==64 and r['reuse']==4 and r['family']==family])
if __name__=='__main__':main()
