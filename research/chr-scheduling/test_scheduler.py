import unittest
from scheduler import Search, Support
from source import Rule


def run(policy, grouping=True, quantum=8):
    rules = (Rule((), (('p', (0,)),), ('or', ('eq', 0, ('a', ())), ('eq', 0, ('a', ())))),)
    search = Search(rules, (('p', (0,)),), (0,), policy, grouping=grouping)
    for _ in range(100000):
        search.advance(quantum)
        if search.exhausted:
            return search
    raise AssertionError('search did not finish')


class Policies(unittest.TestCase):
    def test_support_preserves_paths(self):
        root = Support.root()
        fork = Support.union(root.arm(0), root.arm(1))
        nested = Support.union(fork.arm(0), fork.arm(1))
        self.assertEqual(nested.count, 4)
        self.assertEqual(set(nested.paths()), {(0,0), (0,1), (1,0), (1,1)})

    def test_duplicate_lineages_keep_raw_count(self):
        for policy in ('fifo', 'round', 'async'):
            for grouping in (False, True):
                result = run(policy, grouping)
                self.assertEqual(result.raw, 2)
                self.assertEqual(len(result.answers), 1)
                self.assertEqual(result.answers[0]['outputs'], [('a', ())])

    def test_nested_choice_sharing_and_private_updates(self):
        body = ('or', ('or', ('eq', 0, ('a', ())), ('eq', 0, ('b', ()))),
                ('or', ('eq', 0, ('a', ())), ('eq', 0, ('b', ()))))
        rules = (Rule((), (('p', (0,)),), body),)
        for policy in ('fifo', 'round', 'async'):
            search = Search(rules, (('p', (0,)),), (0,), policy)
            while not search.exhausted and search.actions < 100000:
                search.advance(8)
            self.assertTrue(search.exhausted)
            self.assertEqual(search.raw, 4)
            self.assertEqual({a['outputs'][0] for a in search.answers}, {('a', ()), ('b', ())})
        self.assertLess(run('round', True).source_jobs, run('round', False).source_jobs)

    def test_finite_answer_with_recursive_sibling(self):
        rules = (Rule((), (('start', (0,)),),
                      ('or', ('post', ('spin', ())), ('eq', 0, ('a', ())))),
                 Rule((), (('spin', ()),), ('post', ('spin', ()))))
        for policy in ('fifo', 'round', 'async'):
            search = Search(rules, (('start', (0,)),), (0,), policy)
            while not search.answers and search.actions < 100000:
                search.advance(8)
            self.assertEqual(search.answers, [{'outputs': [('a', ())], 'residual': []}])
            self.assertFalse(search.exhausted)

    def test_quantized_replay(self):
        for policy in ('fifo', 'round', 'async'):
            a, b = run(policy), run(policy)
            self.assertEqual(a.counts, b.counts)
            self.assertEqual(a.answer_actions, b.answer_actions)


if __name__ == '__main__':
    unittest.main()
