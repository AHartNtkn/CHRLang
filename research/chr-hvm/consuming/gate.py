"""Ground source correspondence: occurrence oracle, reference and native compiler."""
import hashlib,importlib.util,itertools,json,subprocess,sys
from collections import Counter,deque
from pathlib import Path
sys.dont_write_bytecode=True
from compiler import compile_source
ROOT=Path(__file__).resolve().parents[3]
spec=importlib.util.spec_from_file_location('structured_gate',Path(__file__).parents[1]/'structured/gate.py')
native=importlib.util.module_from_spec(spec);spec.loader.exec_module(native)
OUT=ROOT/'docs/experiments/results/s03-native-consuming'
BINARY=ROOT/'target/s03-native-consuming/native'
REFERENCE=ROOT/'target/debug/examples/native_ground_reference'
def rule(removed,*arms,kept=()):return dict(kept=list(kept),removed=list(removed),alternatives=[None if a is None else list(a) for a in arms])
def cases():
    families=[
      ('compete',[rule(['a','token'],['oa']),rule(['b','token'],['ob'])],['a','b']),
      ('compete-reverse',[rule(['b','token'],['ob']),rule(['a','token'],['oa'])],['a','b']),
      ('repeated',[rule(['token','token'],['pair'])],[]),
      ('kept-overlap',[rule(['token'],['seen'],kept=['token'])],[]),
      ('watch',[rule(['watch'],['seen'],kept=['token']),rule(['a','token'],['oa'])],['watch','a']),
      ('binary',[rule(['go'],['a'],['b']),rule(['a','token'],['oa']),rule(['b','token'],['ob'])],['go']),
      ('duplicate',[rule(['go'],['a'],['a']),rule(['a','token'],['oa'])],['go']),
      ('nested',[rule(['go'],['again'],['a']),rule(['again'],['b'],['c']),rule(['a','token'],['oa']),rule(['b','token'],['ob']),rule(['c','token'],['oc'])],['go']),
      ('failure',[rule(['go'],['a'],['b']),rule(['a','token'],['oa']),rule(['b','token'],None)],['go']),
      ('loop',[rule(['go'],['loop'],['a']),rule(['loop'],['loop']),rule(['a','token'],['oa'])],['go']),
    ]
    for family,rules,base in families:
      predicates=sorted(set(base+['token']+[p for r in rules for p in r['kept']+r['removed']+sum([a or [] for a in r['alternatives']],[])]))
      for n in range(4):
       for reverse in [False,True]:
        query=base+['token']*n
        if reverse:query.reverse()
        yield dict(name=f'{family}-{n}-{int(reverse)}',predicates=predicates,rules=rules,query=query)
def occurrence_oracle(source):
    # Distinct list positions are actual occurrences; no count-based matching.
    queue=deque([(source['query'],frozenset())]);answers=[];ongoing=False
    while queue:
      store,ancestors=queue.popleft();signature=tuple(sorted(store))
      selected=None
      for r in source['rules']:
       heads=r['kept']+r['removed']
       for indices in itertools.permutations(range(len(store)),len(heads)):
        if all(store[j]==p for j,p in zip(indices,heads)):
         selected=(r,indices);break
       if selected:break
      if not selected:
       answers.append(signature);continue
      r,indices=selected
      if signature in ancestors:
       assert len(r['alternatives'])==1 and r['alternatives'][0] is not None
       assert Counter(r['alternatives'][0])==Counter(r['removed'])
       ongoing=True;continue
      consumed=set(indices[len(r['kept']):])
      remaining=[p for i,p in enumerate(store) if i not in consumed]
      for arm in r['alternatives']:
       if arm is not None:queue.append((remaining+arm,ancestors|{signature}))
    return sorted(answers),ongoing
def reference(source):
    def names(xs):return ','.join(xs) or '-'
    text=names(source['query'])+'\n'
    for r in source['rules']:text+=names(r['kept'])+';'+names(r['removed'])+';'+'|'.join('!' if a is None else names(a) for a in r['alternatives'])+'\n'
    p=subprocess.run([str(REFERENCE)],input=text,text=True,capture_output=True,timeout=5);assert p.returncode==0,p.stderr
    lines=p.stdout.splitlines();done,raw=lines[0].split()
    return dict(exhausted=done=='true',raw=int(raw),answers=sorted(tuple(l.split(',')) if l else () for l in lines[1:]),source=text,stdout=p.stdout)
def answer(counts):
    result='#Nil{}'
    for n in reversed(counts):result=f'#Cons{{{n},{result}}}'
    return '#Answer{'+result+'}'
def main():
    sources=list(cases());OUT.mkdir(parents=True,exist_ok=True)
    frozen=ROOT/'target/s03-native-structured/observer.c'
    old=json.loads((ROOT/'docs/experiments/results/s03-native-structured/validation.json').read_text())
    assert hashlib.sha256(frozen.read_bytes()).hexdigest()==old['hashes'][str(frozen.relative_to(ROOT))]
    text=frozen.read_text();assert text.count('limit>1024')==1
    derived=BINARY.with_suffix('.c');derived.parent.mkdir(parents=True,exist_ok=True);derived.write_text(text.replace('limit>1024','limit>8192'))
    subprocess.run(['clang','-O2',str(derived),'-o',str(BINARY)],check=True,timeout=60)
    subprocess.run(['cargo','build','-p','chr-cases','--example','native_ground_reference'],cwd=ROOT,check=True,timeout=60)
    rows=[];references=[]
    for source in sources:
      expected,ongoing=occurrence_oracle(source);ref=reference(source)
      assert ref['answers']==sorted(set(expected)) and ref['raw']==len(expected) and ref['exhausted']==(not ongoing),(source,ref,expected)
      references.append(dict(case=source['name'],reference=ref))
      source['expected']=[answer([Counter(a)[p] for p in source['predicates']]) for a in expected];source['ongoing']=ongoing
      path=OUT/(source['name']+'.hvm');path.write_text(compile_source({k:source[k] for k in ['predicates','rules','query']}))
    (OUT/'sources.json').write_text(json.dumps(sources,indent=2)+'\n');(OUT/'references.json').write_text(json.dumps(references,indent=2)+'\n')
    with (OUT/'runs.jsonl').open('w') as log:
     for repetition in range(2):
      for fuel in [1,8]:
       for source in sources:
        r=native.invoke(BINARY,OUT/(source['name']+'.hvm'),fuel,4096)
        row=dict(case=source['name'],fuel=fuel,repetition=repetition,result=r);log.write(json.dumps(row)+'\n');log.flush()
        assert r.get('code')==0,row
        events=[json.loads(l) for l in r['stderr'].splitlines()]
        assert sorted(r['stdout'].splitlines())==sorted(source['expected']),row
        assert bool(events[-1]['pending'])==source['ongoing'] and events[-1]['unsupported']==0,row
        assert all(e['visits']<=fuel and e['stack']==1 for e in events[:-1]),row
        rows.append(row)
    count=len(sources)*2
    for i in range(count):assert rows[i]['result']==rows[i+count]['result']
    off=[];cancellations=[];controls=[]
    for i,row in enumerate(rows[:count]):
      path=OUT/(row['case']+'.hvm')
      r=native.invoke(BINARY,path,row['fuel'],4096,off=True)
      assert r['code']==0 and r['stdout']==row['result']['stdout'],row['case']
      ev=[json.loads(l) for l in r['stderr'].splitlines()]
      full=[json.loads(l) for l in row['result']['stderr'].splitlines()]
      assert [{k:v for k,v in e.items() if k!='delta'} for e in ev]==[{k:v for k,v in e.items() if k!='delta'} for e in full]
      assert all(e['delta']==0 for e in ev[:-1])
      off.append(dict(case=row['case'],fuel=row['fuel'],result=r))
      if row['fuel']==1:
       for calls in [0,1,4,16]:
        r=native.invoke(BINARY,path,1,calls);assert r['code']==0
        ev=[json.loads(l) for l in r['stderr'].splitlines()]
        length=min(calls,len(full)-1);assert ev[:-1]==full[:length] and ev[-1]['calls']==length
        assert r['stdout'].splitlines()==row['result']['stdout'].splitlines()[:len(r['stdout'].splitlines())]
        assert ev[-1]['pending']==(full[length-1]['pending'] if length else 1)
        cancellations.append(dict(case=row['case'],calls=calls,result=r))
    for source in sources:
      if source['ongoing']:continue
      r=native.invoke(ROOT/'target/s03-native-service/baseline',OUT/(source['name']+'.hvm'))
      assert r['code']==0 and sorted(r['stdout'].splitlines())==sorted(source['expected']),source['name']
      controls.append(dict(case=source['name'],result=r))
    for name,data in [('counter-off',off),('cancellation',cancellations),('controls',controls)]:
      with (OUT/(name+'.jsonl')).open('w') as f:
       for row in data:f.write(json.dumps(row)+'\n')
    # Reject semantic fields rather than silently ignoring them.
    import copy
    base={k:sources[0][k] for k in ['predicates','rules','query']};rejected=[]
    bad=[]
    v=copy.deepcopy(base);v['rules'][0]['removed']=[];bad.append(('propagation',v))
    v=copy.deepcopy(base);v['rules'][0]['guards']=['false'];bad.append(('guard',v))
    v=copy.deepcopy(base);v['rules'][0]['alternatives']=[['a']*3];bad.append(('growth',v))
    v=copy.deepcopy(base);v['query']=['unknown'];bad.append(('unknown-predicate',v))
    v=copy.deepcopy(base);v['query']=['a']*65;bad.append(('query-bound',v))
    for name,value in bad:
      try:compile_source(value)
      except ValueError:rejected.append(name)
      else:raise AssertionError(name)
    (OUT/'admission.json').write_text(json.dumps(rejected)+'\n')
    inputs=[Path(__file__),Path(__file__).with_name('compiler.py'),ROOT/'research/chr-cases/examples/native_ground_reference.rs',REFERENCE,BINARY,derived,frozen,ROOT/'research/chr-hvm/structured/gate.py',ROOT/'research/chr-hvm/service/gate.py',ROOT/'target/s03-native-service/baseline',*(OUT/(s['name']+'.hvm') for s in sources)]
    (OUT/'validation.json').write_text(json.dumps(dict(sources=len(sources),native_replays=len(rows),counter_off=len(off),cancellations=len(cancellations),controls=len(controls),reference_runs=len(references),admission=rejected,hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs}),indent=2)+'\n')
    print('Native/reference/occurrence correspondence, replay, diagnostic and cancellation gates pass.',len(rows))
if __name__=='__main__':main()
