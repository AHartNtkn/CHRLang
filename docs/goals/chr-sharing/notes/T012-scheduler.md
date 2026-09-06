# A schedulable correctness construction

A FIFO queue of finite, resumable jobs can fairly service explicit alternatives while allowing finite operations to be shared. The construction below gives sufficient progress conditions. It does not prove that compressed scheduling is cheap or that an implementation can omit those conditions.

## Concrete work discipline

A job contains an operation, immutable logical input snapshot, support condition, continuation state and dependencies. A service action advances at most a declared number of instrumented graph/worklist visits. An unfinished runnable job goes to the tail. Normalizing an entire alternative is not one indivisible service action.

Each action admits finitely many jobs, at the tail. Repeated work discovery must not replace an accepted job with a perpetually younger equivalent job. Waiting jobs release their worker; the producer of their dependency is queued independently. A worker cap cannot be filled by waiters while a required producer remains unserviceable.

The correctness construction uses one scheduling ticket per frontier alternative. Tickets can point to common jobs and data. This is explicit per-alternative scheduling overhead, not a claimed compact search representation. A compressed ticket implementation must separately prove that it does not hide starvation within a job's support.

For source-step selection and execution, use stable branch snapshots and a fair reservation sequencer. A request either obtains its complete conflicting support reservation or waits without holding a partial conflicting reservation. Do not acquire further conflicting reservations while holding one. This excludes hold-and-wait deadlock. Only the reserved branch's source work is serialized; other alternatives may progress.

The simple executable control can use one host thread and immutable shared constructor data. More parallel physical updates, lock-free commits, or speculative retries require their own progress arguments. Revalidation that repeatedly restarts a job is not by itself a starvation guarantee.

Resumable source operations keep intermediate bindings and effects in private/shadow state. Only a completed, validated source step becomes visible to other source operations. A failed equation cannot expose partial bindings first. Partial publication is permitted for a support subset only when that subset's complete scalar source operation is ready; it does not expose half a Unify or Apply transition.

A shared job publishes completed support portions independently. A result for B cannot wait for an unrelated divergent calculation in A. New demand becomes new work; it does not continuously enlarge the completion requirement of an accepted batch.

Source OR retires a parent ticket and admits its children fairly. Partitioning support for matching or unification creates bookkeeping jobs, not semantic alternatives. Rule selection commits the chosen application and may invalidate competing applications; the scheduler never forks merely to preserve other CHR schedules.

## Source policy and finite operations

The experiment must declare a committed source policy independently of queue service. A suitable control policy gives pending goals stable birth-order IDs and chooses the oldest pending goal; with none pending, it scans rules/head tuples in a fixed order. This is an experimental policy permitted by the unordered semantics, not a language source-order guarantee.

Binding, condition operations, tuple scans and answer comparisons must yield through worklists. An instrumented node visit is a logical scheduling unit, not an assertion that Python hashing, allocation or operating-system scheduling takes constant wall time. With equality-only operations and finite current data, each such primitive finishes; measured service durations must be reported. A real-time latency guarantee is outside scope.

## Quiescence and answers

For an alternative's fixed logical version, certify no pending goals, consistency, and complete coverage of possible unused rule/head tuples. An exhaustive finite scan supplies the control. An incremental index may replace it only with a demonstrated equivalent completeness invariant.

Reserve that branch version during certification or validate only changes relevant to it. An unrelated alternative's infinite updates cannot restart the scan. There must be no outstanding work capable of changing the certified state under the selected source policy. Suspended guards or constructor matches are not enabled rules; their residual constraints can remain in an answer.

Answer extraction and equality checking are queued jobs. Deduplication processes finite observations incrementally; it cannot wait for the whole search to end. A consumer that requests only a finite prefix can stop draining output by choice; fairness assumes a draining consumer when eventual reporting is claimed.

## Sufficient progress argument

Fix a committed transition policy. Assume every finite execution prefix has finite data and admitted work; jobs receive FIFO finite service; source transitions on stable snapshots require finite implementation work; relevant dependencies terminate; unrelated support does not delay publication; reservations do not starve; and finite-state certification/answer processing terminate.

A job has finitely many predecessors when admitted. Each predecessor receives finite service and finishes or moves behind it; later arrivals cannot overtake it indefinitely. The job therefore receives every finite number of services it needs. Apply this to a source transition's finite dependencies, then induct over a finite successful derivation, including child admission after OR, certification and answer extraction.

Consequently, every successful explicit alternative with a finite derivation under that policy is eventually reported, while memory remains available and output is drained. This does not promise success requiring a different committed CHR schedule, termination of infinite search, bounded latency, or progress from a nonterminating primitive.

## Required adverse cases

- `start <=> loop | answer(a)` with `loop <=> loop`: finite service must allow the answer despite the other branch.
- A common job whose support contains a completed B and divergent A: partial publication must expose B.
- A suspended consumer and queued producer with one worker: waiting cannot occupy the worker.
- B's stable quiescence scan while A mutates forever: only B-relevant versions can invalidate it.
- New work arriving indefinitely: it cannot enlarge an older batch or replace its queue age forever.
- An alternative repeatedly admitting fresh descendants while a finite sibling waits: descendants enter at the tail and cannot continually overtake the sibling.
- Two conflicting commit requests: fair reservations must admit an older finite request.
- Infinite solution enumeration with duplicates: finite recognized answers must pass through incremental deduplication.

These cases can refute an implementation's fairness claim. Passing finitely many cases does not prove the theorem premises for all programs. The implementation must also expose service/admission and reservation events for review.
