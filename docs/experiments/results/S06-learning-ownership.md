# Learning trades saved execution for setup and retained heap

The allocation runner preserves complete answers through engine disposal and releases all task-owned heap after consumer cleanup. Eager subtraction requests more bytes than recomputation in every tested configuration. Later covered-state checks sometimes reduce allocation, but usually add it. These results qualify accounting and identify a setup cost to investigate; they do not rank execution speed.

## What the experiment charges

One process builds source rules, prepares the finite solver and the full caller once, then runs a seed and changing queries through recomputation or one of the two learning policies. All answers remain consumer-owned until after learner and both prepared engines are disposed. Independent scalar and compiled observations validate those retained answers outside measured intervals. The reference interpreter remains unchanged.

The allocator meter charges requested allocations and live requested bytes. It records source construction, both preparations, learner construction, query construction/setup, finite execution with session disposal, caller transport/execution/observation with disposal, input cleanup, learner and preparation cleanup, consumer cleanup and source cleanup. Coupled phases are reported jointly. No execution/session-disposal separation, first-publication time or compilation cost is claimed.

The command-line arguments, diagnostic record buffer and reserved outer consumer vector are outside the measured baseline. Inner answer storage is charged. Validation and final JSON formatting are outside the recorded allocation intervals. Validation must restore its own live-byte baseline before cleanup continues. Phase boundaries are contiguous in live-byte accounting; the maximum recorded phase peak gives the measured ownership-window peak without adding peaks together. This is not process RSS or complete process heap accounting.

## Fixed comparison and checks

The registration crosses four accepted pair relations (none, diagonal, off-diagonal, all), duplicate weights one/two, capacities zero/one/four and follow-up counts one/four/sixteen for all three policies. A seed over a/b is followed by alternating a/b/c and a queries with renamed variables. This gives 216 configurations and 432 independent processes, with a 60-second and 1-GiB address-space limit per process.

Every pair has identical phase readings. Every complete raw outcome agrees with the independent scalar and compiled controls, and analytical enumeration checks total multiplicity. Each run returns to its pre-run live requested-byte baseline. The meter self-check and strict scoped Clippy pass. Prepared source and caller are reused across changed queries; this is not recompilation per query.

The all-failing sixteen-follow-up case retains three distinct failed regions with capacity four; capacity one necessarily evicts regions while preserving outputs. This checks bounded entry storage, not a byte-bounded cache or sustained lifetime. The ownership runner does not yet measure cancellation; the preceding semantic gate checks cancellation validity, and cancellation allocation remains required.

## Allocation findings

Across the 72 configurations for each learning policy, eager subtraction requests more bytes than recomputation in all 72. Covered-state checks request fewer bytes in six and more in 66. These counts describe this fixed grid, not frequencies or workload weights for the language.

The table shows requested bytes across all recorded phases for weight one, capacity four and sixteen follow-ups (seventeen queries including the seed).

| Accepted relation | Recompute | Eager subtraction | Covered-state checks |
|---|---:|---:|---:|
| No pairs | 123,501 | 155,868 | 117,589 |
| Diagonal | 727,539 | 743,740 | 743,740 |
| Off-diagonal | 1,064,374 | 1,073,837 | 1,060,019 |
| All pairs | 1,651,764 | 1,667,965 | 1,667,965 |

Setup can outweigh the execution work avoided. In the all-failing row, recomputation requests 42,942 bytes during query setup and 25,279 during finite execution. Eager subtraction changes those to 95,935 and 4,653 bytes. Later checks use 59,143 and 3,166 bytes. Both learning policies release 2,619 live bytes when the learner is disposed, including its region container. Covered checking's lower allocation traffic in this case still has a slightly higher peak above the reserved baseline: 24,654 versus 24,286 bytes for recomputation.

The diagonal and all-pair cases learn no regions but still pay to construct candidate keys and supports. This is an implementation overhead requiring attention when drawing a learning conclusion. Capacity-zero configurations deliberately expose that overhead; they do not represent ordinary recomputation.

## Architectural consequence and next investigation

The preceding source-step result survives full answer validation, but it does not predict allocation by itself. Query recognition and region subtraction can consume the savings. Preserve both policies as competing implementations and investigate whether a sound containment fast path can avoid constructing subtraction fragments for wholly covered queries. Attribute key construction separately where that could change the comparison. Avoid inferring that allocation count predicts time.

The next package should pair any consequential setup correction with these frozen cases and the common-prefix depth witness. Then qualify ordinary-allocator timing with learning diagnostic counters disabled, and register reuse/depth/capacity timing contrasts before running them. Include canceled sessions and complete disposal accounting. The stronger distinct alternative remains T078's mixed-source coherent-path pilot; reconsider it at that paired setup/measurement boundary, without extending learning tuning merely to improve a component score.

General conflict extraction, direct consuming-resource derivations, native compilation, richer source support and sustained lifetime remain separate unanswered questions. The goal remains active.

## Evidence

- [Prospective registration](../registrations/S06-learning-ownership.md)
- [Raw paired processes](s06-learning-ownership/runs.jsonl), [audit and hashes](s06-learning-ownership/audit.json)
- [Meter self-check](s06-learning-ownership/meter-check.log), [build](s06-learning-ownership/build.log), [Clippy](s06-learning-ownership/clippy.log), [toolchain](s06-learning-ownership/toolchain.log)
- [Runner](../../../research/chr-direct-conditional/examples/learning_ownership.rs), [process gate](../../../research/chr-hvm/learned_regions/ownership_gate.py)
- [Covered-state semantic and work comparison](S06-covered-learning.md)
