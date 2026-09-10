"""Emit explicit native duplications for the identity operation definitions."""
import re
from pathlib import Path
def linear(body,args):
 prefix='';label=9000000
 for arg in args:
  pattern=r'(?<![@\w])'+re.escape(arg)+r'\b'
  count=len(re.findall(pattern,body));current=arg
  for _ in range(count-1):
   name=f'd{label}';prefix+=f'!{name}&({label})={current};'
   body=re.sub(pattern,name+'₀',body,count=1);current=name+'₁';label+=1
  if count:body=re.sub(pattern,current,body,count=1)
 return prefix+body
def lam(args,body):return ''.join('λ'+a+'.' for a in args)+linear(body,args)
def case(arms):return 'λ{'+'; '.join(tag+': '+lam(args,body) for tag,args,body in arms)+'}'
D={}
def fn(name,args,body):D[name]=lam(args,body)
D['choose']='λ{0: λa.λb.b; λn.λa.λb.a}'
fn('resolve',['term','env'],'@resolve_kind(term,env)')
D['resolve_kind']=case([('#Atom',['x','env'],'#Atom{x}'),('#Var',['x','env'],'@lookup(env,x,env)')])
D['lookup']=case([('#Nil',['x','all'],'#Var{x}'),('#Cons',['link','rest','x','all'],'@lookup_link(link,rest,x,all)')])
D['lookup_link']=case([('#Link',['key','value','rest','x','all'],'@choose((key==x),@resolve(value,all),@lookup(rest,x,all))')])
fn('equal',['a','b','env'],'@equal_roots(@resolve(a,env),@resolve(b,env))')
D['equal_roots']=case([('#Var',['x','b'],'@equal_var(b,x)'),('#Atom',['x','b'],'@equal_atom(b,x)')])
D['equal_var']=case([('#Var',['y','x'],'(x==y)'),('#Atom',['y','x'],'0')])
D['equal_atom']=case([('#Var',['y','x'],'0'),('#Atom',['y','x'],'(x==y)')])
fn('bind',['a','b','env'],'@bind_roots(@resolve(a,env),@resolve(b,env),env)')
D['bind_roots']=case([('#Var',['x','b','env'],'@bind_var(b,x,env)'),('#Atom',['x','b','env'],'@bind_atom(b,x,env)')])
D['bind_var']=case([('#Var',['y','x','env'],'@choose((x==y),#Ok{env},@choose((x<y),#Ok{#Cons{#Link{y,#Var{x}},env}},#Ok{#Cons{#Link{x,#Var{y}},env}}))'),('#Atom',['y','x','env'],'#Ok{#Cons{#Link{x,#Atom{y}},env}}')])
D['bind_atom']=case([('#Var',['y','x','env'],'#Ok{#Cons{#Link{y,#Atom{x}},env}}'),('#Atom',['y','x','env'],'@choose((x==y),#Ok{env},#Fail)')])
D['run']=case([('#Nil',['env'],'#Answer{#Cons{@resolve(#Var{0},env),#Cons{@resolve(#Var{1},env),#Cons{@resolve(#Var{2},env),#Cons{@resolve(#Var{3},env),#Nil}}}}}'),('#Cons',['op','rest','env'],'@action(op,rest,env)')])
D['action']=case([('#Bind',['a','b','rest','env'],'@resume(@bind(a,b,env),rest)')])
D['resume']=case([('#Fail',['rest'],'#Fail'),('#Ok',['env','rest'],'@run(rest,env)')])
D['same_list']=case([('#Nil',['b'],'@empty(b)'),('#Cons',['x','xs','b'],'@same_cons(b,x,xs)')])
D['empty']=case([('#Nil',[],'1'),('#Cons',['x','xs'],'0')])
D['same_cons']=case([('#Nil',['x','xs'],'0'),('#Cons',['y','ys','x','xs'],'@choose((x==y),@same_list(xs,ys),0)')])
fn('seen',['rule','ids','history'],'@history(history,rule,ids)')
D['history']=case([('#Nil',['rule','ids'],'0'),('#Cons',['receipt','rest','rule','ids'],'@receipt(receipt,rest,rule,ids)')])
D['receipt']=case([('#Receipt',['r','tuple','rest','rule','ids'],'@choose(((r==rule)&&@same_list(tuple,ids)),1,@history(rest,rule,ids))')])
fn('member',['x','xs'],'@member_list(xs,x)')
D['member_list']=case([('#Nil',['x'],'0'),('#Cons',['y','ys','x'],'@choose((x==y),1,@member_list(ys,x))')])
D['distinct']=case([('#Nil',[],'1'),('#Cons',['x','xs'],'@choose(@member(x,xs),0,@distinct(xs))')])
fn('remove',['id','store'],'@remove_list(store,id)')
D['remove_list']=case([('#Nil',['id'],'#Nil'),('#Cons',['occ','rest','id'],'@remove_occ(occ,rest,id)')])
D['remove_occ']=case([('#Occ',['key','value','rest','id'],'@choose((key==id),rest,#Cons{#Occ{key,value},@remove_list(rest,id)})')])
fn('add',['next','value','store'],'#Store{(next+1),#Cons{#Occ{next,value},store}}')
Path(__file__).with_name('kernel.hvm').write_text('\n'.join('@'+k+' = '+v for k,v in D.items())+'\n')
