# Independent architectural entries after the first relational gate

These are concrete next investigations for A3 and A6. Neither depends on A1
succeeding. They are preparation, not registered measurements or completed
coverage. Exact generators and bounds need registration before comparative runs.

## A3: maintained matching through source updates

Current E09 prefix matching recomputes transient prefixes. Compare it with the
same maintained-row representation under full invalidation and selective delta
invalidation. A row keys rule/prefix length/ordered distinct occurrence IDs and
matched rule-variable values, with supported contexts. Predicate and occurrence
indexes plus inspected-variable dependencies drive updates. Complete rows remain
candidates subject to guards, propagation history and the declared selector.

A proposed three-head family joins left(X,U), edge(X,Y), right(Y,V). Use both
retained-prefix simpagation (keep left/edge, consume right, post out) and a
consuming variant as an adverse contrast. Keeping no prefixes alive would obscure
the intended repeated-discovery opportunity. Generated producers insert right
batches, consume/reintroduce occurrences with fresh IDs, bind keys through alias
chains, and finally invalidate a broadly shared key. Preserve full residuals.

Suggested scale: N=4/16/64 keys, B=1/2/8 explicit contexts, R=1/4 rounds.
Freeze the producer order and lifetime rules before selecting the final matrix.
The initial supported-store gate projects each context and compares exact matches
with the existing prefix control. The integrated source gate must then exercise
those updates as actual CHR applications; a standalone insert/query benchmark is
insufficient. Matched logical selection is a local control, not a universal
architecture requirement.

Adverse cases: equal-valued distinct occurrences, repeated variables, middle-head
arrivals, suspended alias dependencies, support-local consumption, different
bindings for the same occurrence tuple, old versus new propagation support, and
a newly introduced earlier selection. Measure row creation/reuse/invalidation,
dependency/index work, support operations, retained rows/queues and source/output
costs. Ten million matcher actions,30seconds and1GiB are proposed pilot bounds,
not a reason to close a timed-out configuration.

Potential ownership: new maintained_join module and focused harness in
research/chr-scheduling, with narrow source selector/update hooks. Matching logic
belongs to its component, not the scheduler. No reference changes.

## A6: exact graph observation before export

Current persistent State::step exports output/residual trees through Arena::export;
Search::advance clones completed answers before deduplication. A completion-only
view can instead expose output names/root handles, residual predicate/root handles
and immutable branch bindings with borrowed arena access. Snapshot metadata and
retained arena ownership are real costs. Existing Arena::equal cannot substitute
for a two-environment joint-alpha comparison.

Compare unchanged eager export, a competent eager control that compares before
cloning delivery answers, and graph comparison that exports only recognized new
answers. Start without mapping-sensitive comparison memoization; it is a separate
optimization. Both controls and candidate must deliver ordinary complete answers.

Independent small oracle: enumerate all bijections of reachable free variables
(at most four), then compare ordered named outputs and sorted residual multisets.
Construct fixtures independently of the candidate exporter. Reuse the existing
4096three-vertex graph oracle comparisons. Add194targeted rows from12semantic
pair templates ×4sharing layouts ×2alias representations ×2orientations, plus
two same-arena/different-binding cases. Freeze actual constructors before runs.

Templates include ground equality/mismatch, variable renaming, output alias
mismatch, joint output/residual agreement/conflict, residual permutation requiring
rollback, multiplicity, connected versus disconnected cycles, constructor position,
output name and deep repeated nonground structure. Identical node IDs under
different bindings are not an identity shortcut. Different sharing topology must
not change equality. Trial-map rollback must cover any memoized obligations.

A subsequent pilot can use DAG depth4/8, unary length16/64, symmetric residuals,
and tiny outputs retaining64/1024unrelated arena nodes. Each stream has32raw
completions with repeated and mostly-distinct variants. Charge capture, comparison,
new-answer export, live bindings/arena storage, session disposal and output disposal
with separate System/meter runs. Graph retention may outweigh export savings;
this is a required contrary control. Actual engine completion integration is
required before attributing a whole-search gain.
