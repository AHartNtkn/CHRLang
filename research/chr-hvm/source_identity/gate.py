"""Complete identity-bearing source gate; independent expected answers and reference."""
import hashlib,importlib.util,itertools,json,re,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
from cases import cases
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s03-native-identity-source';BINARY=ROOT/'target/s03-native-identity-source/native'
spec=importlib.util.spec_from_file_location('native',Path(__file__).parents[1]/'structured/gate.py');native=importlib.util.module_from_spec(spec);spec.loader.exec_module(native)
def normalize(answer):
 outputs,residual=answer;vars=sorted({x for x in outputs if isinstance(x,int)}|{x for _,args in residual for x in args if isinstance(x,int)})
 # Exhaustive alpha-canonicalization outside native execution; multiplicity retained.
 encodings=[]
 for perm in itertools.permutations(range(len(vars))):
  mapping=dict(zip(vars,perm))
  def t(x):return 'v'+str(mapping[x]) if isinstance(x,int) else 'a'+x
  encodings.append((tuple(map(t,outputs)),tuple(sorted((name,tuple(map(t,args))) for name,args in residual))))
 return min(encodings)
def reference(source):
 def t(x):return 'v'+str(x) if isinstance(x,int) else x
 def c(c):return ':'.join([c[0]]+list(map(t,c[1])))
 def cs(xs):return ','.join(map(c,xs)) or '-'
 text=cs(source['query'])+'\n'+(','.join(map(str,source['outputs'])) or '-')+'\n'
 for r in source['rules']:
  body=','.join(c(a[1]) if a[0]=='add' else '=:'+t(a[1])+':'+t(a[2]) for a in r['body']) or '-'
  text+=cs(r['kept'])+';'+cs(r['removed'])+';'+body+'\n'
 p=subprocess.run([str(ROOT/'target/debug/examples/native_identity_reference')],input=text,text=True,capture_output=True,timeout=3);assert p.returncode==0,p.stderr
 lines=p.stdout.splitlines();assert lines[0]=='true 1',p.stdout
 def term(x):return int(x[1:]) if x.startswith('v') else x
 out=list(map(term,lines[1].split(','))) if lines[1] else []
 residual=[]
 for line in lines[2:]:parts=line.split(':');residual.append([parts[0],list(map(term,parts[1:]))])
 return [out,residual],dict(input=text,stdout=p.stdout,stderr=p.stderr)
def parse(text,predicates,atoms):
 tokens=re.findall(r'#[A-Za-z_]+|[0-9]+|[{},]',text);i=0
 def value():
  nonlocal i
  name=tokens[i];i+=1
  if not name.startswith('#'):return int(name)
  assert tokens[i]=='{';i+=1;args=[]
  while tokens[i]!='}':
   args.append(value())
   if tokens[i]==',':i+=1
   else:assert tokens[i]=='}'
  i+=1;return (name,args)
 root=value();assert i==len(tokens) and root[0]=='#Answer',(text,root)
 def ls(v):
  out=[]
  while v[0]=='#Cons':out.append(v[1][0]);v=v[1][1]
  assert v==('#Nil',[]);return out
 def t(v):
  assert v[0] in ['#Var','#Atom'] and len(v[1])==1
  return v[1][0] if v[0]=='#Var' else atoms[v[1][0]]
 out=list(map(t,ls(root[1][0])));residual=[]
 for fact in ls(root[1][1]):
  assert fact[0]=='#Result';residual.append([predicates[fact[1][0]],list(map(t,ls(fact[1][1])))])
 return [out,residual]
def main():
 sources=list(cases());assert len(sources)==77
 refs=[]
 for source in sources:
  answer,r=reference(source);assert normalize(answer)==normalize(source['expected']),source['name'];refs.append(dict(case=source['name'],result=r))
 (OUT/'references.json').write_text(json.dumps(refs,indent=2)+'\n');(OUT/'cases.json').write_text(json.dumps(sources,indent=2)+'\n')
 if sys.argv[-1]=='red':
  p=OUT/'red.hvm';p.write_text('@main = #Answer{#Nil,#Nil}\n');r=native.invoke(BINARY,p,8,65536);(OUT/'red.json').write_text(json.dumps(r,indent=2)+'\n')
  assert r['code']==0 and normalize(parse(r['stdout'],[],[]))!=normalize(sources[0]['expected']);print('RED: runnable native stub omits the required occurrence and propagation receipt; all61 reference expectations pass.');return
 from compiler import compile_source
 programs={}
 for source in sources:
  program,preds,atoms=compile_source({k:source[k] for k in ['rules','query','outputs']});p=OUT/(source['name']+'.hvm');p.write_text(program);programs[source['name']]=(p,preds,atoms)
 rows=[]
 with (OUT/'runs.jsonl').open('w') as log:
  for rep in range(2):
   for fuel in [1,8]:
    for source in sources:
     p,preds,atoms=programs[source['name']];r=native.invoke(BINARY,p,fuel,65536);row=dict(case=source['name'],rep=rep,fuel=fuel,result=r);log.write(json.dumps(row)+'\n');log.flush()
     assert r.get('code')==0,(row['case'],row['rep'],row['fuel'])
     assert len(r['stdout'].splitlines())==1,(row['case'],row['rep'],row['fuel'])
     assert normalize(parse(r['stdout'],preds,atoms))==normalize(source['expected']),(row['case'],row['rep'],row['fuel'])
     ev=[json.loads(l) for l in r['stderr'].splitlines()];assert ev[-1]['pending']==0 and ev[-1]['unsupported']==0,(row['case'],row['rep'],row['fuel'])
     assert all(e['visits']<=fuel and e['stack']==1 for e in ev[:-1]),(row['case'],row['rep'],row['fuel'])
     rows.append(row)
 count=len(sources)*2
 for a,b in zip(rows[:count],rows[count:]):assert a['result']==b['result']
 diagnostics=[]
 for row in rows[:count]:
  p,_,_=programs[row['case']];full=row['result'];events=[json.loads(l) for l in full['stderr'].splitlines()]
  off=native.invoke(BINARY,p,row['fuel'],65536,off=True);oe=[json.loads(l) for l in off['stderr'].splitlines()]
  assert off['code']==0 and off['stdout']==full['stdout']
  assert [{k:v for k,v in e.items() if k!='delta'} for e in oe]==[{k:v for k,v in e.items() if k!='delta'} for e in events]
  assert all(e['delta']==0 for e in oe[:-1]);diagnostics.append(dict(case=row['case'],fuel=row['fuel'],kind='off',result=off))
  if row['fuel']==1:
   for calls in [0,1,16]:
    r=native.invoke(BINARY,p,1,calls);assert r['code']==0;ev=[json.loads(l) for l in r['stderr'].splitlines()];n=min(calls,len(events)-1)
    assert ev[:-1]==events[:n] and ev[-1]['calls']==n and ev[-1]['pending']==(events[n-1]['pending'] if n else 1)
    assert r['stdout'].splitlines()==full['stdout'].splitlines()[:len(r['stdout'].splitlines())]
    diagnostics.append(dict(case=row['case'],calls=calls,kind='cancel',result=r))
 (OUT/'diagnostics.jsonl').write_text(''.join(json.dumps(r)+'\n' for r in diagnostics))
 controls=[]
 for source in sources:
  p,preds,atoms=programs[source['name']];r=native.invoke(ROOT/'target/s03-native-service/baseline',p);assert r['code']==0 and normalize(parse(r['stdout'],preds,atoms))==normalize(source['expected'])
  controls.append(dict(case=source['name'],result=r))
 (OUT/'controls.json').write_text(json.dumps(controls,indent=2)+'\n')
 paths=[Path(__file__),Path(__file__).with_name('compiler.py'),Path(__file__).with_name('cases.py'),BINARY,BINARY.with_suffix('.c'),ROOT/'target/debug/examples/native_identity_reference',ROOT/'research/chr-cases/examples/native_identity_reference.rs',ROOT/'research/chr-hvm/identity/kernel.hvm',ROOT/'research/chr-hvm/identity/build_kernel.py',ROOT/'docs/experiments/registrations/S03-native-identity-source.md']
 (OUT/'validation.json').write_text(json.dumps(dict(sources=len(sources),runs=len(rows),diagnostics=len(diagnostics),controls=len(controls),hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}),indent=2)+'\n');print('77 complete source cases pass native/reference/direct expectations.')
if __name__=='__main__':main()
