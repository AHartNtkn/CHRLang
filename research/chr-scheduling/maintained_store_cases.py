"""Exact proposed finite supported-store gate; root owns run registration."""
from dataclasses import replace
from source import Rule, State
from maintained_cases import term as t


def cases():
    rules=(Rule((t('left',0),t('edge',0,1)),(t('right',1),),('true',)),)
    states={i:State((),(),2,((0,t('left',t('a'))),(1,t('edge',t('a'),0))),
                    ((0,1),),next_occurrence=2) for i in range(8)}
    updates=[('suspended',states)]
    states={i:replace(s,store=s.store+((2,t('right',t('b'))),),next_occurrence=3) for i,s in states.items()}
    updates.append(('right-insert',states))
    states={i:replace(s,sub=((0,1),(1,t('b' if i%2 else 'c')))) for i,s in states.items()}
    updates.append(('alias-bind',states))
    states=dict(states); states[1]=replace(states[1],store=states[1].store[:2])
    updates.append(('consume-context-one',states))
    yield dict(id='alias-support',rules=rules,updates=updates)
    states={i:State((),(),0,((0,t('left',t('a'))),(2,t('right',t('b')))),next_occurrence=3) for i in range(2)}
    updates=[('missing-middle',states)]
    states={i:replace(s,store=s.store+((1,t('edge',t('a'),t('b'))),)) for i,s in states.items()}
    updates.append(('middle-arrives',states))
    states=dict(states); states[0]=replace(states[0],store=states[0].store[:-2]+((1,t('edge',t('a'),t('b'))),))
    updates.append(('right-consumed',states))
    yield dict(id='middle-arrival',rules=rules,updates=updates)
    rules=(Rule((),(t('p',0,0),t('p',1,1)),('true',)),)
    a=State((),(),1,((0,t('p',0,0)),(1,t('p',0,0))),next_occurrence=2)
    yield dict(id='distinct-bindings',rules=rules,updates=[('unbound',{0:a,1:a}),
        ('one-bound',{0:a,1:replace(a,sub=((0,t('a')),))})])
    rules=(Rule((),(t('p',0),),('true',)),)
    a=State((),(),2,((0,0),),((0,1),),next_occurrence=1)
    yield dict(id='root-predicate',rules=rules,updates=[('unknown',{0:a}),
        ('root-bound',{0:replace(a,sub=((0,1),(1,t('p',t('a')))))})])
