# Finite joint projection preserves hidden dependencies and exact visible sets

Joint logical constraints now produce exact projected sets for declared finite ground domains using the existing elimination engine. Hidden name constraints survive projection correctly. The checked boundary rejects raw CHR observation and exposed hidden variables; unresolved output holes remain outside this entry.

## What the experiment adds

The region declares finite domains and a conjunction of name, name-disequality, finite-tree equality and structural requirements. Each predicate becomes a relation over its actual variable scope, with repeated variables kept as one coordinate. The existing `projection::Problem` chooses an elimination order and removes hidden coordinates under set semantics. The adapter decodes the resulting coordinates back to their original terms.

This connects the qualified joint predicates to an existing projection implementation. It adds no second elimination baseline and changes no existing solver or reference engine. It does introduce preparation work: scope discovery, ground relation construction, term dictionaries and decoded output allocation. These costs have not yet been measured for this source interface.

The output contract is explicit logical sets. Duplicate domain values do not produce duplicate answers. Equality between visible coordinates is preserved as equality of their returned ground terms, and the caller's requested coordinate order is retained. This is distinct from CHR derivation multiplicity and from the identity of unresolved fresh variables.

## Exact-answer evidence

| Check per confirming execution | Scope | Result |
|---|---|---|
| Independent complete sets | 1,536 combinations of normal templates, name subsets, exclusion graphs, visible subsets and optional equality | Exact agreement with independent enumeration of all 27 ground assignments |
| Hidden finite-name dependency | Hidden H differs from X and Y over a two-name domain | Visible answers contain only equal X/Y pairs |
| Changing caller restrictions | Restrict the same preparation to either visible name, incompatible values, outside-domain values or hidden/unknown coordinates | Correct narrowed/empty sets; invalid coordinates remain errors |
| Boundary and limits | Shared hidden variables, literal predicate interaction, raw observation, nonground domains, duplicate/empty domains and resource/capacity bounds | Explicit acceptance, rejection or empty-set outcomes as registered |

The main matrix uses three ground values per coordinate: `a`, `b` and `lam(a,a)`. Structural templates `app(X,Y)`, `app(X,X)` and `lam(X,Y)` distinguish name restrictions and normality rather than testing only interchangeable atoms. Every subset of the three variables is projected, including no outputs and all outputs, in reversed coordinate order.

Four confirming executions pass: two with structural metrics enabled and two disabled. Each passes all three tests and 1,536 exact-set comparisons. Another 62 existing tests pass across 14 executables, including the zero-test library target. Scoped Clippy and formatting pass. All executions remain within 60-second wall/CPU and 1 GiB address-space limits. Table preparation and output use explicit work bounds; a bound failure is an error, not a truncated answer set.

One additional failing test exposed a request-validation defect before confirmation. An out-of-domain restriction could return an empty set before a later unknown coordinate was checked. The adapter now validates every restriction coordinate first. The failing and passing evidence is retained, and all confirming runs use the corrected implementation.

## What closure means here

A region is explicitly declared logical and private. Host constraints may share only visible region variables. Host rules that read or post literal `var`, `norm` or `neq` predicates are conservatively rejected, as are literal theory occurrences in the host. Raw-answer observation is rejected before projection. These checks expose the boundary instead of silently treating residual CHR occurrences as a logical formula.

The checker is conservative. For example, it rejects a rule that posts `norm` from a `trigger` even when the supplied host has no trigger and cannot enable it. This is a concrete precision cost; reachability inference could admit more cases. The current result does not prove a general closure certificate for arbitrary CHR programs or compare inference, checked declarations and mandatory restrictions economically.

Finite domains are another real restriction. Nonground domain values are rejected because the current coordinate decoder cannot express their unresolved aliases and caller freshness. Domains with more than 256 distinct terms exceed the existing coordinate encoding and return a capacity error. Neither limit is an architectural impossibility: both remain possible implementation or language extensions requiring their own evidence.

## Architectural consequence and next decision

The finite declared fragment can preserve joint dependencies through exact elimination and changing visible restrictions. It is therefore eligible for later ownership and cost comparison against whole-assignment enumeration. This does not establish a solver speedup, sustainable memory behavior, raw-source replacement or a preferred language contract.

Next investigate exact nonground output and fresh caller transport under an explicit symbolic observation contract. Preserve repeated holes within an answer, freshness between independent answers/callers, and the hidden dependencies that must remain in a summary. Compare that contract with the finite ground decoder and the established raw-source observer/consumer counterexamples. A finite alphabet must not silently become the language's domain restriction.

The strongest ready alternative is allocation/lifecycle qualification of this finite interface. That would establish its preparation/output cost, but would leave the original fresh-alias obligation untouched. The symbolic-output entry takes priority because it can change whether this solver boundary supports the language's unresolved variables at all. Reconsider finite costs, local branch-copy attribution and demand-driven choices at that gate or an obstruction.

This is package two after the guarded-choice breadth review. Two further packages trigger the next full review. General symbolic projection, broader closure inference, source multiplicity, sustained ownership and complete architecture costs remain required. The research goal remains active.

## Evidence

[Registration](../registrations/S06-joint-projection.md), [region adapter](../../../research/chr-structural/src/joint_region.rs), [independent tests](../../../research/chr-structural/tests/joint_projection.rs), [runner](../../../research/chr-structural/experiments/joint_projection.py), [audit](s06-joint-projection/audit.json), [frozen inputs and binaries](s06-joint-projection/freeze.json) and [raw receipts](s06-joint-projection/). All 38 source inputs and confirming binaries are recorded. The earlier library input is preserved for prior experiment provenance.
