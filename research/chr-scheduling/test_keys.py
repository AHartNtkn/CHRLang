import unittest
from dataclasses import replace
from scheduler import Job, state_equal
from source import initial


def compare(a, b, mode, q=1):
    job = Job(state_equal(a, b, mode))
    while not job.done:
        job.advance(q)
    return job.observe(), dict(job.counts)


class ComparisonVariants(unittest.TestCase):
    def test_variants_preserve_all_fields(self):
        a = initial((('p', (0,)),), (0,))
        changes = dict(pending=(), outputs=(1,), next_var=2, next_occurrence=1,
                       store=((0, ('p', (0,))),), sub=((0, ('a', ())),),
                       history=frozenset({(0, (0,))}))
        for mode in ('reverse', 'forward', 'identity'):
            self.assertTrue(compare(a, a, mode)[0])
            for field, value in changes.items():
                self.assertFalse(compare(a, replace(a, **{field: value}), mode)[0], field)

    def test_partial_maps_and_order_do_not_create_false_matches(self):
        a = initial((), (0,))
        b = replace(a, history=frozenset({(0, (1,)), (1, (0,))}))
        c = replace(a, history=frozenset({(1, (0,)), (0, (1,))}))
        for mode in ('reverse', 'forward', 'identity'):
            self.assertEqual(compare(b, c, mode), compare(b, c, mode, 64))
            self.assertTrue(compare(b, c, mode)[0])


if __name__ == '__main__':
    unittest.main()
