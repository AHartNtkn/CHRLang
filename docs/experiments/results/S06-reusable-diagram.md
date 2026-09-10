# Reusable finite formulas preserve union and hidden correlations

A compiled finite decision diagram now supports exact conjunction, union, existential projection and inclusion. Independent enumeration agrees on 24,384 denotation checks in each of debug and release builds. This establishes a new working mechanism; its compilation and lifecycle costs remain unmeasured.

## What changes in the experiment

The existing symbolic conjunction path rebuilds a feasibility solver for each observation. This experimental path prepares a graph representing the permitted assignments. A node tests one variable, with one edge per declared atomic name. Identical subgraphs share a node, and a test whose children all agree disappears. Queries can then follow this prepared graph without reconstructing the formula.

Union and intersection combine graph nodes directly. Existential projection combines the alternatives for a hidden coordinate with union. Inclusion checks whether any assignment belongs to the left formula but not the right. These operations preserve correlations: forgetting a shared hidden variable does not turn the remaining coordinates into independent domains.

This is a distinct control against rebuilding, but prepared name solving already reuses compiled constraints. The architectural question is whether representing and combining whole solution sets avoids enough further solving to repay compilation and graph ownership. Comparing only against symbolic rebuilding would not answer that question.

## What the checks establish

The oracle enumerates complete integer assignments and evaluates equality and disequality with ordinary comparisons. It does not call the diagram implementation, name solver or reference interpreter.

| Check | Registered scope |
|---|---|
| Formula membership | Three variables; alphabets of one, two and three names; every subset of three disequality edges; optional equality between coordinates 0 and 1 |
| Hidden correlations | Every subset of the three variables existentially forgotten, checked against independently enumerated witnesses |
| Combining alternatives | Every pair of the 16 formulas per alphabet, checking union, intersection and inclusion |
| Boundaries | Empty/universal sets, reflexive contradiction, invalid coordinates and assignments, mismatched universes, zero-variable truth and work-bound exhaustion |

All 48 formulas, 384 projections and 768 formula pairs are checked. Together these produce 24,384 denotation comparisons per build, plus boundary assertions. Each confirmation executes with 60-second CPU/wall limits and a 1-GiB address-space limit. Both finish successfully. Ordinary operations have a one-million-unit work bound; deliberately exhausted operations return errors rather than false answers.

The initial test fails because the implementation does not yet exist; the implemented tests then pass. Scoped Clippy passes. Sources, binaries, toolchain and logs are frozen in the evidence directory. This is correctness confirmation, not a timing sample or evidence about application frequencies.

## Costs and limits that must remain visible

The representation is ordered by coordinate number. A poor order can make a decision diagram large, and this package supplies no claim about good ordering. Compilation combines primitive constraints; the unique-node table retains intermediate nodes as well as the final reachable graph. A later cost comparison must charge that retained state. Cross-formula operations import reachable nodes into a new owner; copying and memoization are real costs, not free sharing.

Each successful formula owns its nodes and unique-node table. Operation memo tables are temporary; a failed operation releases its partial result. There is no cross-query global cache or fresh caller-variable history. This ownership model is simpler than transporting fresh symbolic identities for these integer assignments, but it does not implement that broader transport contract.

The declared universe is a finite alphabet of atomic values. Set union intentionally collapses overlapping assignments, and existential projection forgets witness multiplicity. Those are logical-set semantics, not raw CHR alternative or consuming-resource semantics. Source effects, structured unbounded terms and raw multiplicity need their own correspondence evidence; this gate does not establish them.

## Decision and next work

Proceed to a matched finite-name source-correspondence gate, retaining explicit enumeration, existing prepared names, finite projection and symbolic rebuilding as controls. Test changing caller restrictions, aliased visible coordinates, overlapping alternatives, hidden exclusions and full output. The source gate must state exactly when logical-set observation is valid and retain rejected boundaries.

Then register ownership and complete costs: formula construction, preparation, changing-query setup, membership/full observation, union/projection where requested, retention and disposal. Include both reusable restrictive formulas and cheap or unselective one-shot work. Do not measure only traversal through an already compiled graph.

This is package one of the selected reusable-solving investigation. T076 remains active; T071's targeted timing attribution and broader capability, T072's relevant-read repair, checkpoint/replay and the remaining portfolio are still required. No language or architecture is selected.

## Evidence

[Registration](../registrations/S06-reusable-diagram.md), [implementation](../../../research/chr-structural/src/reusable_diagram.rs), [independent tests](../../../research/chr-structural/tests/reusable_diagram.rs), [confirmation driver](../../../research/chr-structural/experiments/reusable_diagram_gate.py), [frozen sources and binaries](s06-reusable-diagram/freeze.json), [confirmation counts](s06-reusable-diagram/audit.json), [debug log](s06-reusable-diagram/debug-confirmation.log), [release log](s06-reusable-diagram/release-confirmation.log), [Clippy](s06-reusable-diagram/clippy.log) and [existing-theory regressions](s06-reusable-diagram/regressions.log).
