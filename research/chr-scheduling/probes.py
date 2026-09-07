"""Frozen E09 controlled policy workloads."""
from source import Rule


def tree(depth):
    t = ('z', ())
    for _ in range(depth):
        t = ('s', (t,))
    return t


def answer(name):
    return {'outputs': [(name, ())], 'residual': []}


def workloads():
    for family, sizes in [('lag', (4,16,64)), ('duplicates', (2,4,6)),
                          ('opaque', (4,16,64)), ('recursive', (4,16,64))]:
        for n in sizes:
            t = tree(n)
            identity = ('eq', t, t)
            a, b = ('eq', 0, ('a', ())), ('eq', 0, ('b', ()))
            extra = ()
            if family == 'lag':
                body = ('or', ('and', identity, b), ('and', ('true',), a))
                expected, raw, prefix = [answer('a'), answer('b')], 2, False
            elif family == 'duplicates':
                body = a
                for _ in range(n):
                    body = ('and', ('or', ('true',), ('true',)), body)
                expected, raw, prefix = [answer('a')], 2**n, False
            elif family == 'opaque':
                body = ('and', ('or', a, b), identity, identity, identity, identity)
                expected, raw, prefix = [answer('a'), answer('b')], 2, False
            else:
                body = ('or', ('post', ('spin', ())), a)
                extra = (Rule((), (('spin', ()),), ('and', identity, ('post', ('spin', ())))),)
                expected, raw, prefix = [answer('a')], 1, True
            yield dict(id=f'{family}-{n}', family=family, size=n,
                       rules=(Rule((), (('start', (0,)),), body),) + extra,
                       constraints=(('start', (0,)),), outputs=(0,),
                       expected=expected, raw=raw, prefix=prefix)
