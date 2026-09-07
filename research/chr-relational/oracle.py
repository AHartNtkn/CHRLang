"""Test-only independent substitution/occurs-check finite tree oracle.

No production relational module imports this module. Integers are variables;
(name, children) tuples are constructors.
"""

def solve(equations, outputs):
    substitutions = {}
    def dereference(term):
        while isinstance(term, int) and term in substitutions:
            term = substitutions[term]
        return term
    def occurs(variable, term):
        term = dereference(term)
        return term == variable if isinstance(term, int) else any(occurs(variable, x) for x in term[1])
    pending = list(equations)
    while pending:
        left, right = map(dereference, pending.pop())
        if left == right:
            continue
        if isinstance(right, int) and not isinstance(left, int):
            left, right = right, left
        if isinstance(left, int):
            if occurs(left, right):
                return None
            substitutions[left] = right
        elif left[0] != right[0] or len(left[1]) != len(right[1]):
            return None
        else:
            pending.extend(zip(left[1], right[1]))
    holes = {}
    def reify(term):
        term = dereference(term)
        if isinstance(term, int):
            if term not in holes:
                holes[term] = len(holes)
            return holes[term]
        return (term[0], tuple(reify(x) for x in term[1]))
    return tuple(reify(x) for x in outputs)
