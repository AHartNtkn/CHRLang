# Temporary decomposition requires rejoining execution states

Persistent region independence justifies a Cartesian product of completed regional answers. Temporary independence permits a weaker optimization: advance independent operations separately, then rejoin their live states when an interaction becomes possible. It does not justify treating provisional regional quiescence as final success.

## Conservative protocol

Maintain a finite interaction overapproximation over the current active and pending occurrences, logical-variable alias classes, and private operations. Include potential multiheaded tuples, even if a constructor or guard currently suspends them. A cheap predicate-level graph can conservatively join more than necessary. A precise graph may use rigid constructor incompatibility, but must revalidate it when any read variable changes. Equal ground values can matter for joins; no-variable constraints are not automatically isolated.

Select finitely many source operations against one frozen snapshot. Partition the selected operations only when their effects commute: no operation consumes another's required head, their propagation tokens differ, and fresh identities are apart. Apply posts body equations rather than executing them invisibly. Concurrent unifications need a separate disjoint-write/read certificate or serial execution. This uses the T011 commuting lemma rather than treating all currently distinct components as permanently independent.

Before making new pending constraints or bindings visible, update the interaction overapproximation and merge affected scheduling components. An insertion that creates a cross-component partner must expose that partner to matching. A binding that destroys a previous disjointness certificate similarly triggers reunion before another operation relying on that certificate commits. Snapshot/version validation rejects a stale proposed effect, then recomputes it; already committed valid source transitions are not undone merely because later work connects the components.

A selected batch has finite ownership and finite validation work. Administrative reunion can therefore use the sealed-job scheduler. A globally unbounded background process cannot keep extending the same accepted batch indefinitely.

## Why this preserves source execution

Linearize commuting operations in any order, and noncommuting operations in their validated commit order. Each transition's prerequisites hold at its linearization point. The resulting sequence is a reference execution. The graph is an optimization for discovering and scheduling work, not the authority for source semantics. Global quiescence still requires a stable check that covers every live occurrence, potential match, pending equation and private operation.

For example, `p(a)` and a goal that later posts `q(b)` can initially run separate eligible steps. Once q is posted, `p(X),q(Y) <=> r(X,Y)` joins the components. A provisional answer containing p cannot have been published as a final independent factor. The live p occurrence and its identity must remain available for reunion.

## Across alternatives

If independent components each introduce explicit alternatives, the combined state can retain a lazy product of their alternative contexts. Reunion introduces a join over compatible combinations, preserving shared variable correlations and source choice lineage. It must join complete live states: substitutions, occurrence memberships, histories, pending work and committed-policy metadata. Output values alone cannot reconstruct those states.

Before reunion, a component's operations can be shared across the other factor's alternatives. After reunion, distinguish exactly the contexts read by a cross-component operation. A simple correct construction materializes the finite current product at that point and uses the conditional/reference machine. A more compact construction attaches the join to symbolic supports and preserves operation provenance. Both are semantically meaningful; neither promises an inexpensive join. Bounded snapshot products are finite, even when the overall search is infinite.

The policy condition remains material. Component scheduling may choose a different permitted committed CHR execution from a fixed global selector. Preserving a specified selector requires including its dependencies in the interaction graph or proving the reordering commutes. Confluence can remove answer sensitivity within a certified region, but cannot be assumed for arbitrary rulesets.

## Disposition

A conservative temporary-decomposition protocol is now available without a source-language restriction. It supplies execution sharing until actual interaction and safe reunion, but cannot certify independent final answers. The stronger persistent certificate in T017-andorra-and-decomposition.md supplies that guarantee.

Remaining implementation-dependent questions are interaction-graph maintenance costs, false dependencies from conservative analysis, product-join size, version-conflict rates and retained provisional-state memory. More elaborate graph algorithms could improve those costs, but no specific missing semantic mechanism currently prevents constructing this baseline. Measurements should determine which bottleneck merits a specialized algorithm before one is selected.
