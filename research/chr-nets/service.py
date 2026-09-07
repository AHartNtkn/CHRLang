"""A sealed net-unification request with yielding build, reduction and readback.

Inputs are immutable encoded data. The compiled rule system is prepared once by
its owner and shared read-only; compilation is not hidden in request admission.
"""
from net import DATA, Net


class UnificationJob:
    def __init__(self, table, equations, system, status='scan', deltas=None):
        if status not in ('scan', 'count') or (status == 'count') != (deltas is not None):
            raise ValueError('status mode requires matching prepared metadata')
        self.status = status
        self.net = Net(system, deltas=deltas)
        self.phase = 'build'
        self.done = False
        self.counts = dict(build=0, reduce=0, read=0, publish=0)
        self.counts.update(dict(scan=0) if status == 'scan' else dict(maintain=0, status=0))
        self._result = None
        self._iterator = self._charged(table, equations)

    def _charged(self, table, equations):
        work = self._work(table, equations)
        while True:
            before = self.net.maintenance_actions
            try:
                action = next(work)
            except StopIteration as done:
                return done.value
            yield action
            for _ in range(self.net.maintenance_actions-before):
                yield 'maintain'

    def _build(self, data, destination):
        pending = [(data, destination)]
        while pending:
            (tag, children), parent = pending.pop()
            assert tag in DATA and isinstance(children, tuple) and len(children) == DATA[tag]
            node = self.net.node(tag)
            self.net.connect(parent, (node, 0))
            pending.extend((child, (node, i)) for i, child in reversed(list(enumerate(children, 1))))
            yield 'build'

    def _read(self, output):
        pending = [('visit', (output, 0))]
        values = []
        while pending:
            action, value = pending.pop()
            if action == 'visit':
                node, port = self.net.ports[value]
                tag = self.net.nodes[node]
                if port != 0 or tag not in DATA:
                    raise ValueError('stuck or non-data service output')
                arity = DATA[tag]
                pending.append(('assemble', (tag, arity)))
                pending.extend(('visit', (node, p)) for p in range(arity, 0, -1))
            else:
                tag, arity = value
                children = tuple(values[-arity:]) if arity else ()
                if arity:
                    del values[-arity:]
                values.append((tag, children))
            yield 'read'
        assert len(values) == 1
        return values[0]

    def _work(self, table, equations):
        header = []
        for tag in ('Out', 'Out', 'Dup', 'U'):
            header.append(self.net.node(tag))
            yield 'build'
        original, result, dup, controller = header
        for left, right in [((dup, 1), (original, 0)),
                            ((dup, 2), (controller, 1)),
                            ((controller, 2), (result, 0))]:
            self.net.connect(left, right)
            yield 'build'
        yield from self._build(table, (dup, 0))
        yield from self._build(equations, (controller, 0))
        # Partially wired private graphs are never reduced or observed.
        self.phase = 'reduce'
        while self.net.ready:
            self.net.step()
            yield 'reduce'
        self.phase = 'scan'
        if self.status == 'scan':
            for tag in self.net.nodes:
                if tag is not None and tag not in DATA and tag != 'Out':
                    raise ValueError('stuck service controller')
                yield 'scan'
        else:
            yield 'status'
            if self.net.controller_count != 0:
                raise ValueError('stuck service controller')
        self.phase = 'read'
        before = yield from self._read(original)
        after = yield from self._read(result)
        return before, after

    def advance(self, quantum):
        if quantum < 0:
            raise ValueError('negative quantum')
        used = 0
        for _ in range(quantum):
            if self.done:
                break
            try:
                action = next(self._iterator)
            except StopIteration as completed:
                self._result = completed.value
                self.phase = 'done'
                self.done = True
                action = 'publish'
            self.counts[action] += 1
            used += 1
        return used

    def observe(self):
        if not self.done:
            raise ValueError('service request is unfinished')
        return self._result
