# Finite-path solving avoids enumeration on a selective witness

A direct structural solver now intersects finite constructor descriptions and enforces equality between subtrees before enumerating their values. On a description of all 64-bit lists, equating the bit positions yields two answers after 66 grammar-requirement steps. This establishes the mechanism; it is not a runtime or architecture ranking.

The gate also preserves a distinction essential to CHR: one value can have several source derivations. Reducing redundant membership proofs must not erase those alternatives.

## What the mechanism changes

The earlier skeleton prototype enumerates each hole's candidate domain during preparation and then takes a product of those lists. Its equalities join entire holes. The new [finite solver](../../../research/chr-structural/src/finite.rs) instead starts with demanded term positions, joins positions required equal, and propagates constructor-language requirements as search proceeds. The same grammar description at two positions permits independent choices unless an explicit path equality joins them.

A request selects a source grammar state, membership filters and equalities between finite child-index paths. Every mentioned path must exist. Equal subtrees imply equal corresponding children; equality with a proper subtree has no finite-tree solution. An absent path is failure, even when both sides mention that same path. Input grammar cycles are rejected as outside this finite gate, rather than reported as an empty language.

The source state denotes a bag of constructor derivations; filters denote membership predicates. The solver emits each distinct accepted ground term with the number of original source derivations producing it. This count can be expanded to raw alternatives for the tested pure generator fragment. Overlapping filter proofs do not multiply it. Checked count overflow remains an error on subsequent service calls.

The optional structural reduction merges identical bottom-up descriptions and duplicate membership transitions. It retains original alternatives for source counting. It does not minimize all equivalent languages, merge observable source alternatives, or eliminate the exact output-history set.

## Independent checks

The [registration](../registrations/S06-finite-path-source-gate.md) defines the denotation, controls and service bounds. All **13 finite-path tests and 22 package tests pass**, as does scoped strict Clippy.

| Check | Scope and observation |
|---|---|
| Exhaustive small denotations | 7,840 requests, each run unreduced and reduced: 15,680 solver comparisons. An independent control builds owned source derivations, tests membership with explicit sets and traverses paths directly. Values and counts agree. |
| Actual CHR programs | 160 grammar/path sources. An independent scalar CHR evaluator agrees on complete raw answers, including two output fields and the residual marker. The reference interpreter agrees on distinct answers and total completed branches; its normal answer interface deduplicates. |
| Required adverse semantics | Empty domains, overlapping alternatives, duplicate transitions, independent occurrences of a shared description, nested/transitive equalities, all six orders of three interacting equalities, absent paths, arity incompatibility and finite-tree cycles. |
| Service and ownership | Zero budget is not exhaustion; single-step resumption preserves the answers; dropping unfinished search leaves the grammar reusable; returned owned values remain valid after search and grammar disposal. These are semantic ownership checks, not measured heap baselines. |
| Count errors | A real unrepresentable source multiplicity raises a persistent error. An impossible derivation contributes zero even if another part of it has an unrepresentable count. |

The CHR sources explicitly generate constructor alternatives and walk finite paths, failing on missing paths or incompatible endpoint bindings. Their source-correspondence comparison covers the generator plus equality fragment. General grammar filters are checked against independent set enumeration; translating those filters into general consuming CHR is not established by this gate.

No reference code was changed. The scalar evaluator and reference use their own matching, bindings and branch execution; they do not use the solver's position graph or propagation algorithm. The exhaustive control also counts derivations independently rather than calling the candidate's multiplicity routine.

## Selectivity and redundant proofs each matter

The selective source has a linear grammar description for all 64-position lists over two leaf constructors. Its unfiltered value count is analytically `2^64`: each position chooses independently. Equating the first bit with each later position admits only the all-first-leaf and all-second-leaf lists. The solver emits both with multiplicity one, using **66 requirement steps, 67 transition trials and a peak frontier of two states**. It constructs the required owned output trees; it does not enumerate the unfiltered language.

An unselective eight-position control emits all **256** values and agrees with exhaustive enumeration. Compact representation does not remove that output obligation. The 64-position witness supplies no claim that arbitrary path constraints remain compact or cheap.

Redundant filter alternatives exposed a separate avoidable cost. An eight-position filter with two identical leaf transitions admitted one source value through 256 membership proofs. Before reduction, the solver performed **280 requirement steps and checked 255 duplicate values**. Structural reduction brings those counts to **17 and zero**. The accepted source multiplicity remains one; making the duplicate-rich grammar the source instead changes it to 256, as required.

These counts include neither a timing comparison nor the preparation cost of reduction. Original source descriptions remain retained for counting. Equivalent but structurally different overlapping alternatives can still generate duplicate values and require exact observation history. Both obligations must count in the lifecycle comparison.

## A consequential arithmetic defect was corrected

The first counter implementation raised overflow as soon as a child count exceeded its integer range. That was incorrect when a later child made the whole derivation impossible: a zero factor makes its contribution zero. A failing test demonstrates this case and now passes.

The corrected counter carries an explicit positive-overflow result through intermediate arithmetic, lets zero annihilate it, and reports overflow only for the final accepted value's source count. It memoizes repeated state/term subproblems within that observation. This is a correctness repair, not a speed measurement.

## Architectural consequence and next experiment

Finite path equality now has a direct executable trial beyond equality between pre-enumerated holes. The selective witness shows why structural solving could change the amount of search required. The unreduced filter witness also demonstrates why a redundant grammar implementation would be a weak basis for rejecting the approach.

The next package should establish complete costs and observation ownership for unreduced/reduced lazy solving, competent enumeration/filtering and the strongest applicable source-derived control. Include empty/selective/unselective intersections, distinct versus repeated queries, growing required output, overlapping descriptions that survive structural reduction, first witness versus full enumeration, cancellation and disposal. Prepare once for changed queries where the API supports it. Disable counters for primary timing and use separate allocation diagnostics; this gate's counters are diagnostic and always enabled.

That package has greater immediate decision value than another isolated solver feature: it tests whether avoided enumeration repays reduction, search-state copying, source counting and exact output history. Checkpointed restoration/reunion remains the strongest distinct ready alternative and follows this bounded comparison unless new evidence changes that selection. T076 remains active; no whole architecture has been selected.

Still required are general consuming-source correspondence, richer projection/theories, recursive descriptions with honest evaluation bounds, sustained history reclamation and whole-architecture integration. This finite pure fragment cannot resolve those directions.

## Reproduce the gate

Run `cargo test -p chr-structural` and `cargo clippy -p chr-structural --lib --test finite_paths --no-deps -- -D warnings`. Run the finite-path test with `-- --nocapture` for mechanism counts. The tests impose explicit service bounds and require exhaustion on completed cases; no elapsed-time numbers are used as evidence.

[Gate output](s06-finite-path-source-gate/finite-paths.log), [package output](s06-finite-path-source-gate/package.log), [Clippy](s06-finite-path-source-gate/clippy.log), [audit summary](s06-finite-path-source-gate/audit.json), [source hashes](s06-finite-path-source-gate/freeze.sha256) and [toolchain](s06-finite-path-source-gate/toolchain.txt) preserve the evidence. The source hashes include new files not represented by the recorded base commit alone.
