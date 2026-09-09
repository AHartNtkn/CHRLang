# Native execution saves runtime work; compilation and stronger controls change the choice

Generated execution improves runtime over prepared generic plans on most tested sources. Its compilation-inclusive cost remains higher at every measured reuse count. Dedicated subscription execution is faster still, while the choice between retaining and rediscovering matches depends on demand lifetime.

These findings come from the [registered pilot](../registrations/S01-native-cost-pilot.md): 286 timing cells, one warmup and seven measured blocks, five compilation observations per ordinary artifact/configuration, and separate allocation diagnostics. No architecture or workload weighting is selected.

## Generated execution earns bounded runtime gains

The table shows native runtime divided by the prepared-plan control, at the largest measured query count for each profile. Lower is faster. These runtime sums include source construction, preparation, all completed-query phases and disposal, but exclude native compilation and the harness intervals described below.

| Source and query count | Ordinary linking | ThinLTO | Registered interpretation |
|---|---:|---:|---|
| Chain, 32 edges, 16 queries | 0.884 | 0.874 | Practical gain |
| Payload, 64 variables, 256 queries | 0.731 | 0.707 | Practical gain |
| 64 rules, first position, 16 queries | 0.991 | 1.015 | Unresolved |
| 64 rules, middle position, 16 queries | 1.016 | 0.992 | Unresolved |
| Subscription, 16 paths, 16 queries | 0.838 | 0.825 | Practical gain |
| Low-yield stable demands, 8 queries | 0.812 | 0.731 | Practical gain |
| Low-yield reopening, 8 queries | 0.784 | 0.716 | Practical gain |
| Low-yield churn, 8 queries | 0.810 | 0.730 | Practical gain |

Each low-yield query uses size eight and 64 requests. A practical gain requires a median paired ratio at most 0.90 and all seven block ratios below one. This is a descriptive repeated-run criterion, not a confidence interval. The [analysis](s01-native-cost-pilot/analysis.json) preserves every registered contrast and its full ratio range.

The one-query dispatch cases provide contrary evidence: native runtime is 15–32% higher than the prepared-plan control, meeting the loss criterion under both compiler configurations. Preparation contributes, but is not the whole explanation. At the first dispatch position with ordinary linking, median preparation is 131 versus 113 microseconds, and execution is 37 versus 18 microseconds. The pilot does not establish which part of that execution difference is intrinsic to generated code.

Across all seventeen source/size/reuse combinations, ordinary-linked native execution has thirteen practical gains, two losses and two unresolved comparisons against prepared plans. ThinLTO has twelve gains, two losses and three unresolved comparisons. These counts describe coverage; they are not workload scores.

## A stronger source control changes the interpretation

The dedicated subscription executor implements the same checked source contract, with indexed rediscovery, eager retention or subscriptions. It avoids general rule-execution machinery that both the generated and generic engines retain. Its source support is narrower, so the comparison establishes an opportunity for source specialization, not a universal replacement.

At the largest measured reuse, all three dedicated modes beat native execution on all four subscription profiles. Under ThinLTO, dedicated indexed runtime is 31% of native on the ordinary subscription query and 30–47% on the low-yield profiles. Native-versus-generic gains therefore cannot settle the architectural comparison.

The retention mechanism itself has opposing regimes. The following ThinLTO ratios compare modes inside the same dedicated source executor, at the largest measured reuse.

| Profile | Eager retention / indexed | Subscriptions / indexed |
|---|---:|---:|
| Ordinary two-request subscription | 1.034, unresolved | 1.126, practical loss |
| Stable low-yield demands | 0.408, practical gain | 0.424, practical gain |
| Reopening demands | 0.486, practical gain | 1.120, practical loss |
| Low-yield churn | 0.620, practical gain | 0.627, practical gain |

Stable demand retention avoids repeated discovery. Reopening makes subscription establishment and retirement matter enough to reverse that comparison. The source contract and these costs must remain explicit before considering any inferred selection policy.

## Compilation has not amortized within the measured reuse

Median artifact compilation times across five observations are:

| Artifact | Ordinary linking, seconds | ThinLTO, seconds |
|---|---:|---:|
| Generic executable | 0.059 | 2.984 |
| Chain ruleset | 0.179 | 3.287 |
| Payload ruleset | 0.164 | 3.273 |
| Subscription ruleset | 0.498 | 3.517 |
| 64-rule dispatch ruleset | 1.535 | 4.409 |

The generic executable is reusable across rulesets; its compilation is not charged per ruleset. Native source generation, writing, source-buffer disposal, compilation and artifact filesystem disposal are ruleset costs. In every measured source/size/reuse combination, the lowest observed native compilation-inclusive total exceeds the highest observed prepared runtime total, under that accounting convention.

This is a bounded observation, not rejection of native compilation. Linear models based on later queries predict ordinary-link break-even near 2,114 chain queries and 4,751 payload queries; those counts were not measured. Dispatch models require hundreds of thousands of queries and rest on unresolved small runtime differences, so they do not support a crossover claim. The models and their assumptions are recorded separately from observed comparisons.

ThinLTO provides practical runtime gains in 51 of 143 within-mode contrasts, no practical losses, and 92 unresolved comparisons. Its larger compilation cost nevertheless postpones modeled amortization. Compiler configuration belongs in the lifecycle decision rather than being selected solely for its runtime behavior.

The binaries include the experimental runner, source builders and oracle linkage. In particular, ThinLTO optimizes that combined artifact. These compiler costs are measured costs of this experimental boundary, not lower bounds for a minimal production compiler/runtime split.

## Allocation and complexity remain separate dimensions

All 143 diagnostic cells replay exactly and restore query/prepared live-byte baselines. For payload's 256-query lifetime, included requested traffic is 39,755,320 bytes with prepared plans and 29,538,269 with native execution. For the sixteen-query subscription lifetime it is 9,540,918 versus 5,214,026 bytes; the dedicated indexed control requests 1,993,488 bytes.

Lower traffic does not imply a smaller peak. On stable low-yield queries, native's absolute requested-live peak is 305,469 bytes, while the dedicated variants range from 330,198 to 335,593 bytes. These are process-wide requested-allocation peaks including the independent oracle and process state, not isolated engine memory or RSS. The phase readings expose that limit; they cannot be promoted to intrinsic storage lower bounds.

Generated continuations avoid frame snapshots, candidate vectors and interpreted key templates. Prepared and generated update plans both omit unnecessary maintenance. The native path still carries generic rule metadata, kernel terms/equality, occurrence storage, search and observer responsibilities; generating code does not eliminate those boundaries. The dedicated source executor removes more general execution work but requires its own source eligibility and effect-preservation argument. Those responsibilities explain why code generation and source specialization are different architectural choices.

One measured preparation detail is avoidable in the current native-plus-prepared control: it links generated update metadata through the ordinary constructor before installing the column plan. It matters in some cold dispatch contrasts. That control must not define an inherent native preparation cost; the primary native-versus-prepared comparison does not require both update representations.

## Evidence limits and next decision

All 2,288 timing processes, 286 diagnostic processes and 60 source preflights complete. They perform 48,864 independent full-query checks, with no cutoff or substituted cell. Runtime children are pinned to CPU zero on the recorded i7-1260P host. The [validation receipt](s01-native-cost-pilot/validation.json), [source freeze](s01-native-cost-pilot/source-freeze.json), [environment](s01-native-cost-pilot/environment.json) and [raw summary](s01-native-cost-pilot/summary.json) preserve the evidence. Strict emitter/library Clippy passes.

The primary runtime sum excludes input/oracle construction, validation, stdout and process startup/exit. Those operations can affect caches and allocator state; process wall and child CPU remain separate receipts including harness work. Filesystem disposal records syscall time, not durable-storage synchronization. Earlier interruption checks establish ownership, not a comparative cancellation-speed result. This is complete accounting of the stated completed-query and artifact intervals, not whole-architecture lifecycle superiority.

The first complete cost contrast now justifies a targeted long-reuse follow-up on chain and payload. Their measured savings could repay compilation within a feasible extended run, and neither has the dedicated subscription control that already dominates the native path here. Test beyond the modeled crossover, with prospective bounds and repeated observations, before deciding that compilation fails to amortize. This is cheaper and more decision-relevant than another broad access-policy refinement.

Direct pull-tabbing and derivation reuse remain the strongest distinct ready architecture investigation. Reassess that alternative after the targeted amortization result; broader S01 access plans and all other required sequence directions remain open. T070 and the overall goal remain active.
