"""Finite controller rules for private finite-tree unification.

Wire names are compilation conveniences, not runtime logical variables.
"""
from collections import defaultdict


def install(arities, rules, rule_type):
    arities.update(U=2, UEquation=3, ULeft=3, URight=3, UDispatch=4,
                   UVar=4, UVars=5, UApp=5, UTags=5,
                   UZip=4, UZipNil=3, UZipCons=5,
                   UBind=4, UBindAfter=4, UBindDecision=5)

    def add(left, right, nodes):
        occurrences = defaultdict(list)
        for i, wire in enumerate(left[1:] + right[1:]):
            occurrences[wire].append(i)
        for i, node in enumerate(nodes):
            assert len(node) == arities[node[0]] + 2, node
            for port, wire in enumerate(node[1:]):
                occurrences[wire].append((i, port))
        assert len(left) == arities[left[0]] + 1
        assert len(right) == arities[right[0]] + 1
        assert all(len(ends) == 2 for ends in occurrences.values()), (left, right, occurrences)
        rules.append(rule_type(left[0], right[0], tuple(n[0] for n in nodes),
                               tuple(tuple(ends) for ends in occurrences.values())))

    def fail(*garbage):
        return [('None', 'out')] + [('Erase', name) for name in garbage]

    def bind():
        return [('U', 'todo', 'bound', 'out'), ('Cons', 'bound', 'entry', 'table'),
                ('Pair', 'entry', 'id', 'value')]

    add(('U', 'table', 'out'), ('Nil',), [('Some', 'out', 'table')])
    add(('U', 'table', 'out'), ('Cons', 'equation', 'todo'),
        [('UEquation', 'equation', 'todo', 'table', 'out')])
    add(('UEquation', 'todo', 'table', 'out'), ('Pair', 'left', 'right'),
        [('Deref', 'left', 'table', 'response'), ('ULeft', 'response', 'right', 'todo', 'out')])
    add(('ULeft', 'right', 'todo', 'out'), ('Pair', 'table', 'left'),
        [('Deref', 'right', 'table', 'response'), ('URight', 'response', 'left', 'todo', 'out')])
    add(('URight', 'left', 'todo', 'out'), ('Pair', 'table', 'right'),
        [('UDispatch', 'left', 'right', 'table', 'todo', 'out')])
    add(('UDispatch', 'right', 'table', 'todo', 'out'), ('Ref', 'id'),
        [('UVar', 'right', 'id', 'table', 'todo', 'out')])
    add(('UVar', 'id', 'table', 'todo', 'out'), ('Ref', 'other'),
        [('Dup', 'id', 'id_test', 'id_kept'), ('Dup', 'other', 'other_test', 'other_kept'),
         ('Eq', 'id_test', 'other_test', 'equal'),
         ('UVars', 'equal', 'id_kept', 'other_kept', 'table', 'todo', 'out')])
    add(('UVars', 'id', 'other', 'table', 'todo', 'out'), ('T',),
        [('Erase', 'id'), ('Erase', 'other'), ('U', 'todo', 'table', 'out')])
    add(('UVars', 'id', 'other', 'table', 'todo', 'out'), ('F',),
        bind() + [('Ref', 'value', 'other')])
    add(('UVar', 'id', 'table', 'todo', 'out'), ('App', 'tag', 'children'),
        [('UBind', 'term', 'id', 'table', 'todo', 'out'), ('App', 'term', 'tag', 'children')])
    add(('UDispatch', 'right', 'table', 'todo', 'out'), ('App', 'tag', 'children'),
        [('UApp', 'right', 'tag', 'children', 'table', 'todo', 'out')])
    add(('UApp', 'tag', 'children', 'table', 'todo', 'out'), ('Ref', 'id'),
        [('UBind', 'term', 'id', 'table', 'todo', 'out'), ('App', 'term', 'tag', 'children')])
    add(('UApp', 'tag', 'children', 'table', 'todo', 'out'), ('App', 'other_tag', 'other_children'),
        [('Eq', 'tag', 'other_tag', 'equal'),
         ('UTags', 'equal', 'children', 'other_children', 'table', 'todo', 'out')])
    add(('UTags', 'children', 'other_children', 'table', 'todo', 'out'), ('F',),
        fail('children', 'other_children', 'table', 'todo'))
    add(('UTags', 'children', 'other_children', 'table', 'todo', 'out'), ('T',),
        [('UZip', 'children', 'other_children', 'todo', 'table', 'out')])
    add(('UZip', 'right', 'todo', 'table', 'out'), ('Nil',),
        [('UZipNil', 'right', 'todo', 'table', 'out')])
    add(('UZipNil', 'todo', 'table', 'out'), ('Nil',), [('U', 'todo', 'table', 'out')])
    add(('UZipNil', 'todo', 'table', 'out'), ('Cons', 'head', 'tail'),
        fail('todo', 'table', 'head', 'tail'))
    add(('UZip', 'right', 'todo', 'table', 'out'), ('Cons', 'head', 'tail'),
        [('UZipCons', 'right', 'head', 'tail', 'todo', 'table', 'out')])
    add(('UZipCons', 'head', 'tail', 'todo', 'table', 'out'), ('Nil',),
        fail('head', 'tail', 'todo', 'table'))
    add(('UZipCons', 'head', 'tail', 'todo', 'table', 'out'), ('Cons', 'other_head', 'other_tail'),
        [('UZip', 'tail', 'other_tail', 'pending', 'table', 'out'),
         ('Cons', 'pending', 'equation', 'todo'), ('Pair', 'equation', 'head', 'other_head')])
    add(('UBind', 'id', 'table', 'todo', 'out'), ('App', 'tag', 'children'),
        [('App', 'term', 'tag', 'children'), ('Dup', 'term', 'tested', 'kept'),
         ('Dup', 'id', 'tested_id', 'kept_id'), ('Cons', 'pending', 'tested', 'nil'), ('Nil', 'nil'),
         ('Occurs', 'pending', 'tested_id', 'table', 'response'),
         ('UBindAfter', 'response', 'kept_id', 'kept', 'todo', 'out')])
    add(('UBindAfter', 'id', 'value', 'todo', 'out'), ('Pair', 'table', 'cyclic'),
        [('UBindDecision', 'cyclic', 'id', 'value', 'table', 'todo', 'out')])
    add(('UBindDecision', 'id', 'value', 'table', 'todo', 'out'), ('T',),
        fail('id', 'value', 'table', 'todo'))
    add(('UBindDecision', 'id', 'value', 'table', 'todo', 'out'), ('F',), bind())
