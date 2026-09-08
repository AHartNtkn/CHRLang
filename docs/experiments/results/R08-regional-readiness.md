# Regional parallel entry: preserve responsibilities and strengthen the control

T043 examines whether certified independent regions can earn their worker and transport costs against a credible serial execution path. No comparative timing is registered or run by this entry.

## Measurement audit

The regional path spans `chr-factors`, `chr-persistent` and `chr-observe`. Disabling `kernel-metrics` alone does not disable legacy source, observer, certificate, product or transport diagnostics. Two fields also carry non-diagnostic responsibilities: outstanding reservations enforce the admission bound, and source completion totals supply exact raw multiplicity. These responsibilities must remain active and charged in primary execution. A zero diagnostic value cannot stand for a missing semantic count.

The bounded implementation separates the admission gauge and raw-completion count from optional diagnostics, including the raw count conveyed from a regional worker to its owner. Request IDs, slots, response validation, cancellation, source budgets, produced-answer state and exhaustion remain operational. Default builds retain diagnostics. Primary builds must report diagnostic availability across every participating crate, because Cargo feature unification can enable a dependency's counters even when its direct caller disables defaults. Separate target directories alone do not prevent feature unification within a build. Optional source/observer snapshots must also be absent from primary worker messages and initialization, so disabled counters do not leave diagnostic transport traffic in the measured path.

The E16 harness measures construction through joined worker/search disposal as a contiguous interval containing diagnostic snapshot gaps. Workers can run during the stop snapshot, so subtracting its duration does not reconstruct counter-free parallel elapsed time. Its output disposal is separately measured after complete-answer validation. A fresh primary runner must avoid diagnostic snapshots while workers remain live and report construction, first service, execution, shutdown, engine release and output release explicitly. Complete lifecycle totals must include output release, while identifying the validation interruption. Preparation and query setup may be reported together where the regional constructor genuinely combines them; this does not establish reusable preparation or compilation amortization.

Requested allocation traffic and peak live requested bytes remain separate diagnostics, not RSS. Physical work after a prefix or refutation can vary with worker timing. The joined endpoint must account for all worker work and restore allocation ownership without requiring equal speculative work across runs. Matched-quantum accepted answers and source work provide a different check.

## Applicability and serial control

The certificate joins predicate name/arity across every rule head and body, and joins regions sharing query variables. It supports permanently disconnected predicate families. Independent invocations of the same predicate remain one region. This is not a certificate for general call parallelism, temporary independence or connected synthesis.

The observer returns unique full answers and separately tracks raw multiplicity. This must be stated alongside results rather than silently equated with a raw streaming endpoint. Nonground aliases, full residuals, product ownership, refutation and cancellation remain correctness obligations.

Same-quantum Inline, one-worker and two-worker modes share the persistent source executor and appropriately isolate transport/capacity. That executor still constructs generic head/candidate data and maintains consuming-rule history. The carry cases admit current inferred single-head specialization. Therefore a faster worker mode than Inline would not alone establish useful architectural parallelism against a stronger serial alternative.

A finite full-observation gate in `research/chr-compiled/tests/regional_serial_control.rs` checks inferred-specialized serial execution against the existing independently constructed expectations, including raw multiplicity and exact unique answer coverage. It covers balanced and asymmetric carry, zero work, owner-heavy publication, duplicates and combined addition/type inference. The serial control need not reproduce worker microsteps or quantum scheduling. A subsequent bounded pilot must charge its preparation, execution, deduplication, observation and disposal against the same source contract. The combined application is an applicability witness, not an assumed application distribution.

## Decision gate

Proceed to a newly registered bounded comparison only after metrics-off semantics and protocol ownership pass, diagnostic absence is checked, and the stronger serial control passes the finite gate. Include matched-quantum transport controls and the specialized complete serial path. Keep overhead, imbalance, owner-heavy publication and cancellation/refutation controls; do not automatically replay the entire E16 matrix.

If the stronger serial path makes worker savings irrelevant, record that as evidence about this interface and granularity rather than enlarging workloads until workers win. If counter separation requires protocol redesign or applicability has no meaningful boundary beyond synthetic capacity, reconsider static-lowering eligibility and compilation lifecycle. That is the strongest broader alternative: it can eliminate work outright, but needs a reusable fragment and charged transformation/toolchain/loading before amortization claims. No conclusion here rules out parallelism with a different executor or certificate.

## Validation status

The unmodified entry baseline passes all 21 `chr-factors` tests, including full workload expectations and deterministic transport/cancellation checks (`cargo test -p chr-factors --all-targets`). The revised regional suite passes in default and standalone no-default builds, as do persistent and observer suites. The specialized finite gate passes both modes. All 292 workspace all-target tests, workspace strict Clippy, package Clippy in both feature modes and formatting pass.

The explicit off-mode gate establishes four raw completions despite one unique answer, repeated K1 reservation reuse with Inline/one/two workers, correct exhaustion and shutdown, and unavailable zero diagnostics across every dependency. Private cancellation and reply-validation tests remain active in that mode. Independent review checked the separation and serial-control applicability.

[Optimized-code evidence](r08-regional-readiness/optimized-check.txt) shows unused diagnostic pointer arguments in representative observer, unification, matching and export routines. The regional service response is 64 bytes without diagnostics versus 272 with them on this toolchain; the off service reads semantic raw multiplicity without copying diagnostic snapshots. This verifies code and layout, not a measured speedup. [Validation logs](r08-regional-readiness/workspace-tests.log) and the adjacent package/feature logs retain the checks.

The entry is ready for a new runner and prospective bounded registration with the stronger serial control. T043 remains active. This document is a readiness result, not a performance result or goal closure.


Reproduce the optimized-code inspection from the repository root:

```sh
cargo rustc -p chr-observe --release --no-default-features --target-dir /tmp/t043-ir-off --lib -- --emit=llvm-ir
cargo rustc -p chr-persistent --release --no-default-features --target-dir /tmp/t043-ir-off --lib -- --emit=llvm-ir
cargo rustc -p chr-factors --release --no-default-features --target-dir /tmp/t043-ir-off --lib -- --emit=llvm-ir
cargo rustc -p chr-factors --release --target-dir /tmp/t043-ir-on --lib -- --emit=llvm-ir
```

Inspect the named function signatures and service body in the emitted `.ll` files; symbol hashes and layouts are toolchain-specific. Diagnostic absence is supported jointly by feature wiring, semantic off-mode execution, source review and optimized-code inspection, rather than output zeros alone.
