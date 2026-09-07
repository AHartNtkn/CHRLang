# E16 regional repeated cost comparison

Status: proposed and queued; no comparative runs. Architectural assumption audit
and first distinguishing boundary contrast take priority. The execution harness and independent auditor await preparation and freeze.

Confirmatory registration following the [158-observation exploratory pilot](../results/E16-regions-pilot.md).
Keep its ten workloads, 79 cells, exact semantic endpoints, Q/K controls,
System/meter separation, lifecycle intervals and 30-second/1-GiB child bounds.
No engine or fixture revision intervenes. The primary elapsed metric remains
wall through search release including diagnostics; output release is separate.

## Hypotheses and controls

1. On balanced two-work, Q64/K4 two-worker cold and first-answer ranges may lie
   below matched one-worker and Inline ranges. Test all three comparisons,
   including original Baseline Q1 for context; do not substitute it for Inline.
2. Q64 versus Q1 reduces service-message overhead on one-work and two-work.
   Compare source work, issued requests, elapsed phases and total cold wall.
   The quantum also changes owner observation scheduling; counts distinguish
   this from a pure channel-cost claim.
3. One-region, imbalance, product-heavy, duplicate, prefix, refutation and
   mixed application controls may erase the balanced benefit. Retain all cases
   and report contrary evidence; no average score across unlike workloads.
4. K4 versus K1 may help two-work at Q8 at a memory cost. Compare each mode
   separately, including Inline, and record admission/actual work.
5. Prefix/refutation actual-work surplus can vary even when logically accepted
   work is fixed. Treat its variation as a measurement, not a correctness failure.

This tests a retained-region interface and cold lifecycle on one host. It does not
compare the equation and region engines as if they performed identical work, and
does not establish warm-session costs, general fairness or production semantics.

## Runs and interpretation

Run two fresh-process System warmup blocks, seven measured System blocks, then
three separate meter blocks. Each block contains every one of the79cells exactly
once. Shuffle each fresh cell list with successive draws from Python
`random.Random(160917)`, freezing the full order before any child. Thus948children:
158warmups,553measured timings,237memory observations. Warmups are excluded from
estimates. This is machine warmup, not pool reuse; every child constructs its pool.

Report min/median/max, retaining every successful observation with no trimming.
For directional language, require one observed range strictly below the other;
otherwise label the comparison overlapping. These are descriptive repeated
measurements, not confidence intervals or a universal performance claim. Report
diagnostic gap magnitudes, construction, search and release phases, allocation
peaks/calls/bytes, process RSS, accepted and physical source work and observation
costs. Do not use metered timings to rank performance.

Audit all cell/repetition identities and exact seeded order independently; check
all source/binary/registration/runner/auditor hashes. Semantic preflight and
runtime full-output checks remain in force. Match accepted source and global
observer counters to same-Q Inline, and across time/memory repetitions. Compare
actual-work counts only where logically justified; early-stop cancellation is
explicitly timing-sensitive. Shutdown must preserve logical state and account
for every accepted/unaccepted request.

No other builds, tests, profiles or experiments run during measurement. Preserve
failed rows and stop the batch for diagnosis rather than skipping cells. If
ranges overlap, assess larger work/region/worker sweeps or repeated sessions by
expected information gain; overlap is not closure. If Q64 gives a reliable local
benefit, investigate its limits and application occurrence before recommending
an architecture. Warm pools, certificates, connected applications and temporary
reunion remain independent open investigations.
