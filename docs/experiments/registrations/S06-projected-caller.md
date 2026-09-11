# Projected answers entering a consuming caller

Test whether weighted answers can replace source alternatives inside ordinary
consuming execution. Start with a minimal finite caller so differences have an
unambiguous cause: one chosen name, duplicate alternatives, propagation history,
a scarce token and two competing consumers. This is a correctness experiment;
there will be no timing claim from this small fixture.

Compare the original choice rule with (1) binding the projected value before the
caller starts, (2) expanding weighted tuples at the original rule in sorted order,
and (3) expanding at that rule in the original alternative order. The third
transport retains the source choice plan alongside the prepared relation. It must
use the relation's weights and validate each occurrence; the relation alone is
not presumed to contain source order.

Hypotheses: early binding can change consumption; preserving the binding point
can restore complete answer bags; preserving the choice tree and alternative
order additionally restores ordered delivery and cancellation checkpoints.
Check the hypotheses, including counterexamples, rather than assuming them.

Matrix: three domains [a,b], [b,a], [a,b,a]; forward/reverse alternatives;
three token-rule priorities (before the name-specific consumer, between
that consumer and launch, after launch); plain/aliased outputs; fresh variable offsets 10/1000:72 cases.
Run each once in default and counter-free configurations. Independent scalar
execution checks complete raw multiplicities; reference execution checks unique
ordered answers and one-step delivery/exhaustion up to 10000 steps. Compare every
prefix, retain delivered answers across producer disposal, and restart. Exact
answers and residual token/history occurrences matter, not just tuple values.

Record early-binding bag differences and sorted-expansion order differences.
Require the source-plan transport to match all reference checkpoints and scalar
raw bags. A success establishes a working transport for these fixtures, not an
inference procedure for arbitrary consuming rules. Next investigate connected
hidden variables and different derivation lengths, where retaining the whole
source choice plan could erase projection's computational benefit. Prefer that
investigation to another local allocation change because it determines whether
measured logical-query gains can survive complete execution.

## Follow-up registered after the first 72 cases

The first priority arrangement produced no early-binding counterexample: its
name-specific consumer always followed launch. Put that consumer before launch
and place the competing token rule before/between/after them. This produces 24
early-binding bag differences and 24 sorted-order differences; retained source
order matches 1072 checkpoints. Keep that arrangement as the matrix above.

Now cross all 72 cases with a-branch delay 0/1/4 (216 cases total). A delayed branch
posts a recursive wait constraint and binds only after it completes; b remains
immediate. The repair must retain these binding points, recursion and choice
shape, not just reorder final tuples. Use the same 10000-step bound and checks.
Report the cost implication honestly: retained source scheduling is still work;
this test does not demonstrate an acceleration.
