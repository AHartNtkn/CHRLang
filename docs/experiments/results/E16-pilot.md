# E16 resource pilot

All 116 isolated children pass their observation and lifecycle checks: 58 ordinary
System timing runs and 58 separately metered runs. The sizes are feasible under the
registered bounds. The pilot also exposes a workload limitation worth fixing before
a comparative matrix: repeated subtrees create substantial representation work while
requiring relatively few solver pair steps.

The [registration](../registrations/E16-pilot.md), [raw records](E16-pilot.jsonl),
[manifest](E16-pilot-manifest.json) and [audit](E16-pilot-audit.json) retain each cell,
source/binary hashes, resource settings and measurements. All children exit successfully;
no timeout or memory-bound outcome occurs. Maximum process RSS is 10,368 KiB; the
maximum recorded Rust heap peak is 4,826,076 bytes, including the preexisting fixture
baseline. They measure different storage quantities.

## What the pilot establishes

Every configuration has the same committed source counters for its case. At matched
outstanding limits, the new drivers have identical complete solver work, including
speculation. Timing and allocation runs agree on complete work. The synthetic prefix
leaves four accepted requests; joined shutdown accounts for all four without advancing
source execution. The type-synthesis prefix leaves one. The single-chain control has
at most one outstanding request, while the wide cases reach four and SK duplication
reaches three. Outstanding requests alone do not show useful simultaneous computation;
the independent synchronization test supplies the actual overlap witness.

The balanced depth-10 wide case has 2,047-node operands but only 176 owned solver pair
steps across its sixteen equations (eight tags and eight tree equations). Repeated
holes and structurally equal subtrees let the solver avoid most recursive pairs after
binding. The owned path still resolves, clones and compares substantial term data.
Thus increasing this depth alone does not create a clean sweep of difficult unification.
The fresh-hole chain uses independent tasks, so earlier bindings do not simplify later
ones; its difference from the wide case is explicit branch availability and tag overhead.

Illustrative single-observation cold times, including construction, search, shutdown
and search release, are below. These exploratory values do not establish stable rankings.

| Case | Shared | Owned | Inline | One worker | Two workers |
|---|---:|---:|---:|---:|---:|
| Eight cheap alternatives | 0.054 ms | 0.065 ms | 0.066 ms | 0.315 ms | 0.259 ms |
| Eight depth-10 alternatives | 2.976 ms | 8.647 ms | 8.361 ms | 14.475 ms | 11.849 ms |
| Eight depth-10 sequential tasks | 4.815 ms | 6.966 ms | 7.350 ms | 13.803 ms | 15.536 ms |
| SK duplication evaluation | 0.337 ms | 0.891 ms | 1.087 ms | 2.334 ms | 3.168 ms |

The wide depth-10 metered peak is 3,310,244 bytes for Shared, 3,925,179 for Owned,
3,928,055 inline, 4,545,139 with one worker and 4,826,076 with two. These absolute
heap-accounting peaks include the prepared fixture, and worker scheduling can affect
retention. The application setup constructs the E00 registry before selection, so its
process RSS is not an isolated search peak. Metered timings do not support scaling
claims because the allocator updates shared atomic counters.

## Consequences for the next experiment

Retain the repeated-subtree and identity cases: they expose the real advantage of
shared representation and are useful contrary evidence to an owned-worker proposal.
Add balanced terms with distinct holes to increase substitution and solver work; use
matched single-chain controls and measure installation as part of complete execution.
Pilot those sizes before freezing repeated comparisons. A result favoring workers only
over Owned would still need comparison with Shared before supporting project adoption.

Coarse certified-region workers, warm worker reuse, less costly representation transfer,
and resumable services remain independent feasible questions. This pilot closes none
of them. It does not establish a general fairness bound, a production worker count,
or a language restriction.

## Validation and reproduction

Two workload tests check independent reference observations and actual accepted prefix
work. The harness passes workspace Clippy with warnings denied and formatting checks.
Its numeric snapshots are allocation-free; allocation readings occur only before
workers start or after they join. Semantic validation happens after search release and
before a separately measured output drop.

```sh
cargo test -p chr-reuse --test parallel_workloads
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
cargo build --release -p chr-reuse --example parallel_cost --example parallel_memory
PYTHONDONTWRITEBYTECODE=1 python3 research/chr-reuse/scripts/parallel_pilot.py
PYTHONDONTWRITEBYTECODE=1 python3 research/chr-reuse/scripts/audit_parallel_pilot.py
```

Run the pilot without concurrent experimental loads. The runner preserves malformed
output and errors, terminates the child process group on timeout, and checks frozen
hashes after execution. The audit checks all 116 unique cells, source/work agreement,
shutdown accounting and quiescent allocation continuity.
