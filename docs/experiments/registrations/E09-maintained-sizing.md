# A3 maintained matching: resource sizing

Status: registered before any sizing children; start only after the v2 semantic
gate audit passes. This is exploratory sizing and algorithmic work analysis,
not a comparative runtime estimate. It does not depend on A1 or A6 succeeding.

## Question

Do the small gate's retained-update savings survive larger stores, repeated
arrivals, consuming joins and eight distinct explicit alternatives? Which costs
limit this representation: repeated structural matching, scanning retained failed
tests, rebuilding predicate pools, retained metadata, or observation? These
answers select follow-up algorithms and credible timing workloads.

Use the unchanged six-rule `maintained_cases.case` generator with four parameter
points in this order: (N4,B1,R1), (N16,B2,R4), (N64,B1,R1), (N64,B8,R4).
For keep then consume, visit all four points, each prefix/full/selective:24 cells.
Each uses Q8, hash seed0, fresh process, 30 seconds, 1 GiB address space,
100,000 source steps and 10,000,000 instrumented actions. Record all24 outcomes,
including failures and timeouts. No repetitions or timing rankings are authorized
by this sizing protocol. Freeze registration, harness and all transitive local
source/oracle inputs before the first child, and retain exact commands and errors.

Disable per-transition tree replay for these larger runs. Keep complete independent
analytic answer checks, all ground residual multiplicities, raw/unique B,
zero failure and exhaustion checks. The smaller transition gate remains supporting
evidence, not proof of every larger internal transition. Successful matched cells
must agree on full observations and logical source-step counts. Store full answers
in the evidence; resource censoring is an unresolved outcome, not a wrong answer.

## Predictions and interpretation

Selective invalidation should reduce physical matching calls against full
invalidation, especially for kept prefixes. It still visits retained prefixes and
candidate occurrence pools, rebuilds the predicate index on updates, and retains
failed extension tests. High-arity or broad alias updates may erase savings.
Eight contexts can share stored row payloads while executing new tests separately;
measure both, with no claim that support masks merge those computations.
Full invalidation controls representation effects, while prefix is the competent
transient early-mismatch control. Neither is a TREAT, lazy-witness, or multiway
join implementation.

Report retained record/index-edge high-water marks and source/matching/maintenance
counts separately. Wall time includes analytic validation and host implementation
costs and serves only to diagnose resource bounds. It cannot establish a speedup.
A cutoff calls for attribution and either a justified resource adjustment or a
better algorithmic contrast; it does not close A3 or permit favorable-case selection.
Follow-ups remain seed-ordered recomputation, selective/lazy joins, incremental
join-key indexes, cross-context new-test sharing, and full lifecycle memory/time.
