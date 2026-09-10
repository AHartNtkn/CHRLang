# Rust lifecycle runner: cancellation and prepared reuse qualify

The Rust measurement runner preserves complete answers across all 13 configured execution paths and survives cancellation followed by reused preparation. This qualifies the runner's observable behavior and named timing boundaries. It does not establish an architectural cost ordering.

## What the checks establish

The fresh corpus replay covers 338 processes: 2,286 admitted query/configuration checks pass and 587 remain explicitly unsupported. Complete output bytes match both the initial lifecycle runner and the previously qualified batch serializer. Independent source expectations check joint unknowns, residual multiplicity and exhaustion; independently constructed dictionaries check source/query symbols.

The initial cancellation checks cover 39 bounded queries and 13 complete controls. The stronger challenge selects each mode's complete query with the most service calls, breaking ties by corpus order. It runs budgets 0, 1, 64, halfway and 200,000 both independently and under one reused preparation. All 65 reused queries match their 65 standalone controls; each halfway query interrupts unfinished work, and all 13 subsequent full queries complete. Every consumer remains owned until after source and prepared-state disposal.

Three wire tests and nine checked finite-read tests pass under both default and metrics-off configurations. Scoped strict Clippy and formatting pass. The audit checks all corpus positions, complete outputs, dictionaries, phase arithmetic, zero-service cancellation, selection criteria and same-preparation results. These checks preserve the existing independent reference; this runner reuses the qualified parser and execution types rather than introducing an engine.

## What is measured

| Interval | Charged work |
|---|---|
| Input and decoding | Read the source protocol; construct rule and query syntax; dispose of the input buffer |
| Preparation | Prepare the selected engine and source dictionaries |
| Consumer setup | Reserve retained-consumer slots |
| Query setup | Extend dictionaries, construct query execution state, prepare any new prefix artifact and dispose of copied query input |
| Service | Execute the engine, construct typed observations and append each complete answer to owned bytes |
| Serialization within service | Append binary answer bytes and dispose of the typed answer; this interval is nested, not added twice |
| First observation | Time from service entry until the first complete answer has been appended |
| Query disposal | Drop pending engine work, finite-machine state, caller and remaining solutions |
| Session disposal | Drop the query input container, preparation, artifacts, source dictionaries/rules and finally retained consumers |

Finite solving advances a resumable machine and then runs the full original caller. One explicit service allowance covers solving, solution transport and caller work. Dynamic caller creation and disposal during execution remain inside service. Cancelling before service performs zero calls and produces neither an answer nor an exhaustion claim.

`compute_observe_ns` is service minus serialization. It includes typed observation and is not pure reduction time. `lifecycle_ns` sums the named disjoint intervals; it is not process elapsed time. Command-line experiment controls, logging, output transport and orchestration between intervals are outside it. Source/query syntax is decoded upfront, unlike the native numeric query protocol. Those boundaries must be reconciled before a cross-architecture comparison.

First observation here means complete owned bytes available inside the runner. External stdout transport occurs after prepared disposal and is not a measured streaming consumer. Publication/backpressure studies therefore remain separate work.

## Counter settings and remaining accounting

Release observations report compiled, observer and conditional metrics disabled. The recorded Cargo feature tree supplies the dependency configuration. Semantic identities and service/resource-limit bookkeeping remain operational: counter-free timing does not mean removing counters needed to execute the program correctly. Primary execution uses the ordinary allocator; requested-allocation diagnostics are not supplied by these runs.

Native host source emission and dictionary ownership still require lifecycle accounting. Allocation diagnostics must distinguish requested traffic, live retention, allocator or mapping scope and RSS. Clock overhead needs calibration before interpreting short intervals. Independent user-program compilation and artifact lifetime remain unmeasured. No comparative timing matrix or complete architectural lifecycle superiority is claimed.

## Decision and next investigation

Carry this Rust path forward as a qualified measurement candidate. Finish the native host/frontend and allocation boundaries, then calibrate measurement overhead. At full runner qualification, compare the value of one bounded mixed-source pilot with broader direct source analysis, as required by the [current sequence](../next-cycle.md). The latter remains a serious alternative because it could eliminate work that every current path still executes.

T078 and the research goal remain active. Broader native local ownership, constructors, source analysis, sustained lifetime and the other mapped mechanisms remain unresolved.

## Reproduction and evidence

The [prospective registration](../registrations/S10-rust-lifecycle.md) records the boundaries, corpus, cancellation extensions and resource limits. The first validation snapshot predates the final cancellation extension; the fresh validation hashes the current registration. Initial source and output snapshots remain available for the exact-output comparison.

Run the release example build with `cargo build -p chr-direct-conditional --no-default-features --release --example native_lifecycle`, then run `gate.py`, `reuse.py` and `audit.py` in [the qualification directory](../../../research/chr-hvm/rust_lifecycle/). Each child process is bounded at 15 seconds wall, 10 CPU seconds and 1 GiB address space; the complete service allowance is 200,000 calls.

The [raw evidence directory](s10-rust-lifecycle/) contains corpus and cancellation JSONL, retained reuse controls, initial snapshots, build/feature records, semantic test logs and Clippy output. `validation.json` freezes the fresh corpus and runner; `qualification.json` additionally freezes the audit, reuse controls and dependency source state. Timings in these correctness runs are diagnostic data only.
