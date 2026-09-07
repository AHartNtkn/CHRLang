"""Resumable exact alpha equivalence for full CHR observations.

No logical projection or subsumption. Callers retain immutable inputs for the job
lifetime. Counts include explicit traversal and map copying, not host latency.
"""
from collections import Counter


def alpha(left, right, forward, backward):
    todo = [(left, right)]
    while todo:
        a, b = todo.pop()
        yield 'term'
        if isinstance(a, int) or isinstance(b, int):
            if not isinstance(a, int) or not isinstance(b, int):
                return False
            if a in forward:
                if forward[a] != b:
                    return False
            elif b in backward:
                return False
            else:
                forward[a], backward[b] = b, a
        elif a[0] != b[0] or len(a[1]) != len(b[1]):
            return False
        else:
            for pair in reversed(tuple(zip(a[1], b[1]))):
                yield 'term'
                todo.append(pair)
    return True


def copy_map(mapping):
    result = {}
    for k, v in mapping.items():
        yield 'copy'
        result[k] = v
    return result


class EquivalenceJob:
    def __init__(self, left, right):
        self.left, self.right = left, right
        self.done = False
        self.counts = Counter()
        self._runner = self._run()

    def advance(self, quantum):
        if quantum <= 0:
            raise ValueError('positive quantum required')
        used = 0
        while not self.done and used < quantum:
            try:
                label = next(self._runner)
            except StopIteration as result:
                self.done, self.result = True, result.value
                label = 'publish'
            self.counts[label] += 1
            used += 1
        return used

    def observe(self):
        if not self.done:
            raise ValueError('unfinished observation')
        return self.result

    def _run(self):
        a, b = self.left, self.right
        yield 'shape'
        if len(a['outputs']) != len(b['outputs']) or len(a['residual']) != len(b['residual']):
            return False
        forward, backward = {}, {}
        for x, y in zip(a['outputs'], b['outputs']):
            if not (yield from alpha(x, y, forward, backward)):
                return False
        # Explicit DFS retains one immutable used-index set and mapping per level.
        # Each parent resumes at its next candidate after a failed child.
        stack = [(0, 0, frozenset(), forward, backward)]
        total = len(a['residual'])
        while stack:
            index, candidate, used, f, r = stack.pop()
            yield 'search'
            if index == total:
                return True
            if candidate == total:
                continue
            stack.append((index, candidate+1, used, f, r))
            if candidate in used:
                continue
            nf = yield from copy_map(f)
            nr = yield from copy_map(r)
            if not (yield from alpha(a['residual'][index], b['residual'][candidate], nf, nr)):
                continue
            for _ in used:
                yield 'copy'
            stack.append((index+1, 0, used | {candidate}, nf, nr))
        return False
