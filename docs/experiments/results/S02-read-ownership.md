# Recorded-read validation reduces traffic but preserves cache peaks

Recorded-read validation lowers requested allocation in half the tested scenarios compared with rebuilding relevant-state keys. It preserves the same peak live bytes. In one substantive changed-context case it uses less traffic than recomputation, but retains substantially more memory and remains more expensive than Scan and source lowering.

**Next investigate same-input near misses and attribute validation versus key/replay costs.** The saved key construction is useful enough to justify that test. These are allocation findings; comparative runtime and sustainable cache retention remain unmeasured.

## Complete-session comparison

The [registered comparison](../registrations/S02-read-ownership.md) completes **648 configurations, 1,296 allocation processes and 576 ordinary semantic replays**. All 648 allocation pairs reproduce exactly. Every measured phase preserves live-byte continuity and final ownership restores. All complete answers and cancelled prefixes pass independent checks; retained answers remain valid after engine and prepared-owner disposal.

Nine modes share four source families: a single request, shared work, distinct work and changed unrelated contexts. Depths are 0/16, preparation serves one/four changing queries, resource availability varies, and consumers release immediately or retain all answers. Another 72 configurations cancel alternating queries after one service call and then reuse preparation. Each policy has 72 matched scenarios including cancellation.

The new candidate checks saved transitive reads before constructing a key. Its ordered-map variant borrows descriptions; its persistent-map variant can clone descriptions through the existing map API. Both retain the same cache entries and caller-local replay obligations as the rebuilt-key controls. Deduction reuse occurs between contexts within a query; reused rule preparation does not make this a cross-query deduction cache.

## Traffic savings do not remove retained-state costs

Counts below compare each new policy with a competent control across all 72 scenarios. “Lower” and “higher” refer to exact requested allocations in these deterministic runs, not speed or statistical significance.

| Candidate / control | Traffic lower / equal / higher | Peak lower / equal / higher |
|---|---:|---:|
| Ordered validation / ordered rebuilt key | 36 / 36 / 0 | 0 / 72 / 0 |
| Persistent validation / persistent rebuilt key | 36 / 36 / 0 | 0 / 72 / 0 |
| Ordered validation / recomputation | 28 / 0 / 44 | 0 / 0 / 72 |
| Persistent validation / recomputation | 8 / 0 / 64 | 16 / 0 / 56 |
| Ordered validation / exact-state cache | 18 / 0 / 54 | 44 / 0 / 28 |
| Persistent validation / persistent exact-state cache | 10 / 0 / 62 | 18 / 0 / 54 |
| Ordered validation / Scan | 26 / 0 / 46 | 32 / 0 / 40 |
| Persistent validation / Scan | 26 / 0 / 46 | 32 / 0 / 40 |
| Either validation policy / source lowering | 0 / 0 / 72 | 0 / 0 / 72 |

**The changed-context witness reverses one traffic comparison with recomputation.** At depth16, four changing queries, a resource token and retained answers, the complete measured session requests:

| Path | Requested bytes | Peak live bytes above baseline |
|---|---:|---:|
| Contextual recomputation | 669,196 | 38,976 |
| Ordered rebuilt relevant key | 1,292,590 | 72,959 |
| Ordered recorded-read validation | 593,968 | 72,959 |
| Persistent rebuilt relevant key | 1,667,574 | 72,308 |
| Persistent recorded-read validation | 968,952 | 72,308 |
| Scan | 265,971 | 52,934 |
| Source lowering | 43,562 | 9,944 |

Each validation variant saves 698,622 requested bytes against its corresponding rebuilt-key control here. The ordered candidate now requests less than recomputation, while the persistent candidate still requests more. Neither reduces its cache peak. This demonstrates why attribution of a correct but costly implementation mattered, while preserving the stronger source-elimination and scanning controls.

## Existing controls still agree

The independent [auditor](../../../research/chr-relational/experiments/audit_read_ownership.py) reconstructs all phase sums, endpoint flags, feature flags, repeated allocations and final release from raw process receipts. It compares the seven old modes with all 504 configurations from the [previous ownership study](S02-relevant-ownership.md).

Raw records differ because the current executable path is four bytes shorter and the runner now emits an explicit null attribution field. Every live allocation baseline shifts by minus four bytes; phase traffic, allocation counts and peak growth are unchanged. After subtracting each session's pre-existing live baseline and treating absent/null attribution alike, **all 504 complete control records agree**, including per-phase allocation fields. No sample is excluded or substituted. This normalization does not change any candidate comparison.

The extended runner first fails its existing all-path source/interruption test on the new mode. After wiring the qualified candidate into the runner, four relevant-deduction tests and two lifecycle tests pass. Strict scoped Clippy passes. The meter self-check succeeds before the matrix. Every child process stays within the registered 60-second wall/CPU and 1-GiB limits; no cutoff occurs.

## Measurement scope and next decision

Measured ownership includes source construction, preparation, input creation, setup, execution through owned observations, answer handling and producer/consumer disposal. Oracle fixtures, command-line storage and preallocated report/retention containers are outside the measured baseline. Validation happens outside measured intervals. Those exclusions are shared and explicit; this is not process RSS or total process allocation. Meter elapsed times and the single ordinary replays establish no runtime ordering.

**Many same-input variants remain the consequential adverse boundary.** The candidate ranges over existing entries for the input pair and validates them until one matches. A workload with many incompatible saved contexts may pay repeated failed probes before constructing a new key. The present four families do not establish that cost. Qualify this case independently, then measure lookup, key construction, replay and retained entries against rebuilt keys and recomputation.

Preserve both map representations and low-reuse controls. If the new recognition cost dominates, identify whether a sound cheaper lookup could change the result before treating it as intrinsic. If a scoped benefit survives, register counter-free paired lifecycle timing. Demand capability, compact solving, broader restoration, distinct integrated organizations and whole-architecture comparisons remain required. T072 remains active; this result selects no architecture.

## Evidence

[Registration](../registrations/S02-read-ownership.md), [freeze](s02-read-ownership/freeze.json), [raw phase results](s02-read-ownership/results.json), [independent audit and all contrasts](s02-read-ownership/review-audit.json), [driver completion](s02-read-ownership/driver.log), [meter self-check](s02-read-ownership/meter-check.log), [source/interruption tests](s02-read-ownership/gate.log), [initial failure](s02-read-ownership/red.log) and [Clippy](s02-read-ownership/clippy.log) preserve the experiment.

Run `python research/chr-relational/experiments/audit_read_ownership.py` to reconstruct the evidence. The [driver](../../../research/chr-relational/experiments/read_ownership.py) requires the recorded isolated builds and refuses to overwrite its existing run directory.
