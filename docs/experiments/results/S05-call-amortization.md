# Compilation does not repay its cost in the tested sessions

Generated execution does not establish an advantage over the existing combination of prepared data plans and inferred specialization. Charging compilation makes it slower than every installed-runtime control at both 64 and 8,192 queries. Call-trace reuse remains worth investigating: its runtime advantage survives both generated competitors in seven of eight scenarios.

The next experiment should test bounded retention and regeneration of call traces. Compilation remains a candidate for different sources and complete architectures; these results establish no universal winner.

## The comparison now separates three sources of savings

**The existing data-plan control matters.** `PreparedRuleset::new_with_update_plan` projects updates using ordinary prepared data. Generated code performs that projection too. Including the existing constructor, both alone and with inferred specialization, tests whether emitting native code adds a benefit beyond avoiding unnecessary work.

The first campaign was interrupted when this missing control was identified. Its 404 completed process receipts, including 86 timing runs, remain available. The amended campaign uses fresh program artifacts and fresh randomized timing, with the same frozen shared libraries. No timings are pooled across the two campaigns.

**The nine controls execute the same sources and changing queries.** They are call-trace reuse; Direct execution; compiled scan and indexed execution; inferred specialization; projected data plans; projected data plans with inference; generated execution; and generated execution with inference. The source gate covers 1,280 queries and 3,840 cancellation restarts in each of the counter-free and counted configurations.

**Per-program compilation is now measured directly.** Each artifact contains one source ruleset and no query values. Five ordinary compiler repetitions per source and artifact type isolate emission, source writing, buffer disposal and compilation against frozen shared libraries. A generic wrapper measures equivalent packaging cost. An installed generic runtime is charged zero Rust compilation per user program.

## Runtime savings survive some controls, but not the strongest data-plan control

Each row below is one prepared ruleset serving 8,192 queries, exhausting each query and immediately disposing its answers. Values are median complete runtime phase sums, in seconds; they include source construction, preparation, query setup, observation and disposal. They are not per-query execution times.

| Source regime | Trace reuse | Direct | Data plan + inference | Generated + inference |
|---|---:|---:|---:|---:|
| Failed alternative, depth 128, repeated query | 1.268 | 3.290 | 2.609 | 2.733 |
| Fresh output, depth 128, four-query cycle | 1.375 | 3.429 | 2.743 | 2.649 |
| Binding before remaining work, depth 32, four-query cycle | 0.496 | 0.976 | 0.957 | 0.806 |
| Shallow ordinary calls, depth 4, repeated query | 0.141 | 0.161 | 0.218 | 0.213 |

**Generated matching beats ordinary indexed execution in all eight scenarios.** Its paired median runtime ratios range from 0.634 to 0.848. Generated-plus-inference qualifies in seven of eight comparisons against indexed execution. These are real scoped benefits of a competent generated competitor.

**Projection alone does not explain all of those benefits.** Ordinary generated execution also qualifies faster than the projected data-plan control in five scenarios; generated-plus-inference does so in six. However, neither generated mode establishes a gain against data plans combined with inference. Ordinary generated execution loses two such comparisons; the other fourteen remain uncertain.

**The uncertainty is material.** Generated-plus-inference has paired median ratios from 0.900039 to 1.074 against data plans plus inference, and none meets the registered gain rule. That rule requires at least a 10% median improvement and all five repetitions faster. A median close to the boundary is not proof of equivalence or of a meaningful advantage. The table uses medians of absolute times; verdicts use paired ratios, so dividing table entries does not reproduce each verdict.

**Trace reuse's benefit survives optimized execution.** Each generated mode qualifies slower than traces in seven scenarios; the remaining shallow 64-query comparison is uncertain. This makes retention worth improving, rather than assuming native matching makes reuse unnecessary.

## Compilation changes the complete cost comparison

| Program family | Generic wrapper compiler median | Generated compiler median | Generated source size |
|---|---:|---:|---:|
| Ordinary calls | 3.018 s | 3.209 s | 15,507 bytes |
| Failed alternative | 2.984 s | 3.186 s | 17,699 bytes |
| Fresh output | 3.079 s | 3.198 s | 15,642 bytes |
| Binding before remaining work | 3.058 s | 3.160 s | 19,670 bytes |

These timings include compiling and linking each program wrapper. Across all ordinary repetitions, generic wrapper compilation spans 2.620–3.138 seconds and generated compilation spans 2.756–3.446 seconds. Shared dependency/tool construction is recorded separately in the original campaign. Source emission, writing and buffer disposal are additionally charged; compilation is not inferred from a Cargo build of the whole dependency tree.

**Against an installed runtime, all 112 comparisons qualify as losses after compilation.** This includes both generated modes, all seven controls, all four regimes and both query counts. Every verdict considers all 25 combinations of five compiler and five runtime samples. Charging the whole emitter/compiler/runtime process envelope also produces 112 losses; that envelope includes validation and recording overhead and is reported separately from the runtime phase sum.

**Equivalent packaging gives a different, narrower answer.** If both sides pay for a Rust wrapper, five comparisons qualify as generated gains, 32 as losses and 75 remain uncertain. All five gains occur at 8,192 queries, against indexed or projected-plan execution in the failed-alternative and fresh-output families. None establishes an advantage over Direct, traces or data plans plus inference. An interpreted user program does not acquire a Rust compilation requirement because this packaging control exists.

**The registered 32,768-query extension did not qualify.** It required a generated mode to have at least 10% median runtime savings against all five stronger non-reuse controls at 8,192 queries, with no compilation payback yet. No mode meets that condition. The closest planned-inference ratio is 0.900039, and trace reuse remains a faster competitor. More repetitions of these same sources are therefore lower priority than testing whether useful reuse can retain less state. No crossover is extrapolated.

Artifact disposal is included in final costs: each source/binary pair takes 0.564–0.882 milliseconds to dispose. It changes no installed-runtime qualification. Binary artifacts are roughly 5.76–5.79 MB, including the shared experimental runner and validation machinery; that size is not a measurement of a minimal deployed runtime.

## Reuse reduces allocation while retaining more state

Requested allocation and peak live ownership tell different stories. At 8,192 queries, traces request 1.026 GB in the failed-alternative case, versus 3.017 GB for data plans plus inference. Their peak is 152,230 bytes versus 88,784 bytes. In the fresh-output cycle, traces request 1.085 GB versus 2.918 GB but peak at 196,949 bytes versus 96,721 bytes.

After each query, the repeated failed-alternative trace retains 20,603 bytes; the fresh-output cycle settles at 63,337 bytes. The corresponding data-plan controls retain 5,997 and 5,661 bytes. All producer and consumer ownership returns to its starting value at final disposal. These figures describe requested heap ownership, with validation and recording storage outside that ownership boundary.

The shallow case prevents a blanket memory conclusion: trace peak is 16,230 bytes, below the planned-plus-inferred control's 22,016 bytes, but above Direct's 8,682 bytes. Retention experiments must preserve these distinct regimes rather than optimizing an invented weighted average.

## What this changes in the decision

**Keep data plans plus inference in future complete comparisons.** Their runtime competitiveness shows why source analysis and native compilation need separate controls. Source-analysis work can remain useful even when compiling generated code does not pay here.

**Investigate trace retention next.** The preceding long-session experiment found finite working sets plateauing and distinct keys continuing to accumulate. This campaign establishes that optimized generated matching still leaves useful trace runtime gains. Reclamation between queries can now test a concrete tradeoff: how much of that gain survives when completed keys and abandoned calls must be regenerated. Reuse the existing table and independent source checks, including cancellation and retained answers.

**Sparse connected projection is the strongest independent alternative.** It could avoid measured Cartesian elimination overhead under T076. Retention takes priority for one bounded package because it addresses a measured time–memory tradeoff on sources that now have stronger controls. Reassess the full 57-question portfolio after that package, or sooner if safe reclamation exposes an architectural obstruction. Broader caller observers, native/graph organizations, equality integration and coherent complete architectures remain required experimental work.

Generated code also carries responsibilities absent from a prepared data plan: emission, a compiler toolchain, artifact identity/lifetime and source correspondence. Those costs must purchase a measured benefit in the eventual complete architecture. The present result supports neither rejecting compilation generally nor requiring a language restriction.

## Evidence and reproduction

The [initial registration](../registrations/S05-call-amortization.md) and [data-plan amendment](../registrations/S05-call-amortization-planned.md) specify hypotheses, controls, repetitions, selection rules and resource bounds before comparative runs. Primary timing uses the ordinary allocator with counters disabled. Requested heap allocation is measured separately; it is not RSS.

The amended campaign contains 2,377,728 matrix query sessions, plus 192 artifact-preflight queries, across 723 terminal processes: 48 emissions, 48 compilations, 48 artifact preflights, three clock checks, 144 allocation runs, 72 ordinary qualification runs and 360 timing runs. The complete runtime clock floor is 3,000 ns. Frozen source/library hashes, all process commands, randomized schedules, full phase sequences, complete output counts, ownership continuity and final restoration are audited. The 404 interrupted-campaign receipts are preserved and checked independently.

A post-run provenance check repairs an omitted emitter-executable hash. Both the retained emitter and an independent rebuild from archived source reproduce all eight program forms byte-for-byte, covering all 48 archived compiler inputs. Their identities, build command and sixteen emission receipts are recorded in [emitter correspondence](s05-call-amortization-planned/emitter-correspondence.json). The producer now checks emitter identity before and after compilation. This post-run evidence is separate from the original pre-run freeze.

Run `python3 research/chr-reuse/experiments/call_amortization_analysis.py audit` to reproduce the complete evidence audit and attribution. [Final comparisons](s05-call-amortization-planned/analysis.json), [cost attribution](s05-call-amortization-planned/diagnosis.json), [selection](s05-call-amortization-planned/selection.json), [receipt hashes](s05-call-amortization-planned/receipts.json) and [interruption record](s05-call-amortization/interruption.json) retain the detailed evidence.

T075 remains active. This is the third package since the last full portfolio review. The research goal remains active.
