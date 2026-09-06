# Composing the conditional machine's local arguments

The local conditional operations can be composed into a trace-preserving abstract machine using sealed rounds and private source-step updates. This closes a theoretical gap between the individual operations and a whole-machine execution discipline. It does not validate an implementation or prove that the representation is efficient.

This is an original mathematical argument using T011-conditional-kernel.md, T012-matching-and-wakeup.md and T015-symbolic-scheduling.md. It applies to finite-tree equality and guards with a specified pure, terminating decision procedure on the projected state. An additional built-in domain needs the same consistency, entailment and update obligations; an arbitrary host call is not covered by naming it a guard.

## State invariant

For each live alternative a, projection of the runtime yields one well-formed reference state `(G_a,S_a,theta_a,H_a,V_a,I_a)` and its selected output roots. The following invariants make that statement explicit:

- Each occurrence membership and token condition is an exact set of alternatives. A token refers to the same ordered source head identities on every alternative in its support.
- Every reachable variable and occurrence exists on that alternative. A reference's support is contained in the birth supports of its endpoints. Newly posted body graphs are visible only on the contexts where their application has committed.
- The projected binding graph is acyclic and denotes a finite-tree solved substitution. The union of incompatible projected graphs need not be acyclic.
- Occurrence identities are fresh within every projected derivation and are not reused after consumption. Body-local variables are allocated consistently across all references within an application and distinctly from existing variables there.
- The semantic alternative ledger is separate from grouping equal work or equal runtime states. Only executed source ORs add alternatives; failed alternatives cannot publish answers.
- Pending internal work either denotes a selected source transition on an immutable snapshot, a discovery/certification operation, or a representation-only operation. Private partial results are not visible source state.

The initial query satisfies these conditions by allocating distinct occurrence/variable identities, preserving explicitly repeated variable references, and using the identity substitution. Query-level new equation/OR syntax is not needed for this initialization.

## One round and disjoint updates

Freeze the finite current state. Select at most one source transition for each live alternative under the declared committed policy. Use exact symbolic matching and guards, or their exhaustive projected control, to obtain disjoint conditions for the selected operation cases. Selection partitions work; it does not create semantic alternatives.

Every operation runs on its frozen projected inputs and accumulates a private effect. A completed effect on support E changes only projection in E. Two selected effects with disjoint supports therefore commute under projection: on an alternative in neither support both are identities; in exactly one support only that effect applies. Physical aliasing does not invalidate this argument if all mutable updates retain their specified conditions. An implementation that writes an unconditional shared cell does not meet the premise.

This gives a precise use for serialization: a single commit sequencer can implement the conditional updates without concurrent races while computations prepare effects independently. It does not sequentialize the language semantics or impose source statement order. More concurrent commits require another concrete synchronization proof, not another source-language rule.

## Preservation by transition case

**Introduce.** On E, move one selected pending user constraint to the identified store, allocate a fresh occurrence identity, and retain the same argument references. Birth of that occurrence is E. Those arguments already existed under E because the pending goal was well-formed. The projected transition is exactly Introduce; other alternatives retain their goals and stores.

**Apply.** The exact matcher supplies a nonbinding rule-variable witness, live distinct head identities, an entailed guard and absent token on E. Removing consumed membership only on E and adding the token there implements the reference resource update. Instantiate head variables through the witness and allocate body-only variables freshly with birth E. Thus every body reference is valid there and the posted goals are the reference body, up to permitted fresh renaming. Apply itself does not alter the projected substitution.

**Unify.** Work on the frozen projected equations and binding graph. Equal representatives erase, matching constructors decompose, clashes fail, and an unbound representative is bound only where the contextual occurs test is false. The T011 path lemma identifies exactly the contexts in which a back-edge would create a projected cycle. Each elementary successful step preserves the set of unifiers; finite elimination produces an MGU on successful contexts. Publish the complete composition there and failure on the remainder. Private tentative bindings cannot enable a rule in an ultimately failing equation before commit.

**Split.** Replace each affected parent alternative by its two children, add only the chosen body arm to each, and inherit its other state. Pull every existing support back along the child-to-parent map. Pullback preserves intersections, unions and difference, so all membership, endpoint-birth and token relationships are inherited. Assign a fresh dynamic choice identity; earlier choices retain their correlations. Newly allocated child-local objects subsequently receive only their own contexts. One common symbolic split can represent many mutually exclusive parents while preserving two children per parent.

**Fail.** Remove the failed support from Live. Shared data may remain allocated. All assertions and publications are interpreted only on Live, so retaining that physical data does not affect surviving alternatives. Outstanding jobs for failed support are cancelled or restricted before commit/publication.

**Administrative work.** Exact condition normalization preserves the represented sets. Caching a result without committing its source effect, installing subscriptions, traversing a private equation worklist, and resuming a frozen scan leave source projection unchanged. Fresh physical allocations not yet reachable from live source state also leave it unchanged. Reclamation is safe only for objects unreachable from every retained live state, private continuation, cache entry still usable, and pending observation; the construction can conservatively omit reclamation.

A Split may be committed before another operation selected on a disjoint parent support. Lift that operation's frozen support and references by the same pullback. Since its parents were disjoint from the split support, its projection is unchanged. Thus commit order does not introduce an unaccounted interference case.

## Whole-machine safety theorem

Induct over finite sequences of completed commits, allowing any number of representation-only administrative actions between them. Initialization establishes the invariant. Each transition case above preserves it and projects to one legal reference step on each affected alternative; disjoint commits preserve unaffected projections. Split extends the ledger by exactly the reference children, and failure removes exactly failed projections.

Consequently, each live runtime alternative has a permitted reference derivation. Grouping computation cannot generate a reference derivation that uses an unavailable occurrence, an already-used propagation token, or a binding from another alternative. The theorem is a forward simulation; it does not claim that all permitted schedules of a nonconfluent CHR program are explored.

For preservation of the chosen explicit search tree, selection must also cover every live alternative according to the chosen policy. Disjoint partitioning without coverage would be sound but incomplete. The exact finite selector supplies both coverage and disjointness. This is why correctness of an incremental matcher requires a completeness invariant in addition to tests that its returned matches are valid.

## Answers and liveness

On a frozen projected state, exhaustive finite tuple coverage certifies quiescence exactly when no unused enabled application exists and no pending source goal remains. The equality store must be consistent. There is no other selected or unpublished operation for that alternative in the round. Publication therefore observes precisely that quiescent reference state, including residual constraints and existential variables. Groundness of outputs is neither required nor sufficient.

Finite-round termination and fair resumable service give the finite-derivation progress argument in T015-symbolic-scheduling.md. Every successful alternative reached in finitely many transitions under the selected policy eventually reaches certification and the output queue, assuming sufficient memory, terminating primitives and a draining consumer. An infinite sibling's recursive source steps do not form one infinite primitive.

Structural alpha-equivalence of finite output/residual observations is a sufficient deduplication test. Comparing normalized substitutions and residual multisets under one consistent renaming preserves all output alias relationships. Failing to establish this equality retains both answers. Deduplication does not change the pre-dedup alternative ledger or authorize merging continuations with different histories.

## What this argument does and does not settle

The whole-machine composition question now has a stated invariant, all source transition cases, administrative cases, a coverage condition, and a liveness discipline. Further architecture comparison can use it as a reference construction rather than waiting for T013 measurements. The construction still needs independent mathematical review, especially the boundary between exact symbolic matching and effect commit.

An executable version must establish that its concrete condition representation, subscriptions, fresh identities, private updates and scheduler implement these operations. T013's differential checks can find violations, while code review and invariant instrumentation support the correspondence. Neither successful tests nor this paper proof establishes net speed, memory scaling on realistic synthesis, or the adequacy of arbitrary extra guards.
