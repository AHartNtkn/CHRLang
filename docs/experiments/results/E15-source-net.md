# E15 source/net equality composition gate

The resumable source/net boundary passes all 432 independent Rust reference
requests at quanta 1,8,64: 1,296 configurations and exact replay. There are 492
successful configurations (164 distinct requests) and 804 failures. The largest
request takes 1,305 charged actions. Original substitution preservation is checked
on both outcomes; variable IDs remain within the allocated source domain.

The composed source adapter additionally passes all 64 E00 cases at quantum 8,
including their registered nonexhausted prefixes, with exact replay. Every continued
and forked state is checked against the independent tree control. Final observations,
raw multiplicities and exhaustion/prefix contracts agree with hand expectations and
the Rust reference. The independent direct source gate still replays its 192-row
baseline exactly. No reference implementation imports the new bridge.

## Boundary and correspondence

`net_bridge.py` prepares a fixed name table and the existing finite net rule system
once. Each request encodes the complete immutable substitution and equation list,
yields through net construction/reduction/status checking/readback, decodes both
outputs, validates the original table, then returns a complete substitution or
failure. It does not publish partial bindings. Constructor arities and source IDs
survive encoding; unknown constructor names are rejected. Compilation/name-table
preparation is explicit and outside per-request service counts.

The source adapter chooses direct or supplied net equality explicitly. The net
implements only equality; source matching, guards, occurrence/history updates,
explicit OR and completion remain in the host adapter. This is a hybrid service
composition, not a complete net-compiled CHR engine.

The net and direct unifiers can choose different representatives for the same
variable class. The state checker therefore compares exact occurrence identities,
history and fresh counters, plus joint-alpha projections of every allocated
variable, selected output, stored term and pending goal operand. Sequence wrappers
preserve positions and lengths. It separately rejects unallocated variable IDs,
including in substitution entries, so alpha equivalence cannot hide an invented
handle that might collide with later fresh allocation. Raw substitution ordering
is not treated as the source semantics.

## Work exposed by composition

The larger SK cases demonstrate why interface costs must remain visible:

| Source case | Source steps | Total actions | Codec actions | Net actions |
|---|---:|---:|---:|---:|
| SK duplication | 431 | 3,868,948 | 497,944 | 3,324,822 |
| SK ignored hole | 546 | 6,300,045 | 795,229 | 5,430,962 |

For ignored-hole evaluation, net status scans alone take 2,987,678 actions, compared
with 1,298,186 reduction actions, 781,948 readback actions and 362,984 build actions.
For duplication, those counts are 1,819,496, 787,478, 488,524 and 229,185 respectively.
These include actual service phases, not an assumed cost per source equation.
They are work counts, not timing ratios against a native backend. The total may
exceed two million while every individual source step meets its two-million-action
cap; the gate bounds each step separately.

## Next and limits

The next experiment injects this prepared service into the policy scheduler and
checks branch-local failure, source/observer isolation and complete answers across
policies. Register run-level resource bounds using the observed source costs before
comparative runs. Alias orientation may affect exact-state grouping opportunities;
separate that representation effect from service work. Compare interface preparation,
encoding, decoding, status scanning and observation alongside reduction.

The prominent status-scan cost motivates maintained controller-status metadata as
an E06 refinement, but does not block scheduler composition. Routed/shared binding
stores, richer net protocols, native choice provenance, heterogeneous operation
sharing and all other E15 pairings remain open. The current service's high work
count does not close those alternatives or adopt a production architecture.

Evidence: [boundary results](E15-bridge.jsonl), [boundary replay](E15-bridge-replay.jsonl),
[source results](E15-source-net.jsonl), [source replay](E15-source-net-replay.jsonl),
[source hashes and host](E15-source-net-manifest.json).

```sh
cargo run -q -p chr-cases --example net_unification_cases > /tmp/e15-requests.jsonl
PYTHONDONTWRITEBYTECODE=1 python research/chr-scheduling/check_net_bridge.py < /tmp/e15-requests.jsonl
cargo run -q -p chr-symbolic-fixtures --example export_cases > /tmp/e15-cases.jsonl
PYTHONDONTWRITEBYTECODE=1 python research/chr-scheduling/check_net_source.py < /tmp/e15-cases.jsonl
```
