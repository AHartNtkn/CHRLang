# Call reuse saves requested allocation but retains more than Direct

**Trace reuse requests fewer bytes than Direct in 92 of 144 comparisons, including
all repeated-query cases. Its peak ownership exceeds Direct in every case.** It
also requests fewer bytes than both transition-reuse controls throughout, while
cross-query retention causes twelve peak regressions against stride sixteen.

The [registered ownership study](../registrations/S05-call-trace-ownership.md)
qualifies 576 configurations: trace caller, Direct, separated stride-one and
stride-sixteen reuse; four source modes; depths 4/32/128; one query, four repeated
queries or four distinct-depth queries; retain none/all; complete or first-answer
cancellation. Depths now arrive as query arguments, so every preparation can serve
changed calls. The source gate still passes after that fixture change.

All **1,152 allocation runs** repeat exactly. Another **576 ordinary runs** validate
the same answers and endpoints. Every measured record is checked outside the
measurement interval, consumer bytes match across all modes, and every session
returns to its original live allocation after prepared and consumer disposal.

## Complete ownership comparison

Ratios below one favor the trace caller. Each row covers 144 scenarios.

| Control | Requested-byte ratio | Cases requesting less | Peak-owned-byte ratio | Cases peaking lower |
|---|---:|---:|---:|---:|
| Direct | 0.402–1.262 | 92 | 1.324–4.448 | 0 |
| Stride-one transition reuse | 0.033–0.392 | 144 | 0.037–0.335 | 144 |
| Stride-sixteen transition reuse | 0.178–0.714 | 144 | 0.330–1.289 | 132 |

Against Direct, the query policy matters:

| Query policy, 48 cases each | Requested-byte ratio | Cases requesting less |
|---|---:|---:|
| One query | 0.964–1.262 | 20 |
| Four repeated-shape queries | 0.402–0.855 | 48 |
| Four distinct-depth queries | 0.955–1.139 | 24 |

The complete accounting includes source creation, preparation, input and setup,
service/observation, consumption, query disposal, prepared-table disposal and
consumer disposal. Partial traces and completed results may remain in the table
between queries; they are charged to that owner. These are requested heap
allocations, not RSS. Ordinary-run times here are qualification data, not speed
verdicts.

## Why the peak regressions occur

All twelve regressions against stride sixteen occur with four distinct-depth
queries. Eight involve cancellation on ordinary/fresh-output sources; four are
small failed-alternative sources, with either completion or cancellation.

For an ordinary depth-four source, cancelling each distinct query after its first
answer leaves 9,635, 13,929, 20,215 and 25,357 owned bytes after query disposal
when no outputs are retained. Stride sixteen returns to 2,984 bytes each time.
The diagnostic confirms one additional unfinished call per query: 1, 2, 3, 4.
These machines remain available for future replay until prepared disposal.

For the completed failed-alternative source, query-end ownership grows from 8,451
to 14,333 bytes across four queries even though the unfinished-call count remains
zero. Its stored trace nodes grow 2, 4, 6, 8. Completed keys and results also have a
retention cost. This distinguishes retained results from unfinished execution;
neither is an unreleased allocation after owner disposal.

The diagnostic and updated caller gate pass all five tests with counters enabled
and disabled. Scoped runner/test Clippy passes. [Retention witness](s05-call-trace-ownership/retention-metrics.log) ·
[Complete comparison data](s05-call-trace-ownership/analysis.json).
The audit verifies every process, repeated allocation record, consumer owner,
observation count and frozen input.

## Next decision

T075 next measures ordinary complete lifecycle time using these qualified frozen
binaries and all 144 scenarios. The evidence supports a real time–memory question:
repeated-call allocation falls, but peak ownership rises against Direct. Do not
select the architecture from either measure alone.

Bounded retention, abandoning unfinished traces and recomputation are now concrete
follow-ups, with the recorded distinct-query cases as adverse controls. Their
policies must be tested against future revisits before choosing one. Broader
observer interleaving and sparse projection also remain required investigations.
This is package three since the portfolio review; the timing package must include
the next full review. The goal remains active.
