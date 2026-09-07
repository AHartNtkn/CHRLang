import unittest
from source import drain
from net_bridge import NetEquality, Codec


class NetBoundary(unittest.TestCase):
    def test_codec_alias_ids_and_arities(self):
        codec = Codec(('a','f','p'))
        terms = (0, 12, ('a', ()), ('p', (0, ('f', (12,)), 0)))
        for term in terms:
            self.assertEqual(drain(codec.decode(drain(codec.encode(term)))), term)
        with self.assertRaises(ValueError):
            drain(codec.encode(('unknown', ())))

    def test_failure_and_success_do_not_modify_input(self):
        service = NetEquality(('a','f','p'))
        original = ((0, 1),)
        self.assertIsNone(drain(service.solve(original, ((1, ('f', (0,))),))))
        result = drain(service.solve(original, ((1, ('a', ())),)))
        self.assertIsNotNone(result)
        self.assertEqual(original, ((0, 1),))

    def test_publication_waits_for_full_decode(self):
        from scheduler import Job
        job = Job(NetEquality(('a',)).solve((), ((0, ('a', ())),)))
        job.advance(1)
        with self.assertRaises(ValueError):
            job.observe()
        while not job.done:
            job.advance(8)
        self.assertIsNotNone(job.observe())
        self.assertGreater(job.counts['codec.decode'], 0)


if __name__ == '__main__':
    unittest.main()
