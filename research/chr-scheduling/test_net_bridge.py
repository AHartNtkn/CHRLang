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


class NetPolicies(unittest.TestCase):
    def test_private_failure_and_recursive_sibling(self):
        from scheduler import Search
        from source import Rule
        service = NetEquality(('a','f'))
        for policy in ('fifo','round','async'):
            rules = (Rule((), (('start', (0,)),),
                          ('or', ('eq', 0, ('f', (0,))), ('eq', 0, ('a', ())))),)
            search = Search(rules, (('start', (0,)),), (0,), policy, equality=service)
            while not search.exhausted and search.actions < 100000:
                search.advance(8)
            self.assertTrue(search.exhausted)
            self.assertEqual(search.failed, 1)
            self.assertEqual(search.answers, [{'outputs': [('a', ())], 'residual': []}])
            self.assertGreater(search.counts['source.net.reduce'], 0)
            rules = (Rule((), (('start', (0,)),),
                          ('or', ('post', ('spin', ())), ('eq', 0, ('a', ())))),
                     Rule((), (('spin', ()),), ('post', ('spin', ()))))
            search = Search(rules, (('start', (0,)),), (0,), policy, equality=service)
            while not search.answers and search.actions < 100000:
                search.advance(8)
            self.assertEqual(search.answers, [{'outputs': [('a', ())], 'residual': []}])
            self.assertFalse(search.exhausted)


if __name__ == '__main__':
    unittest.main()
