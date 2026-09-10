# Matched observations release their owners, but symbolic transport retains identity history

All four finite-name paths return the same requested observations and restore initial live allocations after disposal. Symbolic transport retains identity records between queries and allocates much more than the prepared name solver on this slice. The result qualifies accounting and identifies costs; it supplies no runtime or general architecture winner.

## The comparison requests the same result

The paths are symbolic conjunctions, existing finite projection, the existing prepared name solver, and explicit enumeration with early rejection of partial exclusion conflicts. Every variable ranges over declared atomic names, so the normality requirement on `app(X,Y)` follows from the domain. The prepared name solver and explicit control can soundly omit that redundant structural check.

Sources cross no exclusions, hidden-star exclusions and a full exclusion clique with three/six variables and two/three names. Query streams contain one, 16 or 128 changing callers. Membership fixes X and Y; full-output queries fix X and request every permitted visible pair. Immediate and retained-all consumer policies are separate cases. The independent oracle enumerates complete assignments before measurement.

There are 576 configurations across the four paths. Both allocation executions of every configuration agree exactly, giving 1,152 passing processes. Every retained observation remains correct after preparation disposal, and final release restores the initial live-byte endpoint. No process reaches its 60-second wall/CPU, 1 GiB address-space or projection work bound. Meter self-check and scoped Clippy pass.

## Identity ownership persists after each instance is disposed

Each symbolic query creates a fresh instance, with a newly identified caller import X. Its shared allocator records both caller imports and generated identities. Disposing the instance releases its formula and terms but leaves those identity records in the allocator. The experiment disposes this owner separately while retaining the prepared formula and consumer observations.

For the six-variable, three-name hidden-star membership source with immediate consumer release:

| Queries | Retained identity records | Live bytes released with allocator | First transport requests | Last transport requests |
|---|---:|---:|---:|---:|
| 1 | 6 | 104 | 2,249 B | 2,249 B |
| 16 | 96 | 1,952 | 2,249 B | 4,097 B |
| 128 | 768 | 14,936 | 2,249 B | 17,081 B |

The corresponding free-exclusion stream retains the same 768 identities and 14,936 bytes at 128 queries. The retained identity population is therefore not explained by a more difficult exclusion problem. Source inspection shows that each transport clones the occupied-ID set transactionally, then records the next instance's identities. Growing history increases both retained storage and subsequent allocation traffic.

This observation does not establish that all reservations can be reclaimed. Caller imports and generated IDs have different obligations. A monotonic allocator with a controlled cursor might avoid remembering generated IDs already below its cursor, while future caller reservations still need protection. The current public allocator state does not itself establish that stricter ownership contract. A bounded alternative allocator remains a required implementation/lifetime comparison.

## Allocation contrasts and a useful limit on refinement

In that 128-query hidden-star membership scenario, measured phase requests are:

| Path | Requested bytes |
|---|---:|
| Symbolic conjunction | 2,747,349 |
| Finite projection | 523,795 |
| Prepared name solver | 128,912 |
| Explicit enumeration | 67,790 |

The symbolic path spends 1,232,024 bytes in transport and another 1,513,836 in answering the queries, plus 1,489 in preparation. Even eliminating all transport allocation would leave query evaluation requesting more than eleven times the prepared name solver's whole measured session. Allocator tuning alone therefore cannot reverse this particular allocation comparison. It could still improve sustained ownership or another source regime; this is a bounded sensitivity argument, not a reason to abandon that work.

Across the 144 matched scenarios, symbolic requests are lower than finite projection in 54 and higher in 90. They exceed both prepared name solving and explicit enumeration in all 144. These counts are not workload frequencies. The strong controls exploit the finite-name premises; they do not replace arbitrary symbolic structural answers or prove broader source eligibility.

The meter counts requested heap traffic and live endpoints, not RSS. Totals include preparation, fresh transport, query observation, instance disposal, consumer ownership and final disposal. Source/oracle/caller fixtures and preallocated harness capacity are excluded. No clocks or timing selection are part of this qualification.

## Decision and remaining work

The prepared and consumer endpoints are now qualified for a matched cost trial. The symbolic path also exposes an owner that a future trial must charge explicitly. Before selecting a symbolic architecture, investigate reusable compiled constraints as well as identity management: the query-allocation bound shows that transport alone is not the whole problem.

This completes the fourth package of the joint-theory cycle: feasibility, finite exact projection, symbolic transport and matched ownership. The [full breadth review](S06-joint-ownership-breadth-review.md) selects the demand evaluator's currently excluded call-output/resource dependency under T071. That source-capability comparison can change which architectures implement the language, whereas further allocator precision cannot reverse the allocation example above.

Joint runtime, symbolic union/inclusion, compiled formula reuse, general source closure, identity lifetime and broader observations remain required under T076. No theory direction is closed by this finite-name comparison. The research goal remains active.

## Evidence

[Registration](../registrations/S06-joint-ownership.md), [harness](../../../research/chr-structural/tests/joint_ownership.rs), [runner](../../../research/chr-structural/experiments/joint_ownership.py), [audit](s06-joint-ownership/audit.json), [per-configuration summaries](s06-joint-ownership/summary.json), [frozen inputs and binary](s06-joint-ownership/freeze.json) and [raw receipts](s06-joint-ownership/). Eighteen inputs are frozen. No engine or reference implementation changed; the package manifest adds the isolated allocation harness and its previous input is preserved.
