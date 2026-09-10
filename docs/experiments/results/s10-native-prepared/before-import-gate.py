"""Prepared native rules with runtime query construction and poisoned disposal."""
import copy,hashlib,importlib.util,json,resource,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s10-native-prepared';BINARY=ROOT/'target/s10-native-prepared/prepared'
sys.path.insert(0,str(Path(__file__).parents[1]/'source_choice'))
from compiler import compile_source
from gate import normalize,parse

def bounds():
 resource.setrlimit(resource.RLIMIT_AS,(96<<30,96<<30));resource.setrlimit(resource.RLIMIT_CPU,(10,10))
def encode(source,preds,atoms,limit,keep):
 preds=preds+sorted({c[0] for c in source['query']}-set(preds))
 atoms=atoms+sorted({x for _,args in source['query'] for x in args if isinstance(x,str)}-set(atoms))
 variables=source['outputs']+[x for _,args in source['query'] for x in args if type(x)is int]
 words=[str(limit),str(int(keep)),str(len(source['outputs'])),str(len(source['query'])),str(max(variables,default=-1)+1)]+list(map(str,source['outputs']))
 for name,args in source['query']:
  words += [str(preds.index(name)),str(len(args))]
  for x in args:words += ['V',str(x)] if type(x)is int else ['A',str(atoms.index(x))]
 return ' '.join(words),preds,atoms

def output_records(stdout):
 lines=stdout.splitlines(keepends=True);result={}
 while lines:
  header=lines.pop(0).split();assert header[0]=='RESULT' and len(header)==3
  index,size=map(int,header[1:]);data=''
  while len(data.encode())<size:data+=lines.pop(0)
  assert len(data.encode())==size and index not in result;result[index]=data
 return result

def main():
 groups=json.loads((ROOT/'docs/experiments/results/s10-native-common-source/groups.json').read_text());plans=[]
 for i,group in enumerate(groups):
  program,preds,atoms=compile_source(dict(rules=group[0]['rules'],query=[],outputs=[]));path=OUT/f'rules-{i}.hvm';path.write_text(program)
  if sys.argv[-1]=='red':
   p=subprocess.run([str(BINARY),str(path),'red'],capture_output=True,text=True,timeout=15,preexec_fn=bounds);assert p.returncode==0,p.stderr
   record=json.loads(p.stdout);assert record['remaining']>0;(OUT/'red.json').write_text(json.dumps(dict(result=record,stderr=p.stderr),indent=2)+'\n');print('RED: process-oriented runtime_free leaves',record['remaining'],'tracked bytes owned by names/parser paths.');return
  queries=[]
  for round in range(2):
   for s in group:queries.append(dict(source=s,limit=65536,keep=round==0,kind='complete'))
  changed=copy.deepcopy(group[0]);changed['name']+='-new-symbols'
  occupied={c[0] for s in group for c in s['query']}|{h[0] for r in group[0]['rules'] for h in r['kept']+r['removed']};extra='zzunused'
  while extra in occupied:extra+='x'
  occupied_vars=[x for _,args in changed['query'] for x in args if type(x)is int]+changed['outputs'];var=max(occupied_vars,default=0)+17
  changed['query'].append([extra,['zznewatom',var]]);changed['outputs'].append(var)
  expected=[]
  for outputs,residual in changed['expected']:
   existing=[x for x in outputs if type(x)is int]+[x for _,args in residual for x in args if type(x)is int];new=max(existing,default=-1)+1
   expected.append([outputs+[new],residual+[[extra,['zznewatom',new]]]])
  changed['expected']=expected
  queries.append(dict(source=changed,limit=65536,keep=True,kind='complete'))
  for limit in [0,1,64,8192]:queries.append(dict(source=group[0],limit=limit,keep=True,kind='cancel'))
  queries.append(dict(source=group[0],limit=65536,keep=False,kind='complete'))
  entries=[];lines=[]
  for q in queries:
   encoded,ps,ats=encode(q['source'],preds,atoms,q['limit'],q['keep']);lines.append(encoded);entries.append(dict(q,predicates=ps,atoms=ats))
  text=str(len(lines))+'\n'+'\n'.join(lines)+'\n';plans.append(dict(group=i,program=str(path.relative_to(ROOT)),prepared_predicates=preds,prepared_atoms=atoms,queries=entries,input=text))
 (OUT/'plans.json').write_text(json.dumps(plans,indent=2)+'\n');records=[]
 with (OUT/'runs.jsonl').open('w') as log:
  for plan in plans:
   p=subprocess.run([str(BINARY),str(ROOT/plan['program'])],input=plan['input'],capture_output=True,text=True,timeout=15,preexec_fn=bounds)
   row=dict(group=plan['group'],code=p.returncode,stdout=p.stdout,stderr=p.stderr);log.write(json.dumps(row)+'\n');log.flush();assert p.returncode==0,(plan['group'],p.stderr[-2000:])
   values=output_records(p.stdout);events=[json.loads(l) for l in p.stderr.splitlines()];assert len(events)==len(plan['queries'])+1 and events[-1]['session_disposed'] and events[-1]['tracked_live']==0
   assert set(values)==set(range(len(plan['queries'])))
   for i,q in enumerate(plan['queries']):
    actual=[normalize(parse(line,q['predicates'],q['atoms'])) for line in values[i].splitlines()];s=q['source'];event=events[i]
    assert event['query']==i and event['prepared_unchanged'] and event['query_restored'] and event['unsupported']==0
    if q['kind']=='complete':
     assert sorted(actual)==sorted(map(normalize,s['expected'])),(plan['group'],i,s['name'])
     assert bool(event['pending'])==s['ongoing'],(plan['group'],i)
    else:
     # Cancellation may publish only a prefix; preserve raw multiplicities.
     full=[normalize(parse(line,q['predicates'],q['atoms'])) for line in values[0].splitlines()]
     assert actual==full[:len(actual)] and event['calls']<=q['limit']
   records.append(row);print('group',plan['group'],'prepared reuse and disposal pass',flush=True)
 inputs=[Path(__file__),Path(__file__).with_name('harness.c'),Path(__file__).with_name('build.py'),BINARY,ROOT/'target/s10-native-prepared/native.c',ROOT/'research/chr-hvm/source_choice/compiler.py',ROOT/'docs/experiments/registrations/S10-native-prepared.md']+[ROOT/p['program'] for p in plans]
 (OUT/'validation.json').write_text(json.dumps(dict(groups=len(plans),queries=sum(len(p['queries']) for p in plans),processes=len(records),hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs}),indent=2)+'\n');print('All prepared native groups pass.')
if __name__=='__main__':main()
