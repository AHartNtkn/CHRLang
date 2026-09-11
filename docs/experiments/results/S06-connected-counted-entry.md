# Connected projection now retains source answer counts

**The term adapter preserves duplicate choices and repeated output slots.** Its
connected source gate passes, so these fixture and interface restrictions no
longer prevent counted experiments. Connectivity alone does not guarantee less
work: the dense control performs more elimination visits than full enumeration.

## What changed

The existing numeric engine already had counted elimination. The term adapter now
encodes every domain choice, including duplicates, against its unique term
dictionary. `Counted` preparation selects the existing weighted algebra. Output
slots map onto unique projected coordinates, so two labels for one variable
produce equal values without creating another choice. `weighted_answers` returns
per-tuple counts; set observations still return distinct tuples.

No reference-interpreter code changed. No replacement solver was introduced.

## What the source gate establishes

The [registration](../registrations/S06-connected-counted-entry.md) specifies
128 configurations: duplicate/plain domains × four connected predicate patterns
× eight visible masks × ordinary/aliased output slots. Both observation modes
are prepared and reused across four requests, producing **1,024 comparisons**.

| Question | Experimental result |
|---|---|
| Do hidden shared variables preserve correlation? | Every weighted tuple matches independent full assignment enumeration. |
| Do duplicate domain alternatives survive? | Counts match the independent scalar CHR interpreter's complete answer bag. |
| Do repeated predicates multiply answers? | They do not; each accepted assignment passes each predicate once. |
| Do repeated output slots multiply answers? | They do not; aliased values and original counts survive. |
| Does contradiction yield an empty result? | Yes, in counted, set, scalar source and reference observations. |
| Can preparation serve changing queries? | Unrestricted, a-restricted, b-restricted and unrestricted-again results all agree. |

The source control lowers each predicate to an explicit finite truth-table rule,
using the test's independent predicate evaluator. Domain alternatives remain
individual CHR choices. All 128 scalar executions have exactly the expected
residual-free answer bag; the unchanged reference yields the corresponding sets.
This checks finite source correspondence. The truth-table source is a correctness
control, not an efficient compilation baseline for a cost comparison.

The pre-existing 1,536-case structural matrix also passes, along with numeric
projection, lifecycle, diagram-source and symbolic-transport regressions: 28 tests
in each feature configuration. The initial alias/count regression failed because
the adapter lacked both operations, then passed after the repair. Scoped Clippy
passes with and without metrics. [Default verification log](s06-connected-counted-entry/default-tests.txt).

## Where elimination removes work—and where it adds it

Both additional controls have eight binary coordinates, two visible outputs and
connected equality relations. Each has two satisfying full assignments. The
existing greedy order chooses the elimination order without inspecting answers.

| Exact operation count | Star | All-pairs dense |
|---|---:|---:|
| Candidate tuples for local binary relations | 28 | 112 |
| Elimination visits | 28 | 504 |
| Largest retained factor table, entries | 2 | 2 |
| Full assignments in enumeration control | 256 | 256 |

The star eliminates leaves before its hub. The dense graph keeps broad variable
scopes even though its factor tables are sparse, so the current elimination loop
still visits their Cartesian products. Small retained tables alone are therefore
insufficient evidence of low computation. These counts exclude domain encoding,
order selection, result decoding and disposal; the next measurements must charge
those phases. Different operation types are not interchangeable units of time.

## Decision and next experiment

T076 remains active. Run the connected lifecycle pilot against existing
enumeration and projection controls, including dense cases, duplicate choices,
aliases, changed restrictions and retained outputs. Measure ordinary counter-free
time and separately requested allocation, charging source construction,
preparation, queries, observation and all disposal. Sparse traversal of dense
factors is a concrete competing implementation to test if attribution confirms
Cartesian visits drive costs.

A counted bag does not encode source scheduling order. The existing
`grouped_expansion_preserves_multiplicity_but_not_source_choice_order` test is an
explicit counterexample. An ordered transport experiment must retain source-choice
provenance and test prefixes, cancellation and restart; claiming raw ordered
substitution from this bag gate would be incorrect. This remains scheduled work,
not a language restriction or a reason to stop investigation.

At this gate, connected lifecycle costs remain more discriminating than another
recognition-stride sweep: projection can eliminate hidden work, while the latest
recognition timings already expose its recognition/recomputation tradeoff.
Whole-call recognition remains a serious alternative at the next portfolio review.
Broader symbolic solving, host interactions and full architecture comparisons
remain active research obligations. Package count is one since the last breadth
review; the research goal is active.
