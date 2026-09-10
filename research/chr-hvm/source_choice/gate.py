"""Independent complete source expectations for native choices and identities."""
import hashlib,importlib.util,json,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
from cases import cases
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s03-native-choice-identity';BINARY=ROOT/'target/s03-native-identity-source/native'
def load(name,path):
 spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
old=load('identity_gate',Path(__file__).parents[1]/'source_identity/gate.py');normalize=old.normalize;parse=old.parse;native=old.native
oldcases=load('identity_cases',Path(__file__).parents[1]/'source_identity/cases.py')
def source_only(s):return {k:s[k] for k in ['rules','query','outputs']}
def reference(source):
 def t(x):return 'v'+str(x) if isinstance(x,int) else x
 def c(c):return ':'.join([c[0]]+list(map(t,c[1])))
 def cs(xs):return ','.join(map(c,xs)) or '-'
 def body(actions):
  return ','.join(c(a[1]) if a[0]=='add' else '=:'+t(a[1])+':'+t(a[2]) if a[0]=='eq' else '('+body(a[1])+'|'+body(a[2])+')' if a[0]=='or' else '!' for a in actions) or '-'
 text=cs(source['query'])+'\n'+(','.join(map(str,source['outputs'])) or '-')+'\n'
 for r in source['rules']:text+=cs(r['kept'])+';'+cs(r['removed'])+';'+body(r['body'])+'\n'
 p=subprocess.run([str(ROOT/'target/debug/examples/native_choice_reference')],input=text,text=True,capture_output=True,timeout=3);assert p.returncode==0,p.stderr
 lines=p.stdout.splitlines();exhausted,raw=lines.pop(0).split();answers=[]
 def term(x):return int(x[1:]) if x.startswith('v') else x
 while lines:
  assert lines.pop(0)=='ANSWER';outs=lines.pop(0);residual=[]
  while lines and lines[0]!='ANSWER':
   parts=lines.pop(0).split(':');residual.append([parts[0],list(map(term,parts[1:]))])
  answers.append([list(map(term,outs.split(','))) if outs else [],residual])
 assert {normalize(a) for a in answers}=={normalize(a) for a in source['expected']},source['name']
 assert int(raw)==len(source['expected']) and (exhausted=='true')==(not source['ongoing']),source['name']
 return dict(input=text,stdout=p.stdout,stderr=p.stderr)
def verify(r,source,preds,atoms,fuel):
 assert r.get('code')==0,(source['name'],r.get('code'))
 assert sorted(normalize(parse(line,preds,atoms)) for line in r['stdout'].splitlines())==sorted(map(normalize,source['expected'])),source['name']
 ev=[json.loads(l) for l in r['stderr'].splitlines()]
 assert bool(ev[-1]['pending'])==source['ongoing'] and ev[-1]['unsupported']==0,source['name']
 assert all(e['visits']<=fuel and e['stack']==1 for e in ev[:-1]),source['name']
 return ev
def main():
 sources=list(cases());assert len(sources)==48
 refs=[dict(case=s['name'],result=reference(s)) for s in sources];(OUT/'references.json').write_text(json.dumps(refs,indent=2)+'\n');(OUT/'cases.json').write_text(json.dumps(sources,indent=2)+'\n')
 if sys.argv[-1]=='red':
  compiler=load('old_compiler',Path(__file__).parents[1]/'source_identity/compiler.py')
  try:compiler.compile_source(source_only(sources[0]))
  except ValueError as e:
   (OUT/'red.json').write_text(json.dumps(dict(case=sources[0]['name'],error=str(e),required_answers=len(sources[0]['expected'])))+'\n');print('RED: required choice source is rejected by deterministic compiler; 48 reference expectations pass.');return
  raise AssertionError('expected missing choice admission')
 from compiler import compile_source
 programs={}
 for s in sources:
  program,preds,atoms=compile_source(source_only(s));path=OUT/(s['name']+'.hvm');path.write_text(program);programs[s['name']]=(path,preds,atoms)
 rows=[]
 with (OUT/'runs.jsonl').open('w') as log:
  for rep in range(2):
   for fuel in [1,8]:
    for s in sources:
     path,preds,atoms=programs[s['name']];r=native.invoke(BINARY,path,fuel,65536);row=dict(case=s['name'],rep=rep,fuel=fuel,result=r);log.write(json.dumps(row)+'\n');log.flush();verify(r,s,preds,atoms,fuel);rows.append(row)
 n=len(sources)*2
 for a,b in zip(rows[:n],rows[n:]):assert a['result']==b['result']
 with (OUT/'diagnostics.jsonl').open('w') as log:
  for row in rows[:n]:
   path,preds,atoms=programs[row['case']];full=row['result'];events=[json.loads(l) for l in full['stderr'].splitlines()]
   off=native.invoke(BINARY,path,row['fuel'],65536,off=True);ev=[json.loads(l) for l in off['stderr'].splitlines()]
   log.write(json.dumps(dict(case=row['case'],kind='off',fuel=row['fuel'],result=off))+'\n');log.flush()
   assert off['code']==0 and off['stdout']==full['stdout'];assert all(e['delta']==0 for e in ev[:-1])
   assert [{k:v for k,v in e.items() if k!='delta'} for e in ev]==[{k:v for k,v in e.items() if k!='delta'} for e in events]
   if row['fuel']==1:
    for calls in sorted({0,1,16,(len(events)-1)//2}):
     r=native.invoke(BINARY,path,1,calls);log.write(json.dumps(dict(case=row['case'],kind='cancel',calls=calls,result=r))+'\n');log.flush();assert r['code']==0
     ev=[json.loads(l) for l in r['stderr'].splitlines()];k=min(calls,len(events)-1)
     assert ev[:-1]==events[:k] and ev[-1]['calls']==k and ev[-1]['pending']==(events[k-1]['pending'] if k else 1)
     assert r['stdout'].splitlines()==full['stdout'].splitlines()[:len(r['stdout'].splitlines())]
 controls=[]
 for s in sources:
  if s['ongoing']:continue
  path,preds,atoms=programs[s['name']];r=native.invoke(ROOT/'target/s03-native-service/baseline',path);assert r['code']==0 and sorted(normalize(parse(l,preds,atoms)) for l in r['stdout'].splitlines())==sorted(map(normalize,s['expected'])),s['name'];controls.append(dict(case=s['name'],result=r))
 (OUT/'controls.json').write_text(json.dumps(controls,indent=2)+'\n')
 regressions=[]
 with (OUT/'deterministic.jsonl').open('w') as log:
  for s in oldcases.cases():
   program,preds,atoms=compile_source(source_only(s));path=OUT/('det-'+s['name']+'.hvm');path.write_text(program)
   r=native.invoke(BINARY,path,8,65536);check=dict(s,expected=[s['expected']],ongoing=False);verify(r,check,preds,atoms,8)
   row=dict(case=s['name'],result=r);log.write(json.dumps(row)+'\n');log.flush();regressions.append(row)
 paths=[Path(__file__),Path(__file__).with_name('cases.py'),Path(__file__).with_name('compiler.py'),BINARY,ROOT/'target/debug/examples/native_choice_reference',ROOT/'research/chr-cases/examples/native_choice_reference.rs',ROOT/'research/chr-hvm/identity/kernel.hvm',ROOT/'research/chr-hvm/identity/build_kernel.py',ROOT/'research/chr-hvm/source_identity/gate.py',ROOT/'research/chr-hvm/source_identity/cases.py',ROOT/'docs/experiments/registrations/S03-native-choice-identity.md']
 (OUT/'validation.json').write_text(json.dumps(dict(sources=len(sources),runs=len(rows),controls=len(controls),deterministic=len(regressions),hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}),indent=2)+'\n');print('Native choice/identity gate and 77 deterministic regressions pass.')
if __name__=='__main__':main()
