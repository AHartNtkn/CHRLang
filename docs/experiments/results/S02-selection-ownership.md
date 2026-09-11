# Stable CHR selection has a large measured ownership cost

The stable CHR protocol preserves the common sources' answers, but requests substantially more heap traffic and reaches higher live-heap peaks than every tested local and conventional control. For six flat requests over four changing queries, CHR Indexed requests **31.04 MB**, versus **57.45 kB** for specialized ordinary execution. Almost all CHR traffic occurs during execution.

The source trace identifies avoidable repeated eligibility work. Even eliminating every execution allocation leaves this CHR implementation above the best complete control in all 48 full-run CHR scenarios. **Intermediate joins are the next investigation.** This allocation bound supports that priority; it neither rejects other CHR organizations nor establishes a speed ordering.

## Complete ownership, separate work evidence

The [registration](../registrations/S02-selection-ownership.md) compares four sources at sizes2/6, eight modes, immediate/all consumers and complete/alternating-cancel sessions. Each prepares once for four alternating a/b queries. The matrix contains256 configurations,512 allocation processes and256 ordinary semantic replays. Another128 diagnostic processes provide two identical work runs for each of64 complete retained-all configurations: **896 registered processes** total.

The controls are local Endpoint and Filtered, conventional Scan/Indexed/inferred specialized Scan, and the CHR protocol under Scan/Indexed/inferred specialized Scan. They reuse the existing lifecycle harness and engines. Full decoded outputs and residuals agree with independent ordinary scalar semantics. Retained answers remain valid after the prepared engine, source and encoding owners are disposed.

All256 allocation pairs agree exactly after process-baseline normalization. Phase boundaries are continuous, final task-owned requested-live heap restores, all64 work pairs agree, and ordinary replay preserves endpoint metadata. Cancellation occurs after one adapter service call on alternate queries, followed by reuse of preparation; that is an ownership check, not matched source progress across architectures.

## The observed allocation comparison

Requested bytes below include rule creation, preparation, source inputs, encoding, engine setup, execution, owned observation/decoding and disposal. Values cover size6, four complete queries and retained-all outputs. Local Endpoint and Filtered agree in these sources.

| Source | Local | Ordinary Scan | Specialized ordinary Scan | CHR Scan | CHR Indexed | Specialized CHR Scan |
|---|---:|---:|---:|---:|---:|---:|
| Flat ready requests | 70,747 | 66,699 | 57,446 | 45,681,393 | 31,042,925 | 45,629,765 |
| Sparse readiness | 51,307 | 54,163 | 53,614 | 1,662,205 | 1,810,257 | 1,665,297 |
| Equality-enabled chain | 60,911 | 73,343 | 64,090 | 45,592,901 | 15,801,277 | 45,561,753 |
| Competing requests | 69,915 | 55,603 | 48,846 | 16,371,897 | 11,942,529 | 16,356,429 |

The full [audit](s02-selection-ownership/audit.json) includes Indexed ordinary execution and every size, consumer and cancellation combination. Across the48 complete CHR scenarios, every CHR mode has more requested traffic and a higher peak than all five non-CHR controls for its matched source/size/consumer. These are unweighted scenario counts, not workload prevalence or a universal architectural verdict.

For the flat retained-all example, CHR Indexed's peak is133,388 bytes above baseline, compared with9,903 for specialized ordinary execution and11,979 for local execution. Requested traffic is allocation volume; peak excess is live requested heap. Neither is RSS, stack usage or code size.

## Execution dominates, but eliminating it is insufficient

The CHR Indexed flat session allocates30,756,668 bytes during execution. Its remaining286,257 bytes comprise source creation34,967; preparation61,554; input construction11,188; encoding48,608; setup96,368; and observation33,572. Disposal requests no additional bytes in this case but releases the owned state.

Subtracting all execution traffic leaves286,257 bytes versus57,446 for the best complete conventional control. The analogous remainder still exceeds the best complete control in all48 full-run CHR scenarios. This is an optimistic sensitivity bound: it charges no replacement execution machinery. It does not establish that encoding, preparation or observation are intrinsic costs, or that a larger reuse count could never change amortization.

An execution-only allocation repair therefore cannot reverse these complete allocation comparisons. The bound says nothing about elapsed time. Counter-free ordinary runs supply semantic replay here; their clocks are not ranked. Native compilation, artifact lifetime and sustained consumers remain outside this study.

## The source trace exposes repeated selection work

An additional exploratory attribution repeats16 independent scalar-encoded traces over all four sources, both sizes and both query values. Decoded answers are checked again. Their total application counts match every corresponding CHR diagnostic mode exactly.

For one size6 flat query, the CHR rules perform212 applications to service six source requests:

| Source-rule work | Applications |
|---|---:|
| Discover eligible requests | 91 |
| Absorb duplicate eligibility facts | 70 |
| Discard younger eligible candidates | 15 |
| Commit source requests | 6 |
| Represented equality and constructor maintenance | 30 |

Eligibility propagation includes a token head. With k identical remaining tokens and k eligible requests, it generates k² candidate facts in that round. Across six rounds,1²+…+6²=91 discoveries. Only21 distinct request candidates are needed across those rounds;70 duplicates are absorbed. This is an avoidable consequence of the present trigger, not a requirement of stable source identity or CHR semantics.

The chain has132 applications:21 eligibility discoveries,15 duplicate absorptions, six commits and additional equality/ancestry/repair work. Its ancestry rules perform35 transitive applications and20 duplicate absorptions. Different sources therefore stress different parts of the encoding; the flat duplicate explanation cannot stand in for all integration costs.

Host discovery amplifies those source operations. Flat CHR Scan visits118,566 candidates and205,204 cursor steps per query; CHR Indexed visits75,089 candidates and160,579 cursor steps. Ordinary Scan visits12 candidates and19 cursor steps for the same six source applications. Inferred CHR specialization executes six specialized applications but does not eliminate the general discovery work. The work counts explain what happens; they do not attribute a precise fraction of allocation traffic to each source rule.

## A cutoff was diagnosed before the matrix

Initial size6 flat qualification hit the200,000-step limit in CHR Scan. The registration prospectively raised all controls to2,000,000 steps while preserving60-second wall/CPU and1-GiB limits. All32 size6 source/control qualifications then completed with correct answers. The diagnostic flat cursor count exceeds the original bound, consistent with substantial unfinished discovery rather than a source-protocol failure.

The initial cutoff and extended qualification are preserved. All registered matrix processes finish within their bounds. No partial answer or cutoff is treated as a completed cost result.

## Next decision: intermediate joins, with CHR follow-through retained

The preceding [portfolio review](S02-stable-selection-review.md) selected one ownership/work package before more capability. This package now exposes both a large complete allocation loss and a specific avoidable eligibility trigger. Eliminating all execution allocation would still not reverse the measured allocation comparison, so further selection-trigger tuning has lower immediate decision value than a distinct discovery mechanism.

T081 next investigates a genuinely reusable many-to-many intermediate join under sparse consuming updates, with broad invalidation and cheap keyed controls. Reconcile the existing full-prefix and partial-join results first; do not repeat them under a new label. The question is whether a different retained intermediate avoids rediscovery economically, including preparation, invalidation, retained state and disposal.

T072 remains unfinished. Retain token-independent eligibility discovery, alternatives to explicit precedence and eager ancestry, credible merge strategies, larger preparation reuse, primary timing where consequential, fresh requests, general bodies and sustained ownership. Reconsider those when they could change a coherent architecture comparison. Flat relations, local handles and other CHR organizations do not inherit this implementation's allocation disposition.

The measured source fragment uses a unary constructor, two constants and one consuming rule. Its matched ordinary-source priority is a comparison contract, not a new language restriction. The CHR kernel still relies on host discovery and propagation history; this study does not show that every proposed integrated organization retains those services.

## Validation and receipts

The existing147-comparison lifecycle smoke path passes. All complete new matrix answers and retained-answer checks pass, as do meter self-check, strict allocation/work Clippy and independent trace Clippy. The [freeze](s02-selection-ownership/freeze.json) preserves the exact source archive, binaries and configurations; [raw receipts](s02-selection-ownership/) retain builds, qualification failures, commands and all process results. The trace has its own source/binary freeze and two raw repeats.

The extended decoder checks forest ownership, repaired references and hidden constructor cycles before producing full owned outputs. Hidden-cycle validation uses a graph traversal rather than constructing throwaway trees. Its necessary checking, scratch storage and output construction are charged in observation. Independent answer comparison occurs outside phase intervals and can still influence later allocator/cache state; no timing inference is made here.

The reference interpreter and unrelated work are unchanged. The result is bounded ownership and work evidence; the research goal remains active.
