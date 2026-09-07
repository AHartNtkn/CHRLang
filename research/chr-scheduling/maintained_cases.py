"""Proposed A3 finite vertical slice. Root must register before matrix execution."""
from source import Rule


def term(name,*args): return name, args

def conjunction(*goals): return ('and', *goals)


def case(n=4, branches=2, rounds=1, consuming=False):
    if n not in (4,16,64) or branches not in (1,2,8) or rounds not in (1,4):
        raise ValueError('outside proposed finite grid')
    t=term
    left=t('left',0,1); edge=t('edge',0,2); right=t('right',2,3)
    join=Rule( () if consuming else (left,edge),
              (left,edge,right) if consuming else (right,), ('post',t('out',1,3)))
    # A binary command tree avoids making source matching traverse a long list.
    rules=[join,
      Rule((),(t('pump',t('seq',0,1)),),conjunction(('post',t('pump',0)),('post',t('pump',1)))),
      Rule((),(t('pump',t('put',0)),),('post',0)),
      Rule((),(t('pump',t('bind',0,1)),),('eq',0,1)),
      Rule((),(t('pump',t('nothing')),),('true',))]
    initial=[]
    keys=[t('key',t(str(i))) for i in range(n)]
    # Query variables 0=output tag, 1->2 is a suspension chain, 3 broad key.
    for i,key in enumerate(keys):
        initial += [t('left',key,t('u',t(str(i)))),t('edge',key,key)]
    initial += [t('left',t('alias'),t('alias_payload')),t('edge',t('alias'),1)]
    for i in range(n):
        initial += [t('left',t('broad',t(str(i))),t('broad_payload',t(str(i)))),
                    t('edge',t('broad',t(str(i))),3)]
    commands=[]
    for r in range(rounds):
        for i,key in enumerate(keys):
            if consuming and r:
                commands += [t('put',t('left',key,t('u',t(str(i))))),t('put',t('edge',key,key))]
            commands.append(t('put',t('right',key,t('v',t(str(r)),t(str(i))))))
    commands += [t('put',t('right',t('awaken'),t('alias_result'))),
                 t('bind',1,2),t('bind',2,t('awaken')),
                 t('put',t('right',t('all'),t('broad_result'))),t('bind',3,t('all'))]
    def balanced(items):
        if not items: return t('nothing')
        if len(items)==1:return items[0]
        middle=len(items)//2
        return t('seq',balanced(items[:middle]),balanced(items[middle:]))
    choices=[('eq',0,t('tag',t(str(i)))) for i in range(branches)]
    def choice(items):
        if len(items)==1:return items[0]
        middle=len(items)//2
        return ('or',choice(items[:middle]),choice(items[middle:]))
    rules.append(Rule((),(t('start',0,1,2,3),),conjunction(choice(choices),('post',t('pump',balanced(commands))))))
    initial.append(t('start',0,1,2,3))
    return dict(id=f'join-{ "consume" if consuming else "keep"}-n{n}-b{branches}-r{rounds}',
                rules=tuple(rules),constraints=tuple(initial),outputs=(0,1,2,3),
                contexts=branches, budget=100000, matcher_cap=10000000,
                expected=expected(n,branches,rounds,consuming),
                parameters=dict(n=n,branches=branches,rounds=rounds,consuming=consuming))


def cases():
    for consuming in (False,True):
        for n in (4,16,64):
            for b in (1,2,8):
                for r in (1,4):
                    yield case(n,b,r,consuming)


def expected(n, branches, rounds, consuming):
    """Closed-form observations, independent of candidate matching/execution."""
    t=term
    residual=[]
    if not consuming:
        for i in range(n):
            key=t('key',t(str(i)))
            residual += [t('left',key,t('u',t(str(i)))),t('edge',key,key)]
        residual += [t('left',t('alias'),t('alias_payload')),t('edge',t('alias'),t('awaken'))]
    for i in range(1 if consuming else 0,n):
        key=t('broad',t(str(i)))
        residual += [t('left',key,t('broad_payload',t(str(i)))),t('edge',key,t('all'))]
    for r in range(rounds):
        for i in range(n):
            residual.append(t('out',t('u',t(str(i))),t('v',t(str(r)),t(str(i)))))
    residual += [t('out',t('alias_payload'),t('alias_result')),
                 t('out',t('broad_payload',t('0')),t('broad_result'))]
    return [dict(outputs=[t('tag',t(str(b))),t('awaken'),t('awaken'),t('all')],residual=list(residual))
            for b in range(branches)]
