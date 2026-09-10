"""Native constructor observer gate with independent complete expectations."""
import hashlib,importlib.util,itertools,json,os,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3]
spec=importlib.util.spec_from_file_location('leaf_gate',Path(__file__).parents[1]/'service/gate.py')
leaf=importlib.util.module_from_spec(spec);spec.loader.exec_module(leaf)
OUT=ROOT/'docs/experiments/results/s03-native-structured'
BUILD=ROOT/'target/s03-native-structured'
CASES=[
 ('plain','#Pair{#A,#Wrap{#B}}',['#Pair{#A{},#Wrap{#B{}}}'],False),
 ('correlated','!X&(1000)=&(1){#A,#B};#Pair{X₀,X₁}',['#Pair{#A{},#A{}}','#Pair{#B{},#B{}}'],False),
 ('independent','#Pair{&(1){#A,#B},&(2){#A,#B}}',[f'#Pair{{#{a}{{}},#{b}{{}}}}' for a in 'AB' for b in 'AB'],False),
 ('duplicates','#Wrap{&(1){#A,#A}}',['#Wrap{#A{}}']*2,False),
 ('nested-correlation','!X&(1000)=&(1){#A,#B};#Pair{#Wrap{X₀},#Wrap{X₁}}',[f'#Pair{{#Wrap{{#{a}{{}}}},#Wrap{{#{a}{{}}}}}}' for a in 'AB'],False),
 ('joined-failure','#Join{#Answer{#A},&{}}',[],False),
 ('off-output-birth','#Join{#Answer{#A},&(1){#Ok,#Ok}}',['#Join{#Answer{#A{}},#Ok{}}']*2,False),
 ('off-output-filter','#Join{#Answer{#A},&(1){#Ok,&{}}}',['#Join{#Answer{#A{}},#Ok{}}'],False),
 ('joined-success','#Join{#Answer{#A},#Ok}',['#Join{#Answer{#A{}},#Ok{}}'],False),
 ('branch-rejection','!X&(1000)=&(1){#A,#B};#Join{#Answer{X₀},@check(X₁)}',['#Join{#Answer{#A{}},#Ok{}}'],False),
 ('loop-first','#Pair{&(1){@spin(#Loop),#A},#B}',['#Pair{#A{},#B{}}'],True),
 ('loop-last','#Pair{&(1){#A,@spin(#Loop)},#B}',['#Pair{#A{},#B{}}'],True),
 ('reference-loop','#Wrap{&(1){@loop,#A}}',['#Wrap{#A{}}'],True),
 ('deep-loop','#Outer{#Inner{&(1){@loop,#A}},#B}',['#Outer{#Inner{#A{}},#B{}}'],True),
 ('disconnected-failure','!unused = &{};#Answer{#A}',['#Answer{#A{}}'],False),
]
def list_term(items):
 result='#Nil'
 for item in reversed(items):result=f'#Cons{{{item},{result}}}'
 return result
for n in [1,2,3]:
 for order in itertools.permutations(range(n)):
  for wrapped in [False,True]:
   prefix=''.join(f'!X{i}&({1000+i})=&({1+i}){{#A,#B}};' for i in range(n))
   left=list_term([f'X{i}₀' for i in order]);right=list_term([f'X{i}₁' for i in reversed(order)])
   expression=prefix+'#Pair{'+left+','+right+'}'
   if wrapped:expression=prefix+'#Outer{#Inner{#Pair{'+left+','+right+'}}}'
   expected=[]
   for values_ in itertools.product('AB',repeat=n):
    left_value=list_term([f'#{values_[i]}{{}}' for i in order]).replace('#Nil','#Nil{}')
    right_value=list_term([f'#{values_[i]}{{}}' for i in reversed(order)]).replace('#Nil','#Nil{}')
    value=f'#Pair{{{left_value},{right_value}}}'
    expected.append(f'#Outer{{#Inner{{{value}}}}}' if wrapped else value)
   CASES.append(('permutation-'+str(n)+'-'+''.join(map(str,order))+'-'+str(int(wrapped)),expression,expected,False))
def invoke(binary,path,fuel=1,calls=1024,off=False):
 env=dict(os.environ,CHR_FUEL=str(fuel),CHR_CALLS=str(calls))
 env.pop('CHR_COUNTERS_OFF',None)
 if off:env['CHR_COUNTERS_OFF']='1'
 try:
  p=subprocess.run([str(binary),str(path),'-C'],env=env,text=True,capture_output=True,preexec_fn=leaf.limits,timeout=3)
  return dict(code=p.returncode,stdout=p.stdout,stderr=p.stderr)
 except subprocess.TimeoutExpired:return dict(timeout=True)
def values(result):return result.get('stdout','').splitlines()
def build():
 source=leaf.CHECKOUT/'src/hvm.c';original=source.read_text()
 assert hashlib.sha256(source.read_bytes()).hexdigest()=='3d2724d0716b5d6b1c07a3b0b3c5cb89f848487a43d364ea689658f9aeb75fe7'
 # Reproduce the registered leaf reducer hooks; keep its measured source untouched.
 gate=(Path(__file__).parents[1]/'service/gate.py').read_text()
 start=gate.index(" patched=original.replace(");end=gate.index(" generated=",start)
 code='\n'.join(line[1:] for line in gate[start:end].splitlines())
 env=dict(original=original,fragment=Path(__file__).with_name('observer.c').read_text())
 exec(code,env)
 generated=BUILD/'observer.c';generated.write_text(env['patched']);binary=BUILD/'observer'
 subprocess.run(['clang','-O2',str(generated),'-o',str(binary)],check=True,timeout=60)
 return binary

def main():
 OUT.mkdir(parents=True,exist_ok=True);BUILD.mkdir(parents=True,exist_ok=True)
 programs={}
 for name,expr,_,_ in CASES:
  p=OUT/(name+'.hvm');p.write_text('@spin = λx.@spin(x)\n@loop = @loop\n@check = λ{#A: #Ok; #B: &{}}\n@main = '+expr+'\n');programs[name]=p
 if sys.argv[-1]=='red':
  r=invoke(ROOT/'target/s03-native-service/service-transitions',programs['plain'],calls=64)
  (OUT/'red.json').write_text(json.dumps(r,indent=2)+'\n');assert values(r)==CASES[0][2],r
  return
 binary=build();rows=[]
 for replay in range(2):
  for fuel in [1,2,8]:
   for name,_,expected,ongoing in CASES:
    r=invoke(binary,programs[name],fuel);row=dict(replay=replay,fuel=fuel,case=name,result=r);rows.append(row)
    (OUT/'runs.json').write_text(json.dumps(rows,indent=2)+'\n')
    assert r.get('code')==0,row
    assert sorted(values(r))==sorted(expected),row
    events=[json.loads(l) for l in r['stderr'].splitlines()];row['events']=events
    assert bool(events[-1]['pending'])==ongoing,row
    assert events[-1]['unsupported']==0,row
    assert all(e['visits']<=fuel and e['stack']==1 for e in events[:-1]),row
 count=len(CASES)*3
 for i in range(count):assert rows[i]['result']==rows[i+count]['result']
 off=[]
 for row in rows[:count]:
  r=invoke(binary,programs[row['case']],row['fuel'],off=True);assert r['code']==0 and r['stdout']==row['result']['stdout'],r
  ev=[json.loads(l) for l in r['stderr'].splitlines()]
  assert [{k:v for k,v in e.items() if k!='delta'} for e in ev]==[{k:v for k,v in e.items() if k!='delta'} for e in row['events']],r
  assert all(e['delta']==0 for e in ev[:-1]),r
  off.append(dict(case=row['case'],fuel=row['fuel'],result=r))
 cancellations=[]
 for name in programs:
  for calls in [0,1,4,16]:
   r=invoke(binary,programs[name],1,calls);assert r['code']==0,r
   events=[json.loads(l) for l in r['stderr'].splitlines()];end=events[-1]
   assert end['calls']<=calls and end['unsupported']==0,r
   full=next(row for row in rows if row['replay']==0 and row['fuel']==1 and row['case']==name)
   full_events=full['events'][:-1]
   completed=min(calls,len(full_events))
   assert end['calls']==completed and events[:-1]==full_events[:completed],r
   assert values(r)==values(full['result'])[:len(values(r))],r
   if calls==0:assert end['pending']==1 and not values(r),r
   elif completed:assert end['pending']==full_events[completed-1]['pending'],r
   cancellations.append(dict(case=name,calls=calls,result=r))
 controls=[]
 for name,_,expected,ongoing in CASES:
  if ongoing:continue
  r=invoke(ROOT/'target/s03-native-service/baseline',programs[name]);assert r['code']==0 and sorted(values(r))==sorted(expected),(name,r)
  controls.append(dict(case=name,result=r))
 rejections=[]
 for name,expr,code,unsupported in [('higher-order','λx.x',0,1),('depth-bound','#W{'*260+'#A'+'}'*260,2,None)]:
  path=OUT/(name+'.hvm');path.write_text('@main = '+expr+'\n')
  r=invoke(binary,path);assert r.get('code')==code and not values(r),r
  if unsupported is not None:assert json.loads(r['stderr'].splitlines()[-1])['unsupported']==unsupported,r
  rejections.append(dict(case=name,result=r))
 (OUT/'rejections.json').write_text(json.dumps(rejections,indent=2)+'\n')
 for name,data in [('runs',rows),('counter-off',off),('cancellation',cancellations),('controls',controls)]:
  (OUT/(name+'.json')).write_text(json.dumps(data,indent=2)+'\n')
 paths=[Path(__file__),Path(__file__).with_name('observer.c'),Path(__file__).parents[1]/'service/gate.py',binary,BUILD/'observer.c',ROOT/'target/s03-native-service/baseline',*programs.values(),OUT/'higher-order.hvm',OUT/'depth-bound.hvm']
 (OUT/'validation.json').write_text(json.dumps(dict(cells=count,replays=2,counter_off=len(off),cancellation=len(cancellations),controls=len(controls),rejections=len(rejections),hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}),indent=2)+'\n')
 print(f'{count} native structured cases replay exactly, with counter-off, cancellation and finite controls.')
if __name__=='__main__':main()
