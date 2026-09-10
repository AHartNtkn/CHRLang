# Failed-check backoff preserves source behavior and reduces some separation work

Bounded backoff can avoid repeated failed independence checks without changing complete answers in the tested sources. More aggressive backoff can also change later separation opportunities and increase source work. This establishes an adaptive mechanism, not a runtime or memory advantage.

All180 policy/source configurations pass independently in both builds. Finite answers progress beside a continuing branch, all-failing alternatives exhaust, and cancellation releases prepared ownership before reused queries. The full package regressions pass:30 tests without diagnostics and32 with diagnostics. Both scoped Clippy checks pass.

## What the policy changes

Repeated reunion normally attempts to divide a state into independent owners at every completed source-body boundary. The new query-wide policy can skip that attempt and continue ordinary source execution. A skipped check is never treated as evidence of independence.

The experiment compares five schedules: every boundary; fixed skips of one/eight boundaries after each attempt; and failed-check backoff capped at one/eight skipped boundaries. Backoff starts with a check, skips1,2,4,… up to the cap after consecutive failures, and resets on success. It skips nothing after a successful attempt. Sibling outcomes affect the query-wide schedule. These are explicit policy choices; there is no source-family recognition, prediction of future outcomes or clock-based tuning.

Scheduling fields exist in both builds because they determine behavior. Attempt/work diagnostics compile separately. The existing `start_repeated` remains the every-boundary control; `start_repeated_with_policy` selects the experimental schedule. The FIFO job queues, source rules, resource ownership and independent evaluator are unchanged.

## Complete behavior and progress

The registered matrix combines plain, inherited-history and late-link sources with zero/one/three additional rounds, private depth zero/four and two query identity seeds. Each of36 source/query cases runs all five policies and agrees with independent scalar full raw answers, Copy and initial-phase reunion. The checks include output aliases, fresh identities, consumed occurrences, duplicate derivations and inherited propagation history.

A separate source has a finite private alternative beside a continuing private branch. Every policy delivers the independently specified finite answer within20,000 service calls. Both arms of a failing private choice exhaust. Cancellation at service positions0,1,5 and20 releases all prepared-engine references, followed by successful reuse of the same preparation. Final release also destroys the prepared owner. These checks establish bounded progress and owner release, not constant-time service, a general fairness theorem or allocation restoration.

Skipping separation remains semantically plausible because ordinary execution is itself a valid path for that state under the admitted source contract. The tests challenge that argument with later joins and history; they do not extend the existing ownership certificate to arbitrary CHR sources. Successful private decomposition still uses the existing certificate and transport implementation.

## Fewer checks do not necessarily mean less total work

The table uses three additional rounds and private depth four. Source steps are private plus coupled `State::step` calls; they exclude independence checking, product construction, transport and policy overhead. They are diagnostic work counts, not time.

| Family and policy | Independence checks | Skipped checks | Source steps | Separation epochs |
|---|---:|---:|---:|---:|
| Plain, every boundary | 31 | 0 | 1,184 | 15 |
| Plain, backoff cap1 | 23 | 8 | 1,184 | 15 |
| Plain, backoff cap8 | 19 | 12 | 1,184 | 15 |
| History, every boundary | 31 | 0 | 1,344 | 31 |
| History, either backoff | 31 | 0 | 1,344 | 31 |
| Late links, every boundary | 199 | 0 | 1,478 | 29 |
| Late links, fixed skip1 | 128 | 127 | 1,490 | 42 |
| Late links, fixed skip8 | 48 | 378 | 1,459 | 27 |
| Late links, backoff cap1 | 114 | 85 | 1,478 | 29 |
| Late links, backoff cap8 | 69 | 152 | 1,488 | 39 |

Capped-one backoff avoids85 checks on late links without changing the recorded source work or epoch count. Capped-eight backoff avoids more checks but executes ten more source steps and creates ten more epochs. Fixed skip8 has fewer checks and source steps here, but that alone cannot establish lower complete costs: different decomposition can alter copying, products, retained states and output delivery.

The history case exposes the policy's limit. Every independence check succeeds, so failed-check backoff behaves like eager checking and adds scheduling responsibility. It cannot avoid eligible but economically unhelpful separation. At depth zero, capped-eight backoff on late links increases source steps from774 to791 despite reducing checks from87 to63. Both favorable and adverse work effects therefore occur within the registered matrix.

## Architectural consequence

Retain failed-check adaptation as a qualified candidate for complete-cost comparison. Do not select a cap or claim adaptation is superior to fixed scheduling. Its mandatory bookkeeping, missed opportunities, allocation, retained ownership, first/full observation and disposal still require measurement. The cost comparison must retain the fixed schedules as serious controls. The every-boundary path now carries the scheduling fields, so archived eager allocation measurements do not describe this build. Cost qualification must also account for the added overhead against a competent eager implementation without adaptive scheduling.

This package answers attempt scheduling after failed eligibility checks. It leaves successful-but-unprofitable separation, delayed choice splitting, checkpoint/replay policies and wider restoration regimes unresolved. The same distinction matters for implementation complexity: this policy adds three small scheduling fields and decision branches, but does not eliminate the independence checker, ordinary executor or reunion machinery.

The [four-package breadth review](S04-adaptive-breadth-review.md) selects sustained lifetime next. Adaptive costs remain required and will be reconsidered at the lifetime source/ownership gate. The research goal remains active.

## Evidence and limits

- [Registration](../registrations/S04-adaptive-entry.md), [implementation](../../../research/chr-restoration/src/reunion.rs), [independent source/progress tests](../../../research/chr-restoration/tests/adaptive_reunion.rs).
- [Primary checks](s04-adaptive-entry/checked-primary.log), [diagnostic checks and all30 work rows](s04-adaptive-entry/checked-diagnostic.log), [machine-readable audit](s04-adaptive-entry/audit.json).
- [Primary regressions](s04-adaptive-entry/regressions-primary.log), [diagnostic regressions](s04-adaptive-entry/regressions-diagnostic.log), [primary Clippy](s04-adaptive-entry/clippy-primary.log), [diagnostic Clippy](s04-adaptive-entry/clippy-diagnostic.log).

The raw directory preserves the initial missing-API test failure and intermediate runs. The audit freezes the final validated sources and receipts after the semantic gate; it is not a pre-run timing manifest. No finite service cutoff or command timeout occurred. This package ran no allocation or comparative timing matrix and makes no solver, compiler or whole-architecture performance claim.
