# Demand execution can follow passive resource dependencies, with explicit cycle validation

The demand evaluator now passes a bounded source gate for passive resources keyed by call outputs. Independent checks exposed a hidden constructor-cycle defect and an alias-identity defect through selected choices; both led to repairs. This broadens the demonstrated source capability; it supplies no performance advantage or general architecture selection.

## What the experiment changes

A resource key can depend on a call that has not finished. Matching that resource may force the producer, which can itself need resources. The evaluator must preserve nonbinding matching, distinct occurrences and contextual consumption while discovering those dependencies.

The implementation distinguishes three cases. An unfinished recursive dependency leaves an unknown value and a residual call. A completed cycle of aliases denotes one unknown equivalence class. A cycle containing a constructor has no finite-tree solution and must fail. Treating all three alike would change the source answers.

Producer progress interrupts partner selection before a claim commits. A witness in which nested forcing consumes an earlier candidate confirms that the outer rule cannot claim that stale partner. Other witnesses preserve branch-local consumption and failed alternatives.

## The consequential defect found during review

The first implementation detected constructor cycles during output normalization. With neither cyclic output requested, demand execution returned one empty answer; the independent scalar and compiled controls returned no answers. The [failing source run](s03-resource-dependencies/hidden-cycle-red.log) demonstrates the difference.

Completed equation links are now checked independently of requested outputs. The check follows applicable cached results, aliases, selected choices and constructor children. It neither runs a producer nor chooses an alternative, so the check itself cannot reorder resource claims. Pure alias cycles remain valid; a cycle containing a constructor fails.

The check runs for serviced obligations and before answer construction. Additional witnesses confirm that an invalid cyclic branch does not invalidate a valid alias sibling, and that finite cyclic failure is serviced beside continuing source work. Existing committed-policy differences remain explicit in the regression suite; this is not a claim that every executor uses the same rule schedule.

## What was checked

| Evidence | Scope and result |
|---|---|
| Main source matrix | 48 configurations: atom/constructor result × deterministic/duplicate alternatives × zero/one/two resources × query order × producer success/failure. All complete raw answers agree. Some axes repeat the same failed behavior; these are not 48 distinct semantics. |
| Additional witnesses | Eight cases covering alias/constructor cycles, mutual waiting, nested claims, failing siblings, unobserved cycles, choice-local cycles and finite failure beside continuing work. |
| Independent controls | Owned-syntax scalar semantics and compiled Scan/Indexed; demand CurrentContext, StaticBirth and MatchDependencies are each compared against complete raw answers. Hand expectations distinguish key failure and alias cases. |
| Qualified confirmations | Two default-feature and two metrics-off executions of the five-test source suite. Each execution covers the matrix and eight witnesses, with a 200,000-turn bound per path. |
| Suspended-source regressions | All 31 tests pass in each build, including fresh derivation, local lifting, claims, late posts, finite service and explicit nonconfluent-policy probes. The self-dependent key now checks its actual unresolved answer; unsupported nonground body posts still fail admission. |
| Other regressions | All 22 direct-choice kernel/equality tests pass. Demand work attribution passes; both demand and pull-tab work suites also pass in an explicit work-diagnostics build. The ordinary pull-tab binary contains zero tests and is not counted as validation. |
| Static checks | Scoped Clippy with warnings denied passes. Source and binary hashes are frozen; previous source versions and confirmation attempts remain recoverable. |

Executables ran under 60-second wall/CPU and 1-GiB address-space bounds. There were no qualified cutoffs. The process durations are administrative receipts, not comparative timings. The reference interpreter and independent scalar implementation were unchanged.

The stronger alias-choice observation also exposed two distinct unknowns where the source required one equivalence class. The [failing run](s03-resource-dependencies/choice-output-red.log) preserves the mismatch. Alias canonicalization now follows already-selected choice links, without choosing an arm. Both requested outputs agree after the repair.

## Costs and limits that remain

The repair adds forcing/normalization stacks and read-only graph traversals with visited sets. Traversal may repeatedly inspect completed results, including programs without the newly admitted dependency. This is an implementation cost to measure and attribute, not a demonstrated intrinsic cost of demand execution. Earlier timing results do not describe this changed source freeze.

The gate does not establish general support for nonground passive body posts, arbitrary writable heads, overlapping call clauses or cyclic ordinary call-input dependencies. Nor does it prove all resource-mediated recursive programs correct or establish sustainable retention. The registered examples provide bounded source correspondence, not a general correctness theorem.

## Next decision

Proceed to package two of the [execution sequence](../next-cycle.md#start-here-the-next-experiments): a prospective dependency lifecycle comparison. Contrast known and delayed keys, nested dependencies, useful and fruitless forcing, ordinary acyclic work and completed cycles. Reuse preparation across changing queries and charge observation, cancellation and disposal. Separate primary counter-free timing from allocation and work diagnostics; include the hidden-cycle check in the measured implementation.

This follow-up can determine whether a newly necessary validation responsibility changes the candidate's competitiveness, and whether its current repeated traversal is a consequential avoidable cost. Contextual integration remains the next distinct investigation; joint symbolic solving is the strongest ready alternative. Another correctness gate alone would not answer these cost questions. This is package one after the joint-ownership breadth review; the goal remains active.

## Reproducible evidence

- [Registration and prospective correctness extension](../registrations/S03-resource-dependencies.md).
- [Source cases](../../../research/chr-direct-conditional/tests/resource_dependencies.rs), [confirmation runner](../../../research/chr-direct-conditional/experiments/resource_dependency_confirmation.py), and [implementation](../../../research/chr-direct-choice/src/demand.rs).
- [Qualified source/binary freeze](s03-resource-dependencies/confirmed/freeze.json) and [13-process confirmation receipt](s03-resource-dependencies/confirmed/confirmation.json).
- The same final freeze and receipt include the two explicit work-diagnostics regressions.
- [First confirmation attempt](s03-resource-dependencies/confirmation.json), [pre-hidden-cycle confirmation](s03-resource-dependencies/final/confirmation.json), and [exact earlier-source recovery](s03-resource-dependencies/final/recovery.json).
- [Hidden-cycle/context source checks](s03-resource-dependencies/choice-output-green.log) and [Clippy](s03-resource-dependencies/clippy-choice-output.log).

The runner creates a new `confirmed` receipt directory and refuses to overwrite one. A repeat must use a newly registered destination and preserved build inputs. Prior binary paths remain separate from qualified builds; recovery manifests map changed source files to exact snapshots.
