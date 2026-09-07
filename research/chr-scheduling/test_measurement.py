import unittest
from run_measurements import order_jobs


class MeasurementProtocol(unittest.TestCase):
    def test_all_warmups_precede_measurements(self):
        jobs = [(case, rep) for case in range(5) for rep in range(6)]
        ordered = order_jobs(jobs, False)
        self.assertEqual(len(ordered), 30)
        self.assertTrue(all(rep == 0 for _, rep in ordered[:5]))
        self.assertTrue(all(rep != 0 for _, rep in ordered[5:]))
        self.assertEqual(set(ordered), set(jobs))
        self.assertEqual(ordered, order_jobs(jobs, False))

    def test_memory_repetitions_are_not_warmups(self):
        jobs = [(case, rep) for case in range(5) for rep in range(2)]
        ordered = order_jobs(jobs, True)
        self.assertEqual(set(ordered), set(jobs))
        self.assertEqual(ordered, order_jobs(jobs, True))


if __name__ == '__main__':
    unittest.main()
