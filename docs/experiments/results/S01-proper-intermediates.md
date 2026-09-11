# Proper intermediates trade retained combinations for final-partner work

Retaining proper prefixes while rediscovering the final partner reduces peak join entries from584 to72 in the width8 many-to-many witness. It also increases head checks from584 to2,348 compared with the existing partial-join strategy. Scanning performs2,924 head checks and retains no join cache. This is a real representation/work tradeoff; allocation and elapsed-time costs remain unmeasured.

The new mode passes76 complete-source configurations with diagnostics enabled and disabled, alongside independent scalar, conventional Scan/Indexed and existing local controls. Two registered repeats agree exactly, and all93 semantic tests pass. **The next package should measure complete ownership and costs before further mechanism changes.**

## What differs from the existing partial join

The [existing partial strategy](S02-partial-join-gate.md) retains successful prefixes and their terminal extensions, including extensions suspended on missing information. The new mode retains only proper prefixes for multihead rules. At source selection, it takes the saved environment of each applicable prefix and matches the final head against live occurrences.

Source rule and occurrence order still determine the selected complete tuple. The implementation compares terminal discoveries with ready single-head applications and stops ordered prefix traversal only when later keys cannot beat an application already found. Complete-tuple history prevents repeated propagation; prefix identity alone does not replace it.

Existing dependency subscriptions wake unfinished proper prefixes. Consuming an occurrence invalidates every retained prefix containing it. Consuming a terminal occurrence need not invalidate a retained terminal product because this mode stores no such product. Terminal matching is repeated at source-selection boundaries, so changed final-head information needs no persistent terminal watcher.

This directly tests a storage decision within intermediate matching. It is not equivalent to materializing every successful prefix and suspended terminal extension. It also does not test every possible join decomposition, factorized relation or selective index.

## The many-to-many witness exercises the tradeoff

N left and N middle occurrences yield N² reusable two-head matches. N right occurrences select the last key pair and are consumed one at a time while the earlier occurrences survive. The new mode retains N+N² entries; partial joins retain N+N²+N³. The original RED test caught that distinction: width2 retained14 entries when routed through the existing strategy, rather than the required6.

| Width | Strategy | Head checks | Fact visits | Intermediate-map visits | Peak retained join entries |
|---|---|---:|---:|---:|---:|
| 2 | Scan | 29 | 96 | 0 | 0 |
| 2 | Partial joins | 14 | 42 | 0 | 14 |
| 2 | Proper intermediates | 17 | 75 | 18 | 6 |
| 4 | Scan | 254 | 1,020 | 0 | 0 |
| 4 | Partial joins | 84 | 252 | 0 | 84 |
| 4 | Proper intermediates | 174 | 842 | 100 | 20 |
| 8 | Scan | 2,924 | 13,032 | 0 | 0 |
| 8 | Partial joins | 584 | 1,752 | 0 | 584 |
| 8 | Proper intermediates | 2,348 | 11,652 | 648 | 72 |

Every row performs exactly N source firings and produces the same complete answer. The final-partner work is an adverse consequence of omitting terminal retention, not missing functionality. Proper intermediates avoid repeatedly matching the earlier heads relative to scanning, while fully retained partial joins avoid more rediscovery.

Peak entries are not bytes. Environments, dependencies, incident maps, propagation history, temporary matching allocations and prepared/source ownership must still be charged. Intermediate-map visits are reported separately rather than hidden inside fact visits. None of these operation types has an assumed common cost.

## Correctness includes sparse and broad change

The shared source checker now runs the new mode with metrics on and off for76 configurations. These generic instances run in the same semantic test executable; no separate timing-build comparison is claimed. Existing cases exercise late equality, constructor information, new partners after a complete body, fresh aliases, duplicate occurrences, propagation history and consumption of suspended prefixes. Independent scalar and conventional Scan/Indexed checks retain their original complete-output and residual-multiplicity responsibilities.

Three new broad-invalidation cases consume every left occurrence before the join can fire. No stale intermediate produces a hit, and all right resources remain. Cache checks require reciprocal occurrence/subscription ownership, a retained successful parent for each longer prefix, and no terminal tuple in the new mode for a multihead rule.

The source contract remains the deterministic, guard-free fragment accepted by the existing local program compiler. Disjunctive/contextual ownership and general guards remain separate obligations. Finite source agreement does not establish cancellation, sustained ownership or a language-wide implementation.

## Why complete costs are next

The intended saving now occurs on an actual reusable many-to-many intermediate: less retained terminal product, with repeated final-partner work as a contrary result. A further work-count refinement cannot decide which cost dominates. Extend the existing multihead lifecycle harness to include this mode and witness, with scanning, indexing, full tuples, partial joins and applicable specialization.

Include sparse terminal consumption, broad prefix invalidation, late information and cheap keyed/one-shot sources. Charge preparation, setup, retained environments, lookup, invalidation, complete observation, cancellation and disposal; reuse prepared rules over changing queries. Qualify ownership before registering counter-free timing. Keep indexing as a competent control—this source must not force the architecture to enumerate a product that a valid selective plan could avoid.

This is the second package after the last full portfolio review, following CHR ownership/work. T081 remains active for one lifecycle comparison. At its result or obstruction, compare causal follow-through with compact solving's missing operations and unfinished integration/demand costs. This gate resolves neither intermediate joins broadly nor the architecture choice.

## Validation and durable evidence

The [registration](../registrations/S01-proper-intermediates.md), [audit](s01-proper-intermediates/audit.json), [source/binary freeze](s01-proper-intermediates/freeze.json) and [raw receipts](s01-proper-intermediates/) preserve the RED witness, implementation qualification and exact work repeats. The repeated multihead target passes10 tests. The library plus all16 semantic integration targets pass93 tests, and strict scoped release Clippy passes.

The first unfiltered package command attempted to invoke an argument-driven cost CLI without a workload. The corrected command selects all semantic test targets from Cargo metadata and manifest harness flags; the exact command and six excluded cost CLIs are recorded. Those CLIs require their own registered inputs. No production test or requirement was weakened to hide a source failure.

Earlier multihead lifecycle and affinity sources have been reconstructed from repository history and checked byte-for-byte against their recorded hashes. Their audits now read the verified archives; lifecycle, affinity and scale audits pass. Existing recorded-read and specialization audits also pass with their preserved sources. Frozen comparative binaries remain intact, and reference-interpreter code is unchanged.
