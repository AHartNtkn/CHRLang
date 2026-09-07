import json
from pathlib import Path
import unittest
from check_unification import canonical


class DirectControls(unittest.TestCase):
    def test_reference_requests_and_transactional_inputs(self):
        from direct_unification import Direct
        source = Path(__file__).resolve().parents[2] / 'docs/experiments/results/E06-unification-reference.jsonl'
        for line in source.read_text().splitlines():
            case = json.loads(line)
            for mode in ('map', 'table', 'encoded'):
                solver = Direct(case['initial'], [case['equation']], mode)
                success = solver.solve()
                original, result, values = solver.extract(success, range(3))
                self.assertEqual(original, case['initial'])
                observed = [canonical(values)] if result is not None else []
                self.assertEqual(observed, [canonical(a) for a in case['answers']], (case['id'], mode))

    def test_registered_cost_workloads(self):
        from direct_unification import Direct
        from unification_workloads import cases
        from check_unification import encoded, listing
        from measure import nat
        from net import unification, read
        from direct_unification import decoded, extract_tables
        from check_unification import items, number
        count = 0
        for case in cases():
            count += 1
            expected = [canonical(a) for a in case['expected']]
            for mode in ('map', 'table', 'encoded'):
                solver = Direct(case['initial'], case['equations'], mode)
                original, result, values = solver.extract(solver.solve(), case['selected'])
                self.assertEqual(original, case['initial'])
                self.assertEqual([canonical(values)] if result is not None else [], expected, (case['id'], mode))
            table = listing([('Pair', (nat(k), encoded(v))) for k, v in case['initial']])
            equations = listing([('Pair', (encoded(a), encoded(b))) for a, b in case['equations']])
            net, original, result = unification(table, equations)
            self.assertEqual(net.advance(200000), 'quiescent', case['id'])
            net.check()
            self.assertEqual(read(net, original), table)
            option = read(net, result)
            bindings = None if option[0] == 'None' else [[number(p[1][0]), decoded(p[1][1])] for p in items(option[1][0])]
            _, result, values = extract_tables(case['initial'], bindings, case['selected'])
            self.assertEqual([canonical(values)] if result is not None else [], expected, case['id'])
        self.assertEqual(count, 96)

    def test_measurement_interface(self):
        from measure_unification import measure
        from unification_workloads import cases
        case = next(c for c in cases() if c['id'] == 'chain-occurs-n1-d4')
        for mode in ('map', 'table', 'encoded', 'net'):
            self.assertTrue(measure(case, mode)['passed'])


if __name__ == '__main__':
    unittest.main()
