# Complete costs of sparse versus Cartesian elimination

Compare sparse projection, Cartesian projection and enumeration in the existing
connected lifecycle runner. Both projection modes use the same greedy order.
Four families: equality star, equality clique, unequal-name star, and a dense star
whose two-coordinate equality compares an expression with itself. The dense case
isolates nonselective relation traversal; a frontend can simplify that predicate,
so it cannot by itself justify a language or whole-architecture recommendation.

Use4/6 coordinates, ordinary/duplicate domain choices, plain/aliased outputs,
1/4 changed queries, immediate/retained outputs and complete/first-record
cancellation:256 scenarios,768 mode cells. First run every cell twice with the
allocation meter; require identical requested/peak/consumer bytes and complete
release. Check actual weighted answers against independent enumeration outside
measured phases, including the dense family's analytical multiplicities.

Then run five randomized ordinary blocks over all cells, seed607101, with counters
disabled and ordinary allocation. No concurrent builds. Include source creation,
preparation, query setup, first observation, remaining output, consumption and
all disposal. Require both median totals above100 times the largest process
clock p99 in each comparison. Gain requires paired median<=0.9 and all five<1;
loss requires median>=1.1 and all five>1; otherwise uncertain. Compare sparse
against Cartesian and enumeration,512 comparisons. Do not treat preparation
alone or a work-count ratio as complete superiority.

Reuse the runner's30 CPU/45 wall second and1GiB per-process bounds. Preserve failed
case diagnostics and repair consequential failures. Record compact samples and
phase totals without artifact bookkeeping. Inspect phase attribution and ownership
for consequential gains/losses before choosing indexed joins, cheaper preparation,
broader caller observers or another independent direction.

## Challenge the dense gains with existing separable solving

The initial matrix gives sparse projection three gains against enumeration, all
on the dense tautology fixture. Add the existing separable solver as a stronger
control: check reflexive equality, omit its filters, then use current separable
preparation. Charge source inspection, temporary construction and disposal.
No new solver. Repeat all64 dense scenarios across sparse, Cartesian, enumeration
and separable:512 allocation and1280 ordinary runs with the same five-block rule,
seed607102. Keep these fresh comparisons separate from the initial matrix.

After timing, run separate preparation-clock diagnostics for four families,
six coordinates, duplicate domains, two visible outputs, four queries, immediate
consumption and complete exhaustion. Three repetitions for each projection mode
(24 processes). Attribute dictionary construction, relation preparation, ordering
and elimination; do not use instrumented totals as ordinary timing results.
