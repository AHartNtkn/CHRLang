# Descriptor-backed source execution works within an explicit serial boundary

The descriptor executor preserves complete source observations in all 316 source-priority runs. Component-only execution agrees on canonical answers but produces six raw insertion-order differences. The experiment implements actual matching and staged storage; it does not establish that a serial source owner has been replaced.

A cancellation defect exposed a necessary distinction: occurrence identity and committed insertion order must be separate. The corrected implementation retains unique reservations while assigning insertion order at commit. A global source-priority commit check and a global observation epoch remain explicit responsibilities.

## What now executes

The cooperative Python executor matches ordered, distinct occurrence tuples from actual rules. It supports atom/unknown terms, nonbinding repeated variables, equality updates, fresh variables, consuming and kept heads, and propagation history. Rule components are derived from predicate read/write interactions and conservative equality dependencies. Equality writers currently connect all rules, which preserves safety in these tests but can prevent useful independence.

Prepared body effects live in descriptor-backed fields. Each installation and cleanup changes one field; descriptor commit or abort determines which value readers see. Multiple descriptors can remain live for independent sources. Cancellation after partial installation preserves the committed store, cleans installed references and retries with fresh reservations. Completed runs leave no pending descriptors.

An observer scans fields while a writer action occurs after its first read. Installation, commit, abort and cleanup are each exercised this way. Changed catalogue/status epochs discard the scan; accepted scans must equal the committed store. This is real cooperative interleaving of heap objects, not weak-memory or machine-parallel validation. It does not establish retry fairness or safe reclamation for unsynchronized readers.

## The controls caught two different problems

The first attempt used the reference adapter to compare residual insertion order. That adapter canonicalizes and sorts residuals, so it cannot prove order. The independent reference remains unchanged and continues to check complete canonical answers and aliases. Fresh runs of the existing native serial compiler/executable supply raw ordered observations for all 79 sources.

The corrected comparison then found an implementation defect: after cancellation, newly reserved occurrence IDs put a retried application's output after another descriptor's output, even when commits followed source priority. Separating IDs from committed insertion ordinals fixes the full registered comparison. The failed attempts, original implementations and traces remain in the evidence.

This repair preserves the ordered-observation requirement. It does not show that assigning insertion order is free or locally distributable. That responsibility must remain visible in any later architecture or cost comparison.

## Verified coverage and remaining differences

The matrix contains the existing 77 deterministic identity-source cases plus independent ground consumers and recursive independent producers. Each runs under two commit policies, two service directions and application cancellation enabled/disabled: 632 runs total.

- All 632 complete canonical answers match independently supplied expectations and fresh reference results.
- All 316 source-priority runs also match raw native residual order modulo variable renaming.
- Six component-policy runs differ in raw residual order, on independent ground and recursive producers. Their canonical answers still agree. Feeding ordered residuals into further consuming execution therefore needs an explicit contract; these results do not silently choose one.
- There are 10,688 checked store snapshots, 1,980 scan retries, and 308 partial-application cancellations followed by successful completion. Quiescent cases have no application to cancel.
- Independent sources reach two simultaneously live descriptors. The executor therefore exercises staged multi-descriptor ownership, while preserving its stated commit and observation controls.

Choices, body failure, nested constructor terms and invalid output declarations are explicitly rejected by this entry executor. Four boundary checks verify rejection. Dynamic alternatives, failed branches, broader terms, query cancellation/reuse and native implementation remain required work; application abort-and-retry does not discharge those obligations.

## Architectural consequence and next selection

Descriptor publication can be connected to real source matching without exposing partial effects in these cooperative tests. However, the implementation still relies on global commit selection, insertion ordering and observation versioning. It also retains descriptor records for the query lifetime. No timing, allocation, native speed or general lifetime advantage has been measured.

The [four-package breadth review](S03-descriptor-breadth-review.md) selects existential projection under T076 next. That distinct mechanism could eliminate hidden structural search rather than reorganize its ownership. T080 remains required for genuinely local source-order admission, native descriptor integration, broader source support and full lifecycle costs. This source gate supplies evidence and controls for that work; it does not reject distributed ownership.

## Evidence

[Main registration](../registrations/S03-descriptor-source.md), [ordered-control correction](../registrations/S03-descriptor-source-controls.md), [executor](../../../research/chr-local-claims/source.py), [source runner](../../../research/chr-local-claims/run_source.py), [native-control runner](../../../research/chr-local-claims/source_controls.py), and the [raw directory](s03-descriptor-source/) preserve configurations, five attempt manifests, generated native programs, failures, repairs and results. The [replay audit](s03-descriptor-source/audit.log) verifies frozen implementations, all final observations, descriptor completion, retained answers and boundary rejections. Earlier protocol replay audits remain passing. The independent reference and native compiler are unchanged.
