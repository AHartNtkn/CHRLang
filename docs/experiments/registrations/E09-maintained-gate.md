# A3 / E09 maintained matching gate

Status: v2 semantic matrix registered before execution; v1 evidence retained. The protocol below
authorizes only the finite gate; larger cost/work runs need a separate protocol.

## Distinguishing question and fixed assumptions

Can retaining partial matches across real source updates save repeated discovery,
and which alias/consumption patterns make invalidation expensive? Compare current
prefix recomputation, the same maintained representation with full invalidation,
and selective delta maintenance. This controls representation separately from
reuse. It does not yet decide whether beta memories, eager matching, or a prefix
join plan are the right architecture; alternative join strategies remain open in the [primary-source assessment](../results/A3-matching-directions.md).

For this local contrast, retain the existing finite terms/unifier, pure guards,
explicit choices and recorded source selector. A1 constructor relationalization
is not a dependency. Matched logical effects are required where the ablation
predicts them; physical matching work and administrative counts may differ.
Source scheduling is a comparison control, not an adopted language requirement.

## Candidate obligations

Rows identify a rule prefix, ordered distinct occurrence IDs, matched rule-variable
values and supported contexts. Equal-valued occurrences remain separate resources.
Maintain enough dependency information for successful and suspended/failed tests:
a binding can enable a previously absent row. Record inspected alias links, not
only terminal representatives. Changes to one context must not invalidate facts
in incompatible contexts.

Insertion must find matches even when its head position is in the middle of a
rule. Consumption subtracts only affected support. Alias changes invalidate and
recompute the tests that read them. Complete rows still require live occurrences,
pure guards, absent propagation history and selector revalidation. A newly enabled
earlier candidate can supersede a retained later candidate. Metadata scans used
for invalidation must be charged; an index is not assumed constant-cost.

## Required gates and measurements

First project a small supported store to each context and compare ordered matches
with the existing independent prefix/tree control. Then integrate the retained
matcher into actual source execution and compare complete outputs, joint residual
aliases/multiplicities, exhaustion and the declared logical selection. A standalone
store-update benchmark is insufficient for a whole-source conclusion.

Hand regressions include repeated variables, equal-valued distinct occurrences,
alias chains enabling suspended tests, support-local consumption, different
bindings for one tuple across contexts, history on old/new support, middle-head
insertion and newly earlier selection. The main family keeps left/edge prefixes
while consuming arriving right occurrences, with a consuming variant to expose
invalidation. The exact producer is frozen below.

Count prefix/test rows created, retained, invalidated and recomputed; candidate
scans, structural/dependency reads, support operations, index updates and metadata
scans; live/peak rows and update queues; selector/history/guard rejection; source
and final observation work. Semantic runs are not timing comparisons. Later
System timing must include preparation, invalidation, reclamation and output,
with separate memory instrumentation and a credible recomputation control.

## Recording and scope

The comparative runner must reserve versioned outputs exclusively and freeze
code, fixtures, oracle, registration, runner and auditor before its first child.
Retain failures and bounds as unresolved observations. Replay semantic and
predicted deterministic fields; do not require identical counters when the
mechanism changes work. No reference modifications are permitted.

Maintained partial joins, seed-ordered recomputation, lazy witness search and
multiway/relational join planning remain different architectural questions.
Success or failure of the first delta-maintenance representation does not close
them. Full source integration and meaningful adverse cases precede performance
conclusions.


## Frozen v1 protocol

`run_maintained_gate.py` runs four supported-store families in generator order,
each full then selective: alias-support (four stages, eight contexts),
middle-arrival (three stages, two contexts), distinct-bindings (two stages, two
contexts), root-predicate (two stages, one context). This is eight children and
88 context projections per pass. Each update checks all ordered complete matches
and support masks against exhaustive ordinary prefix matching without a predicate
index. Internal successful/failed dependency regressions additionally have unit tests.

Source cells iterate keep/consume, N=4, B=1/2, R=1, prefix/full/selective,
Q=1/8: **24 cells**, not 48. Repeat the entire 32-cell store/source order once
as replay, giving **64 isolated children**. All source transitions, selectors,
pending/store/substitution/history and observations are checked against the
independently implemented tree control. Full ground answers also satisfy the
analytic formula in `maintained_cases.expected`. Replay requires identical
semantic observations and deterministic counters. Quanta must preserve outcomes
and source step counts. Equal wall times are neither expected nor evidence.

The six rules are, in order: the kept-prefix or consuming left/edge/right join;
pump(seq(A,B)) posting pump(A),pump(B); pump(put(T)) posting T;
pump(bind(X,Y)) equating X,Y; pump(nothing) succeeding; and start explicitly
choosing a tag then posting the balanced command tree. Ordinary rule priority
is a comparison control. The command tree posts each round's right(key,v),
replenishes left/edge for subsequent consuming rounds, then posts alias and broad
right constraints and three binding commands. The exact immutable fixture,
matcher, source implementation, independent tree control, registration, runner
and auditor are SHA-256 frozen before any child starts.

Expected each branch: outputs [tag(branch),awaken,awaken,all]; NR base out facts,
one alias out, one broad out using the earliest broad tuple. Kept residuals
include 4N+2 left/edge facts; consuming residuals include 2N-2 broad left/edge
facts. No pump/start/right remains. Raw and unique counts both equal B, zero
failed alternatives, exhausted. These formulas do not substitute for the small
per-transition oracle.

Each fresh child has 30 seconds and 1 GiB address-space limits. Source limits
are 100,000 transitions and 10,000,000 instrumented actions, including fork and
maintenance. The recorded high-water counters are retained records/index edges,
not measured heap. Source work counters exclude oracle work. Full results and
errors are retained, with exclusive output creation. On a failed child, preserve
the prefix and investigate before a new version. No favorable-cell selection.

Interpretation: a passing gate establishes bounded source correspondence and
support/dependency behavior for this grid. Counter differences may motivate a
separately registered resource-sizing run over the 36 larger fixture proposals.
Neither a speed claim nor general fairness, cancellation or reclamation claim
follows. New matching tests still run per context; global row support reuse must
not be reported as sharing those computations.


## Frozen v2 reproducibility and sensitivity protocol

V1's 64 children all passed semantic checks. Exact counter replay failed in four
selective consuming pairs: hash-dependent set iteration changes whether an already
affected descendant is also enqueued by its ancestor. Two charged counters vary
together (`maintain.descendant_index_visit`, `maintain.invalidation_visit`); actual
test removals, matches, source effects and observations agree. Frozen v1 inputs
and the exact discrepancy are preserved in the inputs archive and replay diagnostic.

V2 keeps the algorithm and full 32-cell grid, setting PYTHONHASHSEED=0 in each
child. Append eight selective consuming cells: seeds1/2, B1/2, Q1/8, N4/R1.
Then repeat all40 cells exactly, giving80 children. Exact per-seed result/counter
replay remains mandatory. Across seeds, only the two identified duplicate-visit
counts may differ; all other recorded deterministic results must agree. This
adds a semantic sensitivity check rather than changing the invalidation algorithm
for reproducibility. Hash-seed timing sensitivity remains a later cost obligation.
All v1 resource limits, independent controls and interpretation limits apply.
