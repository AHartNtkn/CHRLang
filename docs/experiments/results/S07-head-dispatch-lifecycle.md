# Unary inference saves discovery costs without requiring a declaration

Local unary inference reduces completed-query allocation in 76 of 84 measured configurations, with eight small increases. Valid optional and required declarations have exactly the same allocation readings as inference. This supports inference as a usable compiler fact; it supplies no allocation-efficiency argument for making the declaration mandatory.

The [registered pilot](../registrations/S07-head-dispatch-lifecycle.md) completes 3,612 processes and 516 exact allocation pairs across seven families. General feature-on/off controls match every memory reading in 84 pairs. The 96 inferred-versus-declared/required pairs also match exactly. Preparation, changed queries, retained answers and all disposal checks pass.

## Completed work and cancellation are separate

The table gives requested heap traffic in decimal MB for n=3, four changed queries sharing preparation, and per-query answer release. It includes preparation, input, setup, complete delivery and disposal. It excludes the separately reported cancellation probes; summing three probes into an application workload would assign them an arbitrary frequency. For wide sources, n=3 means 32 unary predicates; history and chain use 48 occurrences or successor steps.

| Source | General conditional | Inferred conditional | Scan | Contextual resumable |
|---|---:|---:|---:|---:|
| Unary history | 9.575 | 9.466 | 0.567 | 0.383 |
| Unary chain | 1.558 | 1.495 | 0.195 | 0.455 |
| Wide unary rules | 4.442 | 4.216 | 0.631 | 0.335 |
| Unary choices | 0.417 | 0.409 | 0.226 | 0.219 |
| Shared-predicate mixed heads | 0.530 | 0.523 | 0.050 | 0.062 |
| Broad independent choices | 3.413 | 3.394 | 0.761 | 1.753 |
| Broad delayed binding | 5.126 | 5.096 | 0.980 | 2.616 |

**The compiler benefit survives complete costs, but is not the dominant remaining cost.** Removing unary discovery machinery saves about 109 kB in history and 226 kB in wide rules here. The conditional execution path still requests substantially more traffic than the complete Scan and contextual controls in these cells. The gate does not attribute that remaining gap to a necessary architectural responsibility.

**Preparation can offset savings.** History preparation grows from 2,098 to 2,827 bytes, while first-query setup falls from 42,922 to 37,426 and first-delivery traffic from 2,341,075 to 2,319,203. Wide preparation grows from 47,924 to 52,354 bytes. In the shared-predicate mixed source, setup itself rises from 8,232 to 8,952 bytes because general discovery remains necessary alongside unary dispatch.

**Empty mixed sources expose the contrary case.** At n=0/reuse=1, completed traffic rises from 10,100 to 10,119 bytes for shared-predicate mixed heads, and from 19,003 to 19,076 for broad independent sources. Peak growth also rises slightly. The 76/8 count describes the registered cells; it is not a workload weighting or a probability of benefit.

## What declarations change

For the four unary families, inferred preparation, optional checked single-head declarations and required declarations produce byte-identical phase readings in every paired configuration. The source check feeds the same prepared dispatch representation; no declaration policy is retained in execution.

Required admission still rejects a missing declaration. Checked admission rejects a mixed source that contradicts a single-head declaration. Those distinctions remain established by the [source/beneficiary tests](S07-head-dispatch-gate.md), and are not counted as successful cheap executions in this matrix. Mixed undeclared sources continue to receive local inference.

This settles a bounded question: the measured allocation benefit does not depend on requiring declarations. It does not decide whether an enforced module/interface promise is valuable, whether checks remain cheap under larger linking regimes, or whether another property can eliminate more work. Head count still does not imply non-overlap, termination, groundness or a unique matching environment.

## Retention and interrupted work

With four retained history answers, general conditional peak growth is 88,382 bytes and inferred growth 78,357. Both retain exactly 46,220 answer bytes. Scan retains 40,268 answer bytes and peaks at 73,350; contextual resumable retains the same 40,268 and peaks at 75,553. Equivalent answers therefore retain different physical allocation layouts across organizations.

The three cancellation probes stop after 1, 64 and 256 service calls. Every search/input disposal restores preparation plus held answers; preparation disposal leaves only those answers; final release restores the initial measured baseline. Discovery-state snapshots replay across processes. These are real interrupted executions, but equal service-call counts need not represent equal logical progress or completed source work.

For wide rules, cancellation traffic is 158,689 bytes in general conditional execution and 169,337 under inference, despite lower completed-query traffic. This is not evidence that cleanup alone became more expensive: the probe includes setup and work before interruption. The summary keeps completed and cancellation traffic separate. Sustained streams, bounded windows and cancellation at comparable logical points remain broader lifetime questions.

## Timing and validation

Five ordinary-allocator samples per cell remain exploratory. Completed wide-query median is 3.343 ms (range 3.255–5.150) for general conditional execution and 2.571 ms (2.498–2.779) for inference. History medians are 5.737 ms (5.371–7.674) and 6.506 ms (5.452–10.569), with overlapping ranges. Lower allocation alone does not establish a speed gain, and no formal small-win classification follows.

The first attempt stopped on a fixture routing error: the unary history rules were paired with the broad history query generator. The error occurred before timing, after seven completed allocation processes. Explicit routing to only the intended broad families corrected it; the full registered matrix then ran successfully. That incomplete attempt is retained as a diagnostic receipt and supplies no comparative evidence.

[Results and audits](s07-head-dispatch-lifecycle/) contain all 3,612 successful process records, exact orders, source/binary hashes, summaries and declaration/feature attributions. Independent raw expectations and the scalar validate every query outside intervals; warm and measured candidate answers preserve residuals, multiplicity and aliases. Held answers are rechecked after preparation is dropped. Both runner builds pass strict Clippy, explicit formatting and Python syntax checks.

Primary timing has ordinary allocation and engine/kernel/work counters disabled. Metering measures requested heap traffic/live memory, not RSS. Preparation includes owned source cloning, inference/checking and maps; original source construction, process startup, native compilation and validation are excluded. Execution and observation remain coupled in delivery. No compilation-inclusive superiority is claimed.

## Next decision

Proceed to the distinct [non-overlap/resource-conflict investigation](S07-overlap-entry.md). The simpler unary property now has executable beneficiaries, contrary cases and complete pilot costs; another unary refinement is less valuable than determining whether a stronger property removes a different responsibility. Broader language/interface, output-lifetime and coherent/held-out architecture obligations remain required. T079 and the goal remain active.
