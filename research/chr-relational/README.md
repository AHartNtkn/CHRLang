# E18 finite relational semantic entry

This prototype presents constructor equations using ordinary flat relations and
ordinary monotone rules. `kernel.py` knows only predicates, ordered fixed node-ID
ports, joins and finite bitset supports. It neither recognizes constructors nor
calls a unifier. `theory.py` generates equality, constructor compatibility,
congruence, value transport and positive reachability rules in that substrate.
Ordinary application rules run in the same closure, including rules that introduce
equalities or descriptors and rules enabled by them.

This is a finite monotone propagation fragment, **not full CHR**. Facts have set
semantics. There is no destructive occurrence consumption, propagation-history
identity, dynamic allocation, choice birth, search fairness or physical fusion.
Every support bit denotes an explicit finite recipe supplied by the harness.
The generic kernel does not inspect the ordinary `bad()` relation; readout rejects
its supported contexts only after successful complete closure. A bound or timeout
is an error, never an answer or refutation.

Application relation schemas must call `value_transport` for all value columns.
Columns representing occurrence identity must be excluded explicitly. Constructor
value columns are transported by the generated axioms. An ordinary premise tests
entailed facts; it does not manufacture an unknown constructor to match a head.
A residual application fact is not proof that every future grounding is valid.
Most-general finite-tree answer correspondence is tested for equation/descriptor
stores, not asserted for arbitrary application rules.

`extract` reifies output roots jointly, preserving free-variable relationships.
`oracle.py` is an independent test-only substitution and occurs-check solver;
production modules do not import it. `gate.py` compares 432 registered recipes,
in 108 batches of four explicit contexts, with that oracle. It enforces a
1,000-round closure bound and a 30-second cooperative per-batch closure deadline.
This deadline is checked inside rule scans and joins; it is not process isolation
or a bound on arbitrary caller-supplied Python code.

Run semantic checks from the repository root:

```sh
PYTHONDONTWRITEBYTECODE=1 python -m unittest discover -s research/chr-relational -v
PYTHONDONTWRITEBYTECODE=1 python research/chr-relational/run_gate.py
```

The isolated runner records source hashes before its first child, creates output
files exclusively, checks the frozen inputs throughout, and preserves failures.
It runs each of the108batches twice in fresh bounded processes. Existing evidence
cannot be overwritten by rerunning the command. The v2 records are diagnostic;
their source hashes were captured after execution. See the versioned result
receipt for authoritative pre-run provenance.

A semantic counterexample drove constructor-column transport: from `f(1,2)`,
`eq(0,1)` and `eq(2,3)`, the ordinary rule `f(0,3) -> hit(0)` must fire in the
supported context. `test_constructor_premises_follow_equal_root_and_child_values`
failed before the correction and passes with ordinary transport rules. An
equation-only grid did not expose this integration defect. Counters are diagnostic
work counts, not evidence of speed or sharing improvement.
