# A checked private phase can reunite into consuming execution

Temporary separation now has an executable source witness. Private components run once per local alternative, then their complete states are combined and ordinary joining rules resume. The gate preserves finite raw answers and demonstrates finite-sibling progress; total efficiency remains unmeasured.

The [registered source gate](../registrations/S04-reunion-source-gate.md) passes **55 parameterized finite configurations** against both an independent scalar evaluator and ordinary Copy execution. Additional tests cover rejected ownership premises, an unfinished source, cancellation and two-to-four-component product construction. All **18 ordinary package tests and 19 diagnostic-feature tests** pass, with strict all-target/all-feature Clippy and formatting. Neither the reference interpreter nor the independent evaluator was changed.

## This performs reunion, not just a product of answers

The source starts with `job(left, …)` and `job(right, …)`. Local rules independently choose values and perform recursive work. A later rule consumes both ready occurrences, checks or creates cross-component equalities, and can enable previously suspended local rules. A shared-permit variant checks which competing consumer wins.

The experimental engine retains bindings, live occurrences, propagation history and identity supplies at each private quiescent state. It combines those states, relocating occurrence/history identities and renaming private fresh variables, before resuming the full ruleset. It does not attempt to reconstruct a continuation from published answers. Failed joins discard their own combined states; cached private alternatives remain available to other combinations.

The existing permanent-region analysis returns **one region in all eight primary source configurations** because their predicates and joining rules connect the computation. The new checked entry forms two private components. This is a genuine distinction from that permanent factoring control. It is not evidence that every possible permanent analysis is equally conservative.

## What work is avoided

At recursive depth12, the two components each have two private alternatives. Four combinations must still be considered at reunion, including equal-valued alternatives with distinct source multiplicity.

| Source | Local transition calls | Resumed transition calls | Ordinary coupled transition calls | Product states |
|---|---:|---:|---:|---:|
| Different choice values; two successful joins | 124 | 18 | 198 | 4 |
| Duplicate choice values; four successful joins | 124 | 24 | 204 | 4 |

Private execution plus resumed execution therefore uses 142 versus 198 calls in the first witness, and 148 versus 204 in the second. These counts include calls to the same source transition operation, including quiescence checks. Product construction, state copying, eligibility checks and scheduling have additional costs; the count reduction is not a speedup claim.

The zero-depth cases exercise little private work and still pay for separation and reunion. Broader sizes, distinct values, branching and frequent reunion must be measured before judging whether saved execution repays ownership and transport costs.

## Why the checked boundary matters

The entry accepts a prefix of local rules followed by joining rules. Every source head has a literal atomic ownership key in its first argument. Each local rule's heads belong to one owner, and every constraint emitted by that rule preserves that owner. Initial variables cannot occur in two components. These are checked restrictions on this experimental optimization, not adopted language requirements or a general independence inference algorithm.

The correspondence argument has four parts:

1. **Private transitions have disjoint effects.** Initial variables are disjoint, fresh variables are local, and local rule bodies cannot emit into another owner. Local heads cannot consume another owner's occurrences. Equality and the supported nonbinding equality guards therefore inspect only local variable state.
2. **Local priority is respected before a join.** The ordinary executor chooses a local-prefix rule whenever one is enabled. On a finite branch it reaches local quiescence before a joining rule can run. Independent owners' private steps commute, allowing their alternatives to be evaluated separately.
3. **Transport preserves later competition.** Original query variables retain their identities; each component's fresh range is relocated disjointly. Occurrence relocation preserves order within each owner, including identifiers referenced only by propagation history. Every source head has a fixed owner, so cross-owner numeric ordering does not choose its first matching occurrence. New resumed occurrences follow all transported occurrences.
4. **Every source alternative keeps its multiplicity.** A product cursor fixes the newly available local alternative and the current alternative counts of the other components. Each tuple is generated when its last member becomes available, exactly once. Equal-valued alternatives remain distinct tuples.

This is an analytical argument for the checked fragment, supported by executable cases, not a machine-checked theorem. Removing fixed-owner heads, allowing private owner escape, or sharing initial variables invalidates premises and requires a new argument. The entry rejects those cases rather than silently applying the optimization.

The finite suite varies source priority and initial/head occurrence order, checks failed choices and duplicate results, preserves fresh residual aliases and propagation suppression, and reconnects formerly independent unknowns. In nine late-binding configurations, reunion enables both owners' suspended local rules. Twenty history/resource variants check fresh aliases, failures and a consumed shared permit; the first eligible consumer remains the winner. Six configurations use two, three and four components to exercise lazy product multiplicity.

## Finite answers can arrive before private search finishes

The resumable entry rotates between a private source step, one product-state construction and a resumed source step. Each queue is FIFO. New private answers can form products while other private alternatives continue; product cursors avoid eagerly materializing an entire Cartesian product in one service call.

The ongoing-sibling witness delivers a joined finite answer within 1,000 service calls, matching the ordinary executor's first complete answer on that source. The engine does not report exhaustion while its ongoing work remains. The finite completion helper instead reports a budget error if its total service allowance is exhausted, without claiming partial success. Exact event order and constant wall-time service quanta are not claimed: one source step can still perform substantial matching or unification, and one product materialization copies complete component states.

A weak-owner check cancels with a product pending and proves that cached private states and the prepared-rule owner are released. A subsequent finite query remains correct. This is ownership evidence, not a requested-allocation or sustained-memory measurement. Saved alternatives currently remain query-owned so later combinations can use them; retention and reclamation remain material costs to investigate.

## Next establish complete costs and reusable preparation

Keep T077 active for a bounded preparation and allocation-ownership gate, then prospectively registered ordinary costs if the controls remain credible. Move source-prefix checking into reusable preparation, validate changed query ownership at setup, and make work diagnostics removable before timing. Compare ordinary Copy, the existing Indexed/persistent controls and permanent factoring on the same complete sources. Account for eligibility, private caches, product transport, resumed execution, first answers, cancellation and disposal.

The strongest ready alternative remains T072 integrated dependency repair and complete consuming-source costs. Reunion now has a distinct mechanism witness and source correspondence evidence but no complete cost comparison; measuring its currently uncharged copies and retention could reverse the apparent advantage from fewer source calls. That is more informative than further timing refinement of the restoration policies already measured. This is T077's third bounded package; the next completed package triggers the four-package breadth review, including integrated execution.

Dynamic separation in the middle of an arbitrary computation, repeated split/reunion cycles, broader ownership inference, adaptive splitting and sustained consumers remain required. The current candidate separates a checked initial private phase and subsequently executes coupled work; it does not resolve those broader directions. Whole-architecture and language selection remain open.

## Evidence

[Registration](../registrations/S04-reunion-source-gate.md), [finite and work tests](s04-reunion-source-gate/tests-all.log), [diagnostic-feature tests](s04-reunion-source-gate/tests-diagnostic.log), [Clippy](s04-reunion-source-gate/clippy.log), [initial failing gate](s04-reunion-source-gate/red.log), [failing progress gate](s04-reunion-source-gate/progress-red.log), [audit](s04-reunion-source-gate/audit.json) and [source hashes](s04-reunion-source-gate/freeze.sha256) record the result. The registration's progress addendum is identified as an extension during correctness development, not a prospective cost registration. No comparative timings ran in this package.

Reproduce with `cargo test -p chr-restoration -- --nocapture`, repeat with `--features replay-diagnostic`, then `cargo clippy -p chr-restoration --all-targets --all-features --no-deps -- -D warnings` and `cargo fmt -p chr-restoration --check`.
