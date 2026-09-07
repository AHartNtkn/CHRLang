"""Experimental retained prefix rows with explicit finite context supports.

Updates are serialized; an abandoned update poisons the instance; this is not a concurrent subscription algorithm. Matching
is nonbinding. Guards, propagation history and committed selection remain source
operations. Metadata scans and candidate-index probes are charged explicitly.
Python tuple hashing/equality and allocation are not constant-time guarantees.
"""
from collections import Counter
from dataclasses import dataclass, field
from source import map_term, match


@dataclass
class Context:
    state: object = None
    store: dict = field(default_factory=dict)
    sub: dict = field(default_factory=dict)
    pools: dict = field(default_factory=dict)
    # (parent row, next occurrence) -> (successful row or None, read variables)
    tests: dict = field(default_factory=dict)
    rows: set = field(default_factory=set)
    by_occ: dict = field(default_factory=dict)
    by_var: dict = field(default_factory=dict)
    by_parent: dict = field(default_factory=dict)


class MaintainedJoin:
    def __init__(self, rules, mode='selective', max_contexts=8):
        if mode not in ('full', 'selective') or not 1 <= max_contexts <= 8:
            raise ValueError('full/selective mode and at most eight contexts required')
        self.rules, self.mode, self.max_contexts = tuple(rules), mode, max_contexts
        self.contexts = {}
        # Payload key contains rule, ordered occurrence IDs and bindings.
        self.rows = {}
        self.counts = Counter()
        self.peak_rows = self.peak_tests = self.peak_dependencies = 0
        self.peak_index_edges = 0
        self.live_tests = self.live_dependencies = 0
        self._busy = False
        self._poisoned = False

    def _record(self, work):
        while True:
            try:
                label = next(work)
            except StopIteration as result:
                return result.value
            self.counts[label] += 1
            yield label

    def sync(self, context, state):
        if not isinstance(context, int) or not 0 <= context < self.max_contexts:
            raise ValueError('context outside finite support')
        return (yield from self._transaction(self._record(self._sync(context,state))))

    def _transaction(self,work):
        if self._poisoned:
            raise ValueError('abandoned matcher update: discard this instance')
        if self._busy:
            raise ValueError('overlapping matcher updates require another protocol')
        self._busy = True
        finished = False
        try:
            result = yield from work
            finished = True
            return result
        finally:
            self._busy = False
            if not finished:
                self._poisoned = True

    def _add_row(self, c, bit, key):
        yield 'maintain.row_payload_create' if key not in self.rows else 'maintain.row_support_extend'
        yield 'maintain.row_add'
        c.rows.add(key)
        self.rows[key] = self.rows.get(key, 0) | bit
        self.peak_rows = max(self.peak_rows,len(self.rows))

    def _drop_row(self, c, bit, key):
        yield 'maintain.row_remove'
        c.rows.remove(key)
        support = self.rows[key] & ~bit
        if support:
            self.rows[key] = support
        else:
            del self.rows[key]

    def _sync(self, context, state):
        c = self.contexts.setdefault(context, Context())
        bit = 1 << context
        yield 'maintain.snapshot'
        if c.state is state:
            return
        store = {}
        for oid, term in state.store:
            yield 'maintain.input_occurrence'
            if oid in store:
                raise ValueError('duplicate occurrence ID')
            store[oid] = term
        sub = {}
        for v, term in state.sub:
            yield 'maintain.input_binding'
            sub[v] = term
        changed_occ, changed_var = set(), set()
        for oid in c.store.keys() | store.keys():
            yield 'maintain.diff_occurrence'
            if oid not in c.store or oid not in store or c.store[oid] != store[oid]:
                changed_occ.add(oid)
        for v in c.sub.keys() | sub.keys():
            yield 'maintain.diff_binding'
            if v not in c.sub or v not in sub or c.sub[v] != sub[v]:
                changed_var.add(v)
        changed = bool(changed_occ or changed_var)
        if self.mode == 'full' and changed:
            for test in tuple(c.tests):
                yield from self._erase_test(c,test)
            for key in tuple(c.rows):
                yield from self._drop_row(c,bit,key)
        else:
            affected = set()
            for oid in changed_occ:
                yield 'maintain.occurrence_index_lookup'
                for test in c.by_occ.get(oid, ()):
                    yield 'maintain.invalidation_candidate'
                    affected.add(test)
            for var in changed_var:
                yield 'maintain.variable_index_lookup'
                for test in c.by_var.get(var, ()):
                    yield 'maintain.invalidation_candidate'
                    affected.add(test)
            todo = list(affected)
            while todo:
                test = todo.pop()
                yield 'maintain.invalidation_visit'
                if test not in c.tests:
                    continue
                row = c.tests[test][0]
                yield from self._erase_test(c,test)
                if row is not None:
                    for child in c.by_parent.get(row, ()):
                        yield 'maintain.descendant_index_visit'
                        todo.append(child)
                    yield from self._drop_row(c,bit,row)
        c.store, c.sub = store, sub
        # Root values may change under aliases. Rebuild this inexpensive index;
        # maintained root subscriptions are a separate refinement.
        pools = {}
        for oid, value in store.items():
            yield 'maintain.index_visit'
            while isinstance(value, int) and value in sub:
                yield 'maintain.index_deref'
                value = sub[value]
            pred = None if isinstance(value, int) else (value[0],len(value[1]))
            pools.setdefault(pred, []).append(oid)
        c.pools = pools
        for ri, rule in enumerate(self.rules):
            heads = rule.kept + rule.removed
            if not heads:
                raise ValueError('empty rule head')
            root = (ri, (), ())
            if root not in c.rows:
                yield from self._add_row(c, bit, root)
            todo = [root]
            while todo:
                parent = todo.pop()
                yield 'maintain.prefix_visit'
                depth = len(parent[1])
                if depth == len(heads):
                    continue
                head = heads[depth]
                pool = store if isinstance(head,int) else pools.get((head[0],len(head[1])), ())
                for oid in pool:
                    yield 'maintain.index_candidate'
                    if oid in parent[1]:
                        continue
                    test = (parent, oid)
                    if test in c.tests:
                        yield 'maintain.test_reuse'
                        row = c.tests[test][0]
                    else:
                        deps = set()
                        def leaf(v):
                            deps.add(v)
                            return (sub[v],True) if v in sub else (v,False)
                        value = yield from map_term(store[oid], leaf, 'maintain.resolve')
                        bindings = dict(parent[2])
                        for _ in bindings:
                            yield 'maintain.binding_copy'
                        yield 'maintain.match_call'
                        ok = yield from match(head,value,bindings)
                        row = (ri,parent[1]+(oid,),tuple(sorted(bindings.items()))) if ok else None
                        c.tests[test] = (row,frozenset(deps))
                        self._test_storage(len(deps),1)
                        yield from self._index_test(c,test,deps)
                        yield 'maintain.test_create'
                        if row is not None:
                            yield from self._add_row(c,bit,row)
                    if row is not None:
                        todo.append(row)
        c.state = state
        yield 'maintain.storage_account'

    def _test_storage(self, dependencies, direction):
        # High-water counts of retained test/dependency/index records. These are
        # not byte/heap peaks, and include fork copies before their publication.
        self.live_tests += direction
        self.live_dependencies += direction * dependencies
        self.peak_tests = max(self.peak_tests,self.live_tests)
        self.peak_dependencies = max(self.peak_dependencies,self.live_dependencies)
        self.peak_index_edges = max(self.peak_index_edges,2*self.live_tests+self.live_dependencies)

    def _index_test(self,c,test,deps):
        for index,key in ((c.by_occ,test[1]),(c.by_parent,test[0])):
            yield 'maintain.index_add'
            index.setdefault(key,set()).add(test)
        for dep in deps:
            yield 'maintain.index_add'
            c.by_var.setdefault(dep,set()).add(test)

    def _erase_test(self,c,test):
        _,deps = c.tests.pop(test)
        self._test_storage(len(deps),-1)
        yield 'maintain.test_remove'
        for index,key in ((c.by_occ,test[1]),(c.by_parent,test[0])):
            yield 'maintain.index_remove'
            index[key].remove(test)
            if not index[key]: del index[key]
        for dep in deps:
            yield 'maintain.index_remove'
            c.by_var[dep].remove(test)
            if not c.by_var[dep]: del c.by_var[dep]

    def fork(self, source, target):
        return (yield from self._transaction(self._fork(source,target)))

    def _fork(self, source, target):
        """Inherit immutable test payloads and add a distinct context support bit."""
        if target in self.contexts or not 0 <= target < self.max_contexts:
            raise ValueError('invalid fork target')
        if source not in self.contexts:
            return
        old = self.contexts[source]
        for _ in old.store:
            self.counts['maintain.fork_occurrence'] += 1
            yield 'maintain.fork_occurrence'
        for _ in old.sub:
            self.counts['maintain.fork_binding'] += 1
            yield 'maintain.fork_binding'
        for pool in old.pools.values():
            for _ in pool:
                self.counts['maintain.fork_predicate_entry'] += 1
                yield 'maintain.fork_predicate_entry'
        new = Context(state=old.state, store=dict(old.store), sub=dict(old.sub),
                      pools={k:list(v) for k,v in old.pools.items()})
        for test, result in old.tests.items():
            self.counts['maintain.fork_test'] += 1
            yield 'maintain.fork_test'
            new.tests[test] = result
            self._test_storage(len(result[1]),1)
            yield from self._record(self._index_test(new,test,result[1]))
        for key in old.rows:
            self.counts['maintain.fork_row'] += 1
            yield 'maintain.fork_row'
            new.rows.add(key)
            self.rows[key] |= 1 << target
        self.contexts[target] = new

    def complete(self, context, rule):
        if self._poisoned or self._busy:
            raise ValueError('matcher state is unavailable during/after incomplete update')
        return (yield from self._record(self._complete(context,rule)))

    def _complete(self, context, rule):
        c = self.contexts[context]
        length = len(self.rules[rule].kept + self.rules[rule].removed)
        found = []
        positions = {}
        for n, oid in enumerate(c.store):
            yield 'maintain.order_position'
            positions[oid] = n
        for key in c.rows:
            yield 'maintain.complete_scan'
            if key[0] == rule and len(key[1]) == length:
                found.append((key[1],key[2]))
        # Sorting host cost is measured in elapsed time; explicit key visits charged.
        for ids, _ in found:
            for _ in ids:
                yield 'maintain.order_key'
        found.sort(key=lambda row:tuple(positions[oid] for oid in row[0]))
        return found


class Selector:
    def __init__(self, index, context):
        self.index, self.context = index, context

    def candidates(self, rule, state):
        yield from self.index.sync(self.context,state)
        for ids, bindings in (yield from self.index.complete(self.context,rule)):
            yield tuple((oid,self.index.contexts[self.context].store[oid]) for oid in ids), dict(bindings)
