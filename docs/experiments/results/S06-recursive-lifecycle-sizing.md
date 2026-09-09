# Recursive updates: promising indexed results, a missing access control

**Contraction reduces discovery and complete time in the exploratory long single-call cases.** Competing-call cases record no contraction and no comparable improvement. Before confirmation, scanning must be added as a competent control: indexed reposting repeatedly traverses the growing open accumulator.

All 2,500 [registered processes](../registrations/S06-recursive-lifecycle-sizing.md) succeed: 600 ordinary timings, 1,200 allocation runs, 600 separate work runs and 100 cancellation runs. All 600 allocation pairs replay exactly. Source applications, forks, failures and answer counts agree across all five configurations. This is sizing evidence, without confirmed practical rankings.

## What actually changes

At depth64/four queries with consumption, the unary recurrence has 266 source-equivalent applications under both same-build specialized execution and contraction. Contraction accounts for 254 skipped recursive applications, reducing counted candidates from 540 to 32 and service steps from 588 to 338. The result still requires all constructor updates and complete observation.

The multi-choice case has 548 source applications, 12 forks and 16 answers in both configurations. It records zero contracted steps; candidates remain 1,148 and service steps 1,346. This confirms that singleton admission prevents useful contraction on this control, rather than guessing that explanation from timings alone.

Work counts sum retired split prefixes and terminal segments exactly once. Source-equivalent applications include certified skipped steps. Candidate counts combine ordinary and specialized instrumentation events; their absolute totals are not a universal unit of computational work. The same-selector contrasts above avoid interpreting differences between selector definitions as speed.

## Complete-cost observations

Single-sample primary milliseconds at depth64/four queries/resources. All modes use Global+Indexed access. Feature-off controls retain the cost of the unextended runtime; same-build controls separate admission from feature/configuration overhead.

| Family | Off, ordinary | Off, specialized | On, ordinary | On, specialized | Contracted |
|---|---:|---:|---:|---:|---:|
| pass | 0.448 | 0.323 | 0.423 | 0.282 | 0.154 |
| unary | 1.230 | 1.070 | 1.071 | 1.149 | 0.328 |
| nested | 2.141 | 1.952 | 2.078 | 2.096 | 0.611 |
| open | 1.755 | 1.415 | 1.778 | 1.337 | 0.302 |
| late | 1.818 | 1.662 | 1.618 | 1.677 | 0.447 |
| malformed | 1.141 | 1.082 | 1.037 | 1.080 | 0.267 |
| choice | 1.456 | 1.422 | 1.510 | 1.143 | 0.565 |
| multi | 2.156 | 1.787 | 2.172 | 1.952 | 1.839 |
| multi-choice | 2.733 | 2.431 | 2.978 | 2.502 | 2.497 |
| fail | 0.715 | 0.382 | 0.552 | 0.348 | 0.206 |

Primary totals include preparation, query setup, complete execution/observation and engine/answer/prepared disposal. Ordinary timing has counters disabled and uses the ordinary allocator. Meter and work elapsed times are not included. Native compilation is excluded.

For unary updates, same-build specialization spends 1.013 ms executing/observing versus 0.223 ms contracted. Source/input-inclusive totals are 1.155 and 0.333 ms. Requested primary allocation falls from 1,036,956 to 372,657 bytes; peak requested growth is nearly unchanged at 47,148 versus 47,222 bytes.

For nested updates, execution/observation is 1.904 versus 0.428 ms, and requested primary allocation is 1,879,041 versus 501,316 bytes. Required constructors and output remain, so these figures do not represent elimination of the entire computation.

The multi-choice control requests slightly more primary memory with contraction enabled: 2,643,061 versus 2,640,737 bytes. Its single timings are 2.497 and 2.502 ms. This is an overhead/no-admission control, not evidence of equivalence within a narrow time tolerance.

Short timings are noisy and contain contrary cases. The zero-depth, one-query nested case measures 0.075 ms contracted versus 0.049 ms same-build specialization and 0.033 ms feature-off specialization. No threshold, outlier exclusion or significance claim is applied to these single samples. The [full summary](s06-recursive-lifecycle-sizing/summary.json) retains every cell, first observation, phases, allocations and work counts.

## The consequential missing control

Code inspection shows a source of avoidable baseline work. In [Core::insert and Core::refresh](../../../research/chr-compiled/src/lib.rs), Global+Indexed execution maintains occurrence dependencies and index keys after each repost. Refresh traverses open term structure. With an unknown seed, an accumulator f^i(seed) grows as the recursion proceeds; repeatedly refreshing those terms can visit a quadratic total number of nodes. This is an analytical mechanism, not a measured complexity fit.

Global+Scan does not require this dependency maintenance. The existing carrier implementation supports that access mode, and its semantic gate already exercises it. Comparing only Indexed execution can therefore exaggerate the case that recursive contraction is necessary: a competent ordinary configuration may avoid much of the same work. The present timing results remain valid for their stated Indexed controls.

The next comparison must cross access mode for ordinary specialization and contraction, retain feature-off controls, and count dependency visits/refreshes and index repair separately. Keep the same source queries and complete lifecycle, including unknown seeds. Successful ground-seed controls may later distinguish term closedness from constructor work; current successful update families carry an unknown seed even when their control spine is ground.

## Validation and next decision

The enlarged depth64 allocation replays and metered cancellation checks pass before ordinary timings. Cancellation at 32 ticks is independently shown to interrupt live prefix inspection in the eight single-call contracted families; the next query completes and all requested-live bytes are restored. This covers active update-job disposal, not merely cancellation before source work begins.

The [audit](s06-recursive-lifecycle-sizing/audit.json) verifies counts, current/frozen source and binary hashes, commands and source-work agreement. The work companion passes its interruption test and strict Clippy in feature-on and feature-off builds. Runtime and timing-runner source are unchanged from the allocation gate. Separate work code follows the same prepared-rule selection and source inputs. The independent evaluator is unchanged.

Requested bytes are not RSS. These short sessions do not establish sustained retention or publication behavior. No exact/direct source-elimination control is present, so even an access-corrected comparison would not claim the best achievable implementation or a whole-architecture winner.

Prioritize the Scan/Indexed control and attribution before repeated timing confirmation. It can change the bounded conclusion at low implementation cost, and is more valuable now than tightening intervals around an incomplete control set. The broader integration, observation and lifetime alternatives remain assigned under the latest breadth review. T073 and the architecture goal remain active.
