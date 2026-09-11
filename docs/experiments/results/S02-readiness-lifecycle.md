# Early equality savings survive, but do not beat the compiled control

**Selective settlement makes early failure substantially cheaper within the relational engine. The existing generic compiled engine remains faster on the decisive deep-source cases.** Readiness alone cannot close that gap: query setup already costs more than the compiled engine's complete lifecycle in the strongest early-failure witness.

The registered pilot completes 1,120 processes. All 160 allocation pairs repeat exactly and release their owned heap. A targeted follow-up adds 88 processes covering 4,400 complete lifecycles to investigate the small-query timing uncertainty. The goal remains active; the [full portfolio review](S02-readiness-portfolio-review.md) selects graph memo storage and lifecycle costs next.

## What was compared

Five executions use the same qualified sources: full equality settlement; selective settlement; bounded output draining with budgets 8 and 256; and the existing generic compiled engine with Global rule policy and Indexed access. The compiled engine is a serious independent execution competitor, not another newly constructed baseline.

Each source has an earlier consumer whose guard can fail and a later permitted consumer. A nested equation either supplies matcher information or affects only output. Outcomes are successful output, early source failure, late equality contradiction and cancellation immediately after query admission. Base depths are 4 and 64; one or sixteen changing queries reuse preparation. Successive queries alternate depth and token multiplicity.

The measured lifecycle includes preparation, query setup, execution through complete observation or failure, engine disposal, preparation disposal and consumer-answer disposal. Answers survive all producer disposal and are checked against the independent scalar evaluator outside measured intervals. Source construction and external compilation are outside this endpoint. Execution and observation are measured jointly because the relational interface produces the answer inside advance; these fixtures have at most one answer.

## The favorable witness and the stronger competitor

Representative sixteen-query sessions at base depth 64, separate output equality, early source failure:

| Execution | Median measured lifecycle | Requested bytes | Peak requested heap above resident inputs |
|---|---:|---:|---:|
| Full settlement | 13.145 ms | 19,268,856 | 140,929 |
| Bounded draining, budget 256 | 2.635 ms | 3,553,841 | 141,756 |
| Generic compiled | 0.998 ms | 1,273,392 | 42,755 |

**The deduction saving survives full query accounting.** Across the five paired timing blocks, budget-256 draining takes 0.098–0.203 times the full-settlement lifecycle in this case; the median paired ratio is 0.192. The selective and budget-8 schedules also show substantial gains. This supports the usefulness of early failure under fixed consumer priority.

**That gain does not establish an advantage over generic compiled execution.** The same budget-256 sessions take 2.526–3.141 times the compiled lifecycle, with median paired ratio 2.770. The selective and budget-8 counterparts are also slower in every paired block. Fewer engine advances are not comparable units of work across engines.

**Shared dependencies expose the cost of readiness.** For successful sixteen-query sessions at depth 64 with a shared matcher/output equation, budget-256 draining takes a median 28.712 ms versus 13.295 ms for full settlement and 0.989 ms for compiled execution. When output equality is separate, budget-256 takes 13.383 ms versus 14.159 ms for full settlement: that small difference is not a qualified general timing gain. Both still owe the successful output's equality work.

## Costs that deduction counts missed

All three readiness schedules request fewer bytes than full settlement in four of the 32 scenarios: the separate early-failure cases. They request more in the other 28 and have higher peak requested heap in all 32. Each requests more bytes and uses higher peak heap than the compiled control in every scenario. These are allocator-request measurements, not RSS.

The phase measurements identify a broader representation cost. In the depth-64, sixteen-query early-failure witness, selective query setup alone has median 1.458 ms; compiled setup has median 0.609 ms. Selective execution/observation has median 0.968 ms, versus 0.196 ms for compiled execution. Phase medians need not add to the median of complete sessions.

An intentionally optimistic sensitivity calculation sets **all selective execution and observation time to zero**. The remaining measured lifecycle still exceeds compiled's complete lifecycle in every paired block: ratios range from 1.413 to 1.643. The corresponding bounds are 1.349–1.573 for budget 8 and 1.500–1.864 for budget 256. This calculation is not a measured optimized engine. It shows why refining readiness alone cannot reverse this witness; constructor representation and query admission would also need a different implementation.

The code identifies responsibilities, not an unmeasured causal ranking: relational admission constructs constructor facts and their relation indexes; equality services descriptors, cycle checks and incidence repair; readiness additionally walks pending and constructor connections. The current data attribute costs to phases. They do not prove which subroutine dominates each phase, or reject a different integrated representation.

## The uncertainty was investigated

The pilot uses five shuffled ordinary timing blocks and two separate allocation blocks. Its prospective practical criterion requires a median difference greater than 10%, all paired ratios on the same side of one, and sufficient distance from the empty-phase clock floor. Against full settlement, many small differences remain unresolved. None supplies a universal budget choice.

Five selective/compiled comparisons at depth 4 and one query initially crossed equality. The [registered follow-up](../registrations/S02-readiness-small-sessions.md) runs 50 fully prepared and disposed sessions per process in nine ordinary blocks, plus two allocation blocks. Four contrasts then qualify as slower than compiled; their median paired ratios are 1.690–1.852. The fifth, shared admission cancellation labelled budget 256, has median 1.721 and range 0.988–2.241, so its individual timing criterion remains unmet.

That last label cannot distinguish draining budgets: cancellation performs zero advances, and all three readiness modes execute the same preparation, setup and disposal code. Their measured ownership is exactly identical. The corresponding selective and budget-8 cancellation comparisons qualify as slower. Further budget-specific precision here would not change a scheduling decision; no drain occurs. Preserve the noisy cell, rather than treating it as a budget-dependent crossover or silently declaring it passed.

## Disposition

Retain selective settlement as a demonstrated way to avoid work on early failure. Retain bounded draining as a repair for repeated output scheduling. Neither is selected as the architecture or as a language policy. Fixed consumer priority and permissive successful serialization remain separate language choices.

Use the compiled control in subsequent complete-path comparisons. The large deep-source gaps and the zero-execution bound make another readiness-only optimization campaign less valuable than the ready graph lifecycle comparison. Broader integrated representations, output-only matcher omission, state-inspecting guards and coherent mixed-source execution remain explicit investigations; this bounded source family does not answer them.

## Evidence and reproduction

- [Pilot registration](../registrations/S02-readiness-lifecycle.md), [precision registration](../registrations/S02-readiness-small-sessions.md)
- [Pilot freeze](s02-readiness-lifecycle/freeze.json), [raw samples](s02-readiness-lifecycle/samples.jsonl), [analysis](s02-readiness-lifecycle/analysis.json)
- [Follow-up freeze](s02-readiness-small-sessions/freeze.json), [raw sessions](s02-readiness-small-sessions/samples.jsonl), [analysis and sensitivity](s02-readiness-small-sessions/analysis.json)
- [80 entry validations](s02-readiness-lifecycle/entry.json), [allocator check](s02-readiness-lifecycle/meter-check.json), [regressions](s02-readiness-lifecycle/regression.log)
- [Ordinary Clippy](s02-readiness-lifecycle/clippy-ordinary.log), [metered Clippy](s02-readiness-lifecycle/clippy-meter.log)
- [Pilot auditor](../../../research/chr-relational/experiments/audit_readiness_lifecycle.py), [follow-up auditor](../../../research/chr-relational/experiments/audit_readiness_small_sessions.py)

Both auditors reconstruct the claims from archived source hashes and raw receipts. Release timing uses ordinary allocation with engine/kernel diagnostics disabled; allocation-run timings are not primary observations. All processes are serial and CPU-pinned with 60-second CPU/wall and 1 GiB address-space limits. No cutoff occurred. The follow-up verifies 800 metered complete sessions with identical normalized ownership and complete release. Compilation, broader source capability and sustained consumer policies still require their own architectural comparisons.
