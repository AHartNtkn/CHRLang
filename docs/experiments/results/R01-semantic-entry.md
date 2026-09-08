# R01: generated execution and activation semantic entry

The four execution cells now run the same finite no-OR programs with independently checked answers and legal applications. Native generation is real rule execution, and partner traversal resumes between candidates. Cost ranking remains open because bound-key access and lifecycle controls still need preparation.

## Implemented contrast

[chr-compiled](../../../research/chr-compiled/README.md) combines prepared generic templates or generated Rust with global scanning or FIFO occurrence activation. The generic path resolves source variables to fixed slots once and caches heads and body templates. Generated code emits constructor tests, repeated-variable checks, guards, body construction and rule-specific cursor state machines. Shared primitive term, binding, store, history and observation operations keep the first factors controlled.

Rulesets accept separate runtime queries. Recursive depths absent from generation are tested against the same generated code. Explicit OR is rejected; this gate makes no search-engine or unrestricted-guard claim.

The candidate uses the persistent engine's term and map primitives through a candidate-only public interface. A successful unification reports committed changed variables; a failed transaction changes neither caller bindings nor the caller's change report. The reference interpreter has no candidate dependency and its runtime is unchanged.

Activation tracks inserted and changed occurrences, including dependencies reached through aliases and nested constructors. A retained occurrence changed by its own body can become active again. Consumed occurrences cannot contribute future tuples. History keys preserve distinct occurrences even when their values are equal.

## Validation and review

Eight focused tests pass. Twelve analytic source programs run in all four cells; the thirteenth deliberately permits different cross-policy outcomes and checks their source legality. Cases include recursive construction, reachability with consuming jobs, delayed bindings, guard wakeups, duplicate-valued propagation, middle-head arrival, fresh variables, occurs failure and unknown structure that must stay unbound.

The independent checker uses source terms rather than candidate matching or equality functions. It checks application eligibility, fresh locals, instantiated bodies, consumed occurrence IDs and propagation history, then independently searches for enabled applications at successful termination. It does not check every intervening primitive effect. Analytic complete answers and atomic-unification tests supplement that boundary. Generic/native traces agree within each policy; rule competition is never converted into implicit source disjunction.

One step examines at most one partner candidate, as checked in all four modes. Matching, equality, pool materialization, body construction and observation remain synchronous. This is resumable traversal, not a primitive-work latency or fairness guarantee.

Independent code review identified unequal preparation in the first generic control and synchronous traversal inconsistent with the entry contract. Prepared generic plans and explicit cursor state machines address those issues. Rereview found no new cursor defect. Review also identified the cost limitations below, which remain open rather than being hidden by this gate.

Root verification passes all thirty-one affected compiled, persistent and boxes tests, plus workspace Clippy and formatting. The broader workspace test run also passed; the affected tests were rerun after moving the reporting API to its owning module. One existing persistent registered experiment test is intentionally ignored by its original harness. The generated source contains thirteen programs and twenty-seven rule selectors. Both current build outputs contain 35,223 bytes with SHA256 `47a16e676f763b394a039f82eb1c41deb35b839c4b7d7142c2a91928b3d465d8`. Artifact size is provenance, not a complexity or performance score.

```sh
cargo test -p chr-compiled -p chr-persistent
cargo clippy -p chr-compiled -p chr-persistent --all-targets -- -D warnings
cargo fmt -p chr-compiled -p chr-persistent -- --check
```

## What remains before a meaningful cost contrast

Partner lookup currently uses predicate buckets. Selective chains have bound arguments that can support much narrower lookup; a comparison without that control could mainly measure avoidable scanning. Unary node keys also grow with store size, coupling lookup scaling to term preparation and observation. Flat keys are needed to separate those dimensions, with structured and delayed-binding cases retained for their own questions.

Runtime preparation currently occurs during engine construction. Reused-ruleset comparisons require an explicit prepared-program lifetime for both generic and generated paths. The generated ruleset check and metadata preparation must be charged once at the appropriate boundary, not silently treated as native compile cost or repeated generic interpretation.

Application traces are currently mandatory; detailed audits are optional. Engine state remains live after completion, and each completed `run()` call re-exports the answer. A cost runner must distinguish diagnostics, execution, extraction, retained storage and destruction. Repeated inspection is not a stream of newly recognized answers.

The next selected implementation is the [access and lifetime control](../registrations/R01-access-lifetime-entry.md), before timing. Its purpose is to make the competing execution paths credible, not to tune a winning candidate. R02's class/index organization retains independent eligibility and need not reuse this representation or await an R01 speed result.
