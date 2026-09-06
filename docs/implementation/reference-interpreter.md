# Reference interpreter: design and implementation plan

The owner approved a straightforward, independently checkable interpreter, even where it copies state or repeats work. The reference implementation is a correctness comparator, not a selected optimized architecture. The implementation is authorized in the current conversation; the earlier research dossier remains a research record.

## Separation

Use a dependency-free Rust workspace. `chr-syntax` contains only owned language/answer data. `chr-programs` constructs reusable rulesets and queries and depends only on syntax. `chr-reference` contains all execution algorithms and depends only on syntax at runtime. Its examples/tests may also use programs. Engine modules (substitution, matching, branch state and answer canonicalization) are private. Future engines consume syntax/programs independently; they must implement their own execution and normalization, not delegate to the reference. No optimized engine or generic runtime framework is created here.

## Semantics and interface

Terms are owned finite constructor trees and numbered variables. Rules have kept/removed heads, pure equality-entailment guards and a RHS goal tree. RHS goals support constraints, finite-tree equations, conjunction, explicit binary OR, true and fail. Queries contain ordinary initial constraints and selected named variables. The equality-only guard representation cannot contain a binding equation or execute host callbacks.

The reference drains pending goals FIFO, then selects the first enabled rule in declaration order and first matching ordered tuple in occurrence order. This is its reproducible committed policy, not a language source-order guarantee. A step advances one queued alternative, which is appended to the FIFO unless failed or completed. OR clones its complete owned state for the second child. Matching never instantiates store variables; repeated rule variables require entailed equality. A selected application freshens remaining local variables, consumes removed heads, records its ordered occurrence token and posts the body. Unification is transactional, performs an occurs check and resolves bindings everywhere by dereferencing. Every alternative is scanned for enabled work before being reported.

`Search::new(rules, query)` validates the source shape. `advance(step_budget)` returns new answers and whether the frontier is exhausted. Exhaustion is distinct from a budget expiring. `stats()` reports actual reference operations. Each finite match/unification/canonicalization operation is atomic to this scheduler: fair service of finite derivations is intended, not a wall-clock latency bound.

Answers include the selected tuple and the full residual multiset. Canonicalization fixes variables in output traversal order and exhaustively renames remaining residual variables, sorting occurrences without deduplicating them. This deliberately simple exact alpha criterion is local to the reference engine. It proves some answer equality, not continuation equivalence.

## Validation tasks

- [x] Language data + transactional unification: write failing checks for aliases, constructor/arity clashes, indirect occurs cycles and rollback; implement private iterative unification.
- [x] CHR execution: failing behavioral tests for simplification/simpagation/propagation, distinct occurrences, nonbinding and repeated-variable matching, guard suspension/recheck, fresh variables, quiescence and committed competition; implement exhaustive branch steps.
- [x] Explicit search and observations: failing checks for branch isolation, inherited/conditional history, nested choice, failure outside selected outputs, fair finite answers beside a loop, nonground residuals, alpha dedup and multiplicity; implement FIFO search and exact answer canonicalization.
- [x] Reusable demonstrations: addition/subtraction, translated SK eval/fold/no_c and SK type inference. Check literal expected arithmetic, identity, duplication, principal types, and a bounded prefix of type-driven synthesis. Verify continued calls resume a live search.
- [x] CLI example and documentation: expose bounded interactive-style batches, answers and counters; document construction, separation, policy and limits.
- [x] Validate with `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, actual demo commands and dependency-tree inspection. Review source against the semantic notes and commit all task-owned work.

## Core test expectations

`X = f(Y), Y = a` resolves X to f(a); adding `Y = X` must fail transactionally when it forms a cycle. A head p(s(X)) must leave p(Y) residual while Y is unknown. A head pair p(X),p(X) requires two distinct occurrences with entailed equal arguments. A propagation token survives later equality and source splitting. `choose <=> (X=a | X=b)` yields both values while ordinary competing simplifications commit once. A loop in one explicit arm must not prevent a finite answer in its sibling. Two answers with identical outputs and alpha-renamed residuals collapse; different alias relationships and residual multiplicities remain distinct.

These expectations come from the reference transition specification and hand-derived examples. There is no performance target or claim of proof completeness.

## Implementation and validation receipt

Implemented the three-crate boundary described above. Cargo's normal dependency tree contains only `chr-programs -> chr-syntax` and `chr-reference -> chr-syntax`; reference runtime modules remain private.

Validation passed: `cargo test --workspace --all-targets` (33 tests), `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo fmt --all -- --check`. Source review found no correctness blocker. A review-requested residual-variable permutation case now checks both alpha-equivalent answers and distinct alias relationships; a temporary mutation restricting canonicalization to one permutation caused that test to fail, confirming it exercises the intended behavior.

Executed demonstrations cover arithmetic, SK identity and duplication, principal type inference, and resumed type-directed synthesis. The unrestricted duplication synthesis query returned no answer within 50,000 reference steps and retained 4,769 pending alternatives. That bounded observation does not establish failure or exhaustion; search performance remains a limitation of this copying implementation.
