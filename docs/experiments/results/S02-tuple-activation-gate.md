# Selective activation avoids matching, but full tuples can create unnecessary work

Selective tuple activation preserves source behavior and sharply reduces repeated head matching on sparse updates. A cold-prefix case exposes the opposite regime: scanning checks 64 heads, while the full-tuple cache checks and retains 4,096 combinations. A partial-join competitor is required before choosing an integrated matching design.

## What the implementation does

The executor now offers source-ordered scanning and selective activation over the same local equality graph. The selective mode identifies a tuple by its rule and ordered occurrence IDs. A failed match subscribes to the stable handles actually inspected; a successful match enters an ordered ready map. Consuming an occurrence invalidates every incident tuple. Kept-tuple history prevents duplicate firings.

Successful head matches remain valid under consistent additional equality: captured handles stay stable and established constructor/equality tests remain established. Failed tests require reinspection when their missing information changes. The implementation records handles touched by graph merges, including child merges, and replaces subscriptions after reinspection. New facts and equality notifications are processed after the complete body, before selecting the next source-ordered tuple.

This is a full-tuple cache. It is not a partial-join index: it initially materializes potential head combinations based on predicate and arity before checking their patterns. Its maps separately own tuples, readiness, failed conditions, handle subscriptions and occurrence incidence. These responsibilities and their retained memory must be charged in a later lifecycle comparison.

## Source evidence

All 47 established source configurations now pass in both graph modes against the independent scalar evaluator and conventional Scan/Indexed execution. Twelve additional configurations cover sparse updates, broad aliases, nested repeated variables and cold prefixes at widths 4/16/64. Complete raw answers agree throughout. Each work configuration also passes with its metrics disabled.

The checks cover distinct equal-valued occurrences, mixed kept/removed heads, ordered propagation history, fresh aliases, new posted facts, consumer competition, body atomicity, source/occurrence order, contradictions and occurs failure. Reciprocal tuple/subscription/occurrence invariants are checked during work-witness execution. All finite runs exhaust within the 200,000-call bound. Guards and body alternatives remain outside this deterministic fragment.

## Work findings at width 64

| Family | Scanning head checks | Selective head checks | Selective generated combinations | Peak retained tuples |
|---|---:|---:|---:|---:|
| One useful descriptor update among unrelated work | 4,388 | 197 | 195 | 67 |
| Broad unknown alias merges followed by grounding | 8,451 | 386 | 258 | 130 |
| One nested repeated-variable equality becomes known | 4,388 | 197 | 195 | 67 |
| Unknown first heads; no tuple can succeed | 64 | 4,096 | 4,096 | 4,096 |

Head checks are attempts to match an occurrence against a source head. They do not include every cost. The sparse case additionally visits 4,751 fact entries during selective candidate generation, versus 9,397 during scanning selection. The cold case visits 8,320 fact entries selectively versus 128 with scanning. Work counters therefore distinguish generating candidates from matching them; neither is a timing measurement.

Broad repair remains substantial even after filtering. The width64 broad case visits 2,143 subscriptions and 2,144 changed handles, but admits only 64 tuple wakeups. Filtering prevents repeated matching while leaving the cost of finding notifications visible. A node-owned or more precise dependency representation could change that cost; these counts are not an intrinsic lower bound.

## Consequential corrections during the gate

The first passing implementation reduced matching but regenerated unrelated tuples whenever a fact was posted. The sparse width64 witness generated 4,355 combinations despite making only 197 head checks. Registration now anchors enumeration on each compatible head position of the new occurrence, reducing combinations to 195 on the identical source. Fact-entry visits are counted separately, including candidate enumeration.

The initial broad-alias implementation reinspected suspended constructor matches on unknown-to-unknown merges. Filtering against each saved failed descriptor/equality condition reduces width64 tuple inspections from 2,401 to 322. Subscription and changed-handle visits remain recorded. The [initial source and logs](s02-tuple-activation-gate/initial/source.log) retain the first result; [current work records](s02-tuple-activation-gate/source.log) and the [audit](s02-tuple-activation-gate/audit.json) verify the changed counts and unchanged scanning controls.

These corrections do not fix full-product materialization. The registered cold-prefix witness shows N scanning head checks against N² selective checks and retained tuples at all three widths. No query update or successful match compensates for that product. This is evidence against treating the full-tuple implementation as a sufficient representative of selective integrated matching, not evidence against all selective designs.

## Next decision: compare partial joins before measuring native costs

Keep T072 active for a source-derived partial-join comparison. Suspend a failed prefix before enumerating its remaining partners, extend it when inspected information becomes available, and preserve source tuple order, history and consumption. Compare with both scanning and full-tuple activation on the same cold-prefix, sparse and broad-update sources. Include late prefix eligibility, multiple possible partners and consumed prefix occurrences. Then determine which representations warrant complete ownership and cost measurement.

This is more informative now than timing full tuples alone: a directly witnessed avoidable product could otherwise dominate the result. Resource-aware source lowering and adaptive reunion remain consequential alternatives, and must be reconsidered at that boundary. The graph's local equality organization, its matching organization and the complete language architecture remain separate unresolved decisions.

No timing, allocation, compilation-inclusive or architecture ranking follows. The [registration](../registrations/S02-tuple-activation-gate.md), source tests, full relational package tests and scoped strict Clippy provide the validation trail. The reference interpreter is unchanged. The research goal remains active.
