"""Decode the admitted atom/unknown common protocol for native source emission."""
import re


def split_top(text, separator):
    depth = 0
    start = 0
    result = []
    for i, char in enumerate(text):
        if char == '(':
            depth += 1
        elif char == ')':
            depth -= 1
            if depth < 0:
                raise ValueError('unmatched closing parenthesis')
        if char == separator and depth == 0:
            result.append(text[start:i])
            start = i + 1
    if depth:
        raise ValueError('unclosed parenthesis')
    result.append(text[start:])
    return result


def term(text):
    if re.fullmatch(r'v[0-9]+', text):
        return int(text[1:])
    if re.fullmatch(r'[a-z][a-z0-9]*', text) and not text.startswith('v'):
        return text
    raise ValueError('unsupported term')


def constraint(text):
    name, *args = text.split(':')
    if not re.fullmatch(r'[a-z][a-z0-9]*', name):
        raise ValueError('invalid predicate')
    return [name, [term(arg) for arg in args]]


def constraints(text):
    return [] if text == '-' else [constraint(x) for x in text.split(',')]


def body(text):
    if text == '-':
        return []
    if text == '!':
        return [['fail']]
    parts = split_top(text, ',')
    if len(parts) > 1:
        return [action for part in parts for action in body(part)]
    if text.startswith('(') and text.endswith(')'):
        arms = split_top(text[1:-1], '|')
        if len(arms) != 2:
            raise ValueError('choice must have two arms')
        return [['or', body(arms[0]), body(arms[1])]]
    if text.startswith('=:'):
        args = text[2:].split(':')
        if len(args) != 2:
            raise ValueError('equality must have two terms')
        return [['eq', *map(term, args)]]
    return [['add', constraint(text)]]


def decode(text):
    rules = []
    queries = []
    for i, section in enumerate(text.split('\nNEXT\n')):
        lines = section.splitlines()
        if len(lines) < 2 or (i and len(lines) != 2):
            raise ValueError('query section length')
        outputs = [] if lines[1] == '-' else [int(x) for x in lines[1].split(',')]
        if any(type(x) is not int for x in outputs):
            raise ValueError('outputs must be variables')
        queries.append(dict(query=constraints(lines[0]), outputs=outputs))
        if i == 0:
            for line in lines[2:]:
                parts = line.split(';')
                if len(parts) != 3:
                    raise ValueError('rule must have kept, consumed and body fields')
                rules.append(dict(kept=constraints(parts[0]), removed=constraints(parts[1]), body=body(parts[2])))
    return rules, queries
