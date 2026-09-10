# Native source execution preserves equality, replacement and propagation history

The native compiler now passes all 61 identity/effect source obligations and 16 compound cases. Complete residuals, multiplicity and output aliases agree with both direct expectations and the unchanged reference interpreter. This qualifies a deterministic atom/unknown source fragment under serial rule ownership; it does not qualify the full language or concurrent ownership.

## The source distinctions now execute together

| Source behavior | Complete native result | Design implication |
|---|---|---|
| Replace an occurrence with another of the same value | Propagation fires again for the new occurrence | Value equality does not erase occurrence history |
| Bind an existing unknown after propagation | Earlier observations acquire the binding without another propagation firing | Equality updates and application identity remain distinct |
| Introduce a fresh body unknown | Old outputs retain their unknown while the replacement receives a fresh one | Fresh allocation is part of body execution, not host-side expected-answer construction |
| Match two kept occurrences | Ordered tuples produce the required receipts, including equal-valued occurrences | Matching preserves distinct resources and ordered application multiplicity |
| Enable equality before or after a competing consumer | The source-priority winner agrees with the reference | Disjoint consumed heads alone do not justify reordering binding effects |
| Combine propagation, replacement, linking and consumption | Early consumption leaves one receipt and a ticket; replacement followed by linking leaves two receipts | The individual identity operations compose on these interacting sources |

The compound comparison crosses initial aliases, binder rule priority, query order and variable numbering. The earlier kernel tests established separate operations; this gate exercises actual source-selected rule execution and complete observations.

## What the compiler actually generates

Compilation emits a rule selector and a scan for each source head. At runtime, those scans enumerate distinct occurrences, build rule-variable substitutions and check repeated variables using nonbinding equality. A successful propagation match consults the ordered rule/occurrence history before firing. Unsuccessful deeper head matches return to the preceding scan.

Body execution removes the selected consumed occurrences, allocates fresh body variables, inserts new occurrences in source order and performs equality updates. The selector restarts from the first rule after the body completes. Fresh identities have an explicit namespace limit; exhaustion produces a limit result rather than silently reusing an ID.

**The host compiler does not execute the source or embed its answers.** It walks source structure to emit matching and body functions. Queries and selected output variables initialize native state. The native program performs the deductions, and observation resolves outputs and residual arguments. Exhaustive renaming comparison happens outside execution and retains residual multiplicity.

## The native organization still has substantial responsibilities

This is a serial functional organization over explicit occurrence lists, equality links, substitutions and history. Native graph reduction executes it, but it has not eliminated the equality service, history bookkeeping or source-priority owner. Generated scans also require continuation state and explicit duplication of reused native values.

Those facts matter when comparing architecture complexity. This result establishes feasibility of the tested mapping. It does not establish that its list representation, scan strategy or copying behavior is economical, nor that a more local graph organization needs the same structures.

## A cutoff was investigated before the bound changed

The initial quota-1 run reaches the 8,192-call limit on three replacements. The identical emitted program completes at quota 8 in 1,476 calls and agrees with ordinary native collapse. The larger quota changes work per call, so this is a diagnosis of insufficient initial service fuel rather than a speed comparison.

A separate runtime binary raises only the accepted call-limit ceiling from 8,192 to 65,536. The original frozen runtime remains intact. The audit verifies that single source change, exact replay of the six initially completed cases, and the full 8,192-event prefix of the incomplete case. No incomplete run counts as an answer or a failed source branch.

All final cases complete within the corrected bound; the maximum is 28,416 quota-1 calls. This establishes bounded completion for the gate. The number of native visits is not directly comparable to CHR steps or elapsed time, and does not settle efficiency.

## Validation and supported scope

The final matrix contains 308 native runs across 77 sources, two quotas and two repetitions; 385 diagnostic/cancellation runs; and 77 ordinary native controls. Every source also passes the independent complete expectation through the reference adapter. The 244 native runs from the earlier 61-source gate replay exactly after adding the compound cases.

The audit reconstructs generated programs, validates complete answers and alpha-equivalence, retains raw multiplicity, checks service visits and stack baseline, and verifies counter-off behavior and cancellation prefixes. The reference adapter only translates the admitted syntax and prints observations; the reference engine is unchanged. All 15 containing-package tests, scoped strict Clippy and formatting pass. After formatting the adapter, all 77 reference observations replay exactly; the measured source snapshot and current hashes record that distinction.

Admission checks reject guards, alternatives, constructor terms, empty heads and an additional query-goal field. The admitted fragment has deterministic conjunctions, atom/unknown arguments, kept and consumed heads, insertion, equality and selected query variables. This gate does not prove correctness for every admitted program; it supplies discriminating evidence for the stated obligations and their composition.

General constructors, choices interacting with identity-bearing effects, failed and ongoing source branches, reusable-query ownership and local competing claims remain required. The compiler maps a conflicting binding to native erasure, but these 77 successful-source cases do not qualify its complete failure behavior. Cancellation checks cover recorded service prefixes, not cancellation during a validated concurrent claim protocol or sustained heap reclamation.

## Next: combine native choices with the identity-bearing source path

Continue T080 with choices, failure and finite-answer progress on this source representation. The prior ground compiler has those behaviors, but separate ground-choice and deterministic-identity gates cannot establish their interaction. A choice that changes aliases or propagation history could expose incorrect correlation or effect ownership despite both separate gates passing.

| Ready investigation | Decision it could change | Priority at this boundary |
|---|---|---|
| Choices and failure with native identity-bearing effects | Whether the native candidate can combine search and the newly qualified source semantics | Selected: both starting mechanisms exist, and their composition remains an essential untested boundary |
| Integrated graph guards/search and coherent architecture qualification | Whether the earlier graph-scan gains survive broader language responsibilities | Required; reconsider after the native composition gate or a consequential obstruction |
| Native local claim/commit ownership | Whether connected effects can run without the serial source owner | Required; distinct from the present serial correctness result |
| Richer solving, reuse and sustained ownership | Whether other organizations eliminate more work or retain less state | Required; existing bounded results do not resolve these directions |

This selection is a judgment about decision value and implementation readiness, not evidence against another design. Do not begin native timing or parallel scaling by treating the serial fragment as a complete language implementation. The research goal remains active.

## Evidence

[Registration and bound correction](../registrations/S03-native-identity-source.md), [source compiler](../../../research/chr-hvm/source_identity/compiler.py), [independent source expectations](../../../research/chr-hvm/source_identity/cases.py), [reference adapter](../../../research/chr-cases/examples/native_identity_reference.rs), [audit](s03-native-identity-source/audit.json), [native records](s03-native-identity-source/runs.jsonl), [reference observations](s03-native-identity-source/references.json), [diagnostics](s03-native-identity-source/diagnostics.jsonl), [ordinary controls](s03-native-identity-source/controls.json), [cutoff diagnosis](s03-native-identity-source/cutoff-diagnosis.json), [build derivation](s03-native-identity-source/build.json), [measured input hashes](s03-native-identity-source/validation.json), and [reference revalidation](s03-native-identity-source/reference-reverification.json) retain the evidence.
