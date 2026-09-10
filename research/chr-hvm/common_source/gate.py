"""Read existing source receipts; qualify prepared Rust paths on changing queries."""
import collections,hashlib,itertools,json,resource,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s10-native-common-source';BINARY=ROOT/'target/debug/examples/native_common_source'
MODES=['scan','indexed','special-scan','special-indexed','context','shared','persistent','persistent-shared','resumable','conditional','prefix','finite']
def normalize(answer):
 outputs,residual=answer;variables=sorted({x for x in outputs if isinstance(x,int)}|{x for _,args in residual for x in args if isinstance(x,int)});encodings=[]
 for permutation in itertools.permutations(range(len(variables))):
  mapping=dict(zip(variables,permutation))
  def t(x):return 'v'+str(mapping[x]) if isinstance(x,int) else 'a'+x
  encodings.append((tuple(map(t,outputs)),tuple(sorted((name,tuple(map(t,args))) for name,args in residual))))
 return min(encodings)
def sources():
 result=[]
 for family,folder in [('choice','s03-native-choice-identity'),('det','s03-native-identity-source')]:
  root=ROOT/'docs/experiments/results'/folder;refs={r['case']:r['result']['input'] for r in json.loads((root/'references.json').read_text())}
  for s in json.loads((root/'cases.json').read_text()):
   result.append(dict(s,name=family+'-'+s['name'],expected=s['expected'] if family=='choice' else [s['expected']],ongoing=s.get('ongoing',False),input=refs[s['name']]))
 return result
def parse(stdout):
 lines=stdout.splitlines();queries=[]
 def term(x):return int(x[1:]) if x.startswith('v') else x
 while lines:
  header=lines.pop(0).split();assert header[0]=='QUERY' and len(header)==4
  index=int(header[1]);answers=[]
  while lines and not lines[0].startswith('QUERY '):
   assert lines.pop(0)=='ANSWER';out=lines.pop(0);residual=[]
   while lines and lines[0]!='ANSWER' and not lines[0].startswith('QUERY '):
    parts=lines.pop(0).split(':');residual.append([parts[0],list(map(term,parts[1:]))])
   answers.append([list(map(term,out.split(','))) if out else [],residual])
  assert int(header[3])==len(answers)
  queries.append(dict(index=index,exhausted=header[2]=='true',answers=answers))
 return queries
def bounds():
 resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(10,10))
def main():
 groups={}
 for source in sources():groups.setdefault(json.dumps(source['rules'],sort_keys=True),[]).append(source)
 groups=list(groups.values());(OUT/'groups.json').write_text(json.dumps(groups,indent=2)+'\n');records=[];results=[]
 with (OUT/'runs.jsonl').open('w') as log:
  for group_id,group in enumerate(groups):
   text=group[0]['input'].rstrip('\n')
   for s in group[1:]:text+='\nNEXT\n'+'\n'.join(s['input'].splitlines()[:2])
   text+='\n'
   for mode,label in enumerate(MODES):
    try:
     p=subprocess.run([str(BINARY),str(mode)],input=text,text=True,capture_output=True,timeout=15,preexec_fn=bounds)
     row=dict(group=group_id,mode=label,code=p.returncode,stdout=p.stdout,stderr=p.stderr)
    except subprocess.TimeoutExpired as e:row=dict(group=group_id,mode=label,timeout=True,stdout=str(e.stdout),stderr=str(e.stderr))
    log.write(json.dumps(row)+'\n');log.flush();records.append(row)
    if row.get('code')!=0:
     results.extend(dict(case=s['name'],mode=label,status='process-failure') for s in group);continue
    if row['stdout'].startswith('UNSUPPORTED '):
     assert label in ['prefix','finite']
     results.extend(dict(case=s['name'],mode=label,status='unsupported',reason=row['stdout'].strip()) for s in group);continue
    actual=parse(row['stdout']);assert len(actual)==len(group),(group_id,label)
    for index,(s,a) in enumerate(zip(group,actual)):
     assert a['index']==index
     answers=sorted(map(normalize,a['answers']))==sorted(map(normalize,s['expected']));exhaustion=a['exhausted']==(not s['ongoing'])
     results.append(dict(case=s['name'],mode=label,status='pass' if answers and exhaustion else 'mismatch',answers_match=answers,exhaustion_matches=exhaustion,actual=a))
   print('group',group_id+1,'of',len(groups),'complete',flush=True)
 (OUT/'results.json').write_text(json.dumps(results,indent=2)+'\n')
 inputs=[Path(__file__),BINARY,ROOT/'research/chr-direct-conditional/examples/native_common_source.rs',ROOT/'research/chr-direct-conditional/tests/composition_support/mod.rs',ROOT/'research/chr-direct-conditional/examples/support/finite_bridge.rs',ROOT/'research/chr-compiled/experiments/finite_phase.rs',ROOT/'docs/experiments/registrations/S10-native-common-source.md']
 inputs+=list((ROOT/'research/chr-compiled/src').glob('*.rs'))+list((ROOT/'research/chr-direct-conditional/src').glob('*.rs'))+list((ROOT/'research/chr-relational/src').glob('*.rs'))
 (OUT/'validation.json').write_text(json.dumps(dict(groups=len(groups),processes=len(records),comparisons=len(results),hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs}),indent=2)+'\n')
 for mode in MODES:print(mode,dict(collections.Counter(r['status'] for r in results if r['mode']==mode)))
if __name__=='__main__':main()
