# Learned failure regions are sound, but early splitting can duplicate work

A completed failed finite query can safely prune part of a later successful query with wider domains. The implemented learner preserves complete answers, aliases and derivation counts in the registered gate. Its eager subtraction can also duplicate common work, so it is not yet a credible total-cost winner.

## What is learned—and what is not

The learner records a domain region only after the existing finite solver completes with no solutions. It owns that region under one immutable prepared ruleset. Query-variable names are normalized, while ordered private goals, constants, duplicate occurrences and alias topology remain part of the key.

For matching private goals, a failed region proves that all assignments inside it are infeasible. A later query may have wider or overlapping domains. The learner subtracts only the covered intersection, dividing the remainder into disjoint domain boxes. It retains the later query's choice weights, so successful raw derivation multiplicity is unchanged. Caller state and nonempty independent producer domains may vary because the checked private phase cannot depend on them before completion.

This goes beyond an exact-query failure-cache hit: the first focused example learns failure over `{a,b}`, then solves `{a,b,c}` successfully while skipping the failed portion. An executable exact-query result cache correctly serves repeated identical queries, but must solve the different wider query. No successful solutions are stored by the region learner.

This is not general conflict learning. It does not extract a minimal explanation from a failed subderivation inside a successful query. It learns the full failed input region after complete solving. Private variables must have finite initial domains; that is the present learner's eligibility check, not a necessary language restriction. Broader explanations and admission remain open.

## Independent correctness evidence

The fixed matrix contains 96 seed sessions and 1,920 follow-up sessions per build. It varies accepted pair relations, domain support, duplicate-choice weights, cache capacity, shared versus distinct variables and variable renaming. Both default and metrics-off builds agree exactly on recorded work and answer counts.

Every matrix result is checked against independent owned-syntax scalar execution and existing compiled Scan. A separate analytical audit enumerates allowed atom pairs and verifies multiplicity. Full caller execution includes token consumption and residual observations. Focused checks also show the caller can create new private work after the learned phase, using the full original rules.

Cancellation, suspended work, exhausted limits and invalid learning admission never install a failure region. A partition-limit error preserves previously learned facts. Zero-capacity retention learns nothing; bounded eviction changes reuse opportunity without changing answers. Changed private goals miss, and unrelated producer domains retain their full successful alternatives.

Seven learning tests and 23 existing finite-phase/bridge/read tests pass under both feature configurations. Scoped strict Clippy and formatting pass. The reference interpreter and independent scalar semantics are unchanged.

## Work reductions and adverse cases

| Fixed-matrix outcome | Follow-up queries | Meaning |
|---|---:|---|
| Fewer solver steps | 266 | All are failed queries; this matrix does not establish successful-query savings |
| Same solver steps | 1,640 | Includes all successful queries in this matrix |
| More solver steps | 14 | All reject under the unconstrained failing rule; recomputation takes two steps, while learned subtraction splits the work |

The focused one-dimensional wider-query test does demonstrate successful-query savings. To test whether that benefit survives shared work, a separately registered attribution uses a failed `{a,b} × {a,b}` region and a wider successful `{a,b,c} × {a,b,c}` query. A pair succeeds when either argument is `c`. The source performs a variable amount of deterministic work before that check.

| Deterministic prefix depth | Recompute: solver steps | Eager learned subtraction: solver steps |
|---|---:|---:|
| 0 | 10 | 9 |
| 1 | 11 | 11 |
| 4 | 14 | 17 |
| 16 | 26 | 41 |

These counts repeat with duplicate-choice weights one and two. The query has five distinct successful pairs; weight two produces 20 raw answers, all independently verified. Partition counts remain seven on both paths. The extra solver steps come from executing the prefix separately in surviving boxes, not from a changed answer set.

Reported solver steps exclude key construction, region lookup and subtraction. Consequently, even a decrease is not a total-operation or speed result. No allocation, retention-byte or timing comparison has run.

## Validity argument and implementation responsibilities

For fixed private goals and prepared rules, complete failure establishes infeasibility of the tested domain product. Its intersection with a later domain product remains infeasible. Ordered subtraction produces disjoint survivors, preserving coverage without duplicating successful assignments. Positive weights change multiplicity but not infeasibility. Renaming preserves variable relationships; a changed alias pattern changes the key.

The complete-phase boundary is essential. Only private-phase failure is learned; later caller failure cannot supply a region. Errors and cancellation provide no completed proof. An impossible ground producer input also supplies no generalizable region. Each learner borrows one immutable prepared owner, so facts cannot cross to different rules through the API.

The new responsibilities are normalized private-goal keys, retained domain sets, region lookup, disjoint subtraction, bounded eviction and transactional installation on completion. A session mutably borrows its learner, serializing access to that learning store. These responsibilities need allocation and lifetime measurement; they are not free because some solver work disappears.

## Next investigation and comparison with T078

Keep T073 active to compare eager subtraction with checking learned-region coverage after ordinary source/domain refinement. Such a policy could preserve the shared prefix and prune a failing state only when its remaining assignments are known to be covered. It would introduce repeated validity checks, so favorable and adverse cases must charge that work separately.

This is selected over T078 allocation qualification at the current gate because the measured duplication is consequential and a concrete alternative could change whether learned regions belong in the architecture. Timing the eager design alone would risk attributing its representation choice to learning generally. Revisit the T078 allocation and coherent-path pilot after that paired mechanism gate or a consequential obstruction. Neither the current learner nor a later policy resolves richer conflict learning, resource derivations, native compilation or the research goal.

## Reproduction

The [registration](../registrations/S06-learned-regions.md) records the initial gate, full matrix and prospective common-work attribution extension. The learner is in [finite_learning.rs](../../../research/chr-compiled/experiments/finite_learning.rs), attached to the existing finite phase as an explicit experimental API. [The tests](../../../research/chr-direct-conditional/tests/finite_learning_gate.rs) generate the source cases and compare all controls.

Run the five test targets `finite_learning_gate`, `finite_phase_gate`, `finite_bridge_gate`, `finite_bridge_futures` and `finite_kept_read_gate` in package `chr-direct-conditional`, both normally and with `--no-default-features`, using `-- --nocapture` to record the matrix. [The audit](../../../research/chr-hvm/learned_regions/audit.py) checks all positions, analytical multiplicities, cross-build work equality and the common-work formula. [Raw evidence](s06-learned-regions/) includes both gate logs, the initial executable checks, Clippy and source/evidence hashes.
