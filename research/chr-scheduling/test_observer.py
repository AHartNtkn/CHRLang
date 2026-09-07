import unittest
from observer import EquivalenceJob


def finish(a, b, quantum=1):
    job = EquivalenceJob(a, b)
    with unittest.TestCase().assertRaises(ValueError):
        job.observe()
    for _ in range(100000):
        assert job.advance(quantum) <= quantum
        if job.done:
            return job.observe(), dict(job.counts)
    raise AssertionError('observer did not finish')


def answer(outputs=(), residual=()):
    return dict(outputs=outputs, residual=residual)


class ExactObserver(unittest.TestCase):
    def test_joint_aliases_and_multiplicity(self):
        a = answer((0,), (('p', (0, 1)), ('p', (1, 1))))
        b = answer((8,), (('p', (9, 9)), ('p', (8, 9))))
        self.assertTrue(finish(a, b)[0])
        self.assertFalse(finish(a, answer((8,), (('p', (9, 9)), ('p', (8, 10)))))[0])
        self.assertFalse(finish(a, answer((0,), (('p', (0, 1)),)))[0])

    def test_backtracking_and_quantum_invariance(self):
        a = answer((), (('p', (0, 1)), ('p', (1, 1))))
        b = answer((), (('p', (9, 9)), ('p', (8, 9))))
        expected = finish(a, b)
        self.assertTrue(expected[0])
        self.assertEqual(expected, finish(a, b, 8))
        self.assertEqual(expected, finish(a, b, 64))

    def test_late_mismatch_is_private(self):
        a = answer(tuple(range(200)))
        b = answer(tuple(range(199)) + (0,))
        job = EquivalenceJob(a, b)
        job.advance(8)
        self.assertFalse(job.done)
        self.assertFalse(finish(a, b)[0])


if __name__ == '__main__':
    unittest.main()
