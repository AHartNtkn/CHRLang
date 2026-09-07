"""E09 mixed selection/binding workloads with independent expected answers."""
from source import Rule

A, B, OK = ('a', ()), ('b', ()), ('ok', ())


def workloads():
    for family in ('scan-lag', 'binding', 'guard-binding', 'history'):
        for n in (4,8,12):
            noise = tuple(('noise', ((f'n{i}', ()),)) for i in range(n))
            impossible = Rule((), (('q', (0,)), ('q', (1,)), ('q', (2,))), ('fail',))
            if family == 'scan-lag':
                slow = ('and', *(('post', t) for t in noise), ('eq', 0, B))
                fast = ('and', *(('true',) for _ in noise), ('post', ('finish', (0,))))
                rules = (Rule((), (('start', (0,)),), ('or', slow, fast)), impossible,
                         Rule((), (('finish', (0,)),), ('eq', 0, A)))
                constraints, outputs = (('start', (0,)),), (0,)
                expected = [dict(outputs=[A], residual=[]), dict(outputs=[B], residual=list(noise))]
            elif family == 'binding':
                body = ('or', ('and', ('eq', 0, A), ('eq', 1, ('left', ()))),
                        ('and', ('eq', 0, B), ('eq', 1, ('right', ()))))
                rules = (Rule((), (('p', (A,)),), ('true',)), impossible,
                         Rule((), (('bind', (0,1)),), body))
                constraints, outputs = (('p', (0,)), ('bind', (0,1))) + noise, (0,1)
                expected = [dict(outputs=[A, ('left', ())], residual=list(noise)),
                            dict(outputs=[B, ('right', ())], residual=[('p', (B,)), *noise])]
            elif family == 'guard-binding':
                rules = (Rule((('p', (0,)),), (('mark', (1,)),), ('eq', 1, OK), ((0, A),)),
                         impossible, Rule((), (('bind', (0,)),), ('or', ('eq', 0, A), ('eq', 0, B))))
                constraints, outputs = (('p', (0,)), ('mark', (1,)), ('bind', (0,))) + noise, (0,1)
                expected = [dict(outputs=[A, OK], residual=[('p', (A,)), *noise]),
                            dict(outputs=[B, 1], residual=[('p', (B,)), ('mark', (1,)), *noise])]
            else:
                rules = (Rule((('p', (0,)),), (), ('post', ('seen', (0,))), ((0, A),)),
                         impossible, Rule((), (('bind', (0,)),), ('or', ('eq', 0, A), ('eq', 0, A))))
                constraints, outputs = (('p', (0,)), ('bind', (0,))) + noise, (0,)
                expected = [dict(outputs=[A], residual=[('p', (A,)), *noise, ('seen', (A,))])]
            yield dict(id=f'{family}-{n}', rules=rules, constraints=constraints,
                       outputs=outputs, expected=expected, raw=2)
