# Nonground posts: conditional ownership gains and expensive resource matching

Demand execution has favorable allocation regimes on the new post sources, but repeated unsuccessful matching remains expensive. The enclosing-output binder is not the dominant measured allocation cost. These results justify a causal investigation of partner matching before selecting a runtime or treating the current demand implementation's losses as intrinsic.

**The next package investigates copying during resource matching.** Establish how much traffic comes from copying candidate arguments and match environments, then qualify a comparison that avoids unnecessary copies without changing bindings, claims or source competition. Direct integration remains the strongest distinct alternative at that boundary. No timing winner is selected here.

## What was measured

The [registration](../registrations/S03-post-ownership.md) covers six kinds of posts: input references, enclosing-output references, forward-produced values, unmatched values, duplicate occurrences and repeated template-eligible applications. Sizes 8/32 and both arrival orders use independently checked complete answers. Four a/b queries reuse preparation. Immediate release, a two-query window and retained-all consumers are crossed with full execution and cancellation of alternate queries.

Three demand configurations use static-birth result validity: ordinary demand, within-turn miss reuse, and miss reuse with derivation templates. Controls use compiled Scan, Indexed and inferred specialization. There are no source choices in these families; this matrix does not re-evaluate every demand reuse or choice policy. The earlier [source gate](S03-nonground-posts.md) retains their broader semantic checks.

All 864 configurations have two allocation-meter runs and one ordinary semantic replay, for 2,592 workload processes. All allocation pairs match exactly; ordinary endpoints agree, all phases join at the same live-allocation boundary, and final task-owned heap returns to baseline. Two separate work screens produce identical 288-row results. No cutoff occurred.

## The important contrasts

**Demand allocation is neither uniformly better nor uniformly worse than competent explicit execution.** For each of the three demand configurations, 15 of 72 complete scenarios request fewer bytes than all three explicit controls; 33 request more than all three; 24 lie between or equal an explicit result. These are unweighted matrix counts, not a language workload distribution or an executable policy that picks a different control for every metric.

The following retained-all examples include source construction, preparation, four queries and disposal. The Demand column uses ordinary static-birth demand, without miss reuse or templates. Figures are requested heap bytes; they are not RSS.

| Source and arrival | Demand | Scan | Indexed | Specialized |
|---|---:|---:|---:|---:|
| 8 forward-produced posts, reverse | 335,367 | 395,262 | 397,054 | 369,566 |
| 8 duplicate-post groups, reverse | 295,066 | 403,698 | 445,618 | 431,154 |
| 32 enclosing-output posts, original | 1,590,871 | 1,259,424 | 1,239,360 | 1,176,128 |
| 32 unmatched posts, original | 13,311,655 | 1,566,568 | 1,055,688 | 998,792 |

The first two examples show actual favorable ownership cases rather than relying on a weak scanned control. The last two preserve the contrary results. Reversing arrival can change the comparison: the 8-group duplicate case requests 517,758 bytes under original arrival and 295,066 under reverse arrival. No source semantics changed.

**Low allocation traffic does not imply the lowest peak.** Ordinary demand has a lower peak than specialization in 58 of 72 complete scenarios and a higher peak in 14, but no complete scenario has a demand peak below all three explicit controls. Scan supplies a smaller-peak competitor in these cases. These are separate ownership tradeoffs; allocation cannot stand in for runtime.

**The output binder is a real graph obligation with a small measured traffic difference here.** On 32 original-order input posts, ordinary demand requests 1,590,815 bytes and retains 292 graph nodes after the first query. Referencing the enclosing output instead requests 1,590,871 bytes and retains 324 nodes, with exactly 32 binders. Force entries rise from 3,720 to 3,752. The observed 56-byte session difference does not establish a universal per-binder cost or that extra nodes are free; allocation-capacity effects can matter.

**Miss reuse reduces work without making the tested miss case economical.** On 32 original-order unmatched posts, first-query candidate visits fall from 13,216 to 11,936 and force entries from 32,513 to 29,669. Four-query allocation falls from 13,311,655 to 12,259,743 bytes. Specialized explicit execution requests 998,792 bytes. At size 8, candidate visits are 312 without miss reuse and 232 with it; the larger population exposes much more repeated work. Two sizes do not establish a general asymptotic law.

**Template hits do not guarantee useful work elimination.** The repeated template source records 31 hits at size 32. Adding templates to miss reuse increases retained nodes from 356 to 421, requested bytes from 2,214,015 to 2,298,591, and peak excess from 93,497 to 95,482 bytes. These templates follow no source calls: they reuse a small post plan while runtime effects still execute. This is an overhead case, not a rejection of substantive derivation reuse across fresh applications.

## Why partner matching is next

Execution with owned observation accounts for 11,671,384 of 12,259,743 requested bytes in the 32-post miss-reuse example. Source, preparation, input and setup account for the remaining 588,359 bytes. In the current partner search, each live matching-signature candidate copies its argument vector and the existing match environment before checking its arguments. The work screen establishes repeated candidate visits, but does not isolate the allocator cost of those copies.

A hypothetical zero-allocation execution would leave less requested traffic than specialization's998,792 bytes. This is only an upper bound on possible improvement; it is not a realizable speedup or a claim that copying accounts for the whole difference. It shows why attributing this cost could change the ownership conclusion, unlike refining the already-small binder traffic.

The next causal comparison must preserve partially bound heads, late aliases, misses, duplicate resources and consuming claims. If copying is consequential, test borrowing already-bound match information or postponing copies until a candidate can extend it. Do not introduce an index merely because scanning is visible; do not weaken matching to make the path cheap. Price any new metadata, successful-candidate copies and adverse cases.

This follow-through is preferred to timing the current implementation immediately because a concrete potentially avoidable cost may dominate the result. It is also preferred, for one package, to starting direct integration: the current source and ownership evidence makes the missing causal question inexpensive to isolate. Reconsider direct integration and intermediate joins at that result or obstruction. Broader demand capabilities and sustained lifetime remain required; this does not assign the familiar prototype permanent priority.

## Measurement limits and necessary complexity

The runner charges source creation, preparation, query input, setup, execution with owned observation, engine disposal, consumer handling, prepared disposal and final consumer disposal. Complete scalar/analytical validation and harness storage are outside measured intervals. Retained answers survive producer disposal; cancellation is followed by reuse of the same preparation.

Work diagnostics separately count posts, output binders, resource candidates, forcing, matching, equation validation and template hits. Ownership builds reject enabled engine/kernel/work diagnostics. Ordinary runs validate semantics; their clocks are deliberately not interpreted as primary timing. Requested bytes and live requested peaks are not RSS. Compilation, long streams, arbitrary-depth safety and sustainable retention remain unmeasured here.

The new capability still owns resource births, claims, output links, template identities and completed-equation validation. Explicit alternatives own their matching and update structures instead. The result establishes costs within these responsibilities, not that their composition has the least necessary complexity or that any language-wide restriction should be adopted.

## Receipts and validation

The [driver](../../../research/chr-direct-conditional/experiments/post_ownership.py) preserves every subprocess result and configuration. The [independent audit](../../../research/chr-direct-conditional/experiments/audit_post_ownership.py) reconstructs phase continuity, exact repeats, complete endpoints, registered work counts and source/binary hashes. Its [results](s03-post-ownership/audit.json) contain every scenario and work row.

The [freeze](s03-post-ownership/freeze.json) includes a complete compressed source snapshot, allowing future audits to verify these measurements without requiring the working tree to remain at this implementation. The runner source/reuse gate, scoped strict Clippy, ordinary/meter endpoint checks and exact post/binder/template work assertions pass. The initial runner-mode rejection and qualification screen remain in the run directory alongside the two registered work repetitions.

T071 remains active for the selected causal comparison. The architecture goal remains active, with the broader sequence and all 57 reviewed questions intact.
