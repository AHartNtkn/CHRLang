# R03 conditional runtime components: support and births

The direct conditional candidate now has resumable Boolean support operations and causal source-birth enumeration. Independent exhaustive checks pass. This is partial T035 implementation: source equality, matching, consuming commits and completion are not implemented, and no runtime correctness or efficiency claim follows yet.

## Implemented behavior

The query-owned support arena stores canonical ordered Boolean decision nodes, not enumerated branch tickets. NOT, AND, OR and difference jobs carry explicit continuations and local memo tables. Each tick processes one apply frame, pushes at most three frames and creates at most one canonical node. Node inspection and cached maximum dependency are constant-time indexed reads. Ordered-map and allocation operations retain their size-dependent cost; this is an algorithmic service boundary, not a hard latency guarantee.

The birth ledger owns source Boolean variables and records each birth's activation guard. Guards can depend only on earlier births. Equal source alternatives still allocate a birth; an inactive birth contributes no branching. Observation cursors follow one guard node, traversal frame or output bit per tick. They preserve raw duplicate alternatives and enumerate only causally reachable histories. A cursor may be cancelled without consuming births.

A cursor freezes the existing birth prefix. This permits later independent work to add births without changing that cursor's traversal, but it does not establish publication safety. The eventual runtime must prove completion on the observation support and that later work cannot change it. No checker in this package substitutes prefix freezing for completion.

## Independent checks and boundaries

The external truth-table test constructs all 256 functions of three variables and checks every ordered operand pair for AND, OR and difference: 196,608 comparisons. It separately checks 256 negations. Expected results use integer Boolean arithmetic; canonical handles must also agree with independently constructed decision trees. The candidate never uses these truth tables during operation.

The birth test enumerates all 128 causal activation configurations for three births and compares their histories with an independent Boolean reachability predicate. Focused cases check `true OR true` multiplicity, the three histories of a nested conditional birth, Cartesian independent births, cancellation and prefix stability. These are source-choice component witnesses, not full source executions.

Unit tests additionally check invalid node order, node reduction/canonical identity, interleaved jobs, dependency bounds and a long job's explicit service steps. An independent review prompted a further check that cancels a Boolean job after intermediate node creation, runs another job and successfully restarts the cancelled operation. All eleven package tests pass with metrics enabled and disabled. Initial missing-API failures were observed before implementation; [receipts](R03-conditional-components/) preserve those checks and final validation. Workspace all-target tests, Clippy with warnings denied and formatting pass. No comparative measurements were run.

## Remaining work under T035

[Supported finite-tree equality and nonbinding equality demand](R03-conditional-equality.md) now pass independent checks. Structural head matching, occurrence/propagation ownership, body sequencing and direct activation remain to integrate. The independent full-path gate must still prove support-local finite-sibling progress, complete source effects and exactly-once joint output/residual publication. The [reviewed protocol](R03-conditional-protocol.md) remains authoritative for those obligations.

Query-scoped retention is intentional for this first candidate: cancelled jobs may leave canonical nodes in the arena until query disposal. Within-query reclamation and long-lived stream costs remain unmeasured. These component checks do not establish the architecture's benefit against compiled source execution or finite lowering.
