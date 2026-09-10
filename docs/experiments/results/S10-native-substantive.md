# Substantive mixed sources pass; a solver admission question changes the next step

All 96 mixed sources now pass the reference, ten general Rust configurations and both native paths. Ten native cases needed a larger service bound; all complete after that extension. The current source-derived controls reject this family, so the next investigation tests a concrete admission boundary before committing to the cost matrix.

## The sources do observable work

A cursor traverses and consumes a chain of edge occurrences, explicitly equating each pair of endpoints. A kept watch/cursor rule emits one seen occurrence at every position. A chain of length n must therefore leave exactly n+1 seen occurrences, including their multiplicity. Termination consumes the cursor and finish occurrence.

Independent choices bind selected unknowns to a or b. They run either before the chain or after it. Other variants include a failed sibling and unrelated ground edges that cannot match the cursor. These distinguish common work, branch-specific work, discovery overhead and abandoned work without assigning workload weights.

The 96 queries vary chain length 0/2/8, nonmatching edges 0/8, independent choices 1/3, early/late choices, successful/failed siblings and two sets of variable numbers. They share four ordered rulesets. Successful-choice cases have 2^k answers; failed-sibling cases have one. All outputs and residuals are ground and specified independently from the execution implementations.

## What passed and what initially stopped

| Check | Result |
|---|---|
| Independent expectations against unchanged reference | All 96 agree, including raw answer counts and residual multiplicity |
| Ten general Rust configurations | All 960 configuration/query checks pass with prepared reuse |
| Existing prefix lowering and finite solving | 192 configuration/query admissions are unsupported |
| Initial native screen | Each build completes 86 queries and reaches its service bound on ten |
| Extended native screen | Both builds complete all 96 queries |
| Original native cutoff prefixes | All 20 build/query prefixes replay exactly |
| Changing native queries with retained consumers | All 192 build/query replays agree with standalone runs; ownership sessions end at zero tracked live bytes |

The initial native bound was 65,536 service calls. The extension changes only the harness's accepted input limit to 1,048,576; it leaves the reducer, observer quota and process resource bounds unchanged. Previously complete observations replay exactly. The largest case finishes in 385,420 calls and uses 12,323,994 dynamic heap words. These are service and arena-use counts, not comparative speed, live reachable size or RSS.

The difficult cases combine early alternatives with useful chain work and nonmatching edges; the largest late-choice case also crosses the initial bound. None remains incomplete after the extension. No source size was reduced to obtain completion.

## Why the solver rejection needs investigation

The leading choice rule consumes `pick(X)` while reading a kept zero-argument `ready` occurrence. The finite solver currently requires unguarded single-head consumption and rejects any kept head. Prefix lowering likewise finds no admitted private pure prefix in these sources. These are precise implementation admission boundaries, not evidence that source analysis cannot simplify the programs.

For early-choice queries, readiness is already present and choice production does not consume it. That suggests a bounded question: can an admission check account for this read once, then apply the existing finite producer machinery while preserving residual readiness and caller behavior? This is a hypothesis, not a proven transformation. Late-choice queries lack readiness until the chain completes and provide an immediate challenge to any unconditional rewrite.

A valid investigation must also challenge absent or duplicate readiness, repeated kept heads, shared consumables, non-ground enabling patterns, later enabling and source priority. It must preserve complete raw multiplicity and finite service, and charge admission and transport. Merely omitting the kept head would not establish the proposed control.

## Breadth review and selection

This reviews the four-package native comparison entry: common-source qualification, prepared ownership, ordinary timing boundaries and this substantive screen.

| Ready investigation | Architectural question it could change | Current judgment |
|---|---|---|
| Complete native/Rust cost matrix | Whether coherent serial paths repay preparation, service and observation costs | Valuable, but still needs Rust timing qualification and credible treatment of the newly exposed lowering boundary |
| Finite production with a checked kept read | Whether source analysis can remove discovery/execution responsibilities in a mixed source currently excluded by admission | Selected: an existing solver supplies the implementation base, and absent/late readiness supplies a direct falsifier |
| Native local claims and broader terms | Whether local effect ownership or general data changes native feasibility and efficiency | Required; substantially more implementation and correctness work than resolving this specific control gap |
| Broader call reuse or restoration | Whether validity/reclamation or recomputation changes the complete cost of alternative futures | Required; this screen supplies no new evidence resolving those mechanisms, and they remain candidates at the next review |

Continue T078 with a bounded kept-read eligibility and complete-source gate. If it succeeds, include the admitted solver-plus-caller path in the subsequent cost qualification. If it fails, identify the semantic obstruction and decide whether a different derivation is warranted. Neither outcome resolves richer resource solving or source analysis generally.

Review breadth again at that gate's boundary before further eligibility expansion. The native/Rust cost study remains pending, with Rust counter/consumer qualification, source construction/emission, clock overhead and allocation-accounting scope still necessary. Local native ownership, general terms, sustained graph handles and every other consequential mapped question remain open. No architecture ranking follows from this package.

## Evidence

[Registration and bound extension](../registrations/S10-native-substantive.md), [source generator](../../../research/chr-hvm/substantive/cases.py), [frozen cases](s10-native-substantive/cases.json), [reference observations](s10-native-substantive/references.json), [Rust records](s10-native-substantive/rust.jsonl), [finite admission probes](s10-native-substantive/finite-admission.json), [initial native screen](s10-native-substantive/native.jsonl), [extended runs](s10-native-substantive/extension.jsonl), [cutoff prefix replays](s10-native-substantive/prefix-replays.jsonl), [prepared reuse](s10-native-substantive/reuse.jsonl), [audit](s10-native-substantive/audit.json), [build receipts](s10-native-substantive/extension-build.json) and [input hashes](s10-native-substantive/validation.json) retain the complete evidence. The existing common-source, native ownership and ordinary-timing audits pass unchanged.
