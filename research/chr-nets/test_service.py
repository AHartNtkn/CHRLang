import unittest
from check_unification import encoded, listing
from measure import nat
from net import unification, read, data_system


class FiniteService(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.system = data_system()

    def test_quantized_full_request_matches_atomic_interface(self):
        from service import UnificationJob
        a = ['a', []]
        deep = a
        for _ in range(32):
            deep = ['f', [deep]]
        requests = [([], [[0, deep]]), ([], [[0, ['f', [0]]]]),
                    ([[0, 1]], [[1, deep]]), ([], [[0, a], [0, ['b', []]]])]
        for initial, equations in requests:
            table = listing([('Pair', (nat(k), encoded(v))) for k, v in initial])
            pending = listing([('Pair', (encoded(a), encoded(b))) for a, b in equations])
            baseline, before, after = unification(table, pending)
            self.assertEqual(baseline.advance(200000), 'quiescent')
            expected = read(baseline, before), read(baseline, after)
            counters = []
            for quantum in (1, 4, 64):
                job = UnificationJob(table, pending, self.system)
                with self.assertRaises(ValueError):
                    job.observe()
                for _ in range(200000):
                    used = job.advance(quantum)
                    self.assertLessEqual(used, quantum)
                    if job.done:
                        break
                self.assertTrue(job.done)
                self.assertEqual(job.observe(), expected)
                self.assertEqual(job.net.interactions, baseline.interactions)
                self.assertGreater(job.counts['build'], 0)
                self.assertGreater(job.counts['scan'], 0)
                self.assertGreater(job.counts['read'], 0)
                counters.append(job.counts)
            self.assertEqual(counters[0], counters[1])
            self.assertEqual(counters[0], counters[2])

    def test_small_request_progresses_during_large_build_and_read(self):
        from service import UnificationJob
        deep = ['a', []]
        for _ in range(128):
            deep = ['f', [deep]]
        big_input = listing([('Pair', (encoded(0), encoded(deep)))])
        small_input = listing([('Pair', (encoded(0), encoded(0)))])
        for phase in ('build', 'read'):
            big = UnificationJob(('Nil', ()), big_input, self.system)
            while big.phase != phase:
                big.advance(1)
                self.assertFalse(big.done)
            small = UnificationJob(('Nil', ()), small_input, self.system)
            for _ in range(1000):
                big.advance(1)
                small.advance(1)
                if small.done:
                    break
            self.assertTrue(small.done)
            self.assertFalse(big.done)
            self.assertEqual(big.phase, phase)
            self.assertEqual(small.observe()[1][0], 'Some')

    def test_partial_build_and_read_are_not_observable(self):
        from service import UnificationJob
        job = UnificationJob(('Nil', ()), listing([('Pair', (encoded(0), encoded(['a', []])))]), self.system)
        phases = set()
        while not job.done:
            phases.add(job.phase)
            with self.assertRaises(ValueError):
                job.observe()
            job.advance(1)
        self.assertTrue({'build', 'reduce', 'scan', 'read'} <= phases)


if __name__ == '__main__':
    unittest.main()
