# Demand lifetime now has a source contract and checked join kernels

Demand-driven retention can avoid maintaining joins for inactive requests and avoid rediscovering them while a demand stays live. The new kernels demonstrate both mechanisms and agree with an independent tuple oracle. Their full source integration and runtime tradeoffs are still unmeasured.

## The architectural contrast

The [subscription entry](S01-subscription-entry.md) asks which demands justify retaining work. This gate gives that distinction executable meaning: a demand selects a left key and a right value across `left(key, f(a)), middle(a, g(b)), right(b, h(value))`. Explicit request pulses observe matches while the demand is live. Opening, retiring and reopening a demand are separate source actions; updates may occur between pulses.

The three kernel policies share occurrence storage and exact indexes. Indexed discovery starts from the smaller left-key or right-value bucket, follows the middle relation and probes the other endpoint by its complete key. Eager retention maintains all three-row matches, regardless of demands. Subscription retention maintains those matches only for currently demanded endpoint pairs, sharing maintenance among duplicate demands while preserving their distinct identities.

These three-row tuples are partial matches of the complete source rule, which also requires a demand and a pulse. This gate compares demand ownership of that same retained stage. Alternative choices of earlier intermediate stages remain distinct work; one fixed join stage does not settle them all.

## Source behavior is independently checked

The source has explicit open, close, request, insert/remove and equality instructions. Rule priority drains each pulse before advancing the driver. A second variant consumes the right occurrence. The accepted behavior includes duplicate rows and duplicate demands, unchanged-demand repeated requests, retirement/reopening, updates to every data relation, structural unknowns, alias binding and blocked resource updates.

Six source tests exercise 44 configurations. Hand-stated receipt expectations check the important effects; two cases additionally state the entire residual answer. The existing independent owned-syntax evaluator and both compiled Global Scan/Indexed controls agree on complete answers. The reference interpreter is unchanged.

A duplicate demand is a separate source occurrence: two identical live demands produce two receipts per matching triple. Retiring one leaves the other live. Repeating a pulse allows the same retained data tuple to produce a new receipt, while propagation history prevents repeats within a pulse. In the consuming variant, a right occurrence can contribute only once. A missing removal blocks the remaining driver script rather than silently advancing.

## The join owner has separate correctness evidence

A declarative Cartesian-product oracle enumerates matching occurrence triples without using candidate indexes, subscription registrations or incidence lists. Thirty-two deterministic update sequences per policy check three demanded endpoint pairs after each of 72 insertions/removals. Additional checks retire, share and reopen demands during those sequences. All three policies must agree exactly on ordered occurrence triples.

Directed checks establish the proposed mechanism. Eight occurrences in each relation form 512 matching triples: eager retention holds them before any demand, while subscriptions hold none. Opening a demand enables retention; three requests perform no further join discovery under either retained policy. Last-demand retirement releases subscription tuples. Indexed discovery retains none and repeats discovery when requested.

The selective-control check has 128 left rows and one matching final row. Its request traverses one right endpoint and one middle row, with an exact final probe, rather than scanning the broad left side. This prevents the retention comparison from relying on a control that ignores available selectivity.

Three deliberate faults test whether the oracle and lifetime checks can detect unsound maintenance: keeping tuples after last-demand retirement, collapsing equal-valued occurrence combinations and omitting invalidation on row removal. Their release-mode failures and the restored implementation's successful run are recorded with exact substitutions.

## Responsibility and cost obligations

Each retained tuple has reverse incidence in all three data relations. Row removal invalidates incident tuples before removing the row. Subscription keys also own demand registrations and endpoint indexes; only the last subscriber releases a shared tuple set. Indexed and eager modes do not maintain these subscription-only indexes.

Insertion is anchored at the new occurrence. It does not rebuild existing tuples. Left/right updates use active endpoint indexes; middle updates currently examine all active demand keys and then use exact endpoint probes. Broad demand populations can therefore impose real subscription overhead, even when few tuples change. That dimension belongs in the eventual cost matrix.

The kernel receives resolved source projections. It does not yet implement source equality, structural eligibility, pulse history, source-order consumption or complete answer publication. Source and kernel gates are separate evidence; passing both does not establish correspondence between them. The next implementation must connect them and repeat the independent full-answer checks, including adverse validity and resource cases.

Requests currently return an owned vector of matching triples. A lifecycle comparison must charge that materialization and investigate it if it obscures discovery or publication costs. No timing rank, necessary-complexity winner or language restriction follows from these kernel checks.

## Next work and evidence

T068 remains active. Integrate the kernels with the exact source contract, preserve demand and occurrence identity across bindings and consumption, and establish complete-answer/cancellation checks. Then register independent demand-frequency, update-density, fanout and selectivity contrasts before comparative timing. Preparation, indexes, retained tuples, invalidation, observation and complete disposal all participate.

The source and kernel tests are [subscription_source_gate.rs](../../../research/chr-compiled/tests/subscription_source_gate.rs) and [subscription_join_gate.rs](../../../research/chr-compiled/tests/subscription_join_gate.rs). The [validation runner](../../../research/chr-compiled/experiments/subscription_gate.py) records release metrics-on/off checks, Clippy, deliberate fault failures, restoration and [source hashes](s01-subscription-kernel/source-hashes.json). Its per-command receipts live in [s01-subscription-kernel](s01-subscription-kernel).
