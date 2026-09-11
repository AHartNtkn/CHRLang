# Call-reuse lifecycle pilot

This exploratory pilot asks whether saved call execution repays recognition and replay over a complete finite session. It cannot select a universal architecture or establish compilation economics. Registration precedes comparative runs.

## Hypotheses and competing explanations

- Separated memoization can save substantive repeated work across inert distinct callers; its full time may improve despite higher requested allocation.
- Whole-state and compact recognition may miss useful equivalence or incur costs that grow with retained residuals. The known whole-key capacity failure remains an unfinished configuration.
- Prepared/inferred and activation controls may eliminate enough ordinary work to change reuse's competitiveness. Compare actual complete paths; do not attribute their entire difference to caching.
- Caller relevance, depth, preparation reuse and answer retention can change the time–memory tradeoff. No workload weights or overall average ranking will be used.

## Frozen matrix

Ten modes: direct, whole, compact, separate, memo, scan, indexed, sealed, active-scan, active-indexed. The first five are Direct, AlphaLive, CompactLive, separated uncached and separated memo. The remaining modes are Global Scan/Indexed, Global inferred Scan and Active Scan/Indexed. Unsupported Active inferred execution is excluded by its existing explicit qualification result, not assigned a cost.

Six existing source families: inert caller, readable caller, late-bound caller, other-arity reader, duplicate residuals and early failure. Depths 0, 4, 32, 128; identical/distinct callers; one/four changing queries sharing prepared rules; releasing/window-one/retained-all consumers. All primary sessions run to exhaustion. This is 288 scenarios × 10 modes = 2,880 cells. Each query has two answers except the early-failure family, which has one; raw multiplicity and FIFO correspondence are checked independently.

Five ordinary repetitions per cell, two separate allocation repetitions. Each process performs complete candidate warmup/preflight outside measurement, then one measured session. No external warmup is added. Mode order is shuffled within each scenario/repetition with seed 750511; scenario order is shuffled each repetition. All allocation blocks precede ordinary timing blocks. No concurrent benchmark processes. A failed cell is retried once in its next repetition; after two failures in the same build, remaining repetitions are explicitly skipped. Preserve every failure and skipped job; do not substitute a completed cost or a successful sample from another cell.

## Measurement and controls

Use release builds with default features disabled. Ordinary timing has neither allocation metering, stage profiling nor engine/kernel metrics. Allocation build enables only alloc-meter. Check Cargo artifact features. Freeze source archive, exact job order, build logs and binary hashes before running. Calibrate 10,000 empty intervals in each of three ordinary processes.

Charge source construction, preparation (including its source clone), source disposal, each query's input/setup (including query clone), service through owned observation, consumer action, engine/input disposal, prepared disposal and consumer disposal. Reuse preparation across query identities. First observation is the first service_observe interval; execution and owned observation are inseparable here. Total is the sum of these intervals. Report phases rather than treating process elapsed as engine time.

Independent scalar complete answers and Direct FIFO answers are established outside measurement. Candidate complete preflight and validation of every measured answer are also outside intervals. Held answers are checked after producer disposal. Allocation checks require final restoration and releasing-query restoration to prepared ownership. Per-phase requested allocation and peaks describe requested heap, not RSS. Validation may affect caches even though excluded from time. Recording capacity, oracle storage, process startup, warmup and native/user-program compilation are excluded. This is finite-session lifecycle evidence, not sustained service or complete compilation lifecycle evidence.

Cancellation is covered by the existing lifecycle unit matrix: all six families, ten modes, three consumers, depth four, four queries, cancellation after first answer and full completion. It is a correctness/lifetime check, not a cancellation timing comparison.

## Bounds and analysis

Pin each child to CPU 0 after verifying availability. Limit each child to 60 seconds wall time, 60 seconds CPU, 1 GiB address space and the runner's 2,000,000 service calls per query. Campaign bound 30 minutes, including subprocess launches but excluding builds. Stop and preserve partial data if the campaign bound is reached. Do not restart an unfinished campaign under another name.

For each successful cell report median full session, preparation, first observation and execution/observation; requested bytes, phase peak excess and consumer bytes separately. Allocation repeats must match exactly after subtracting the initial live baseline. Compare median paired ratios to Direct and the best measured applicable nonmemo control within each scenario; retain all individual controls. With five exploratory repetitions, these are sizing results, not confidence claims or a frozen selection policy. Flag any full-session sample below 100 times the maximum empty-interval median times its number of measured intervals. Do not rank signal-limited results as reliable gains.

Audit every raw outcome against the frozen job, exact expected answer counts, phase structure, meter restoration and feature mode. A cutoff, validation failure or inconsistent allocation repeat triggers investigation before architectural interpretation. Follow consequential favorable or adverse differences through phase attribution and qualified confirmation; select that follow-through against integrated eligibility, demand repair, broader recognition and complete paths at the first result boundary. This package is the first after the call-key allocation portfolio review.
