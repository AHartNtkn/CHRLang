import unittest
from gate import run_batch


class IndependentOracleSmoke(unittest.TestCase):
    def test_first_and_last_registered_batches(self):
        for offset in (0, 428):
            result = run_batch(offset)
            self.assertEqual(result['expected'], result['actual'])
