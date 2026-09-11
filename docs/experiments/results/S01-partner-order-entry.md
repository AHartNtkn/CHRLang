# Partner order: available keys avoid discovery, but selection matters

**Choosing the partner with an available key avoids substantial work in the qualified source.** With 128 rows, Indexed/Global execution takes seven candidate visits in the useful order and 515 in the opposite order. Switching which endpoint is bound reverses the useful order. Unrestricted reordering also changes which competing match consumes a resource, so a planner must preserve source selection or establish an explicit eligibility premise.

This is source and work evidence, not a timing or architecture ranking. The [portfolio review](S08-context-portfolio-review.md) selects this investigation over further context-predicate tuning because it can avoid larger discovery paths.

## Fixing the investigative source

The first source joins two uniquely linked tables and consumes a request. It is a useful neutral control: table activations already supply the linking key. At the larger successful endpoint, Indexed/Global takes 513 candidate visits in either order. Active takes 643 or 644. That source does not expose the intended larger order benefit.

We added a separately registered source with the request as the first kept head and a separate consumed token. Now the request key is available before table selection. Both orders still have the same single successful match, or the same complete residual answer on a miss. This source tests the intended mechanism without claiming that moving a consuming request into a kept head is a compiler transformation.

## Opposite useful orders

The table shows candidate visits at 128 rows, successful last-key query, forward insertion and indexed access. Counts cover the complete session, including unsuccessful activation work and exhaustion.

| Available endpoint | Policy | Left table first | Right table first |
|---|---|---:|---:|
| Left key | Global | **7** | 515 |
| Right key | Global | 515 | **7** |
| Left key | Active | **524** | 905 |
| Right key | Active | 905 | **524** |

In the bound-request source, using the available endpoint first lowers candidate visits in all 192 matched order pairs: all three sizes, both endpoint keys, success/miss, both insertion orders, two changing targets, both policies and both access methods. This count describes the registered cases, not a workload distribution.

Indexes and activation do not erase the distinction. Active prebinding gives a table occurrence useful information, but complete-query activation work can remain large. Global indexed selection benefits greatly when an earlier request binding restricts the first table. Individual candidate, structural, pool and index counts are retained separately; they are not interchangeable units or an elapsed-time estimate.

## Why a generic reorder is not yet qualified

The ambiguous sentinel has two possible joins and one consumed resource. Left facts occur in X0,X1 order while right facts occur in X1,X0 order. Under Global, left-first selects X0 and right-first selects X1, with both Scan and Indexed. Complete residual validation preserves the different receipt; it does not canonicalize the difference away.

The unique-match family demonstrates an economic opportunity. The sentinel demonstrates that arbitrary enumeration order is a semantic choice. Neither result requires a mandatory uniqueness restriction on the language. Possible implementations include preserving the original tuple priority while using a selective probe, or proving a specific reorder cannot change the selected match. Preparation, proof/checking and probe costs must count.

## Next implementation and competing investigations

T082 next qualifies a prepared selective-probe plan that retains original head identities and committed tuple order. Use both source families, the ambiguous sentinel and adverse probes with little selectivity. A selective lookup that merely changes the first successful tuple fails the gate. Then register complete lifecycle costs with changed queries, separate diagnostics and preparation/disposal accounting; compare the same plan before claiming a generation benefit.

This remains more immediately valuable than another context selector because the source now demonstrates avoidance of hundreds of candidate visits and has an independent complete-answer gate. Memo storage still needs fresh attribution; integrated admission, coarser reuse, solving and complete architectures remain required. Reconsider those alternatives at the prepared-plan gate or obstruction. This is package one after the full review; the goal remains active.

## Evidence

Each source family has 384 sessions per build, repeated twice with metrics and once with counters disabled: **2,304 independently checked complete sessions** in total. The 768 diagnostic rows repeat exactly. Each ruleset is reused across two endpoint queries. All original table facts, retained requests/tokens, missing outcomes and successful receipts are checked as complete ground multisets constructed independently of the engine. All **24 ambiguous sentinel executions** match their order-specific expectations.

The frozen builds use ordinary test execution for correctness and diagnostic work; they make no timing claim. Every session exhausts within the two-million-step bound. Scoped Clippy passes for both tests. The reference interpreter and production implementation are unchanged.

Registrations: [initial control](../registrations/S01-partner-order-entry.md), [bound-request witness](../registrations/S01-partner-order-bound.md). Audited evidence: [control](s01-partner-order-entry/analysis.json), [witness](s01-partner-order-bound/analysis.json). Reproduce with `python3 research/chr-compiled/experiments/audit_partner_order.py`; source archives, build flags and complete receipts accompany each analysis.
