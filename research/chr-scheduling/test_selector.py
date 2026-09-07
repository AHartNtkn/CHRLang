import unittest
from dataclasses import replace
from source import Rule, initial, StepJob
from test_source import finish


class PredicateSelection(unittest.TestCase):
    def test_root_alias_and_unknown_root(self):
        state = replace(initial((), (0,)), store=((0, 0),), next_occurrence=1)
        rules = (Rule((), (('p', ()),), ('true',)),)
        self.assertEqual(finish(StepJob(state, rules, 'predicate'))[0], 'answer')
        bound = replace(state, sub=((0, 1), (1, ('p', ()))), next_var=2)
        job = StepJob(bound, rules, 'predicate')
        self.assertEqual(finish(job)[0], 'continue')
        self.assertEqual(job.action, ('apply', 0, (0,)))
        self.assertGreaterEqual(job.counts['index_deref'], 2)

    def test_same_order_and_distinct_occurrences(self):
        state = replace(initial((), ()), store=((0, ('q', ())), (1, ('p', ())),
                                               (2, ('p', ())), (3, ('q', ()))), next_occurrence=4)
        rules = (Rule((), (('p', ()), ('q', ())), ('true',)),)
        jobs = [StepJob(state, rules, mode) for mode in ('scan', 'predicate')]
        results = [finish(j) for j in jobs]
        self.assertEqual(results[0], results[1])
        self.assertEqual(jobs[1].action, ('apply', 0, (1, 0)))
        same = (Rule((), (('p', ()), ('p', ())), ('true',)),)
        job = StepJob(state, same, 'predicate')
        finish(job)
        self.assertEqual(job.action, ('apply', 0, (1, 2)))


class PrefixSelection(unittest.TestCase):
    def test_failed_prefix_rolls_back_and_keeps_order(self):
        state = replace(initial((), ()), store=(
            (0, ('p', (('a', ()), ('b', ())))),
            (1, ('p', (('c', ()), ('c', ())))),
            (2, ('q', (('c', ()),)))), next_occurrence=3)
        rules = (Rule((), (('p', (0,0)), ('q', (0,))), ('true',)),)
        jobs = [StepJob(state, rules, mode) for mode in ('scan','predicate','prefix')]
        results = [finish(j) for j in jobs]
        self.assertEqual(results[0], results[2])
        self.assertEqual(results[1], results[2])
        self.assertEqual(jobs[2].action, ('apply', 0, (1,2)))

    def test_history_exclusion_does_not_stop_prefix_search(self):
        state = replace(initial((), ()), store=((0, ('p', ())), (1, ('p', ()))),
                        history=frozenset({(0, (0,1))}), next_occurrence=2)
        rules = (Rule((('p', ()), ('p', ())), (), ('true',)),)
        job = StepJob(state, rules, 'prefix')
        finish(job)
        self.assertEqual(job.action, ('apply', 0, (1,0)))


if __name__ == '__main__':
    unittest.main()
