"""Audit source composition, raw multiplicity, service progress and boundaries."""
import gzip,hashlib,json
from gate import ROOT,OUT,BINARY,cases,oldcases,source_only,normalize,parse
from compiler import compile_source

def rows(name):
 with gzip.open(OUT/(name+'.gz'),'rt') as f:return [json.loads(l) for l in f]
def main():
 for name,receipt in json.loads((OUT/'archives.json').read_text()).items():
  path=OUT/(name+'.gz');assert hashlib.sha256(path.read_bytes()).hexdigest()==receipt['archive_sha256']
  with gzip.open(path,'rb') as f:assert hashlib.file_digest(f,'sha256').hexdigest()==receipt['uncompressed_sha256']
 v=json.loads((OUT/'validation.json').read_text())
 for path,h in v['hashes'].items():assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==h,path
 prior=json.loads((ROOT/'docs/experiments/results/s03-native-identity-source/validation.json').read_text());assert v['hashes'][str(BINARY.relative_to(ROOT))]==prior['hashes'][str(BINARY.relative_to(ROOT))]
 sources={s['name']:s for s in cases()};assert len(sources)==48;metadata={};program_hashes={}
 for name,s in sources.items():
  program,preds,atoms=compile_source(source_only(s));p=OUT/(name+'.hvm');assert p.read_text()==program
  metadata[name]=(preds,atoms);program_hashes[str(p.relative_to(ROOT))]=hashlib.sha256(p.read_bytes()).hexdigest()
 full={};seen=set();maximum=0;answers=0
 for row in rows('runs.jsonl'):
  name,rep,fuel=row['case'],row['rep'],row['fuel'];key=(name,rep,fuel);assert key not in seen;seen.add(key);s=sources[name];r=row['result']
  assert r['code']==0 and sorted(normalize(parse(l,*metadata[name])) for l in r['stdout'].splitlines())==sorted(map(normalize,s['expected']))
  ev=[json.loads(l) for l in r['stderr'].splitlines()];assert bool(ev[-1]['pending'])==s['ongoing'] and ev[-1]['unsupported']==0 and ev[-1]['calls']==len(ev)-1
  assert all(e['visits']<=fuel and e['stack']==1 for e in ev[:-1])
  if not s['ongoing']:maximum=max(maximum,len(ev)-1)
  if rep==0:full[name,fuel]=r;answers+=len(r['stdout'].splitlines())
  else:assert full[name,fuel]==r
 assert seen=={(n,r,f) for n in sources for r in [0,1] for f in [1,8]}
 seen=set();half_progress=[]
 for row in rows('diagnostics.jsonl'):
  name=row['case'];fuel=row.get('fuel',1);kind=row['kind'];calls=row.get('calls');key=(name,kind,fuel,calls);assert key not in seen;seen.add(key)
  original=full[name,fuel];oe=[json.loads(l) for l in original['stderr'].splitlines()];r=row['result'];assert r['code']==0;ev=[json.loads(l) for l in r['stderr'].splitlines()]
  if kind=='off':
   assert r['stdout']==original['stdout'] and all(e['delta']==0 for e in ev[:-1])
   assert [{k:v for k,v in e.items() if k!='delta'} for e in ev]==[{k:v for k,v in e.items() if k!='delta'} for e in oe]
  else:
   assert kind=='cancel';n=min(calls,len(oe)-1);assert ev[:-1]==oe[:n] and ev[-1]['calls']==n and ev[-1]['pending']==(oe[n-1]['pending'] if n else 1)
   assert r['stdout'].splitlines()==original['stdout'].splitlines()[:len(r['stdout'].splitlines())]
   if sources[name]['ongoing'] and calls==32768:assert r['stdout']==original['stdout'] and ev[-1]['pending']>0;half_progress.append(name)
 expected={(n,'off',f,None) for n in sources for f in [1,8]}
 for name in sources:
  calls=len(full[name,1]['stderr'].splitlines())-1
  expected|={(name,'cancel',1,n) for n in {0,1,16,calls//2}}
 assert seen==expected and len(half_progress)==8
 controls=json.loads((OUT/'controls.json').read_text());assert len(controls)==40 and {r['case'] for r in controls}=={n for n,s in sources.items() if not s['ongoing']}
 for row in controls:assert row['result']['code']==0 and sorted(normalize(parse(l,*metadata[row['case']])) for l in row['result']['stdout'].splitlines())==sorted(map(normalize,sources[row['case']]['expected']))
 det={s['name']:s for s in oldcases.cases()};regressions=rows('deterministic.jsonl');assert len(regressions)==77 and {r['case'] for r in regressions}==set(det)
 for row in regressions:
  s=det[row['case']];program,preds,atoms=compile_source(source_only(s));p=OUT/('det-'+s['name']+'.hvm');assert p.read_text()==program
  r=row['result'];assert r['code']==0 and len(r['stdout'].splitlines())==1 and normalize(parse(r['stdout'],preds,atoms))==normalize(s['expected'])
  ev=[json.loads(l) for l in r['stderr'].splitlines()];assert ev[-1]['pending']==0 and ev[-1]['unsupported']==0
 boundaries=json.loads((OUT/'boundaries.json').read_text());assert len(boundaries)==11
 for row in boundaries:
  r=row['result'];assert r['code']==0
  if row.get('kind')=='ordinary':assert not r['stdout'];continue
  ev=[json.loads(l) for l in r['stderr'].splitlines()];assert ev[-1]['pending']==0 and ev[-1]['unsupported']==0
  if row['disposition']=='incomplete-choice-namespace':assert r['stdout'].splitlines()==['#ChoiceLimit{}'] and row['reference']['stdout'].startswith('true 1\n')
  else:assert row['disposition']=='complete-source-failure' and not r['stdout'] and row['reference']['stdout']=='true 0\n'
 result=dict(choice_sources=48,native_runs=192,diagnostics=len(seen),ordinary_controls=40,deterministic_regressions=77,boundary_runs=11,finite_max_calls=maximum,ongoing_with_finite_answer_at_half_budget=half_progress,program_hashes=program_hashes)
 (OUT/'audit.json').write_text(json.dumps(result,indent=2)+'\n');print({k:v for k,v in result.items() if k!='program_hashes'})
if __name__=='__main__':main()
