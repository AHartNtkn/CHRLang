# Compiled Global controls pass; Active depends on query order

Both existing compiled Global controls—scanned and indexed—match all 40 finite source configurations used for the direct graph. Active scheduling agrees on the word family only when the token is inserted first. **The cost comparison now has validated Global controls; Active can be admitted only for its checked subset.**

This follows the [prospective control gate](../registrations/S03-compiled-control-gate.md). The [source test](../../../research/chr-direct-conditional/tests/direct_graph_entry.rs) compares complete raw answers against mathematical or hand-derived expectations, with independent scalar and Conditional checks. It reuses the existing compiled executor; it creates no additional production baseline.

## What is admitted

| Control | Complete-answer evidence | Disposition |
|---|---|---|
| Global, scanned access | All 40 finite configurations; 303 expected raw answers per run; finite sibling observed once during 20,000 steps without exhaustion | Admitted across this gate |
| Global, indexed access | Same full gate and finite-sibling checks | Admitted across this gate |
| Active, either access, token-first word queries | All 14 word configurations agree completely for each access | Admitted on this subset only |
| Active, either access, token-last word queries | All 14 word configurations per access have the same raw answer counts but lack the token observation | Different completed work; exclude from equal-answer timing on this subset |

Global controls preserve complete output/residual aliases, raw equal-arm multiplicity, propagation over identified tuples, kept/consumed heads, late binding and disconnected failure. These are generic prepared compiled-search controls. This gate does not validate generated source-specific code or the graphless word lowering still needed to distinguish compilation savings from graph sharing.

## Why Active differs

Active services source occurrences in an order that can consume the token before its propagation rule observes it. The no-choice trace makes this concrete: token-last runs `base`, then `use`; token-first runs `watch`, `base`, then `use`. The source rules are unchanged. This is a committed-schedule difference, not evidence that Active violates the language's scheduling freedom.

The diagnostic checks all 56 word/access configurations. Exactly 28 agree with the original complete answers. For every other configuration, the actual multiset agrees exactly after excluding the expected `seen(H)` occurrence; no other answer difference is hidden by the classification. Raw answer counts alone agree in all configurations and would have missed the difference in completed work.

The comparison must therefore retain the insertion-order premise when using Active. Its success on token-first words does not establish general admission for effectful or competing-resource sources. Conversely, its token-last difference does not reject activation as an architecture.

## Validation and next investigation

Default, fresh replay and metrics-off builds pass all twelve test functions, including the control gate and Active diagnostics. The Active agreement classifications and explaining traces repeat in all three builds. Strict Clippy passes in both configurations, as does formatting. Every process stayed within its 60-second bound. [Commands and hashes](s03-compiled-control/validation.json), [default results](s03-compiled-control/default.log), [replay](s03-compiled-control/replay.log), [metrics-off](s03-compiled-control/off.log). No timing result is taken from these tests.

T062 remains active for the bounded lifecycle pilot. Next, add a checked graphless direct control for the word-source lowering and gate a substantive common-work source with immediate-discrimination and no-choice contrasts. Then freeze the configurations, lifecycle boundaries, repetitions and interpretation before comparative runs. The graph executor returns materialized answers from service calls, so execution and observation must be measured jointly where they cannot be credibly separated; artificial phase accounting must not favor one API.

This control gate resolves a necessary comparison premise, not the architecture choice. S01's selective/update comparison and the distinct S02/S06 designs remain open. No further broad scheduling screen is needed before the first bounded pilot: use the demonstrated admission boundaries and investigate any new source separately.
