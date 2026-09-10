# Hidden disequalities depend on the name domain

Hidden name constraints can be eliminated without search under an unbounded supply of atoms, but the same elimination is unsound over a finite alphabet. The compiled equality/exclusion representation agrees with the independent finite oracle and delayed-binding source controls. This establishes an experimental logical interface, not a replacement for raw CHR answers or a timing advantage.

## A small example changes the required solver

Consider `exists H. H != X and H != Y`. If there are exactly two available names, a value for H exists precisely when X and Y have the same value. When X and Y use both names, none remains for H. With an unbounded supply, H can always receive a fresh name.

The experiment checks both outcomes. Simply discarding edges involving hidden variables would therefore change the finite-domain answer. A triangle of pairwise exclusions similarly fails over two names and succeeds over three or an unbounded supply. These are semantic differences between domain contracts, not different implementations of one answer set.

## What was implemented

The [candidate](../../../research/chr-structural/src/name_disequality.rs) compiles equalities into classes and retains disequality edges between them. Merging unequal fixed atoms, or making the endpoints of an exclusion equal, establishes inconsistency. Distinct unknown variables retain an exclusion until their eventual values are known; different variable identities do not establish different names.

The query endpoint supplies values for some variables and asks whether values for the rest exist. In the finite-alphabet implementation, unresolved classes are assigned by backtracking, checking exclusions as values become available. A simple degree order chooses classes. In the unbounded implementation, the candidate checks assigned classes and uses the fresh-value extension argument for the rest. Neither endpoint enumerates all visible answers or serializes a projected formula.

**The unbounded argument is independent of the measurements.** After equality normalization, give each unassigned class a different atom outside the finite set of fixed and supplied names. Every exclusion incident to an unassigned class then holds. Exclusions between assigned classes and contradictions inside an equality class still require checking. This argument assumes only finitely many classes and an unbounded atom supply; it does not justify finite-alphabet elimination or arbitrary-term disequality.

## Independent checks

The oracle enumerates complete assignments and evaluates the original equations and exclusions directly. It does not reuse the candidate's equality classes, ordering or pruning. It then projects successful assignments onto each visible-variable subset.

| Check per confirming execution | Cases | Scope |
|---|---:|---|
| Finite projection membership | 22,656 | All 64 graphs on four variables, alphabets of size 0–3, every visible subset and assignment |
| Unbounded projection membership | 5,184 | The same graphs and visible subsets, with an independent pool containing four fresh atoms beyond the two visible names |
| Equality, delayed binding and literal source answers | 5,000 | 625 original equation/exclusion combinations and all eight assignments of three variables to two atoms |

Four confirming executions pass: two default and two metrics-off. The unbounded oracle's four fresh atoms suffice for at most four unassigned variables; the general claim rests on the separate extension argument above. Empty finite alphabets, isolated declared variables, fixed-atom contradictions, reflexive equality, duplicate exclusions, late alias collapse and explicit caller-variable remapping are included.

The 5,000 source queries post an ordinary `neq` occurrence before body equations supply equality and ground values. The unchanged reference must exhaust and return either failure or the exact residual `neq` occurrence. Separate unknown-argument controls retain `neq(X,Y)` and reject `neq(X,X)`. Thus logical feasibility does not silently stand in for source residual observation.

Another 51 structural tests pass across 11 regression executables, including the prior names matrix, delayed source wake-up, finite paths and projection; one executable contains no tests. Clippy and formatting checks pass. Reference and independent oracle code are unchanged. Confirming processes have 60-second wall/CPU and 1 GiB address-space bounds, with 2,000 reference steps per source query.

## What the work counts do—and do not—show

The diagnostic witnesses record partial assignment attempts, while the enumeration oracle counts complete assignments. These are different events and cannot be divided to obtain a speedup.

| Exclusion graph | Alphabet size | Candidate partial assignment attempts | Complete assignments in the oracle space | Feasible? |
|---|---:|---:|---:|---|
| Triangle | 2 | 10 | 8 | No |
| Triangle | 3 | 6 | 27 | Yes |
| Four-way clique | 2 | 10 | 16 | No |
| Four-way clique | 3 | 48 | 81 | No |

The triangle over two names makes the limitation visible: pruning does not imply fewer counted operations under every counting convention. The successful triangle asks only for an extension, so its early success cannot be reported as producing all assignments. These observations qualify the mechanism for a later matched-endpoint cost comparison; they establish no wall-time, memory or full-lifecycle superiority.

## Architectural consequence and remaining obligations

**A name-domain contract can eliminate a solver responsibility.** Unbounded pure name exclusions need no hidden-value enumeration after equality normalization. Finite alphabets retain a constraint-search problem and can impose additional relations on visible names. The language or an eligibility certificate must establish which contract applies; it is not an allocator choice.

**The closed atomic fragment remains narrower than literal source behavior.** As the [names entry](S06-names-entry.md) demonstrates, the source rules also leave some structured arguments residual. This candidate accepts atomic operands only. General finite-tree disequality, source-domain certification, host effects, formula serialization, cross-query reuse, retained owners and finite-signature name encoding remain required investigations. The finite backtracking implementation is a correctness candidate, not a measured competitive solver selection.

Next investigate the recorded normal/neutral theory under T076. It adds recursive constructor requirements and repeated-hole correlations, which these root and exclusion checks cannot answer. The strongest ready alternative is integrating and measuring the name/disequality formula with a source boundary. The normal/neutral entry comes first because it can change the representation and interface that integration would otherwise assume. Integrated CHR execution and continuing conditional ownership remain alternatives at that gate or an obstruction.

This is the third package since the conditional breadth review. The next package requires a full breadth review. The architecture research goal remains active.

## Evidence

The [prospective registration](../registrations/S06-disequality-entry.md), [frozen inputs and binaries](s06-disequality-entry/freeze.json), [raw receipts](s06-disequality-entry/), [audit summary](s06-disequality-entry/audit.json), and [runner](../../../research/chr-structural/experiments/disequality_entry.py) preserve the comparison. The [prior module snapshot](s06-disequality-entry/before/structural-lib.rs) preserves the earlier names-entry input before this module was added; its hash was checked against that entry's frozen manifest.
