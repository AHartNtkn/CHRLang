# Renaming and dead-history projection enable distinct continuation reuse

**Whole-state renaming now reuses future work that exact identity keys miss.** A separate projection of dead rule-history tokens enables another case that renaming alone cannot merge. These are correctness and work-count results; no comparative lifecycle timings have run.

The [selection review](S08-observation-breadth-review.md) identified generalized continuation reuse as a distinct way to avoid recomputation. This source gate implements two mechanisms without treating either as a complete answer to call-level or general relevance-based reuse.

## What runs

| Mode | Key and execution responsibility |
|---|---|
| Direct | Execute each queued derivation separately. |
| ExactIds | Reuse complete transitions only when existing state keys match, including absolute IDs and fresh counters. |
| Alpha | Rename variables and occurrence IDs consistently across the complete state. Retain pending work, live resources, outputs, ordering and all history. |
| AlphaLive | Before renaming, omit only history tokens containing an occurrence that is no longer live. All fully live history remains. |

Each logical derivation keeps its own queue entry. A shared transition can execute once and replay for several entries, including later splits. Complete raw answers are now available separately from unique-answer presentation, so equality of unique answer sets cannot hide lost multiplicity. The current implementation still retains owned exported keys and cached transition/answer data; those costs must be measured.

## Why these keys are sound within this executor

Variable renaming traverses pending work, the ordered live store and named outputs with one map. Repeated variables stay repeated across all roots; constructor names, arities and argument order remain. Occurrence renaming is monotone across IDs appearing in the store or history, preserving selection order and history references. Pending goal order, rules and named output labels are not changed.

Fresh counters are omitted as absolute numbers only in the canonical key. The retained representative cursor keeps its actual counters and bindings. New identities therefore remain fresh within that entire representative state. Replayed complete answers preserve the named outputs and their joint alias structure up to permitted renaming. There is no omitted caller into which these internal identities must be transplanted: this is whole-state reuse. Call-level reuse will need its own transport and effect argument.

Dead-history projection has a narrower argument than general state irrelevance. A future application can use only live occurrences. IDs are fresh and never reused, so a history token containing a consumed occurrence cannot block any future application. Tokens whose occurrences are all live remain, including propagation history. The representative cursor may retain dead tokens physically; they cannot affect its future applications.

The argument assumes the existing finite-tree source contract and its opaque internal identities. It does not justify arbitrary resource projection, reordering live occurrences, ignoring propagation history, merging output aliases, or replacing caller-specific resource effects. Owned key export and canonicalization are implementation choices, not an architectural requirement.

## Independent source evidence

The renaming witness takes two paths with different pending work and fresh variable names, then produces the same pair containing a repeated unknown. Direct and exact-ID execution each perform 13 source steps; Alpha and AlphaLive perform nine. Both raw answer lineages and the single unique answer remain.

A second witness consumes an extra temporary occurrence on one path. Direct, ExactIds and Alpha each perform 15 steps. AlphaLive performs 12. This establishes why renaming and relevance projection are distinct: retaining the extra history token prevents the first mechanism from recognizing reuse. The initial renaming-only probe correctly produced no hit here; the evidence motivated the separately justified projection.

The independent owned-syntax scalar oracle checks 384 source configurations, each under all four modes: 1,536 executions. The matrix varies all ordered pairs of eight branch bodies, three source-variable offsets and both branch orders. Cases include equal and distinct aliases, fresh replay variables, token consumption versus absent tokens, one versus duplicate residual occurrences, live propagation and incompatible bindings. Every complete raw answer multiset agrees with the independent oracle. The two focused witnesses also agree with the independent reference interpreter.

Existing registered continuation cases run under all four modes, preserving finite prefixes, exhaustion, failed branches and raw completion counts. The full persistent/reuse package suite passes 64 tests; the seven continuation tests also pass with metrics disabled. Scoped strict Clippy passes. The [receipt](s05-generalized-source-gate/audit.json) records source hashes, commands and logs. Reference-interpreter code is unchanged.

## What this does not establish

Fewer executed steps do not establish lower time, allocation or retained memory. Canonicalization currently exports owned syntax, scans history and builds maps. Replayed answers also participate in raw and unique presentation. All four controls expose the same presentation responsibilities; future measurements must charge them explicitly. Existing earlier timings do not describe this new raw-output API. The reuse wrapper currently updates its own work counters unconditionally, so disabling dependency metrics alone is insufficient for primary timing; establish a counter-free configuration before the cost pilot.

These tests establish favorable witnesses and bounded adversarial checks, not a proof over arbitrary CHR programs or an architectural preference. Call-level keys, broader relevance projection, eviction, changed-query reuse and long-lived retention remain open. Renaming is deliberately conservative about occurrence ordering and fully live history; missing additional equivalences is an opportunity to investigate, not a correctness failure.

## Next experiment and competing priorities

Continue T075 with a registered lifecycle pilot now that its distinctive mechanisms actually run. Compare Direct, ExactIds, Alpha and AlphaLive on true reconvergence, unique futures, irrelevant-history growth, retained resources, fresh aliases and useful work after convergence. Vary depth, duplicate multiplicity, key size and useful continuation work independently. Include preparation, key construction, lookup, replay, complete raw observation, cancellation and disposal; measure work and requested allocation separately from counter-free timing. Inspect retention and owned-key costs if they could reverse a conclusion. Use a competent compiled control and applicable graph/direct-source control for the same source contract, not only the current generic cursor loop.

This is selected ahead of sustained S08 observation because it can now measure a distinct mechanism with positive source-work evidence and explicit adverse controls. Sustained S08 is the strongest readily executable alternative; remaining S02 organizations require a distinct source/representation gate. Reconsider both after the bounded cost package rather than extending canonicalization automatically. T075 remains active, and the broader architecture goal remains open.
