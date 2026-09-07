import unittest
from itertools import combinations
from oracle import evaluate, Ineligible, OracleLimit


def network(edges, given=()):
    return dict(choose=() if given else ('x0', 'x1', 'x2', 'x3'),
                given=given,
                forbid=tuple((f'x{i}', f'x{j}', a, a)
                             for i, j in edges for a in range(3)),
                outputs=tuple((f'o{i}', f'x{i}') for i in range(4)))


class OracleTests(unittest.TestCase):
    def test_graph_cardinalities_from_closed_forms(self):
        # Missing any forbidden edge or merging independent variables changes these.
        cases = [([(0, 1), (1, 2), (2, 3)], 24),
                 ([(0, 0), (1, 2), (2, 3)], 0),
                 ([e for e in combinations(range(4), 2) if e != (0, 1)], 6),
                 (list(combinations(range(4), 2)), 0)]
        for edges, expected in cases:
            with self.subTest(edges=edges):
                result = evaluate(network(edges))
                self.assertEqual(result['raw'], expected)
                self.assertEqual(len(result['answers']), expected)

    def test_unobserved_choice_is_not_erased_from_raw_count(self):
        result = evaluate(dict(choose=('x',), outputs=()))
        self.assertEqual(result['raw'], 3)
        self.assertEqual(result['answers'], (((), ()),))

    def test_repeated_chooser_has_one_successful_arm_per_assignment(self):
        result = evaluate(dict(choose=('x', 'x'), outputs=(('out', 'x'),)))
        self.assertEqual(result['raw'], 3)
        self.assertEqual(result['answers'],
                         (((('out', 0),), ()), ((('out', 1),), ()), ((('out', 2),), ())))

    def test_residual_multiplicity_and_substitution(self):
        result = evaluate(dict(given=(('x', 0), ('y', 1)),
            forbid=(('x', 'y', 0, 0), ('x', 'y', 0, 0)),
            outputs=(('left', 'x'), ('alias', 'x'), ('right', 'y'))))
        self.assertEqual(result['raw'], 1)
        self.assertEqual(result['answers'],
            (((('left', 0), ('alias', 0), ('right', 1)), ((0, 1, 0, 0), (0, 1, 0, 0))),))

    def test_off_diagonal_self_edge_is_retained_but_never_rejects(self):
        result = evaluate(dict(choose=('x',), forbid=(('x', 'x', 0, 1),)))
        self.assertEqual(result['raw'], 3)
        self.assertEqual(result['answers'],
                         (((), ((0, 0, 0, 1),)), ((), ((1, 1, 0, 1),)),
                          ((), ((2, 2, 0, 1),))))

    def test_conflicting_givens_are_refuted(self):
        self.assertEqual(evaluate(dict(given=(('x', 0), ('x', 1))))['raw'], 0)

    def test_uncovered_hole_is_ineligible_not_a_ground_answer_or_failure(self):
        with self.assertRaises(Ineligible):
            evaluate(dict(forbid=(('x', 0, 1, 2),)))
        with self.assertRaises(Ineligible):
            evaluate(dict(outputs=(('out', 'x'),)))

    def test_ground_queries_and_no_or_supplied_assignment(self):
        self.assertEqual(evaluate(dict(forbid=((0, 1, 0, 1),)))['raw'], 0)
        self.assertEqual(evaluate(dict(forbid=((0, 1, 0, 0),)))['raw'], 1)
        q = network([(0, 1), (1, 2), (2, 3)],
                    tuple((f'x{i}', v) for i, v in enumerate((0, 1, 0, 1))))
        self.assertEqual(evaluate(q)['raw'], 1)
        q['given'] = tuple((f'x{i}', 0) for i in range(4))
        self.assertEqual(evaluate(q)['raw'], 0)

    def test_unsupported_constraint_cannot_be_silently_ignored(self):
        with self.assertRaises(Ineligible):
            evaluate(dict(other_constraints=(('fail',),)))

    def test_empty_query_and_ground_chooser_each_have_one_completion(self):
        self.assertEqual(evaluate({})['answers'], (((), ()),))
        self.assertEqual(evaluate(dict(choose=(1, 1)))['raw'], 1)

    def test_outputs_match_source_query_contract(self):
        with self.assertRaises(Ineligible):
            evaluate(dict(outputs=(('out', 0),)))
        with self.assertRaises(Ineligible):
            evaluate(dict(choose=('x',), outputs=(('out', 'x'), ('out', 'x'))))

    def test_invalid_atom_and_oracle_bound_have_different_status(self):
        with self.assertRaises(Ineligible):
            evaluate(dict(given=(('x', 3),)))
        with self.assertRaises(OracleLimit):
            evaluate(dict(choose=('x', 'y')), variable_limit=1)


if __name__ == '__main__':
    unittest.main()
