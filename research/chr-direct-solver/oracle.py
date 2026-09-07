"""Independent finite-assignment oracle; not a CHR or solver runtime.

Strings denote variables; integers 0, 1, 2 denote the three domain atoms.
Answers contain ordered outputs and a sorted multiset of ground forbid tuples.
"""
from itertools import product


class Ineligible(ValueError):
    """Query is outside the exact finite fragment, not necessarily unsatisfiable."""


class OracleLimit(RuntimeError):
    """Eligible query exceeds this exhaustive oracle's resource bound."""


def evaluate(query, variable_limit=8):
    if set(query) - {'choose', 'given', 'forbid', 'outputs'}:
        raise Ineligible('Only choose, given, forbid and outputs are supported')
    choose = tuple(query.get('choose', ()))
    given = tuple(query.get('given', ()))
    forbid = tuple(query.get('forbid', ()))
    outputs = tuple(query.get('outputs', ()))
    variables = set()

    def atom(value):
        if type(value) is not int or value not in range(3):
            raise Ineligible(f'Expected domain atom 0, 1 or 2: {value!r}')

    def endpoint(value):
        if isinstance(value, str):
            variables.add(value)
        else:
            atom(value)

    for value in choose:
        endpoint(value)
    for value, supplied in given:
        endpoint(value)
        atom(supplied)
    for left, right, a, b in forbid:
        endpoint(left)
        endpoint(right)
        atom(a)
        atom(b)
    output_names = set()
    for name, value in outputs:
        if not isinstance(value, str) or name in output_names:
            raise Ineligible('Outputs need unique names and variable endpoints')
        output_names.add(name)
        endpoint(value)
    covered = {v for v in choose if isinstance(v, str)}
    covered.update(v for v, _ in given if isinstance(v, str))
    if variables - covered:
        raise Ineligible(f'Variables without choose or given: {sorted(variables - covered)}')
    variables = tuple(sorted(variables))
    if len(variables) > variable_limit:
        raise OracleLimit(f'{len(variables)} variables exceeds limit {variable_limit}')

    assignments = []
    answers = set()
    for values in product(range(3), repeat=len(variables)):
        assignment = dict(zip(variables, values))

        def value(term):
            return assignment[term] if isinstance(term, str) else term

        if any(value(term) != supplied for term, supplied in given):
            continue
        if any(value(left) == a and value(right) == b for left, right, a, b in forbid):
            continue
        assignments.append(values)
        observed = tuple((name, value(term)) for name, term in outputs)
        residual = tuple(sorted((value(left), value(right), a, b)
                                for left, right, a, b in forbid))
        answers.add((observed, residual))
    return dict(variables=variables, raw=len(assignments),
                answers=tuple(sorted(answers)), assignments=tuple(assignments))
