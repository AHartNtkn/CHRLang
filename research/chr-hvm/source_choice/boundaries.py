"""Distinguish whole-source failure from exhausted choice identities."""
import copy,json
from gate import OUT,BINARY,ROOT,native,reference,source_only,normalize,parse
from compiler import compile_source
from cases import c,rule,add,eq,choice

def main():
 cases=[]
 for name,body in [('fail',[['fail']]),('both-fail',[choice([['fail']],[['fail']])]),('insert-then-conflict',[add('done',0),eq(0,'a'),eq(0,'b')])]:
  cases.append(dict(name='boundary-'+name,rules=[rule([],[c('go',0)],body)],query=[c('go',10)],outputs=[10],expected=[],ongoing=False))
 tail=[add('done',0)]
 for _ in range(24):tail=[choice([['fail']],tail)]
 cases.append(dict(name='boundary-choice-limit',rules=[rule([],[c('go',0)],tail)],query=[c('go',10)],outputs=[10],expected=[[[0],[c('done',0)]]],ongoing=False))
 rows=[]
 for source in cases:
  ref=reference(source);program,preds,atoms=compile_source(source_only(source));path=OUT/(source['name']+'.hvm');path.write_text(program)
  for fuel in [1,8]:
   r=native.invoke(BINARY,path,fuel,65536);assert r['code']==0;events=[json.loads(l) for l in r['stderr'].splitlines()]
   if source['name'].endswith('choice-limit'):
    assert r['stdout'].splitlines()==['#ChoiceLimit{}'] and events[-1]['pending']==0
    disposition='incomplete-choice-namespace'
   else:
    assert not r['stdout'] and events[-1]['pending']==0
    disposition='complete-source-failure'
   assert events[-1]['unsupported']==0 and all(e['visits']<=fuel and e['stack']==1 for e in events[:-1])
   rows.append(dict(source=source,reference=ref,fuel=fuel,result=r,disposition=disposition))
  if not source['name'].endswith('choice-limit'):
   r=native.invoke(ROOT/'target/s03-native-service/baseline',path);assert r['code']==0 and not r['stdout'];rows.append(dict(case=source['name'],kind='ordinary',result=r))
 (OUT/'boundaries.json').write_text(json.dumps(rows,indent=2)+'\n')
 base=source_only(cases[0]);rejections=[]
 for name,edit in [('guard',lambda s:s['rules'][0].update(guards=[])),('constructor',lambda s:s['query'][0][1].append(['f',0])),('malformed-choice',lambda s:s['rules'][0].update(body=[['or',[],42]])),('extra-semantics',lambda s:s.update(goal='fail'))]:
  s=copy.deepcopy(base);edit(s)
  try:compile_source(s)
  except ValueError:rejections.append(name)
  else:raise AssertionError(name)
 (OUT/'admission.json').write_text(json.dumps(rejections)+'\n');print('Three whole-source failures and explicit choice-namespace incompleteness verified at both quotas.')
if __name__=='__main__':main()
