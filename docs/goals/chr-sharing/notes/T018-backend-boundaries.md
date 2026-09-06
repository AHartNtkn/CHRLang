# Backend correspondence and search encapsulation

## Native named choices: a sufficient first-order calculus

The projection model in T016 can be made a small executable specification independently of HVM's higher-order compiler. Use ordinary constructor nodes, opaque logical-variable handles, source-choice nodes C(d,L,R), and finite service requests. The source-choice identity d has a birth support and a ledger entry. No source rule can inspect C as a constructor.

A service that only passes a handle or builds a constructor does not inspect a choice beneath that handle. A service that reads a field follows this algorithm:

1. Resolve the field under the request's current support. If every projected value agrees on the information read, compute that case once.
2. Otherwise choose a relevant source label d, split the request support by d, and restrict *all* operands and contextual services consistently in each part. Recursively process each nonempty part.
3. Reassemble equal immutable results by sharing their representation; retain supported effect records and source lineage separately.

For finitely many labels in a frozen request, each split fixes a previously unresolved label. Consequently the case analysis terminates, provided the underlying finite operation terminates. Every projection takes exactly its selected subrequest, whose operands are its original operands. Induction on unresolved labels proves equality with the scalar service result. The algorithm can use a work queue instead of recursive execution to satisfy finite scheduling quanta.

Copying a data family gives two references to the same family, not its left and right alternatives. A graph implementation using DUP must therefore distinguish ordinary copying from restriction by d. Conditional variable binding, occurrence membership and propagation history use the same supported-effect interface as the conditional kernel. This is a concrete service construction, including a correct but potentially expensive full decision expansion when values differ everywhere.

The distinction among representations is now operational rather than semantic: Boolean supports, named-choice decision graphs and contextual relation graphs can all implement these requests. Their intersection, distribution, memoization and allocation costs differ. This construction does not claim that their asymptotic or practical costs coincide.

## What the inspected HVM revision actually establishes

The pinned [HVM4 source](https://github.com/HigherOrderCO/HVM4/blob/6defdfc7dae2a3cca5dd6e74ed0612385b5646a8/src/hvm.c) contains relevant local mechanisms:

- `wnf_dup_sup` distinguishes same-label annihilation from different-label commutation.
- `wnf_app_sup` and `wnf_app_mat_sup` distribute application while cloning its other input or matcher with the choice label.
- `wnf_dup_lam` introduces a superposition at the duplicated binder. Thus administrative superpositions are part of function copying, not solely representations of source OR.
- Dynamic DSU/DDU nodes evaluate a supplied numeric label. The term format masks its extension field, and parser-generated labels occupy a finite namespace. Dynamic label syntax itself does not allocate a fresh source-choice identity.
- The collapse worker calls `cnf` before returning to its queue. An output normalizer is not a source-level quiescence check or a proof of fair execution.

These observations rule out a transparent mapping “source OR becomes SUP, source variables become binders, then collapse.” That mapping lacks source unification, scoped resource effects, label allocation and finite service control. Repeated argument use and independent recursive choices are concrete counterexamples to careless label reuse, as shown in T016.

## Three integration contracts

**Encoded machine on an existing functional backend.** Encode the reference or conditional machine, including its choices, as ordinary backend data. Source labels are unbounded identifiers in that data model; backend administrative labels are not source alternatives. Return one bounded machine continuation at a time to a controlling scheduler. This settles expressibility through a deterministic machine encoding and avoids relying on native collapse for source search. Its performance must include interpretation and normalization overhead. A concrete generated program is needed to validate that backend steps implement the intended finite continuation boundary.

**First-order native family services.** Implement the projection-preserving service algorithm above with immutable data sharing and contextual effects. It can use a graph runtime but need not use higher-order lambda copying at all. The finite-net service encoding supplies one conservative compilation route, with explicit request/reply and linear endpoints. This has a paper correctness route without a claim that it is already compiled by HVM4.

**Native HVM integration.** Use HVM's lambda/duplication machinery together with native SUP for source alternatives. This additionally requires a compiler invariant connecting source label birth, administrative copies, function duplication and observation. A useful implementation experiment must expose generated graphs and label traces; merely running ground examples through collapse does not validate it. Until that compiler is specified, “HVM integration” denotes this engineering choice, not a theorem supplied by the backend's name.

The source inspection has resolved the question of whether the present HVM mechanisms automatically supply the project's semantics: they do not. It has also identified two constructive integration contracts whose correctness does not rely on that assumption. Designing and validating a full native compiler is substantial implementation/formalization work, with an exact obligation above. Further browsing about superpositions cannot settle the generated-code invariant for a compiler that has not been built. The generic family-service construction remains available for experiments without waiting for that integration.

## Encapsulated search as an internal interface

Van Roy et al., [The Oz experience](https://webperso.info.ucl.ac.be/~pvr/tutFinal.pdf), §7.4, separates nested computation spaces from their search strategies. It describes local binding visibility, stability, explicit choices, cloning, commitment and merging. Ask returns status upon stability; cloning is specified for stable spaces. The surrounding Oz model includes rational-tree equality. This supplies a useful abstraction boundary, not this project's equality semantics or fairness guarantee.

An internal CHR interface can similarly separate the store engine from the search policy:

- `advance(handle, budget)` returns resumable progress and newly available source-choice or answer events.
- `restrict(handle, decision)` creates or selects a supported view of an explicit alternative.
- `observe(handle)` returns a result only with the source quiescence certificate.
- Optional checkpoint/replay operations preserve occurrence identity, fresh allocation lineage and the chosen committed policy.

These names describe runtime operations, not CHR predicates or a proposal that programs pattern-match on choice status. Exposing them inside the language would be a separate design decision. As an internal API, they support interchangeable scheduling while keeping choices opaque.

Waiting for whole-space stability before giving another alternative service can starve on recursive source work. The bounded advance operation is therefore necessary for the intended fair default; it must not secretly run to a fixpoint. Cloning a space provides branch isolation and perhaps storage sharing, but does not itself reuse the owner's post-choice opaque computation. Family services, deferred distribution or event memoization still provide that property.

## Disposition

Representation/service compatibility and search-interface separation are sufficiently resolved for comparative implementation contracts. Actual generated-code correspondence, label lifetime reclamation, backend normalization yields, retained graph size and post-choice operation counts need concrete implementations or formalizations. No HVM performance advantage, turnkey embedding or architecture selection is claimed. The source language does not need a new restriction merely to compare the encoded-machine and first-order native contracts.
