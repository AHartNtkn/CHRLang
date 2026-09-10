"""Source obligations with directly specified complete observations."""
import itertools

def c(name,*args):return [name,list(args)]
def rule(kept,removed,body):return dict(kept=kept,removed=removed,body=body)
def add(name,*args):return ['add',c(name,*args)]
def bind(a,b):return ['eq',a,b]
def cases():
 watch=rule([c('p',0)],[],[add('seen',0)])
 for n,reverse in itertools.product(range(4),[False,True]):
  query=[c('p','a')]+[c('ticket')]*n
  yield dict(name=f'replace-{n}-{int(reverse)}',rules=[watch,rule([],[c('ticket'),c('p',0)],[add('p',0)])],query=query[::-1] if reverse else query,outputs=[],expected=[[],[c('p','a')]+[c('seen','a')]*(n+1)])
 for id,reverse in itertools.product([10,100,1000],[False,True]):
  query=[c('p',id),c('bind',id)]
  yield dict(name=f'bind-{id}-{int(reverse)}',rules=[watch,rule([],[c('bind',0)],[bind(0,'a')])],query=query[::-1] if reverse else query,outputs=[id],expected=[['a'],[c('p','a'),c('seen','a')]])
  query=[c('p',id),c('ticket')]
  yield dict(name=f'fresh-{id}-{int(reverse)}',rules=[watch,rule([],[c('ticket'),c('p',0)],[add('p',1)])],query=query[::-1] if reverse else query,outputs=[id],expected=[[0],[c('seen',0),c('seen',1),c('p',1)]])
 for n in range(4):
  yield dict(name=f'pairs-{n}',rules=[rule([c('p',0),c('p',1)],[],[add('seen',0,1)])],query=[c('p','a')]*n,outputs=[],expected=[[],[c('p','a')]*n+[c('seen','a','a')]*(n*(n-1))])
 yield dict(name='pairs-distinct',rules=[rule([c('p',0),c('p',1)],[],[add('seen',0,1)])],query=[c('p','a'),c('p','b')],outputs=[],expected=[[],[c('p','a'),c('p','b'),c('seen','a','b'),c('seen','b','a')]])
 for id,aliased,first,reverse in itertools.product([10,100,1000],[False,True],[False,True],[False,True]):
  other=id if aliased else id+1
  join=rule([],[c('p',0),c('q',0)],[add('joined',0)]);take=rule([],[c('p',0),c('token')],[add('taken',0)]);binder=rule([],[c('bind',0,1)],[bind(0,1)])
  query=[c('p',id),c('q',other),c('bind',id,other),c('token')]
  yield dict(name=f'order-{id}-{int(aliased)}-{int(first)}-{int(reverse)}',rules=[binder,join,take] if first else [join,take,binder],query=query[::-1] if reverse else query,outputs=[id,other],expected=[[0,0],[c('joined',0),c('token')] if aliased or first else [c('taken',0),c('q',0)]])
 for id,reverse_rules,reverse in itertools.product([10,100,1000],[False,True],[False,True]):
  rules=[rule([],[c('left',0)],[bind(0,'a'),add('doneleft',0)]),rule([],[c('right',0)],[bind(0,'b'),add('doneright',0)])];query=[c('left',id),c('right',id+1)]
  yield dict(name=f'commute-{id}-{int(reverse_rules)}-{int(reverse)}',rules=rules[::-1] if reverse_rules else rules,query=query[::-1] if reverse else query,outputs=[id,id+1],expected=[['a','b'],[c('doneleft','a'),c('doneright','b')]])

    # Compound source challenge: old/new occurrence histories meet late equality.
 for id,aliased,first,reverse in itertools.product([10,1000],[False,True],[False,True],[False,True]):
  other=id if aliased else id+1
  join=rule([],[c('p',0),c('q',0)],[add('joined',0)])
  replace=rule([],[c('ticket'),c('p',0)],[add('p',1),add('link',0,1)])
  link=rule([],[c('link',0,1)],[bind(0,1)])
  binder=rule([],[c('bind',0,1)],[bind(0,1)])
  rules=[watch,binder,join,replace,link] if first else [watch,join,replace,link,binder]
  query=[c('p',id),c('q',other),c('ticket'),c('bind',id,other)]
  early=aliased or first
  residual=[c('seen',0),c('joined',0)]+([c('ticket')] if early else [c('seen',0)])
  yield dict(name=f'compound-{id}-{int(aliased)}-{int(first)}-{int(reverse)}',rules=rules,query=query[::-1] if reverse else query,outputs=[id,other],expected=[[0,0],residual])
