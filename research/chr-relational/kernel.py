"""Finite monotone relations. Predicate names have no kernel semantics."""
from dataclasses import dataclass
from collections import defaultdict
from time import monotonic


@dataclass(frozen=True)
class Atom:
    predicate: str
    ports: tuple


@dataclass(frozen=True)
class Rule:
    premises: tuple
    conclusions: tuple


class Engine:
    def __init__(self, universe, context_mask, rules):
        self.universe = frozenset(universe)
        if context_mask <= 0:
            raise ValueError('positive context mask required')
        self.context_mask = context_mask
        self.rules = tuple(rules)
        self.facts = {}
        self.index = defaultdict(dict)
        self.stats = defaultdict(int)
        self.complete = False
        for rule in self.rules:
            bound = {p for a in rule.premises for p in a.ports if isinstance(p, str)}
            for a in rule.premises + rule.conclusions:
                for p in a.ports:
                    if isinstance(p, str):
                        if a in rule.conclusions and p not in bound:
                            raise ValueError('rule is not range restricted')
                    elif p not in self.universe:
                        raise ValueError('constant outside universe')

    def add(self, fact, support):
        if any(type(p) is not int or p not in self.universe for p in fact.ports):
            raise ValueError('fact ports must be fixed universe IDs')
        if support < 0 or support & ~self.context_mask:
            raise ValueError('support outside explicit contexts')
        previous = self.facts.get(fact, 0)
        self.stats['support_unions'] += 1
        added = support & ~previous
        if added:
            self.facts[fact] = previous | support
            self.index[(fact.predicate, len(fact.ports))][fact] = previous | support
            self.stats['support_bits_added'] += added.bit_count()
            self.stats['facts_added'] += not previous
            self.complete = False
        return bool(added)

    def support(self, fact):
        return self.facts.get(fact, 0)

    def saturate(self, max_rounds=1000, deadline=None):
        for _ in range(max_rounds):
            self.stats['rounds'] += 1
            changed = False
            for rule in self.rules:
                if deadline is not None and monotonic() > deadline:
                    raise TimeoutError('relational closure deadline')
                self.stats['rule_scans'] += 1
                partial = [({}, self.context_mask)]
                for premise in rule.premises:
                    following = []
                    rows = tuple(self.index[(premise.predicate, len(premise.ports))].items())
                    for binding, support in partial:
                        for fact, available in rows:
                            if deadline is not None and monotonic() > deadline:
                                raise TimeoutError('relational closure deadline')
                            self.stats['joins'] += 1
                            self.stats['support_intersections'] += 1
                            common = support & available
                            if not common:
                                continue
                            result = dict(binding)
                            for pattern, value in zip(premise.ports, fact.ports):
                                if isinstance(pattern, str):
                                    if pattern in result and result[pattern] != value:
                                        break
                                    result[pattern] = value
                                elif pattern != value:
                                    break
                            else:
                                self.stats['matches'] += 1
                                following.append((result, common))
                    partial = following
                    if not partial:
                        break
                for binding, support in partial:
                    for conclusion in rule.conclusions:
                        fact = Atom(conclusion.predicate, tuple(binding[p] if isinstance(p, str) else p
                                                               for p in conclusion.ports))
                        changed = self.add(fact, support) or changed
            if not changed:
                self.complete = True
                return
        raise RuntimeError('relational closure round bound exhausted')
