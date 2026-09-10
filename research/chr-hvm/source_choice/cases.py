"""Complete source expectations for choices interacting with identity and effects."""
import itertools

def c(name,*args):return [name,list(args)]
def rule(kept,removed,body):return dict(kept=kept,removed=removed,body=body)
def add(name,*args):return ['add',c(name,*args)]
def eq(a,b):return ['eq',a,b]
def choice(left,right):return ['or',left,right]
def cases():
 for id,reverse in itertools.product([10,1000],[False,True]):
  watch=rule([c('p',0)],[],[add('seen',0)])
  binding=rule([],[c('go',0)],[choice([eq(0,'a')],[eq(0,'b')])])
  families=[]
  families.append(('binding',[watch,binding],[c('p',id),c('go',id)],[id],[[[a],[c('p',a),c('seen',a)]] for a in ['a','b']],False))
  correlated=rule([],[c('go',0,1)],[choice([eq(0,'a'),eq(1,'a')],[eq(0,'b'),eq(1,'b')]),add('pair',0,1)])
  families.append(('correlation',[correlated],[c('go',id,id+1)],[id,id+1],[[[a,a],[c('pair',a,a)]] for a in ['a','b']],False))
  families.append(('independent',[binding],[c('go',id),c('go',id+1)],[id,id+1],[[list(a),[]] for a in itertools.product(['a','b'],repeat=2)],False))
  duplicate=rule([],[c('go',0)],[choice([eq(0,'a')],[eq(0,'a')])])
  families.append(('duplicate',[watch,duplicate],[c('p',id),c('go',id)],[id],[[['a'],[c('p','a'),c('seen','a')]]]*2,False))
  # One arm aliases before consumption; the other leaves equality unentailed.
  select=rule([],[c('go',0,1)],[choice([eq(0,1)],[])])
  join=rule([],[c('p',0),c('q',0)],[add('joined',0)])
  take=rule([],[c('p',0),c('token')],[add('taken',0)])
  families.append(('compete',[select,join,take],[c('go',id,id+1),c('p',id),c('q',id+1),c('token')],[id,id+1],[[[0,0],[c('joined',0),c('token')]],[[0,1],[c('taken',0),c('q',1)]]],False))
  replacement=rule([],[c('ticket'),c('p',0)],[choice([add('p',1)], [add('p',0)])])
  families.append(('replacement',[watch,replacement],[c('p',id),c('ticket')],[id],[[[0],[c('seen',0),c('seen',1),c('p',1)]],[[0],[c('seen',0),c('seen',0),c('p',0)]]],False))
  pairs=rule([c('p',0),c('p',1)],[],[choice([add('seen',0,1)],[add('seen',0,1)])])
  families.append(('history-choice',[pairs],[c('p','a'),c('p','b')],[],[[[],[c('p','a'),c('p','b'),c('seen','a','b'),c('seen','b','a')]]]*4,False))
  nested=rule([],[c('go',0)],[choice([eq(0,'a')],[choice([eq(0,'b')],[eq(0,'a')])]),add('done',0)])
  families.append(('nested',[nested],[c('go',id)],[id],[[[a],[c('done',a)]] for a in ['a','b','a']],False))
  conflict=rule([],[c('go',0)],[choice([eq(0,'a')],[eq(0,'b')]),eq(0,'a'),add('done',0)])
  families.append(('binding-failure',[conflict],[c('go',id)],[id],[[['a'],[c('done','a')]]],False))
  # Failed work has no output-variable dependency but must invalidate the branch.
  off=rule([],[c('go')],[choice([add('done')],[['fail']])])
  families.append(('off-output-failure',[off],[c('go')],[id],[[[0],[c('done')]]],False))
  for loopfirst in [False,True]:
   finite=[eq(0,'a'),add('done',0)];ongoing=[add('loop',0)]
   start=rule([],[c('go',0)],[choice(ongoing,finite) if loopfirst else choice(finite,ongoing)])
   loop=rule([],[c('loop',0)],[add('loop',0)])
   families.append(('loop-'+str(int(loopfirst)),[start,loop],[c('go',id)],[id],[[['a'],[c('done','a')]]],True))
  for name,rules,query,outputs,expected,ongoing in families:
   yield dict(name=f'{name}-{id}-{int(reverse)}',rules=rules,query=query[::-1] if reverse else query,outputs=outputs,expected=expected,ongoing=ongoing)
