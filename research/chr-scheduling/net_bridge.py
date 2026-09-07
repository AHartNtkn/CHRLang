"""Resumable finite source-term/net equality boundary, no reference imports."""
from pathlib import Path
import sys
from types import MappingProxyType
from source import equal
sys.path.append(str(Path(__file__).resolve().parents[1] / 'chr-nets'))
from net import data_system
from service import UnificationJob


def natural(value):
    if not isinstance(value, int) or value < 0:
        raise ValueError('nonnegative ID required')
    result = ('Z', ())
    yield 'codec.id_encode'
    for _ in range(value):
        result = ('S', (result,))
        yield 'codec.id_encode'
    return result


def number(value):
    result = 0
    while value[0] == 'S' and len(value[1]) == 1:
        yield 'codec.id_decode'
        result += 1
        value = value[1][0]
    yield 'codec.id_decode'
    if value != ('Z', ()):
        raise ValueError('invalid unary ID')
    return result


def listing(items):
    result = ('Nil', ())
    for item in reversed(items):
        yield 'codec.list_encode'
        result = ('Cons', (item, result))
    return result


def items(value):
    result = []
    while value[0] == 'Cons' and len(value[1]) == 2:
        yield 'codec.list_decode'
        item, value = value[1]
        result.append(item)
    if value != ('Nil', ()):
        raise ValueError('invalid encoded list')
    return result


class Codec:
    def __init__(self, names):
        self.names = tuple(names)
        if len(set(self.names)) != len(self.names):
            raise ValueError('duplicate constructor name')
        self.ids = MappingProxyType({name:i for i,name in enumerate(self.names)})

    def encode(self, term):
        pending = [('visit', term)]
        values = []
        while pending:
            action, t = pending.pop()
            yield 'codec.encode'
            if action == 'assemble':
                name, count = t
                children = values[-count:] if count else []
                if count:
                    del values[-count:]
                args = yield from listing(children)
                tag = yield from natural(self.ids[name])
                values.append(('App', (tag, args)))
            elif isinstance(t, int):
                values.append(('Ref', ((yield from natural(t)),)))
            else:
                if t[0] not in self.ids:
                    raise ValueError(f'unknown constructor {t[0]}')
                pending.append(('assemble', (t[0], len(t[1]))))
                for child in reversed(t[1]):
                    yield 'codec.encode'
                    pending.append(('visit', child))
        return values[0]

    def decode(self, term):
        pending = [('visit', term)]
        values = []
        while pending:
            action, t = pending.pop()
            yield 'codec.decode'
            if action == 'assemble':
                name, count = t
                args = tuple(values[-count:]) if count else ()
                if count:
                    del values[-count:]
                values.append((name, args))
            elif t[0] == 'Ref' and len(t[1]) == 1:
                values.append((yield from number(t[1][0])))
            elif t[0] == 'App' and len(t[1]) == 2:
                tag = yield from number(t[1][0])
                if tag >= len(self.names):
                    raise ValueError('unknown decoded constructor ID')
                args = yield from items(t[1][1])
                pending.append(('assemble', (self.names[tag], len(args))))
                for arg in reversed(args):
                    yield 'codec.decode'
                    pending.append(('visit', arg))
            else:
                raise ValueError('invalid encoded source term')
        return values[0]

    def decode_table(self, encoded):
        table = []
        seen = set()
        for pair in (yield from items(encoded)):
            yield 'codec.table_decode'
            if pair[0] != 'Pair' or len(pair[1]) != 2:
                raise ValueError('invalid table entry')
            key = yield from number(pair[1][0])
            if key in seen:
                raise ValueError('duplicate variable entry')
            seen.add(key)
            value = yield from self.decode(pair[1][1])
            table.append((key, value))
        return tuple(table)


class NetEquality:
    def __init__(self, names):
        # Explicit preparation: callers retain one service for all its requests.
        self.codec = Codec(names)
        self.system = data_system()

    def solve(self, initial, equations):
        entries = []
        for key, term in initial:
            yield 'codec.table_encode'
            key = yield from natural(key)
            term = yield from self.codec.encode(term)
            entries.append(('Pair', (key, term)))
        table = yield from listing(entries)
        pending = []
        for left, right in equations:
            left = yield from self.codec.encode(left)
            right = yield from self.codec.encode(right)
            pending.append(('Pair', (left, right)))
        equations = yield from listing(pending)
        job = UnificationJob(table, equations, self.system)
        while not job.done:
            before = dict(job.counts)
            job.advance(1)
            for label, count in job.counts.items():
                if count != before[label]:
                    assert count == before[label]+1
                    yield 'net.'+label
        original, result = job.observe()
        original = yield from self.codec.decode_table(original)
        if len(original) != len(initial):
            raise ValueError('original table changed')
        for (k, a), (j, b) in zip(original, initial):
            yield 'codec.validate'
            if k != j or not (yield from equal(a, b)):
                raise ValueError('original table changed')
        if result == ('None', ()):
            return None
        if result[0] != 'Some' or len(result[1]) != 1:
            raise ValueError('invalid service result')
        return (yield from self.codec.decode_table(result[1][0]))
