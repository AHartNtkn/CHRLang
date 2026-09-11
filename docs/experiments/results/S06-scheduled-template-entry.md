# Prederived calls can retain source scheduling

**The scheduled-template experiment preserves the scarce-resource winner and avoids runtime matching on prederived calls.** A depth-eight chain takes nine source progress steps with both zero-follow and scheduled templates; runtime matcher entries fall from 25 to 3. This is work-count evidence, not a timing gain: matching also happens during derivation and must be charged.

## What changed

The `scheduled-templates` feature records each followed call as a prederived body with its source name and arguments. Runtime expansion creates an ordinary call obligation. Its body is instantiated only when that call is serviced, and it returns the usual progress event. Choices and resource posts inside the body therefore appear at that source step rather than when the whole derivation is first instantiated.

The implementation uses the existing call queue and result-validity rules. A query-owned table associates call IDs with shared prederived bodies. Consuming calls remain ordinary; a prederived body does not authorize a resource claim. The feature-off implementation and zero-follow mode remain experimental controls.

The [registration](../registrations/S06-scheduled-template-entry.md) preceded implementation. The scarce-token assertion first failed with the scalar short-caller winner missing, then passed with scheduled bodies. Expected scalar answers were unchanged.

## Validation

| Gate | Evidence |
|---|---|
| Complete stream answers | 72 queries, 432 executor searches pass with scheduled templates; repeated/distinct calls, aliases, failure, consumption, depth and insertion controls |
| Source suite | All 32 tests pass, covering unknowns, choices, competing consumers, off-output failure and finite service beside divergence |
| Added resource challenge | 24 arrangements: four caller-depth pairs × zero/one/two tokens × insertion reversal; zero-follow and scheduled modes both match scalar answers after partial-run disposal and restart |
| Service and work | Depth-eight chain: `(progress, runtime matcher entries)` is `(9,25)` for zero-follow and `(9,3)` for scheduled templates |
| Derivation bounds and sharing | Exact scheduled-chain lengths checked at size/fuel boundaries; 17 demand unit tests pass |
| Cache interaction | Source gates pass with completed traversal, seeking contexts and answer validity; 20 demand unit tests pass in that combination |
| Feature-off control | 16 demand unit tests pass; scoped Clippy passes for scheduled demand and source tests |

Three separately registered native measurement tests are outside the unit invocation. No unit-test duration is used as performance evidence.

## What this means for architecture

Avoiding runtime pattern matching does not require collapsing source service steps. The prototype establishes a concrete way to preserve those steps, including the resource-order counterexample that ordinary contraction fails.

That preservation has a structural cost. The existing two-request derivation witness retains 18 completed `build` call records with scheduling, versus two under contraction. The scheduled-body table and shared plan wrappers also occupy memory. These are live implementation costs, not bookkeeping that can be omitted from the next comparison. Complete allocation and time may outweigh the matcher reduction.

T073 next measures this tradeoff against zero-follow templates, ordinary dependency execution and the existing explicit carrier control. Register preparation/derivation, changing-query reuse, execution, answer ownership, cancellation and all disposal before comparative runs. Include deterministic chains as well as choice-bearing sources so choice work is not attributed to deterministic contraction. Repair or add fixtures for every missing boundary.

This remains more useful now than coarser key tuning: the correspondence gate has produced a candidate with a specific saved operation and a specific ownership cost. Direct solving stays a required competitor for eliminating whole operations; it does not make source scheduling free on resource-sensitive programs. Reassess all directions after the lifecycle package, at the required full portfolio review. Package count three; the research goal remains active.

Reproduce:

```
cargo test -p chr-direct-conditional --features scheduled-template-entry --test suspended_source
cargo test -p chr-reuse --no-default-features --features carrier-contraction,chr-direct-choice/scheduled-templates --test matched_carrier
cargo test -p chr-direct-choice --features scheduled-templates,work-diagnostics --lib
cargo test -p chr-direct-choice --features scheduled-templates,work-diagnostics,completed-traversal,seek-context,answer-validity --lib
```

The source suite also runs with `scheduled-template-entry,chr-direct-choice/completed-traversal,chr-direct-choice/seek-context,chr-direct-choice/answer-validity`. The added resource challenge passes in both configurations. Source tests and their implementation commit are the correspondence evidence; comparative cost receipts remain the next experiment.
