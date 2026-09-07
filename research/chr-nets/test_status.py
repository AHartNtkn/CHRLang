import unittest
from net import Net, data_system, controller_deltas, DATA
from service import UnificationJob


class ControllerStatus(unittest.TestCase):
    def test_count_through_build_and_rewrites(self):
        system = data_system()
        table = ('Nil', ())
        term = ('App', (('Z', ()), ('Nil', ())))
        equation = ('Pair', (('Ref', (('Z', ()),)), term))
        job = UnificationJob(table, ('Cons', (equation, table)), system,
                             status='count', deltas=controller_deltas(system))
        while not job.done:
            job.advance(1)
            actual = sum(t is not None and t not in DATA and t != 'Out' for t in job.net.nodes)
            self.assertEqual(job.net.controller_count, actual)
        self.assertEqual(job.net.controller_count, 0)
        self.assertGreater(job.counts['maintain'], 0)
        self.assertEqual(job.counts['status'], 1)
        self.assertNotIn('scan', job.counts)

    def test_stuck_controller_is_counted(self):
        system = data_system()
        net = Net(system, deltas=controller_deltas(system))
        net.node('U')
        self.assertFalse(net.ready)
        self.assertEqual(net.controller_count, 1)

    def test_stuck_service_cannot_publish(self):
        system = data_system()
        job = UnificationJob(('Nil', ()), ('Nil', ()), system,
                             status='count', deltas=controller_deltas(system))
        job.net.node('U')
        with self.assertRaisesRegex(ValueError, 'stuck service controller'):
            while not job.done:
                job.advance(1)
        with self.assertRaises(ValueError):
            job.observe()
