# More reuse can repay recognition, but elimination is a stronger control

The pilot finds a plausible timing crossover when many alternatives share the same future computation. Distinct futures remain expensive for reuse tables. The existing compiler can also contract this source's recursion, so confirmation must include that control before drawing a cost conclusion.

**This is exploratory evidence.** Each timing cell has one ordinary-allocator sample. The run establishes neither a reliable speed ranking nor a general choice between reuse, graph execution and compilation. The [registered confirmation](../registrations/S05-reuse-opportunity-confirmation.md) is the next bounded comparison; the broader [experimental sequence](../sequence.md) remains required.

## What this experiment separates

**The number of reusable alternatives now varies independently from computation and data size.** Earlier continuation trials used four alternatives. Here one, four or 16 alternatives perform zero or 32 countdown steps while carrying a closed payload of depth zero or 32. Two changed queries reuse prepared rules. Token consumption is independently absent or present.

**Five families give reuse different opportunities.** Exact futures share identities; renamed futures require consistent variable renaming. History families differ in whether temporary resources are consumed before or after common work. Distinct futures carry different ground tags and should not merge. Temporary work varies only from zero to three calls per alternative, preventing history size from automatically growing with branch count.

**Twelve controls include different ways of avoiding work.** Direct execution, exact/renamed/live-history tables and their compact-key variants are compared with scanning, specialization, dependency-aware graphs, derivation templates and checked exact-source elimination. The last directly constructs the answer after checking the complete source and query shape. That checking and construction are charged; it is a hand-derived control, not a general compiler.

## What actually happened

**All 4,344 registered processes succeeded.** The 120 configurations produce 1,440 engine/source cells: two allocation runs per cell, one ordinary timing run, and 24 cancellation runs. All 1,440 allocation replays agree exactly, and query/prepared requested-live allocations return to their starting values. The [independent audit](s05-reuse-opportunity-sizing/audit.json) checks receipts, parameters, phases, hashes and summary arithmetic against the [prospective registration](../registrations/S05-reuse-opportunity-sizing.md).

**The intended reuse occurs on the actual source.** Separate counter-enabled diagnostics at depth 32, width 16, payload 32 and with token consumption give the following single-query counts. A transition here is one continuation-machine operation; it is not comparable numerically to another engine's rule or service step.

| Future relationship | Direct transitions executed | Compact renamed key | Compact key ignoring dead history |
|---|---:|---:|---:|
| Exact identities | 1,139 | 78 | 78 |
| Renamed variables | 1,139 | 78 | 78 |
| Temporary resources consumed late | 1,227 | 311 | 296 |
| Temporary resources consumed early | 1,227 | 311 | 98 |
| Distinct ground tags | 1,139 | 1,139 | 1,139 |

**Compact keys preserve the owned keys' work decisions.** All 180 owned/compact diagnostic pairs agree on logical steps, executed steps and hits across 60 configurations. This supports interpreting allocation differences as representation costs without changing which futures are reused. It does not establish call-level or more general relevance projection.

## The potential crossover, with its contrary cases

**More alternatives can change the balance between lookup cost and avoided work.** These ordinary primary times include preparation, both changed queries, complete observation and disposal. They exclude source/input construction, which is separately available in the [full summary](s05-reuse-opportunity-sizing/summary.json), and native compilation. All rows use depth 32, payload 32 and token consumption. Values are single samples in milliseconds, not confirmed gains.

| Family / alternatives | Direct | Compact live-history key | Specialized scan | Exact-source elimination |
|---|---:|---:|---:|---:|
| Renamed / 1 | 0.130 | 0.456 | 0.135 | 0.034 |
| Renamed / 16 | 0.803 | 0.506 | 1.125 | 0.037 |
| Early history / 1 | 0.112 | 0.499 | 0.138 | 0.019 |
| Early history / 16 | 1.001 | 0.636 | 1.180 | 0.031 |
| Distinct / 1 | 0.119 | 0.394 | 0.142 | 0.018 |
| Distinct / 16 | 0.825 | 6.112 | 1.047 | 0.031 |

**Lower allocation traffic can coexist with greater retained memory.** On renamed/16, direct execution requests 1.64 MB during primary phases, versus 0.80 MB for compact live-history reuse. Peak requested growth moves in the opposite direction: 77 KB versus 189 KB. On distinct/16, reuse requests 10.36 MB versus direct's 1.63 MB, with peaks of 2.37 MB versus 77 KB. These are requested heap allocations, not RSS; sustained lifetimes remain a separate required investigation.

**The carried payload exposes avoidable interpretation, rather than inherently necessary computation.** It enlarges keys and state but does not appear in the answer. The exact-source control can bypass the countdown entirely and is substantially faster in these representative samples. This is evidence to investigate available elimination, not proof that a general analyzer can derive the exact hand-written result for arbitrary programs.

## The stronger compiler control is now verified

**Existing source-derived recursive contraction accepts this four-argument recurrence.** No new contraction algorithm was needed. After the pilot, the runner gained a mode using inferred specialization followed by checked contraction of `work/4`. All 240 independent source configurations pass across 13 modes, including complete raw answers, multiplicity, resources and cancellation.

**Contraction actually skips recursive execution.** In all 60 separate work configurations it preserves the specialized control's source-equivalent application count and contracts 31 steps per alternative at depth 32. At width 16 that is 496 contracted steps, with 512 inspection checks. Inspection and remaining ordinary execution still cost time. The [540-row work receipt](s05-reuse-opportunity-sizing/work.jsonl) contains both this comparison and the reuse counts above.

**Contraction has no timing result in this pilot.** The frozen 12-mode runner and binaries remain the authority for its measurements. Current `Cargo.toml` and the lifecycle runner additionally enable the thirteenth mode; the work diagnostic and audit are post-pilot additions. The audit records these differences explicitly. Counter-free source tests and scoped strict Clippy pass; a diagnostic guard lint repair has an exact 540-row work replay.

## What comes next, and why

**Confirm the crossover against contraction before refining reuse further.** The [next registration](../registrations/S05-reuse-opportunity-confirmation.md) includes same-build specialization/contraction and feature-off controls, owned/compact keys, graph execution and exact-source elimination. It keeps no-choice, low-work, large-data and distinct-future cases. Seven shuffled paired timing blocks will distinguish practical differences from the noise in these single samples.

**This package takes priority over sustained-consumer work because it can reverse the immediate reuse interpretation at low implementation cost.** Both contraction and the candidate already pass the source gate; the missing task is a bounded comparison with allocation and feature controls. S08 sustained publication and reclamation is the strongest ready distinct alternative and must be reconsidered immediately after confirmation. Another key optimization is not the default follow-up.

**The broader questions remain required regardless of the timing outcome.** Call-level reuse needs a sound argument for transporting bindings, fresh identities and resource effects into a different caller. Broader relevance, eviction, changing-query cache lifetime, noncontractible useful work, source restrictions and complete architecture comparisons are not answered by this whole-state countdown experiment. T075 and the research goal remain active.
