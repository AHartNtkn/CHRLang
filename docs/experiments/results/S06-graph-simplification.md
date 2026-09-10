# Constraint-based simplification qualifies a stronger finite-source control

The new simplifier derives visible relations from disequality graphs without workload labels or oracle answers. It preserves 3,072 exhaustive graph projections and 128 union projections per build, and agrees with the 360-configuration source comparison. It is ready to enter the lifecycle comparison; its analysis cost remains unmeasured.

## What the analysis actually proves

The input declares a finite alphabet, visible coordinates and alternative disequality graphs. A graph represents proper assignments of names to vertices. Hidden coordinates are existential. The output retains exact equality/disequality constraints, including any hidden structure the implemented rules cannot eliminate.

With one name, an edge is impossible. With two names, bipartite coloring detects contradictions and gives the parity relation between visible vertices in each connected component. Vertices with the same parity must have equal names; opposite parity requires different names.

With three or more names, the analysis repeatedly eliminates a hidden vertex whose degree is smaller than the alphabet size. Any coloring of the remaining graph extends to that vertex because its neighbors cannot occupy every available name. Eliminating several such vertices is sound by restoring their choices in reverse order. An oversized complete connected component is a separate contradiction certificate.

These are constraint-based rules. They neither enumerate all source assignments nor recognize the six experiment family names. An unresolved core remains an explicit set of constraints. Direct visible-membership queries reject an unresolved result instead of returning a guessed Boolean answer.

## Independent evidence and boundaries

The independent oracle enumerates assignments and compares the original and reduced existential sets. It covers all 64 simple graphs on four vertices, all 16 visible subsets and alphabets of one, two and three names. Another matrix checks every pair of three-vertex graphs over two/three names. Visible order is deliberately reversed in those comparisons.

Boundary checks cover duplicate/reversed edges, self-edges, empty unions, isolated coordinates, malformed coordinates and explicit work exhaustion. Every source coordinate is validated before a universal or contradictory branch can short-circuit analysis. Closed results are queried directly against the independently enumerated visible sets.

The three-name K3,3 witness demonstrates a real limit. It is colorable, but each hidden vertex has three neighbors and its component is not a clique. The simplifier leaves the core unresolved and rejects a direct Boolean query. This result is not a failure of three-colorability or a reason to exclude the source; it identifies work that these particular reduction rules do not settle.

The simplifier is also integrated into the existing source gate as a fifth candidate. All 360 source configurations and 1,800 candidate projected-set comparisons pass per build. Full canonical observations and completed branches remain independently checked against the reference. The host-observer witness continues to demonstrate why preserving a visible set does not preserve every residual effect.

Debug and release each confirm both test executables under 60-second CPU/wall and 1-GiB address-space bounds. Operations use the registered one-million-unit budget. The initial missing-implementation test fails, implemented tests pass, and final scoped Clippy passes. No reference implementation changes are involved.

## What remains to price

Preparation constructs adjacency sets, discovers components, propagates parity, removes vertices and owns the resulting constraints and visible-coordinate map. Those costs cannot be omitted merely because direct queries allocate nothing. Repeated graph scans can also be inefficient on larger or adverse inputs; this package provides no complexity-performance crossover.

For the selected ownership corpus, the derived relations are closed and can be queried without rebuilding a solver or allocating a complete hidden assignment. This supplies the missing strong control identified by the allocation evidence. It does not establish that source elimination is always cheaper, and it does not replace symbolic operations on unresolved graphs or broader structural terms.

The [four-package breadth review](S06-graph-simplification-breadth-review.md) selects a bounded completion of this lifecycle comparison, including the simplifier, before returning to search restoration. Allocation/clock qualification and prospective primary timing remain required. Further graph-rule expansion is not the default next task.

## Evidence

[Registration](../registrations/S06-graph-simplification.md), [constraint-based implementation](../../../research/chr-structural/src/graph_simplification.rs), [independent graph tests](../../../research/chr-structural/tests/graph_simplification.rs), [source comparison](../../../research/chr-structural/tests/diagram_source.rs), [bounded confirmation driver](../../../research/chr-structural/experiments/graph_simplification_gate.py), [frozen inputs and binaries](s06-graph-simplification/isolated-confirmation/freeze.json), [four confirmation receipts](s06-graph-simplification/isolated-confirmation/audit.json), [independent artifact auditor](../../../research/chr-structural/experiments/graph_simplification_audit.py), [initial failing test](s06-graph-simplification/red.log) and [final Clippy](s06-graph-simplification/clippy-final.log).
