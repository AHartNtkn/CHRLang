import unittest
from machine import Rule
from direct import Direct
from symbolic import Bounded
from check_cases import equivalent

class ExtensionTests(unittest.TestCase):
    def test_extension_matches_fresh_and_direct_after_observations(self):
        cases=[
            (Rule((),(('p',(0,)),),('or',('post',('q',(0,))),('eq',0,('a',())))),6),
            (Rule((),(('p',(0,)),),('or',('eq',0,('f',(0,))),('true',))),5),
            (Rule((('p',(0,)),),(),('true',)),3),
            (Rule((),(('p',(0,)),),('post',('q',(0,)))),2),
        ]
        for rule,nodes in cases:
            kwargs=dict(nodes=nodes,occurrences=2,pending=2,service=3)
            inc=Bounded([rule],[('p',(0,))],[0],transitions=0,representation='terms',**kwargs)
            for bound in range(9):
                inc.extend(bound)
                fresh=Bounded([rule],[('p',(0,))],[0],transitions=bound,representation='terms',**kwargs)
                direct=Direct([rule],[('p',(0,))],[0],transitions=bound,**kwargs)
                actual=inc.answers()
                for other in (fresh,direct):
                    expected=other.answers()
                    self.assertEqual(len(actual),len(expected))
                    self.assertTrue(all(any(equivalent(a,b) for b in expected) for a in actual))
                    self.assertEqual(inc.models,other.models)
                    self.assertEqual(inc.boundary_status(),other.boundary_status())
            with self.assertRaises(ValueError):inc.extend(7)
