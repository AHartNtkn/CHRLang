# Relational equality can update joins and consuming resources directly

The relational candidate now maintains equality, constructor facts and source occurrence indexes in one mutable owner. Independent checks pass after equality updates and resource consumption. This establishes a working mechanism; it does not yet compare complete CHR execution or cost.

## What the experiment establishes

**Equality changes repair the same relations used for matching.** Value classes carry links to their incident rows. A merge updates those rows and their column indexes, and constructor congruence queues further equalities. The candidate does not reconstruct an exported term store to update these joins.

**Useful consumption can occur before unrelated equality finishes.** In the directed witness, equating an unknown with `f(a)` exposes `open(f(X)), ticket(X)`. Both occurrences can be consumed while another equation remains queued. Export is unavailable at that point. This establishes interleaving capability; measuring an advantage still requires a complete source path and a matched control.

**Equal values do not erase resource identity.** Two `p(a)` occurrences yield two ordered kept/removed assignments. A stale or duplicate claim fails atomically, and a fresh occurrence receives a fresh identity. Consumption retires only removed heads from live indexes.

**Copied interpretations isolate equality, failure and consumption.** One fork can bind and consume successfully while another fails an occurs check. This is a copying control, with copying costs still to measure. It does not demonstrate shared context-local equality.

## Independent evidence

The owned-term oracle uses recursive substitution and occurs checking; it does not use the candidate's union-find, relation indexes or equality operations. Its eight terms cover two unknowns, distinct constants, equal constructor names with different children and a different constructor name. All 36 unordered term pairs are combined in both equation positions: **1,296 ordered two-equation cases**, checked after each equation.

For each consistent intermediate state, the gate compares jointly normalized exported terms and every ordered distinct-resource repeated-variable join. For failed states it checks that export and matches are unavailable. Four directed tests additionally cover pending-work interleaving, parent congruence, constructor/arity clashes, direct and indirect cycles, alias-only cycles, fork isolation and stale atomic claims. The earlier head-plan tests remain passing, including their 1,664 independently checked configurations.

The [prospective registration](../registrations/S02-relational-owner-gate.md) states the scope. [Validation receipts](s02-owner/validation.json) record commands and source hashes; tests and replay pass, as do strict Clippy and formatting. Initial implementation diagnostics are retained with the receipts. No comparative timing ran in this gate.

## What remains before an architectural conclusion

The owner does not yet execute source bodies, guards, propagation history or a source completion protocol. `consume` checks resource ownership, not rule authorization; its caller must establish the rule and guard. Equality-settled export is not proof that source rules are quiescent. Positive matches may be serviced during pending deductions, but a later contradiction must invalidate that interpretation before answer publication.

Each equality step may perform size-dependent incidence repair and cycle traversal. Retired row storage remains allocated, and join enumeration materializes candidate lists. These are costs to measure or investigate, not established necessary complexity. Native compilation, long-lived reclamation and shared equality across contexts remain separate obligations.

T063 stays active for a complete source gate and then a prospective cost comparison with the integrated and dedicated-service controls. The broader S02 stage also retains its distinct representation alternatives. This gate supplies no rejection of those alternatives and no architecture selection.
