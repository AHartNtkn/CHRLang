# Direct solving changes the selective comparison; unselective work still has a cost

The checked finite solver now allocates less than every measured control on the shown selective and duplicate-answer batches. Specialization remains cheaper in requested allocation on unselective work and on the tiny no-choice query. This is evidence for a conditional compilation opportunity, not a general architecture winner.

The corrected lifecycle pilot completed 1,428 processes. A separate 252-process attribution corrected an avoidable state copy and measured its effect. Both include independent full-answer replay, ordinary caller execution, output materialization and disposal.

## Complete costs across four changing queries

These batches use depths 4, 5, 6 and 7, work 2, payload 4, a consuming token boundary and immediate answer release. Numbers are **millions of requested heap bytes**, including source construction, preparation, query work, caller work and disposal. They are not RSS.

| Source | Finite solver, corrected | Scan | Specialization | Prepared prefix | Conditional ascending | Conditional reverse |
|---|---:|---:|---:|---:|---:|---:|
| Selective, oldest first | 0.795 | 4.017 | 3.304 | 4.282 | 0.961 | 0.827 |
| Selective, newest first | 0.795 | 4.017 | 3.304 | 4.282 | 0.819 | 0.968 |
| All choices accepted | 15.772 | 12.690 | 9.237 | 14.183 | 100.133 | 69.289 |
| Duplicate successful choices | 1.034 | 75.070 | 59.972 | 79.517 | 115.053 | 211.156 |

The corrected finite numbers come from the paired attribution on exactly these configurations. The other controls retain their completed lifecycle results; unchanged Scan allocation calibrations agree across the builds. Timing samples from the two experiments are not combined.

**The selective result changes the architecture evidence.** The qualified compiler controls previously retained the explicit choice tree. Direct source-derived solving now supplies a cheaper allocation path for that favorable conditional example. This does not invalidate other conditional regimes, such as useful shared work outside the finite phase's admission boundary.

**Duplicate derivations are a substantial opportunity here.** The solver keeps their counts, executes an equivalent resumed caller once per solution row, and materializes each required raw answer. The four queries deliver 16, 32, 64 and 128 answers. It does not substitute a distinct-answer set for the source multiset. Independent tests also check later caller choices and fresh aliases within returned answers.

**Unselective work retains a real tradeoff.** Corrected finite solving requests 15.77 MB versus specialization's 9.24 MB, while its peak requested growth is about 0.447 MB versus 1.125 MB. Reverse conditional execution has a still smaller peak, about 0.303 MB, but requests 69.29 MB cumulatively. Allocation traffic, retained memory and elapsed time cannot be collapsed into one ranking without workload assumptions.

**Cold overhead remains visible.** On the oldest-first, zero-choice, one-query case, finite solving requests 50,721 bytes; Scan requests 31,666 and specialization 34,869. Preparing two execution organizations is not free even when there is almost no choice work to eliminate.

## Timing evidence and its limits

The ordinary-allocator pilot uses five repetitions per cell, separately from allocation diagnostics. Its median batch times for the initial finite implementation were about 0.56 ms on selective work, 9.83 ms on unselective work and 0.82 ms on duplicate-success work. Corresponding specialization medians were 2.50–3.58 ms, 8.50 ms and 54.49 ms. These are exploratory pointwise observations, not significance or speed-adoption claims; the full ranges are retained in the raw summaries.

The copy correction has its own paired timings. On unselective work, baseline and corrected medians are 9.08 and 9.17 ms, with overlapping ranges of 7.93–9.84 and 7.55–9.65 ms. The correction establishes lower allocation, not a measured speed improvement.

First raw-answer latency is recorded from query input construction, separately from reusable preparation. It overlaps the measured phase total and is never added to it. The finite machine currently completes admission and builds its solution rows before publishing a caller answer. Answer export remains inside execution/observation; no isolated export-kernel time is claimed.

## The cutoff was investigated, not classified as a loss

The initial allocation pass stopped during preflight on ascending conditional execution of the unselective depth-eight query. At the two-million-call bound it had produced no answers. A separate diagnostic continued it to 4,112,933 calls and all 256 independently checked answers. Reverse order completed at 3,064,787 calls.

The ascending diagnostic attributes 1,737,768 calls to application service, 1,003,104 to body service and 1,368,316 to observation. At four million calls it had delivered 235 answers. This establishes ongoing work and completion; it is not evidence of a stuck process or a proof that these costs are intrinsic to conditional execution.

The common runner bound was prospectively raised to eight million calls, preserving the source, engines, process limits and matrix. The complete matrix then ran afresh. All 23 completed initial samples have identical work and allocation records after normalizing the external root baseline. Original binaries, source snapshots and the cutoff record remain available.

## A consequential allocation defect was corrected

Private solving accounted for 12.18 MB of the initial unselective batch's 16.39 MB. Inspection found that a match copied a whole state even when its finite domain contained only the matching value. The copy was immediately discarded because no unmatched assignments remained.

The implementation now checks cardinality before making that copy. The [paired attribution](S06-empty-complement-attribution.md) lowers requested allocation in all 12 nonempty finite cases and leaves four zero-choice cases unchanged. Work signatures, answers and other allocation phases agree. Selective traffic falls from 868,762 to 795,100 bytes, changing its close comparison with the conditional controls. Unselective traffic falls from 16,392,746 to 15,772,090 bytes; that disadvantage survives.

There is no evidence yet for eliminating the remaining unselective cost through a small caller-bridge adjustment. The separately measured caller setup/execution phases request about 4.12 MB; private solving alone still requests 11.56 MB. That private phase also constructs solution rows and copies caller terms, so it is **not an intrinsic solving-kernel lower bound**. Shared input, solution representation, publication and more general lowering remain required investigations.

## Ownership and correctness checks

The pilot has 204 configurations, two exact allocation replays and five ordinary timing runs per configuration. It covers cold/reused preparation, changing queries, immediate/window/all answer retention, failure, cancellation after four service calls and cancellation after two raw answers. The prepared-prefix control retains artifacts by ordered predicate/arity signature; their construction and release are charged.

Every native process first completes the exact mode/query outside measured intervals and compares full answers with independent scalar semantics and analytical source results. The measured run then checks endpoints and raw counts. The replay warms within-process execution; startup and validation are excluded from the timings.

All 204 allocation pairs, 204 diagnostic/timing work signatures, phase live-byte boundaries and final ownership roots pass. The audit finds 156 groups whose allocation work is independent of consumer retention. Query state, consumer payloads, cached artifacts and prepared state all release at their recorded boundaries. Fixed harness buffers are preallocated outside measurement; dynamic answers remain charged.

After the correction, default and counter-free package runs each pass all 154 tests. The bridge tests cover nontrivial output bindings, source-name collisions, weighted later choices and fresh output aliases. Scoped strict Clippy passes. Reference-interpreter code is unchanged.

## Four-package breadth review: investigate native graph correspondence next

The four packages since the effect lifecycle review are the direct-solving obligations, finite-phase mechanism, complete lifecycle pilot and empty-complement attribution. They establish a useful finite compilation regime and contrary costs. They do not settle execution outside its restricted phase.

**T080 native graph source correspondence is selected next.** It can change the organization for dynamic choices, effects and progress beyond the finite solver's eligibility. Establish an actual source-to-graph execution mapping, preserving named-choice correlation, consuming occurrences, off-output failure and a finite answer beside continuing work. Require real service and cancellation boundaries. Compare serial effect ownership with local claim/commit obligations before scheduling a connected-parallel cost study.

The next gate must distinguish native graph execution from a graph-encoded equality service or a wrapper around a blocking evaluator. Existing prototypes have no automatic priority. Inspect available substrates and current correspondence evidence, then select a concrete witness and control; do not rerun an earlier encoding merely because it exists.

This is a prioritization judgment over further finite-solver representation work, broader reuse and effect precision. Those can still change conditional tradeoffs and remain required. Native correspondence addresses a broader unanswered capability at this boundary. T073 remains unfinished for richer resources/domains, solution ownership, admission/publication, learning, compilation costs and timing confirmation. Whole architectures and held-out closure remain required; the goal is active.

## Inspect or reproduce

[Lifecycle registration](../registrations/S06-finite-lifecycle.md), [driver](../../../research/chr-direct-conditional/experiments/finite_lifecycle.py), [lifecycle summary](s06-finite-lifecycle-extended/summary.json), [completion](s06-finite-lifecycle-extended/completion.json), [initial cutoff](s06-finite-lifecycle/meter/0023.json), [bound correction audit](s06-finite-lifecycle-extended/bound-correction-audit.json), [ascending progress diagnostic](s06-finite-lifecycle-sizing/service-ascending-8.log) and [reverse diagnostic](s06-finite-lifecycle-sizing/service-reverse-8.log) retain the evidence.

Run `python3 research/chr-direct-conditional/experiments/audit_finite_lifecycle.py`, `audit_finite_bound.py` and `audit_empty_complement.py` from that same directory path for the three audits. Drivers preserve fixed result directories and refuse to overwrite a run. The allocation meter measures requested heap storage, not RSS; no source-program native compilation or complete architectural lifecycle superiority is claimed.
