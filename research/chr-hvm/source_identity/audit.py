"""Audit full native source observations, source generation and cutoff replay."""
import copy,hashlib,json,sys
from pathlib import Path
sys.dont_write_bytecode=True
from gate import ROOT,OUT,BINARY,normalize,parse,reference
from compiler import compile_source
from cases import cases

def rows(name):return [json.loads(l) for l in (OUT/name).read_text().splitlines()]
def main():
 sources=list(cases());assert len(sources)==77;byname={s['name']:s for s in sources}
 v=json.loads((OUT/'validation.json').read_text());current={}
 # Formatting affects source/debug hashes; recheck every reference observation.
 oldrefs={r['case']:r['result'] for r in json.loads((OUT/'references.json').read_text())}
 for source in sources:
  answer,r=reference(source);assert normalize(answer)==normalize(source['expected']) and r==oldrefs[source['name']]
 reference_paths=['research/chr-cases/examples/native_identity_reference.rs','target/debug/examples/native_identity_reference']
 snapshot=OUT/'reference-measured.rs';assert hashlib.sha256(snapshot.read_bytes()).hexdigest()==v['hashes'][reference_paths[0]]
 for path,h in v['hashes'].items():
  actual=hashlib.sha256((ROOT/path).read_bytes()).hexdigest()
  if path in reference_paths:current[path]=actual
  else:assert actual==h,path
 (OUT/'reference-reverification.json').write_text(json.dumps(dict(complete_observation_replays=77,hashes=current),indent=2)+'\n')
 base=(ROOT/'target/s03-native-consuming/native.c').read_text();derived=BINARY.with_suffix('.c').read_text();assert base.count('limit>8192')==1 and derived==base.replace('limit>8192','limit>65536')
 metadata={};hashes={}
 for source in sources:
  program,preds,atoms=compile_source({k:source[k] for k in ['rules','query','outputs']});path=OUT/(source['name']+'.hvm')
  assert path.read_text()==program;metadata[source['name']]=(preds,atoms);hashes[str(path.relative_to(ROOT))]=hashlib.sha256(path.read_bytes()).hexdigest()
 full={};seen=set();maximum=0
 runs=rows('runs.jsonl');assert len(runs)==308
 for row in runs:
  name,rep,fuel=row['case'],row['rep'],row['fuel'];key=name,rep,fuel;assert key not in seen;seen.add(key)
  r=row['result'];assert r['code']==0 and len(r['stdout'].splitlines())==1
  assert normalize(parse(r['stdout'],*metadata[name]))==normalize(byname[name]['expected'])
  ev=[json.loads(l) for l in r['stderr'].splitlines()];assert ev[-1]['pending']==0 and ev[-1]['unsupported']==0 and ev[-1]['calls']==len(ev)-1
  assert all(e['visits']<=fuel and e['stack']==1 for e in ev[:-1]);maximum=max(maximum,len(ev)-1)
  if rep==0:full[name,fuel]=r
  else:assert full[name,fuel]==r
 assert seen=={(name,rep,fuel) for name in byname for rep in [0,1] for fuel in [1,8]}
 initial=rows('initial-cutoff.jsonl');assert len(initial)==7
 for row in initial:
  old=row['result'];new=full[row['case'],row['fuel']];a=[json.loads(l) for l in old['stderr'].splitlines()];b=[json.loads(l) for l in new['stderr'].splitlines()]
  if a[-1]['pending']:assert a[:-1]==b[:len(a)-1] and a[-1]['calls']==8192
  else:assert old==new
 previous=rows('obligations-runs.jsonl');assert len(previous)==244
 for row in previous:assert full[row['case'],row['fuel']]==row['result']
 seen=set();diagnostics=rows('diagnostics.jsonl');assert len(diagnostics)==385
 for row in diagnostics:
  name=row['case'];fuel=row.get('fuel',1);kind=row['kind'];calls=row.get('calls');key=(name,kind,fuel,calls);assert key not in seen;seen.add(key)
  original=full[name,fuel];oe=[json.loads(l) for l in original['stderr'].splitlines()];r=row['result'];assert r['code']==0;ev=[json.loads(l) for l in r['stderr'].splitlines()]
  if kind=='off':
   assert r['stdout']==original['stdout'];assert all(e['delta']==0 for e in ev[:-1])
   assert [{k:v for k,v in e.items() if k!='delta'} for e in ev]==[{k:v for k,v in e.items() if k!='delta'} for e in oe]
  else:
   assert kind=='cancel';n=min(calls,len(oe)-1)
   assert ev[:-1]==oe[:n] and ev[-1]['calls']==n and ev[-1]['pending']==(oe[n-1]['pending'] if n else 1)
   assert r['stdout'].splitlines()==original['stdout'].splitlines()[:len(r['stdout'].splitlines())]
 assert seen=={(c,'off',f,None) for c in byname for f in [1,8]}|{(c,'cancel',1,n) for c in byname for n in [0,1,16]}
 controls=json.loads((OUT/'controls.json').read_text());assert len(controls)==77 and {r['case'] for r in controls}==set(byname)
 for row in controls:assert row['result']['code']==0 and normalize(parse(row['result']['stdout'],*metadata[row['case']]))==normalize(byname[row['case']]['expected'])
 # Unsupported semantics must be rejected rather than ignored.
 base={k:sources[0][k] for k in ['rules','query','outputs']};bad=[]
 for label,edit in [('guard',lambda x:x['rules'][0].update(guards=[])),('choice',lambda x:x['rules'][0]['body'].append(['or',[],[]])),('constructor',lambda x:x['query'][0][1].append(['f',0])),('empty-heads',lambda x:x['rules'][0].update(kept=[],removed=[])),('query-goal',lambda x:x.update(goal='fail'))]:
  x=copy.deepcopy(base);edit(x)
  try:compile_source(x)
  except ValueError:bad.append(label)
  else:raise AssertionError(label)
 result=dict(sources=77,native_runs=308,diagnostics=385,ordinary_controls=77,reference_replays=77,prior_complete_replays=244,initial_prefix_checks=7,maximum_calls=maximum,admission_rejections=bad,program_hashes=hashes)
 (OUT/'audit.json').write_text(json.dumps(result,indent=2)+'\n');print({k:v for k,v in result.items() if k!='program_hashes'})
if __name__=='__main__':main()
