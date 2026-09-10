"""Audit prepared definitions, runtime query encodings, observations and ownership."""
import collections,hashlib,json
from gate import ROOT,OUT,BINARY,compile_source,encode,normalize,parse,output_records

def main():
 v=json.loads((OUT/'validation.json').read_text())
 for path,h in v['hashes'].items():assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==h,path
 original=ROOT/'target/s03-native-identity-source/native.c';assert original.read_bytes()==(ROOT/'target/s10-native-prepared/native.c').read_bytes()
 plans=json.loads((OUT/'plans.json').read_text());common=json.loads((ROOT/'docs/experiments/results/s10-native-common-source/groups.json').read_text());assert len(plans)==len(common)==22
 rows=[json.loads(l) for l in (OUT/'runs.jsonl').read_text().splitlines()];assert len(rows)==22
 peak=0;prepared=[];complete=0;cancel=0;newnames=0
 for plan,group,row in zip(plans,common,rows):
  assert plan['group']==row['group'];program,preds,atoms=compile_source(dict(rules=group[0]['rules'],query=[],outputs=[]))
  assert program==(ROOT/plan['program']).read_text() and plan['prepared_predicates']==preds and plan['prepared_atoms']==atoms
  lines=[]
  for q in plan['queries']:
   line,ps,ats=encode(q['source'],preds,atoms,q['limit'],q['keep']);assert q['predicates']==ps and q['atoms']==ats
   assert ps[:len(preds)]==preds and ats[:len(atoms)]==atoms;lines.append(line)
  assert plan['input']==str(len(lines))+'\n'+'\n'.join(lines)+'\n'
  assert row['code']==0;results=output_records(row['stdout']);events=[json.loads(l) for l in row['stderr'].splitlines()]
  assert set(results)==set(range(len(plan['queries']))) and len(events)==len(plan['queries'])+1
  for i,q in enumerate(plan['queries']):
   e=events[i];assert e['query']==i and e['prepared_unchanged'] and e['query_restored'] and e['unsupported']==0 and e['calls']<=q['limit']
   assert e['retained_bytes']==len(results[i].encode())+1
   actual=[normalize(parse(l,q['predicates'],q['atoms'])) for l in results[i].splitlines()]
   if q['kind']=='complete':
    assert sorted(actual)==sorted(map(normalize,q['source']['expected'])) and bool(e['pending'])==q['source']['ongoing'];complete+=1
    if q['source']['name'].endswith(('new-symbols','active-new-atom')):newnames+=1
   else:
    assert q['kind']=='cancel';full=[normalize(parse(l,q['predicates'],q['atoms'])) for l in results[0].splitlines()];assert actual==full[:len(actual)];cancel+=1
  final=events[-1];assert final['session_disposed'] and final['consumer_survives_prepared_drop'] and final['tracked_live']==0
  peak=max(peak,final['peak_dynamic_words']);prepared.append(final['prepared_words'])
 assert (complete,cancel,newnames)==(295,88,23)
 oldplans=json.loads((OUT/'before-symbol-plans.json').read_text());oldrows=[json.loads(l) for l in (OUT/'before-symbol-runs.jsonl').read_text().splitlines()];replays=0
 def indexed(plan,row):
  counts=collections.Counter();out={};values=output_records(row['stdout']);events=[json.loads(l) for l in row['stderr'].splitlines()]
  for i,q in enumerate(plan['queries']):
   key=(q['source']['name'],q['kind'],q['limit'],q['keep']);counts[key]+=1
   out[key+(counts[key],)]=(values[i],{k:v for k,v in events[i].items() if k!='query'})
  return out
 for op,orr,np,nr in zip(oldplans,oldrows,plans,rows):
  old=indexed(op,orr);new=indexed(np,nr)
  for key,value in old.items():assert new[key]==value;replays+=1
 assert replays==382
 multi=json.loads((OUT/'multi-session.json').read_text());assert multi['code']==0 and multi['groups']==[0,21,0]
 parts=multi['stdout'].split('SESSION ');assert len(parts)==4 and parts[0]==''
 for i,(part,g) in enumerate(zip(parts[1:],multi['groups'])):number,body=part.split('\n',1);assert int(number)==i and body==rows[g]['stdout']
 assert multi['stderr']==''.join(rows[g]['stderr'] for g in multi['groups'])
 red=json.loads((OUT/'red.json').read_text())['result'];assert red['remaining']==red['name_path_bytes']==1314
 ubsan=json.loads((OUT/'ubsan.json').read_text());assert ubsan['groups']==22 and ubsan['query_replays']==383 and ubsan['results']==rows
 supplement=json.loads((OUT/'supplement-hashes.json').read_text())
 for path,h in supplement.items():assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==h,path
 symbolrefs=json.loads((OUT/'symbol-references.json').read_text());assert len(symbolrefs)==23
 result=dict(groups=22,query_runs=383,complete_queries=complete,cancellations=cancel,reference_checked_symbol_queries=newnames,old_exact_query_replays=replays,same_process_sessions=3,same_process_queries=sum(len(plans[g]['queries']) for g in multi['groups']),prepared_words_min=min(prepared),prepared_words_max=max(prepared),peak_dynamic_words=peak,red_name_path_bytes=red['remaining'],tracked_final_live=0,sanitized_exact_query_replays=383)
 (OUT/'audit.json').write_text(json.dumps(result,indent=2)+'\n');print(result)
if __name__=='__main__':main()
