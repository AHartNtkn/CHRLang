"""Native identity operations checked against independent equivalence classes."""
import hashlib,importlib.util,itertools,json,sys
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s03-native-identity-kernel'
spec=importlib.util.spec_from_file_location('native',Path(__file__).parents[1]/'structured/gate.py');native=importlib.util.module_from_spec(spec);spec.loader.exec_module(native)
BINARY=ROOT/'target/s03-native-consuming/native'
def term(x):return f'#Var{{{x}}}' if isinstance(x,int) else f'#Atom{{{ord(x)-97}}}'
def lst(xs):
 s='#Nil'
 for x in reversed(xs):s='#Cons{'+x+','+s+'}'
 return s
def printed(s):return s.replace('#Nil','#Nil{}').replace('#Fail','#Fail{}')
def oracle(ops):
 # Explicit partition merging, rather than directed substitution lookup.
 classes=[{i} for i in range(4)]+[{'a'},{'b'}]
 for a,b in ops:
  left=next(c for c in classes if a in c);right=next(c for c in classes if b in c)
  if left is right:continue
  merged=left|right
  if 'a' in merged and 'b' in merged:return '#Fail{}'
  classes=[c for c in classes if c is not left and c is not right]+[merged]
 values=[]
 for i in range(4):
  c=next(c for c in classes if i in c);atoms=c&{'a','b'}
  # Canonical minimum unknown is an observation convention, not a root policy.
  values.append(term(next(iter(atoms)) if atoms else min(c)))
 return printed('#Answer{'+lst(values)+'}')
def cases():
 pairs=list(itertools.product([0,1,'a','b'],repeat=2))
 for i,ops in enumerate(itertools.product(pairs,repeat=2)):
  yield f'bind-{i}', '@run('+lst(['#Bind{'+term(a)+','+term(b)+'}' for a,b in ops])+',#Nil)',[oracle(ops)]
 for i,ops in enumerate([[(0,1),(1,2),(2,3),(3,'a')],[(3,2),(2,1),(1,0)],[(0,'a'),(1,'b'),(0,1)]]):
  yield f'chain-{i}','@run('+lst(['#Bind{'+term(a)+','+term(b)+'}' for a,b in ops])+',#Nil)',[oracle(ops)]
 for ops in [[(0,1)],[(0,'a')],[(0,1),(1,'b')]]:
  for a,b in itertools.product([0,1,'a','b'],repeat=2):
   env='#Nil'
   # Chain actual native binds through continuations, then observe the same env.
   probe='λenv.!E&(9100000)=env;#Probe{@equal('+term(a)+','+term(b)+',E₀),@run(#Nil,E₁)}'
   expr=probe
   for left,right in reversed(ops):
    expr='λenv.@after(@bind('+term(left)+','+term(right)+',env),('+expr+'))'
   expr='('+expr+')(#Nil)'
   # Entailment means adding this equality leaves the independent partition unchanged.
   before=oracle(ops);after=oracle(ops+[(a,b)])
   equal=int(before==after and after!='#Fail{}')
   yield 'probe-'+str(len(ops))+'-'+str(ops[0][1])+'-'+str(a)+'-'+str(b),expr,['#Probe{'+str(equal)+','+before+'}']
 for a,b in itertools.product([0,1,'a','b'],repeat=2):
  yield f'equal-{a}-{b}',f'@equal({term(a)},{term(b)},#Nil)',[str(int(a==b))]
 # Receipt order and occurrence identity are independent of argument value.
 h='#Cons{#Receipt{7,'+lst(['2','3'])+'},#Nil}'
 for rule,ids,expected in [(7,[2,3],1),(7,[3,2],0),(8,[2,3],0),(7,[2,4],0)]:
  yield f'history-{rule}-{ids[0]}-{ids[1]}',f'@seen({rule},{lst(list(map(str,ids)))},{h})',[str(expected)]
 for ids in [[],[0],[0,1],[1,0],[0,0],[0,1,0]]:
  yield 'distinct-'+''.join(map(str,ids)),f'@distinct({lst(list(map(str,ids)))})',[str(int(len(ids)==len(set(ids))))]
 for target in [0,1,2]:
  store=lst(['#Occ{0,#Var{0}}','#Occ{1,#Var{0}}'])
  remaining=[f'#Occ{{{i},#Var{{0}}}}' for i in [0,1] if i!=target]
  yield f'remove-{target}',f'@remove({target},{store})',[printed(lst(remaining))]
 yield 'replacement','@add(2,#Var{1},@remove(0,'+lst(['#Occ{0,#Var{0}}','#Occ{1,#Var{0}}'])+'))',[printed('#Store{3,'+lst(['#Occ{2,#Var{1}}','#Occ{1,#Var{0}}'])+'}')]
 for name,arms,expected in [('choice',['a','b'],['a','b']),('duplicate',['a','a'],['a','a'])]:
  expr='@run('+lst(['#Bind{#Var{0},&(1){'+term(arms[0])+','+term(arms[1])+'}}'])+',#Nil)'
  yield name,expr,[oracle([(0,a)]) for a in expected]
 yield 'choice-failure','@run('+lst(['#Bind{#Var{0},&(1){#Atom{0},#Atom{1}}}','#Bind{#Var{0},#Atom{0}}'])+',#Nil)',[oracle([(0,'a')]),'#Fail{}']
def main():
 if sys.argv[-1]=='red':
  path=OUT/'red.hvm';path.write_text('@equal = λa.λb.λenv.0\n@main = @equal(#Var{0},#Var{0},#Nil)\n')
  r=native.invoke(BINARY,path,8,8192);(OUT/'red.json').write_text(json.dumps(r,indent=2)+'\n')
  assert r['code']==0 and r['stdout'].splitlines()==['0'],r
  print('RED: executable equality returns 0 for the same unknown; required result is 1.');return
 lib=Path(__file__).with_name('kernel.hvm').read_text()+'\n@after = λ{#Ok: λenv.λk.k(env); #Fail: λk.#Fail}\n';sources=list(cases())
 (OUT/'cases.json').write_text(json.dumps(sources,indent=2)+'\n')
 rows=[]
 with (OUT/'runs.jsonl').open('w') as log:
  for rep in range(2):
   for fuel in [1,8]:
    for name,expr,expected in sources:
     p=OUT/(name+'.hvm');p.write_text(lib+'\n@main = '+expr+'\n')
     r=native.invoke(BINARY,p,fuel,8192);row=dict(case=name,rep=rep,fuel=fuel,result=r);log.write(json.dumps(row)+'\n');log.flush()
     assert r.get('code')==0 and sorted(r['stdout'].splitlines())==sorted(expected),row
     ev=[json.loads(l) for l in r['stderr'].splitlines()];assert ev[-1]['pending']==0 and ev[-1]['unsupported']==0,row
     assert all(e['visits']<=fuel and e['stack']==1 for e in ev[:-1]),row
     rows.append(row)
 count=len(sources)*2
 for a,b in zip(rows[:count],rows[count:]):assert a['result']==b['result']
 diagnostics=[]
 for row in rows[:count]:
  p=OUT/(row['case']+'.hvm');full=row['result'];events=[json.loads(l) for l in full['stderr'].splitlines()]
  off=native.invoke(BINARY,p,row['fuel'],8192,off=True);oe=[json.loads(l) for l in off['stderr'].splitlines()]
  assert off['code']==0 and off['stdout']==full['stdout']
  assert [{k:v for k,v in e.items() if k!='delta'} for e in oe]==[{k:v for k,v in e.items() if k!='delta'} for e in events]
  assert all(e['delta']==0 for e in oe[:-1]);diagnostics.append(dict(case=row['case'],fuel=row['fuel'],kind='off',result=off))
  if row['fuel']==1:
   for calls in [0,1,16]:
    r=native.invoke(BINARY,p,1,calls);assert r['code']==0
    ev=[json.loads(l) for l in r['stderr'].splitlines()];n=min(calls,len(events)-1)
    assert ev[:-1]==events[:n] and ev[-1]['calls']==n
    assert ev[-1]['pending']==(events[n-1]['pending'] if n else 1)
    assert r['stdout'].splitlines()==full['stdout'].splitlines()[:len(r['stdout'].splitlines())]
    diagnostics.append(dict(case=row['case'],calls=calls,kind='cancel',result=r))
 with (OUT/'diagnostics.jsonl').open('w') as f:
  for row in diagnostics:f.write(json.dumps(row)+'\n')
 controls=[]
 for name,expr,expected in sources[-20:]:
  r=native.invoke(ROOT/'target/s03-native-service/baseline',OUT/(name+'.hvm'));assert r['code']==0 and sorted(r['stdout'].splitlines())==sorted(expected),(name,r)
  controls.append(dict(case=name,result=r))
 (OUT/'controls.json').write_text(json.dumps(controls,indent=2)+'\n')
 paths=[Path(__file__),Path(__file__).with_name('kernel.hvm'),Path(__file__).with_name('build_kernel.py'),BINARY,ROOT/'research/chr-hvm/structured/gate.py',ROOT/'research/chr-hvm/service/gate.py',ROOT/'target/s03-native-service/baseline',ROOT/'docs/experiments/registrations/S03-native-identity-kernel.md']
 (OUT/'validation.json').write_text(json.dumps(dict(cases=len(sources),runs=len(rows),diagnostics=len(diagnostics),controls=len(controls),hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}),indent=2)+'\n')
 print('Native identity operations pass:',len(sources),'cases',len(rows),'replays',len(diagnostics),'diagnostics')
if __name__=='__main__':main()
