import unittest
from frontend import decode

class SourceText(unittest.TestCase):
    def test_nested_choices_and_identities(self):
        rules, queries = decode('p:v7:a\n7\nwatch:v7;p:v7;(=:v7:a|(!|q:v7)),z\nNEXT\np:v9:b\n9\n')
        self.assertEqual(rules, [dict(kept=[['watch',[7]]], removed=[['p',[7]]], body=[['or',[['eq',7,'a']],[['or',[['fail']],[['add',['q',[7]]]]]]],['add',['z',[]]]])])
        self.assertEqual(queries, [dict(query=[['p',[7,'a']]],outputs=[7]),dict(query=[['p',[9,'b']]],outputs=[9])])
    def test_empty_query_and_body(self):
        self.assertEqual(decode('-\n-\n-;p;-\n'),([dict(kept=[],removed=[['p',[]]],body=[])],[dict(query=[],outputs=[])]))
    def test_reject_malformed_structure(self):
        for text in ['p\n-\n-;p;(q|r\n','p\n-\n-;p;q)\n','p\n-\n-;p;(q|r|s)\n','p\n-\n-;p;=:v1\n','p\n-\n-;p;q\nNEXT\np\n-\nextra\n']:
            with self.subTest(text=text),self.assertRaises(ValueError):decode(text)
if __name__=='__main__':unittest.main()
