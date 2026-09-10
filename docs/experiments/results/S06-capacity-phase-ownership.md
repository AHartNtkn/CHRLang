# Resource-phase composition trades allocation traffic for lower live memory

Direct resource solving remains useful inside an ordinary caller, but composition changes its cost tradeoff. Selective and failing queries request fewer heap bytes; output-heavy complete queries request more than specialized whole-source scanning. Peak live heap is lower on those output-heavy queries. Runtime has not been compared.

All 300 configurations passed one ordinary-allocator replay and two allocation replays: **900 processes, 300 exact diagnostic pairs, complete independent outcomes and full heap restoration**. Both consumer head orders and cancellation followed by preparation reuse passed. This qualifies the composed measurement endpoint; it does not establish general resource lowering or complete architectural superiority.

## What was compared

The candidate solves the initial resource phase, constructs ordered continuations, transports private bindings, and runs an ordinary specialized scanned caller. Four controls run the whole original source: generic scanned/indexed and inferred-specialized scanned/indexed execution. Both candidate preparations count, including the full caller source and binding replay rule.

The matrix varies two/four requests, empty/tight/spare supplies, one/four changing queries, both consumer head orders and two callers. One caller consumes the first completed resource; the other creates later private work and binds an unknown shared with outside state. Supplemental cases exercise zero requests, weighted duplicate derivations and first-answer cancellation. Reused queries alternate independent and aliased private variables and change their identities.

All delivered answers remain owned by the consumer through prepared disposal. Independent scalar and compiled source execution validate complete answers outside measurement. The parent separately enumerates assignments, capacities, alias restrictions and raw multiplicity to predict counts. On cancellation, the first answer must belong to the complete expected multiset; these trials do not require identical first-answer order.

## The complete allocation comparison

The table reports **requested heap bytes across all measured phases**, including preparation, changing-query construction, transport, observation and disposal. It is not RSS or execution time. Each row uses need-first consumption and the caller that consumes the first completed resource.

| Query batch | Delivered answers per query | Phase + caller | Specialized whole-source scan | Phase/control |
|---|---|---:|---:|---:|
| Zero requests, one query | 1 | 39,149 | 30,805 | 1.271 |
| Two requests, tight supply, one query | 2 | 70,963 | 65,674 | 1.081 |
| Four requests, tight supply, four queries | 6, 0, 6, 0 | 355,451 | 531,097 | 0.669 |
| Four requests, no supply, four queries | 0, 0, 0, 0 | 57,123 | 395,225 | 0.145 |
| Four requests, spare supply, weight two, four complete queries | 256, 32, 256, 32 | 11,149,903 | 10,152,117 | 1.098 |

Across all 60 matched scenarios, the candidate requests fewer bytes than specialized scanning in 32 and more in 28. Against generic scanning the counts are also 32/28; against generic indexing they are 49/11, and against specialized indexing 47/13. These are descriptions of this matrix, not workload weights or a vote for an architecture.

Both consumer head orders preserve the direction of the representative comparisons. For output-heavy complete queries, candidate/specialized-scan requested-byte ratios range from 1.092 to 1.117 across the two callers and head orders. Thus the adverse result is present with later private work as well as the simple consuming caller.

## Why the complete output-heavy case allocates more

The phase and the resumed caller both allocate substantial representations. For four spare-supply, weight-two queries with later private work, the candidate requests **19,360,059 bytes**, versus **17,725,793** for specialized whole-source scanning. The following disjoint phase accounting identifies where the difference appears.

| Measured owner/work | Phase + caller | Specialized whole-source scan |
|---|---:|---:|
| Source construction | 6,890 | 6,890 |
| Preparation | 23,489 | 17,907 |
| Query construction | 12,288 | 12,288 |
| Phase solving and ordered continuation construction | 1,538,704 | — |
| Transport, caller setup, execution, observation and run disposal | 17,778,688 | 17,688,708 |
| Remaining disposal allocation | 0 | 0 |

The added phase construction accounts for 1,538,704 of the 1,634,266-byte difference. The caller interval requests slightly more than whole-source execution, so avoided resource execution does not repay the added representation traffic here. This is accounting attribution, not a claim that phase construction could simply be omitted while preserving behavior. Caller setup, binding transport and execution are joint intervals; their individual costs are not isolated.

The implementation eagerly materializes raw continuations and starts an ordinary caller for each. A compact multiplicity-bearing continuation or a reusable caller setup could change this result, but neither has been measured or qualified by this package. Such a change must preserve ordered consumption, private binding transport, raw answers and cancellation. Allocation traffic alone does not yet justify choosing that implementation work over a distinct architecture investigation.

## Lower peak memory is a separate benefit

For the same output-heavy later-work case, peak task-owned live heap is **2,179,083 bytes** for the candidate and **4,341,436** for specialized scanning. Both consumers retain exactly **2,148,480 bytes** of answers before release. The candidate's peak is therefore close to the retained-answer requirement even though its cumulative allocation traffic is higher.

Peak live heap is lower than each whole-source control in 56 of 60 scenarios. The four zero-request scenarios have higher candidate peaks: preparation and phase overhead dominate. This does not establish sustainable stream behavior; it measures finite query batches with answers retained to the end.

First-answer cancellation also restores ownership and permits the next three queries to complete on the same preparation. In the output-heavy later-work case it delivers 1, 64, 512 and 64 answers, requests 11,441,555 bytes versus specialized scanning's 15,907,346, and reaches a lower peak. The candidate still constructs all initial phase continuations before the first caller runs; unused continuations are dropped inside the measured caller interval. No claim of interruption within phase solving or lower first-answer latency follows.

## What this changes, and the next investigation

The closed capacity solver's timing result cannot be assigned to its composed use. The current composition has a real additional representation cost, alongside favorable selective allocation and live-memory regimes. Keep both the direct phase and whole-source execution as credible alternatives for a bounded runtime comparison.

**Select a prospective counter-free cost pilot next.** It can determine whether this extra allocation is a consequential runtime loss or whether eliminating source execution still repays composition. The already qualified ordinary-allocator runner makes that comparison inexpensive. Include zero/small overhead, selective success, complete output-heavy answers and cancellation; retain both head orders and callers. Freeze the matrix, paired repetitions and practical interpretation criteria before runs. Do not rank the diagnostic clocks from this package.

Adaptive restoration/reunion is the strongest ready distinct alternative: it can change how much shared state explicit search needs, but still requires an adaptive policy and progress qualification. Local resource claims need a conflict/cancellation protocol; richer theories need separate denotations; sustained lifetime can begin with existing candidates. Their [concrete entry experiments](../next-cycle.md#concrete-entry-experiments-for-the-next-breadth-review) remain required. Choosing this cost pilot is a judgment about the cost of resolving an observed tradeoff, not adverse evidence for those directions.

This is package two after the matched-reuse breadth review. Reconsider the alternatives at the cost pilot or an obstruction, and conduct the required full breadth review no later than package four. A credible runtime loss prompts investigation of a consequential avoidable cost; a gain must face adverse and sustained-lifetime checks. Overlap needs further precision only if it can change the bounded architectural decision. The research goal remains active.

## Evidence and reproducibility

- [Prospective registration](../registrations/S06-capacity-phase-ownership.md): 300 configurations; one primary and two diagnostic runs; 60-second wall/CPU and 1 GiB address-space limits per process; 600-second launcher bound. Caller service permits 200,000 ticks **per continuation**, not an equal aggregate work budget. No process reached a bound.
- [Runner](../../../research/chr-direct-conditional/examples/capacity_phase_lifecycle.rs) and [launcher/analyzer](../../../research/chr-hvm/resource_capacity/phase_ownership.py). The primary build disables engine counters and uses the ordinary allocator; diagnostic builds use the requested-allocation meter separately.
- [Pre-run manifest](s06-capacity-phase-ownership/manifest.json), [raw process receipts](s06-capacity-phase-ownership/runs.jsonl), and [reproducible audit](s06-capacity-phase-ownership/audit.json). The manifest freezes the exact commands, matrix, binaries and relevant source files. All hashes remained unchanged after runs and build verification.
- [Meter self-check](s06-capacity-phase-ownership/meter-check.log), [default semantic tests](s06-capacity-phase-ownership/test-default.log), [metrics-off semantic tests](s06-capacity-phase-ownership/test-metrics-off.log): all twelve tests pass in each build, including the ordered source gate. [Primary Clippy](s06-capacity-phase-ownership/clippy-primary.log) and [diagnostic Clippy](s06-capacity-phase-ownership/clippy-diagnostic.log) pass.

All phase boundaries have continuous live-byte accounting; independent validation restores its own heap baseline; final source/consumer disposal restores the task baseline. Preallocated outer bookkeeping is excluded consistently. Compilation, RSS, sustained consumers, general source eligibility and whole-architecture complexity remain outside this package's measured claim.
