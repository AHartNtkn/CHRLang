# Inference can remove unary discovery machinery without restricting mixed sources

Prepared unary dispatch removes three query-time discovery structures for all-unary sources: predicate buckets, queued tuple discovery and the known-tuple deduplication table. Inference also exploits unary rules inside mixed sources. A checked declaration selects the same beneficiary; requiring that declaration changes admission, not the accepted source's execution path.

This implements a concrete compiler beneficiary after the [head-property witnesses](S07-head-property-gate.md). It does not establish a lifecycle gain or resolve non-overlap properties.

## What the property removes

A unary rule needs only the newly posted occurrence to identify its candidate tuple. Preparation builds a predicate/arity-to-rule map. Posting an occurrence routes it directly to those readers, keeping rule order and applying the existing immutable-head filter. Unknown terms remain candidates so later bindings can activate them.

No second anchor can rediscover a unary tuple, so direct routing does not need the known-tuple table. All-unary sources also need no query predicate buckets or queued tuple-discovery jobs. Tests inspect those three structures after every service step and find all three empty, including searches that continue after the public prepared handle is dropped.

This fact concerns an occurrence tuple, not a unique matching environment. Matching, guards, source priority, supported consuming claims, propagation history, late dependencies, choices and observation remain in the existing pipeline. The earlier partial-constructor witness still applies; the property does not collapse its intermediate environments.

Mixed sources receive local inference too. Their unary readers use direct routing; only predicates read by multi-head rules need general occurrence indexing/discovery. General discovery skips unary rules already dispatched. A predicate shared by unary and multi-head rules participates in both appropriate paths. This avoids making a whole-source restriction look useful merely because inference declines all optimization on mixed code.

## Admission and inference share one authority

`PreparedRuleset::with_head_contract` checks and compiles the same owned source. The experimental `head-dispatch` feature enables this interface; ordinary preparation remains the general control. No default-policy change is made.

| Input to the experimental interface | All-unary source | Mixed source |
|---|---|---|
| No declaration, optional admission | Infer direct dispatch | Infer local unary dispatch and retain general joins |
| Checked single-head declaration, optional admission | Same direct dispatch | Reject the contradicted declaration |
| Checked single-head declaration, required admission | Same direct dispatch | Reject the contradicted declaration |
| No declaration, required admission | Reject missing declaration | Reject missing declaration |

The declaration means exactly one kept-or-removed head per rule in the supplied complete source. It does not assert non-overlap, consuming-only behavior, groundness, termination or determinism. The checked admission choice is not retained as a runtime policy flag.

A linked multi-head extension must be checked as a new prepared source. The old prepared source remains valid for its original rules. Tests show both checked modes reject the extension, while undeclared inference executes the extended program and retains direct dispatch for its unary rule. Reusing a prior acceptance Boolean for the new source is not part of the interface.

## Measured work and executable coverage

On the late-binding/multiplicity witness, inference, optional checked declaration and required declaration each take 370 service calls and preserve the same complete answer. Their three query discovery structures remain empty throughout. All modes share the same prepared-dispatch representation.

| Unary propagation occurrences | General filtered service calls | Inferred direct calls | Candidates in either path |
|---|---:|---:|---:|
| 0 | 18 | 18 | 0 |
| 1 | 115 | 108 | 1 |
| 16 | 2,770 | 2,658 | 16 |

The propagation rule still fires once per occurrence and preserves duplicate-valued residual occurrences. Preparation is reused across changed queries. These are service counts, not elapsed time or requested allocation; the prepared reader map has a cost not measured here.

The prior source-property corpus now also exercises inferred preparation, including overlapping unary rules, late bindings, propagation, atomic joins and linked/bundled updates. The independent runtime corpus compares both ordinary and inferred execution with scalar full answers and projected source-application traces. This directly checks that early routing does not alter rule priority or branch-local application sequences.

A finite answer beside an ongoing unary loop arrives within 10,000 service calls; 1,024 subsequent calls continue without false exhaustion or extra answers. Its owned answer remains valid after the query engine and public preparation are dropped. This is a bounded service/lifetime gate, not measured cancellation allocation or a sustained stream.

All 136 crate tests pass in the combined feature build. Eighteen selected tests pass counter-free, including the runtime trace corpus and head/property tests. Scoped strict Clippy passes for both experimental and ordinary configurations. [Logs and source hashes](s07-head-dispatch-gate/) record the validation. The initial interface test failed before implementation.

## What this changes in the language comparison

A mandatory single-head declaration is not necessary to obtain this compiler benefit on known unary rules. Inference can exploit the property locally, while checked declarations can enforce an intended whole-source boundary. The gate establishes those responsibilities and admission differences; it does not establish that either interface has lower total cost or that enforcement is valuable enough to justify excluding mixed programs.

Register lifecycle attribution next: general discovery, inferred dispatch, optional checked and required declarations on equivalent accepted programs, plus undeclared mixed-source controls and explicit rejection cases. Charge inference, both prepared maps, query setup, changed queries, delivery, cancellation and all owners. Keep rejection costs separate from completed execution. Include shared predicates with unary/multi-head readers and the caller/update obligations of any bundled reformulation.

Non-overlap remains a distinct required comparison: this implementation does not certify freedom from competing applications or remove matching-environment multiplicity. The strongest ready alternative is a resource-conflict certificate with its own executable beneficiary. First establish the complete cost of this simpler, now-working property control, then reconsider that alternative before adding more declarations. T079 and the architecture goal remain active.
