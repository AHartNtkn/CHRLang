"""Generate native runtime matching for deterministic atom/unknown CHR source."""
import importlib.util,re,sys
from pathlib import Path
sys.dont_write_bytecode=True
spec=importlib.util.spec_from_file_location('emit',Path(__file__).parents[1]/'identity/build_kernel.py');emit=importlib.util.module_from_spec(spec);spec.loader.exec_module(emit)
def ls(xs):
 s='#Nil'
 for x in reversed(xs):s='#Cons{'+x+','+s+'}'
 return s
def compile_source(source):
 if set(source)!={'rules','query','outputs'}:raise ValueError('source admission')
 arities={};atoms=set()
 def term(x):
  if type(x) is int and 0<=x<1000000:return
  if isinstance(x,str) and re.fullmatch('[a-z][a-z0-9]*',x) and not x.startswith('v'):atoms.add(x);return
  raise ValueError('atom/unknown admission')
 def constraint(c):
  if not isinstance(c,list) or len(c)!=2 or not isinstance(c[0],str) or not re.fullmatch('[a-z][a-z0-9]*',c[0]) or not isinstance(c[1],list):raise ValueError('constraint admission')
  if c[0] in arities and arities[c[0]]!=len(c[1]):raise ValueError('arity')
  arities[c[0]]=len(c[1])
  for x in c[1]:term(x)
 for c in source['query']:constraint(c)
 for x in source['outputs']:
  if type(x) is not int:raise ValueError('output variable')
  term(x)
 for r in source['rules']:
  if set(r)!={'kept','removed','body'} or not r['kept']+r['removed']:raise ValueError('rule admission')
  for c in r['kept']+r['removed']:constraint(c)
  for a in r['body']:
   if not isinstance(a,list) or not a:raise ValueError('body admission')
   if a[0]=='add' and len(a)==2:constraint(a[1])
   elif a[0]=='eq' and len(a)==3:term(a[1]);term(a[2])
   else:raise ValueError('body admission')
 if len(source['query'])>=1000000:raise ValueError('occurrence namespace')
 predicates=sorted(arities);atoms=sorted(atoms);D={}
 def fn(n,args,body):D[n]=emit.lam(args,body)
 def case(n,arms):D[n]=emit.case(arms)
 def t(x):return f'#Var{{{x}}}' if type(x) is int else f'#Atom{{{atoms.index(x)}}}'
 def pat(c):return ls([t(x) for x in c[1]])
 # Pattern substitutions bind rule variables to terms without binding source unknowns.
 case('s_find',[('#Nil',['key'],'#Missing'),('#Cons',['link','rest','key'],'@s_find_link(link,rest,key)')])
 case('s_find_link',[('#Link',['id','value','rest','key'],'@choose((id==key),#Present{value},@s_find(rest,key))')])
 case('s_args',[('#Nil',['args','eqenv','sub'],'#Yes{sub}'),('#Cons',['p','ps','args','eqenv','sub'],'@s_arg_list(args,p,ps,eqenv,sub)')])
 case('s_arg_list',[('#Cons',['a','rest','p','ps','eqenv','sub'],'@s_arg_next(@s_pattern(p,a,eqenv,sub),ps,rest,eqenv)')])
 case('s_arg_next',[('#No',['ps','rest','eqenv'],'#No'),('#Yes',['sub','ps','rest','eqenv'],'@s_args(ps,rest,eqenv,sub)')])
 case('s_pattern',[('#Atom',['id','actual','eqenv','sub'],'@choose(@equal(#Atom{id},actual,eqenv),#Yes{sub},#No)'),('#Var',['id','actual','eqenv','sub'],'@s_pattern_var(@s_find(sub,id),id,actual,eqenv,sub)')])
 case('s_pattern_var',[('#Missing',['id','actual','eqenv','sub'],'#Yes{#Cons{#Link{id,actual},sub}}'),('#Present',['value','id','actual','eqenv','sub'],'@choose(@equal(value,actual,eqenv),#Yes{sub},#No)')])
 case('s_inst',[('#Atom',['id','sub'],'#Atom{id}'),('#Var',['id','sub'],'@s_required(@s_find(sub,id))')])
 case('s_required',[('#Present',['value'],'value')])
 case('s_values',[('#Nil',['sub'],'#Nil'),('#Cons',['a','rest','sub'],'#Cons{@s_inst(a,sub),@s_values(rest,sub)}')])
 case('s_remove',[('#Nil',['ids'],'#Nil'),('#Cons',['fact','rest','ids'],'@s_remove_fact(fact,rest,ids)')])
 case('s_remove_fact',[('#Fact',['id','pred','args','rest','ids'],'@choose(@member(id,ids),@s_remove(rest,ids),#Cons{#Fact{id,pred,args},@s_remove(rest,ids)})')])
 case('s_append',[('#Nil',['fact'],'#Cons{fact,#Nil}'),('#Cons',['head','rest','fact'],'#Cons{head,@s_append(rest,fact)}')])
 case('s_resolve_values',[('#Nil',['eqenv'],'#Nil'),('#Cons',['a','rest','eqenv'],'#Cons{@resolve(a,eqenv),@s_resolve_values(rest,eqenv)}')])
 case('s_observe',[('#Nil',['eqenv'],'#Nil'),('#Cons',['fact','rest','eqenv'],'@s_observe_fact(fact,rest,eqenv)')])
 case('s_observe_fact',[('#Fact',['id','pred','args','rest','eqenv'],'#Cons{#Result{pred,@s_resolve_values(args,eqenv)},@s_observe(rest,eqenv)}')])
 stateargs=['store','eqenv','hist','next','fresh','outputs']
 state='#State{'+','.join(stateargs)+'}'
 # Rule selectors and per-head scans are generated from source structure, not results.
 for ri,r in enumerate(source['rules']):
  heads=r['kept']+r['removed'];n=len(heads);prefix=f's_r{ri}'
  case(prefix,[('#State',stateargs,f'@{prefix}_decide(@{prefix}_h0(store,store,eqenv,hist,#Nil,#Nil,#Nil),{state})')])
  headvars={x for _,args in heads for x in args if type(x)is int};bodyvars={x for a in r['body'] for x in (a[1][1] if a[0]=='add' else a[1:]) if type(x)is int};freshvars=sorted(bodyvars-headvars)
  sub='sub'
  for offset,x in enumerate(freshvars):sub=f'#Cons{{#Link{{{x},#Var{{(fresh+{offset})}}}},{sub}}}'
  history=f'#Cons{{#Receipt{{{ri},ids}},hist}}' if not r['removed'] else 'hist'
  bodycall=f'@{prefix}_body0(sub,store,eqenv,hist,next,fresh,outputs)'
  case(prefix+'_decide',[('#No',['state'],f'@s_r{ri+1}(state)'),('#Hit',['sub','ids','removed','state'],f'@{prefix}_fire(state,sub,ids,removed)')])
  case(prefix+'_fire',[('#State',stateargs+['sub','ids','removed'],f'@choose(((next<1000000)&&((fresh+{len(freshvars)})<1000000)),@{prefix}_body0({sub},@s_remove(store,removed),eqenv,{history},next,(fresh+{len(freshvars)}),outputs),#Limit)')])
  common=['store','eqenv','hist','ids','removed','sub']
  for j,c in enumerate(heads):
   name=f'{prefix}_h{j}';call=lambda todo:f'@{name}('+','.join([todo]+common)+')'
   case(name,[('#Nil',common,'#No'),('#Cons',['fact','rest']+common,f'@{name}_fact('+','.join(['fact','rest']+common)+')')])
   args=['id','pred','args','rest']+common
   matched=f'@{name}_matched(@s_args({pat(c)},args,eqenv,sub),'+','.join(['id','rest']+common)+')'
   case(name+'_fact',[('#Fact',args,f'@choose(((pred=={predicates.index(c[0])})&&(1-@member(id,ids))),{matched},{call("rest")})')])
   nextids='#Cons{id,ids}';nextremoved='#Cons{id,removed}' if j>=len(r['kept']) else 'removed'
   nextcall=f'@{prefix}_h{j+1}(store,store,eqenv,hist,{nextids},{nextremoved},newsub)'
   fallback=call('rest')
   # No match or failed deeper extension continues the current candidate scan.
   case(name+'_matched',[('#No',['id','rest']+common,fallback),('#Yes',['newsub','id','rest']+common,f'@{name}_retry({nextcall},'+','.join(['rest']+common)+')')])
   case(name+'_retry',[('#No',['rest']+common,fallback),('#Hit',['foundsub','foundids','foundremoved','rest']+common,'#Hit{foundsub,foundids,foundremoved}')])
  terminal=f'#Hit{{sub,ids,removed}}'
  if not r['removed']:terminal=f'@choose(@seen({ri},ids,hist),#No,{terminal})'
  fn(prefix+f'_h{n}',['todo']+common,terminal)
  bodyargs=['sub']+stateargs
  for bi,a in enumerate(r['body']):
   name=prefix+f'_body{bi}';following=prefix+f'_body{bi+1}'
   if a[0]=='add':
    c=a[1];newstore=f'@s_append(store,#Fact{{next,{predicates.index(c[0])},@s_values({pat(c)},sub)}})'
    fn(name,bodyargs,f'@choose((next<1000000),@{following}(sub,{newstore},eqenv,hist,(next+1),fresh,outputs),#Limit)')
   else:
    fn(name,bodyargs,f'@{name}_bound(@bind(@s_inst({t(a[1])},sub),@s_inst({t(a[2])},sub),eqenv),'+','.join(bodyargs)+')')
    case(name+'_bound',[('#Fail',bodyargs,'&{}'),('#Ok',['boundenv']+bodyargs,f'@{following}(sub,store,boundenv,hist,next,fresh,outputs)')])
  fn(prefix+f'_body{len(r["body"])}',bodyargs,f'@s_r0({state})')
 case(f's_r{len(source["rules"])}',[('#State',stateargs,'#Answer{@s_resolve_values(outputs,eqenv),@s_observe(store,eqenv)}')])
 allvars=[x for _,args in source['query'] for x in args if type(x)is int]+source['outputs'];fresh=max(allvars,default=-1)+1
 query=ls([f'#Fact{{{i},{predicates.index(c[0])},{pat(c)}}}' for i,c in enumerate(source['query'])]);outputs=ls([t(x) for x in source['outputs']])
 program=(Path(__file__).parents[1]/'identity/kernel.hvm').read_text()+'\n'+'\n'.join('@'+name+' = '+body for name,body in D.items())+f'\n@main = @s_r0(#State{{{query},#Nil,#Nil,{len(source["query"])},{fresh},{outputs}}})\n'
 return program,predicates,atoms
