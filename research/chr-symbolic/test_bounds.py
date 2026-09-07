import unittest
from derive_bounds import derive

class BoundDerivationTests(unittest.TestCase):
    def test_both_arms_allocate_fresh_variables_and_constructors(self):
        case={'id':'manual-or','rules':[{'kept':(),'removed':(('start',(0,)),),'guards':(),
              'body':('or',('eq',0,('f',(1,))),('eq',0,('g',(2,))))}],
              'constraints':(('start',(0,)),),'outputs':(0,),'budget':30,'limit':None,
              'exhausted':True,'raw':2,'expected':[
                  {'outputs':[('f',(1,))],'residual':[]},
                  {'outputs':[('g',(2,))],'residual':[]}]}
        self.assertEqual(derive(case)['bounds'],{'transitions':5,'nodes':6,'occurrences':1,'pending':1,'service':8})

    def test_successful_guard_freshness_counts_without_body_use(self):
        case={'id':'manual-guard','rules':[{'kept':(),'removed':(('start',(0,)),),'guards':((1,1),),'body':('true',)}],
              'constraints':(('start',(0,)),),'outputs':(0,),'budget':20,'limit':None,
              'exhausted':True,'raw':1,'expected':[{'outputs':[0],'residual':[]}]}
        self.assertEqual(derive(case)['bounds'],{'transitions':4,'nodes':3,'occurrences':1,'pending':1,'service':8})
