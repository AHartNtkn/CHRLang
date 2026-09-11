# Incidence storage is the largest admission allocation responsibility

**Incidence maintenance accounts for42–56% of requested query-setup bytes across the relational configurations. Constructor lookup accounts for2–4%.** This makes incidence storage the next allocation-focused intervention. It does not prove that incidence dominates CPU time.

The [registered attribution](../registrations/S02-admission-attribution.md) completes512 isolated processes:128 configurations in metered control and diagnostic builds, repeated twice. Every complete answer passes the independent scalar check. Every session releases its owned heap; allocation counts, requested bytes, root-relative live/peak bytes and advance counts match exactly between builds and repetitions. Compiled controls emit no relational admission scopes.

## Where setup allocation goes

For selective settlement, base depth64, separate output equality, early failure and16 changing queries:

| Exclusive responsibility | Requested bytes | Share of setup |
|---|---:|---:|
| Incidence maintenance |1,305,136|54.4%|
| Column indexes |439,808|18.3%|
| Relation insertion, excluding indexes/incidence |338,528|14.1%|
| Fresh value arrays |130,048|5.4%|
| Constructor lookup |100,496|4.2%|
| Constructor handling outside those scopes |51,856|2.2%|
| Other admission and occurrence posting |31,880|1.3%|
| **Total setup** |**2,397,752**|**100%**|

The scope counts record2,080 constructor lookups and4,256 column/incidence attachments. Percentages are rounded. Nested scopes are exclusive: their allocation totals equal measured setup exactly, so the table does not count child work twice.

Across all96 relational configurations, incidence consumes42.3–56.3% of setup allocation, columns18.3–24.8%, relation insertion11.1–22.5%, and constructor lookup1.8–4.4%. Full, selective and bounded settlement are included, as are shared/separate dependencies, successful output, early failure, late contradiction and admission cancellation. Execution is excluded from these attribution scopes; complete lifecycle phases remain recorded alongside them.

## What is measured and what is inferred

The measurement identifies requested allocation by responsibility, not physical residency or execution time. The source explains what incidence owns: a map from values to ordered sets of `(Relation, row)` references, maintained for every attached column. Each relation key carries its name. This is a concrete representation cost, but the present result does not separately rank set-node capacity against relation-name copies.

The next bounded intervention will compare the ordered-set incidence buckets with sorted vectors preserving the same ordering and uniqueness. That directly tests bucket ownership without changing equality, relation indexes or source semantics. Low-degree buckets could save tree-node allocation; high-degree insertion/removal could cost more movement. Those are hypotheses to test, not promised improvements. Relation-key interning remains a separate alternative if attribution after this intervention warrants it.

The existing compiled control remains a serious competitor. The earlier readiness lifecycle result demonstrates a large total-cost gap; reducing one relational allocation responsibility does not establish that the gap closes.

## Verification and the audit correction

The profiler reuses allocation-free callbacks and fixed storage. It records only the admission scope and its descendants, with execution excluded. Source/store/library regressions pass in ordinary and diagnostic builds; both the new runner and the existing deduction profiler pass Clippy.

The first cross-build audit found a uniform two-byte offset in resident live-byte gauges. The profile binary's filename is two characters longer than the meter binary's filename, and the runner retains its command-line arguments outside measured phases. The audit now compares live/peak gauges relative to each process's pre-measurement root. It still requires exact allocation calls, requested bytes, frees and full owner restoration. All256 matched diagnostic/control pairs pass; no measured allocation difference was waived.

The frozen archive preserves the execution driver before that audit correction. The current auditor reconstructs the root-relative comparisons, exact repetitions, exclusive attribution and zero-scope compiled control from raw records.

## Next decision

Qualify sorted-vector incidence buckets with the existing source/equality tests and an independent broad-incidence/merge challenge before measuring complete ownership. Charge attach, detach, merge repair, source execution and disposal; preserve order and repeated-column uniqueness. Repair any source or fixture obstruction rather than treating it as a restriction.

This is a narrower and cheaper next test than a new integrated executor, with a measured responsibility to challenge. Fresh graph attribution and direct solving remain strong alternatives; reconsider them at the incidence result. Package count is one since the portfolio review, T072 remains active, and no integration direction is declared complete.

[Raw evidence and frozen sources](s02-admission-attribution/) · [Per-configuration attribution](s02-admission-attribution/analysis.json) · [Runnable auditor](../../../research/chr-relational/experiments/admission_attribution.py)
