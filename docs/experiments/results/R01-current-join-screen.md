# Current ordinary-CHR join work screen (T057)

Active+Indexed eliminates table-size-dependent repeated discovery in the registered
stable keyed-request family. This evidence does not justify a maintained-join
backend or more runs of this family. It does not settle weakly keyed, many-to-many
or frequently changing joins.

All32 processes and16 cells complete with exact repeated non-time results, full
receipt/alias observations, source applications and exhaustion. Root and independent
reader validate the [audit](r01-current-join-screen/audit.json), request boundaries
and source/binary hashes at `b170835`. The
[registration](../registrations/R01-current-join-screen.md) preceded every run;
[semantic gates](R01-current-join-entry.md) include binding and replacement.

## Candidate discovery

Counts include producer/acknowledgement service. Subsequent requests exclude the
first request, which includes setup. Final remainder starts immediately after the
last acknowledgement commit and includes driver reposting, cleanup and exhaustion.
R1 therefore has no subsequent-request interval.

| N / requests / policy / access | Total candidates | Subsequent requests | Final remainder |
|---|---:|---:|---:|
| 16 / 1 / global / scan | 2,505 | 0 | 1,955 |
| 16 / 1 / global / indexed | 425 | 0 | 355 |
| 16 / 1 / active / scan | 1,195 | 0 | 68 |
| 16 / 1 / active / indexed | 235 | 0 | 68 |
| 16 / 32 / global / scan | 24,115 | 21,610 | 1,955 |
| 16 / 32 / global / indexed | 3,075 | 2,650 | 355 |
| 16 / 32 / active / scan | 1,892 | 697 | 68 |
| 16 / 32 / active / indexed | 452 | 217 | 68 |
| 128 / 1 / global / scan | 765,449 | 0 | 732,419 |
| 128 / 1 / global / indexed | 17,673 | 0 | 17,155 |
| 128 / 1 / active / scan | 66,827 | 0 | 516 |
| 128 / 1 / active / indexed | 1,803 | 0 | 516 |
| 128 / 32 / global / scan | 2,055,571 | 1,290,122 | 732,419 |
| 128 / 32 / global / indexed | 37,795 | 20,122 | 17,155 |
| 128 / 32 / active / scan | 71,108 | 4,281 | 516 |
| 128 / 32 / active / indexed | 2,020 | 217 | 516 |

At both N16 and N128, each of the31 subsequent Active+Indexed request cycles uses
exactly7 candidate visits,10 cursor steps,2 indexed bucket entries,1 predicate-pool
entry,4 activation pops and3 history checks. The first interval has10N+7 candidate
visits, subsequent intervals7(R−1), and final remainder4N+4, giving observed total
14N+7R+4 across these four cells. These equalities describe measured source work,
not an all-join asymptotic theorem or a runtime speedup.

Active selection dispatches rules mentioning the current occurrence. Generic
matching prebinds the nonfirst request anchor before enumerating earlier kept
heads. Its known key then selects each partner directly. Global lacks that anchor
when restarting source order; Active+Scan retains partner scanning. Their repeated
work does not establish a deficiency in the competent combined control.

[Full phase results](r01-current-join-screen/summary.json) retain access, activation,
history and live gauges. Index/dependency live-entry gauges may decrease and are
not additive work. Instrumentation is enabled intentionally. There is no primary
timing, requested-heap, compilation or total-lifecycle conclusion here. Process
wall time is only a resource diagnostic including validation.

## Disposition and next decision

The historical retained-prefix implementation's repeated extension visits are
bounded mechanism evidence, not a reason to port its maintenance state into this
current engine. The present competent recomputation path services repeated keyed
requests without N-dependent steady discovery. Further maintenance requires a
separately motivated surviving multiway/update bottleneck, not another favorable
source selection. Independent reviews agree to stop this bounded investigation.

T058 reassesses full-goal sufficiency and the strongest remaining concrete direction:
substantive equation/construction sharing. T056 supplies a valid source oracle;
E12 supplies actual-boundary replay precedent and contrary interface costs. Decide
whether a new shared-representation reuse control could materially change the
recommendation relative to its certificate, dependency, cache and replay costs.
Do not treat every possible optimization as a mandatory backend, or call the goal
complete merely because these gates and screens pass.
