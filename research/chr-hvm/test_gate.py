import unittest
from continuation_gate import parse_data


class Decoding(unittest.TestCase):
    def test_constructor_states(self):
        self.assertEqual(parse_data('#More{#Done{#A{}}}'),
                         ('More', (('Done', (('A', ()),)),)))
        self.assertEqual(parse_data('#Loop'), parse_data('#Loop{}'))
        self.assertEqual(parse_data('#More{#Loop{}} \x1b[2m#7\x1b[0m', True),
                         parse_data('#More{#Loop}'))
        self.assertNotEqual(parse_data('#More{#Bind{#A}}'), parse_data('#More{#Done{#A}}'))

    def test_incomplete_and_extra_results(self):
        for text in ('#More{#Loop', '#More{#Loop} #Other', '#More{#Loop},', 'λu.#Loop', '#More{λu.#Loop}'):
            with self.assertRaises(ValueError):
                parse_data(text, True)
        with self.assertRaises(ValueError):
            parse_data('#More{#Loop} #7')


if __name__ == '__main__':
    unittest.main()
