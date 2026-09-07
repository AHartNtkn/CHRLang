import unittest
from source import Rule, initial, StepJob


def finish(job, quantum=1):
    for _ in range(200000):
        assert job.advance(quantum) <= quantum
        if job.done:
            return job.observe()
    raise AssertionError('source step did not finish')


class SourceSteps(unittest.TestCase):
    def test_nonbinding_match_waits_for_equation(self):
        rules = (Rule((), (('p', (('a', ()),)),), ('post', ('done', ()))),)
        state = initial((('p', (0,)),), (0,))
        state = finish(StepJob(state, rules))[1]
        event = finish(StepJob(state, rules))
        self.assertEqual(event[0], 'answer')
        self.assertEqual(event[1]['residual'], [('p', (0,))])
        from dataclasses import replace
        state = replace(state, pending=(('eq', 0, ('a', ())),))
        state = finish(StepJob(state, rules))[1]
        state = finish(StepJob(state, rules))[1]
        self.assertEqual(state.pending, (('post', ('done', ())),))

    def test_failure_is_private_and_source_snapshot_stable(self):
        state = initial((), (0,))
        from dataclasses import replace
        state = replace(state, pending=(('eq', 0, ('s', (0,))),))
        for quantum in (1, 8):
            job = StepJob(state, ())
            with self.assertRaises(ValueError):
                job.observe()
            self.assertEqual(finish(job, quantum)[0], 'failed')
            self.assertEqual(state.sub, ())

    def test_tuple_scanning_yields(self):
        rules = (Rule((), (('q', (0,)), ('q', (1,))), ('true',)),)
        state = initial(tuple(('p', (i,)) for i in range(16)), ())
        while state.pending:
            state = finish(StepJob(state, rules))[1]
        job = StepJob(state, rules)
        job.advance(1)
        self.assertFalse(job.done)
        self.assertEqual(finish(job, 8)[0], 'answer')
        self.assertGreaterEqual(job.counts['tuple'], 240)


if __name__ == '__main__':
    unittest.main()
