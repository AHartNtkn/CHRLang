# Complete costs of reunion after specialization and elimination

This prospective exploratory pilot asks whether checked temporary reunion pays for its machinery once available specialization and finite countdown elimination count. T077 remains active. The strongest distinct ready alternative is T072 integrated dependency repair; reassess that comparison immediately after this bounded pilot. Six T077 packages have completed, with the breadth review after package four. This package is justified because all competing paths are now qualified and the missing costs could reverse the consequence of the source-work comparison.

## Hypotheses and decision

- **H1:** Reunion can reduce complete cost on substantial private work, but preparation, product construction and observation can outweigh that benefit on short or payload-heavy work.
- **H2:** Checked countdown elimination can make simpler coupled execution preferable even where original reunion saves source steps. Compare shortened reunion against shortened Copy and specialized Scan/Indexed; a combination must justify its additional machinery.
- **H3:** Preparation reuse and query/consumer ownership change the time–memory tradeoff. Allocation traffic, peak live demand and time need separate conclusions.

A gain requires adverse/lifetime follow-up; a consequential avoidable loss requires attribution/repair. Overlap is unresolved. A process cutoff is unfinished evidence and requires diagnosis. No workload weights, universal ranking or complete compilation-lifecycle claim follows.

## Fixed sources and controls

Use `examples/support/reunion_source.rs` unchanged: plain, equal, late and payload families; owners 2/4; base depths 0/12/48; reuse 1/4. Query seed is its zero-based reuse index; effective depth is base plus seed modulo two. This gives 48 source/lifetime cells. Keep complete residual payloads and raw source alternatives. Equal yields two answers, other families yield 2^owners on these sources.

Twelve modes: Copy, reunion, generic Scan, generic Indexed, specialized Scan, specialized Indexed, shortened Copy, shortened reunion, shortened specialized Scan, shortened specialized Indexed, permanent factoring, and generic Indexed with arena COW. All use Global source selection where configurable. Specialization is inferred from prepared rules. The checked shortcut verifies the exact finite source family and each query; it does not generalize to unknown tails or arbitrary programs. Verified source is handed to backend preparation; only the query checker is retained alongside the backend. Transient preparation copies are charged.

COW runs in separate feature builds and only for generic Indexed. Factoring uses its existing API, which prepares source again during query setup; report that implementation responsibility explicitly. These measured sources have distinct raw answers; this does not equate unique-answer publication with raw semantics in general.

## Measurement and validation

Runner: `research/chr-restoration/examples/reunion_complete_cost.rs`. Driver: `research/chr-restoration/experiments/reunion_complete.py`.

Every process constructs source rules and independent scalar expected answers outside measurement, warms one prepared object with every changed query, checks full raw answers, then disposes the warm object. Measured preparation is fresh. Complete raw answers are validated outside measured phases. Source-rule generation and scalar oracle cost are excluded. No Rust compilation isolation is attempted; build receipts establish reproducibility only.

Phases: prepare once; per query input construction, eligibility/lowering, setup, execution including complete observation and exhaustion, engine disposal, answer disposal, lowered-input disposal and original-input disposal. Every mode uses the same receipt/answer collection harness. Nonshortened lowering returns no query; its empty phase is still timed. All answers are retained until that query ends. First observation is recorded within execution; it is a latency endpoint, not another additive cost. Backend observation and execution are inseparable here.

After full queries, independently run seed-zero input/lowering/setup to its first answer, validate membership, then cancel by disposing engine, answer and inputs. Finally dispose prepared state. Cancellation is a separate consumer scenario; exclude its phases from the complete-query sum. Prepared disposal occurs after cancellation in this harness; report that order. Full-query total is prepare plus all full-query phases plus prepared disposal. First-answer end-to-end cost can be reconstructed from preparation/input/lowering/setup and first latency; do not add it again to execution.

Use release builds with ordinary allocator and engine/kernel/work counters disabled for primary timing. Separate alloc-meter builds use the same sources. Diagnostic elapsed time supplies no speed evidence. Meter readings are requested allocation calls/bytes, live demand and peak; they are not RSS. Each allocation run must restore its prepared baseline after every query and cancellation, then its initial baseline after final disposal. Consecutive phases must have continuous live-byte endpoints; per-cell diagnostic pairs must match every memory field exactly.

## Repetitions, freeze and bounds

48 cells × 12 modes × (five ordinary + two allocation repetitions) = **4,032 processes**: 2,880 ordinary and 1,152 allocation. Run allocation blocks first; audit both repetitions before ordinary sampling. Each block contains every cell once, shuffled with `random.Random(20260911)` in allocation-then-ordinary order. Serial child processes use the minimum available CPU affinity ID. Each child has 60-second wall and 1-GiB address-space limits, and each execution has a 200,000-service limit. No concurrent experimental processes. Failed receipts stop the driver; do not retry by overwriting evidence. The overall planned cap is 4,032 successful processes; resource failures trigger selection/diagnosis rather than silently changing the matrix.

Freeze registration, runner and transitive Rust sources/manifests, Cargo.lock, driver, binaries, toolchain, CPU affinity and current source revision before sampling. Save build and feature-tree receipts. No metrics, kernel-metrics, compiled-work or replay-diagnostic feature may be active. Separate build directories preserve historical experiment binaries. Resume only exact successful commands under unchanged source and binary hashes.

## Interpretation

Report per-cell medians and full observed ranges of ordinary phase totals and first latency; show preparation, full-query and cancellation costs separately. Five samples are exploratory: no significance, practical-win count or architecture ranking is assigned from separated ranges. Compare requested traffic and peak demand independently, with identical diagnostic pairs. Do not average across families or invent workload weights.

Prioritize inspection of four-owner/four-query depth-zero and depth-48 regimes for all families, plus two-owner/one-query short cases. Explain any reversal when shortening is available and whether it comes from preparation, execution, observation or ownership. Investigate consequential defects before carrying a conclusion into the decision map. If an additional cost comparison cannot change the bounded decision, advance to T072; broader reunion, inference, sustained lifetime, coherent architectures and held-out challenges remain required.
