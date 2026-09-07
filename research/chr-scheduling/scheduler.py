"""E09 exact-state support scheduling. Experimental fixed source selector.

Support expressions retain path identity; only equal complete states share steps.
Host allocation/hash/big-int costs are not constant-time service guarantees.
"""
from collections import Counter, deque
from dataclasses import dataclass
from source import StepJob, initial
from observer import EquivalenceJob


@dataclass(frozen=True, eq=False)
class Support:
    kind: str
    count: int
    left: object = None
    right: object = None

    @staticmethod
    def root():
        return Support('root', 1)

    def arm(self, bit):
        assert bit in (0, 1)
        return Support('arm', self.count, self, bit)

    @staticmethod
    def union(a, b):
        # Scheduler ownership ensures disjoint inputs; not a set-union solver.
        return Support('union', a.count+b.count, a, b)

    def paths(self):
        """Validation only: explicitly expand finite support."""
        todo = [(self, ())]
        while todo:
            node, suffix = todo.pop()
            if node.kind == 'root':
                yield suffix
            elif node.kind == 'arm':
                todo.append((node.left, (node.right,) + suffix))
            else:
                todo.append((node.right, suffix))
                todo.append((node.left, suffix))


def state_equal(a, b, mode="reverse"):
    if mode not in ("reverse", "forward", "identity"):
        raise ValueError("unknown comparison mode")
    fields = ('pending', 'outputs', 'next_var', 'next_occurrence', 'store', 'sub')
    todo = []
    for field in fields:
        yield 'field'
        todo.append((getattr(a, field), getattr(b, field)))
    while todo:
        x, y = todo.pop()
        yield 'compare'
        if mode == 'identity':
            yield 'identity'
            if x is y:
                continue
        if type(x) is not type(y):
            return False
        if isinstance(x, tuple):
            if len(x) != len(y):
                return False
            indices = range(len(x)) if mode == 'reverse' else range(len(x)-1, -1, -1)
            for i in indices:
                pair = (x[i], y[i])
                yield 'compare'
                todo.append(pair)
        elif x != y:
            return False
    if len(a.history) != len(b.history):
        return False
    for token in a.history:
        yield 'history'
        if token not in b.history:
            return False
    return True


class Job:
    def __init__(self, runner):
        self.runner = runner
        self.done = False
        self.counts = Counter()

    def advance(self, quantum):
        used = 0
        while not self.done and used < quantum:
            try:
                label = next(self.runner)
            except StopIteration as result:
                self.result, self.done = result.value, True
                label = 'publish'
            self.counts[label] += 1
            used += 1
        return used

    def observe(self):
        if not self.done:
            raise ValueError('unfinished job')
        return self.result


def group_batch(entries, grouping, compare_mode="reverse"):
    groups = []
    for state, support in entries:
        yield 'entry'
        joined = False
        if grouping:
            for i, (old, owned) in enumerate(groups):
                if (yield from state_equal(state, old, compare_mode)):
                    yield 'union'
                    groups[i] = (old, Support.union(owned, support))
                    joined = True
                    break
        if not joined:
            groups.append((state, support))
    return groups


def observe(answer, accepted):
    for old in accepted:
        yield 'candidate'
        job = EquivalenceJob(answer, old)
        while not job.done:
            before = job.counts.copy()
            job.advance(1)
            for label, count in (job.counts-before).items():
                assert count == 1
                yield label
        if job.observe():
            return False
    return True


class Search:
    def __init__(self, rules, constraints, outputs, policy, grouping=True, batch_size=8, compare_mode="reverse", selector="scan"):
        if policy not in ('fifo', 'round', 'async'):
            raise ValueError('unknown policy')
        if batch_size <= 0:
            raise ValueError('positive batch size required')
        if compare_mode not in ("reverse", "forward", "identity"):
            raise ValueError("unknown comparison mode")
        if selector not in ("scan", "predicate", "prefix", "guard-prefix"):
            raise ValueError("unknown selector")
        self.selector = selector
        self.compare_mode = compare_mode
        self.policy = policy
        self.grouping = grouping and policy != 'fifo'
        self.batch_size = 1 if policy == 'fifo' else batch_size
        self.rules = rules
        # Query admission is explicitly outside service-action counts.
        self.ready = deque([(initial(constraints, outputs), Support.root())])
        self.parked = deque()
        self.jobs = deque()
        self.observations = deque()
        self.observer = None
        self.observer_turn = False
        self.counts = Counter()
        self.answers = []
        self.answer_actions = []
        self.raw = self.failed = self.logical_steps = self.source_jobs = 0
        self.rounds = 0
        self.peak_jobs = self.peak_ready = self.peak_observations = 0

    @property
    def exhausted(self):
        return not (self.ready or self.parked or self.jobs or self.observations or self.observer)

    @property
    def actions(self):
        return sum(self.counts.values())

    def _admit(self):
        if self.policy == 'round' and not self.jobs and not self.ready and self.parked:
            self.ready, self.parked = self.parked, deque()
            self.rounds += 1
        if self.ready:
            entries = []
            for _ in range(min(self.batch_size, len(self.ready))):
                entries.append(self.ready.popleft())
                self.counts['admit'] += 1
            self.jobs.append(('group', Job(group_batch(entries, self.grouping, self.compare_mode)), None))

    def _advance(self, job, quantum, prefix):
        before = job.counts.copy()
        job.advance(quantum)
        for label, count in (job.counts-before).items():
            self.counts[prefix + '.' + label] += count

    def advance(self, quantum):
        """One FIFO service opportunity plus bounded admission/commit bookkeeping.

        quantum limits a service resume, not total actions in this method.
        All extra admission/dispatch/support actions are recorded separately.
        """
        if quantum <= 0:
            raise ValueError('positive quantum required')
        self._admit()
        if self.observer is None and self.observations:
            answer, support = self.observations.popleft()
            self.observer = (Job(observe(answer, self.answers)), answer, support)
            self.counts['observer_admit'] += 1
        self.counts['dispatch'] += 1
        if self.observer is not None and (self.observer_turn or not self.jobs):
            job, answer, _ = self.observer
            self._advance(job, quantum, 'observe')
            if job.done:
                if job.observe():
                    self.answers.append(answer)
                    self.answer_actions.append(self.actions)
                self.observer = None
            self.observer_turn = False
        elif self.jobs:
            kind, job, support = self.jobs.popleft()
            self._advance(job, quantum, kind)
            self.observer_turn = True
            if not job.done:
                self.jobs.append((kind, job, support))
            elif kind == 'group':
                for state, owned in job.observe():
                    self.counts['source_admit'] += 1
                    self.jobs.append(('source', StepJob(state, self.rules, self.selector), owned))
                    self.source_jobs += 1
            else:
                event = job.observe()
                self.logical_steps += support.count
                self.counts['commit'] += 1
                target = self.parked if self.policy == 'round' else self.ready
                if event[0] == 'continue':
                    target.append((event[1], support))
                elif event[0] == 'split':
                    for bit, state in enumerate(event[1:]):
                        self.counts['support_arm'] += 1
                        target.append((state, support.arm(bit)))
                elif event[0] == 'failed':
                    self.failed += support.count
                elif event[0] == 'answer':
                    self.raw += support.count
                    self.observations.append((event[1], support))
                else:
                    raise AssertionError(event[0])
        self.peak_jobs = max(self.peak_jobs, len(self.jobs))
        self.peak_ready = max(self.peak_ready, len(self.ready)+len(self.parked))
        self.peak_observations = max(self.peak_observations, len(self.observations)+bool(self.observer))
