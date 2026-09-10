# Competing execution paths now agree on the same mixed sources

Ten existing Rust configurations pass all 125 common-source cases, reusing prepared rules across changing queries. Prepared prefix lowering passes its 20 admitted cases, and the finite-phase solver with its caller bridge passes four. The native path's existing source evidence remains valid for the same cases.

This establishes a common correctness entry for an architecture comparison. No native-versus-Rust timing or allocation matrix has run; the next obstacle is credible accounting for preparation, reuse and ownership.

## The common comparison

The sources combine equality, occurrence replacement, ordered propagation history, alternatives, consuming effects, failure and finite siblings beside ongoing work. They comprise the 48 choice/identity cases and 77 deterministic identity cases already specified independently. The gate groups them into 22 identical ordered rulesets and executes each group's changing queries using one preparation.

| Execution organization | Complete cases passing | Unsupported cases |
|---|---:|---:|
| Conventional Global Scan | 125 | 0 |
| Conventional Global Indexed | 125 | 0 |
| Inferred specialization, Scan | 125 | 0 |
| Inferred specialization, Indexed | 125 | 0 |
| Contextual equality | 125 | 0 |
| Shared contextual deductions | 125 | 0 |
| Persistent contextual equality | 125 | 0 |
| Persistent equality with shared deductions | 125 | 0 |
| Resumable contextual execution | 125 | 0 |
| Direct conditional execution | 125 | 0 |
| Prepared pure-prefix lowering with Scan | 20 | 105 |
| Finite-phase solving with the existing caller bridge | 4 | 121 |

Every accepted run checks complete raw answer multiplicity, residuals, joint output aliases and exhaustion. Ongoing cases must publish their expected finite answers without claiming exhaustion. A matching set of unique answers alone would be insufficient.

These are configurations of several organizations, not twelve independently designed architectures. Their common observations allow subsequent costs to be compared without assuming that a subsystem benefit composes. The reference remains independent correctness evidence rather than another performance baseline.

## Source-derived competitors apply to a meaningful subset

Prepared prefix lowering eliminates a private acyclic prefix in three ruleset groups: independent choices, alias-dependent competing consumption, and the deterministic binder-first effect-order cases. It passes all 20 associated queries. Artifacts are cached by ordered query predicate/arity shape; changed variable numbers and aliases do not force a new artifact, while changed shapes do.

The remaining 19 groups have no prefix admitted by the existing transformation. The adapter initially called `unwrap` on that expected admission error, producing recorded panics. The corrected adapter reports the exact unsupported reason. The lowering algorithm is unchanged, and all previously accepted observations replay exactly. These are admission limits, not measured runtime losses or a rejection of broader source analysis.

The finite-phase admission probe tests every nonempty source prefix. Only the independent-choice group is admitted. Its complete solver-plus-caller path passes all four queries, including complete answer multiplicity. This control can remove source execution in that regime and therefore belongs in the cost comparison; merely comparing two engines that enumerate the same work would omit an applicable alternative.

Other finite prefixes reject propagation/private-head structure, non-producer alternatives, or absence of a finite producer. Those precise boundaries are recorded. Richer resource solving and broader admission remain required investigations.

## Native preparation currently has a different boundary

The native compiler currently emits query initialization together with matching functions. Its predicate and atom dictionaries include the supplied query, and `@main` embeds the initial store, output variables and fresh counters. Changing a query can therefore change the emitted program, even when source rules are unchanged.

The current native executable initializes global tables and a heap, reads and parses the emitted program, resolves its entry, executes and observes it, then frees the runtime. Static definitions and dynamic execution occupy the same heap allocation. Existing source gates demonstrate fresh process execution; they do not establish a reusable prepared ruleset with independent query disposal.

The Rust gate genuinely reuses prepared rules. Conventional preparation owns rule structures and dispatch/index plans. Contextual preparation owns rules and arrival maps. Conditional preparation owns its source plans. Each start creates query execution state, while completion transfers owned answers for observation. Prefix lowering additionally owns query-shape artifacts; finite solving owns a prepared phase and caller bridge.

Comparing a native execution-only interval with those complete Rust paths would omit material costs. Conversely, repeatedly charging native process startup and source emission would measure the current process organization, not prove that native graph execution intrinsically cannot reuse preparation. Both distinctions need explicit evidence.

## What the complete-cost runner must establish

| Boundary | Required measurement or correctness evidence |
|---|---|
| Source input and analysis | Source construction, native emission and applicable specialization/lowering analysis, each charged to its actual owner |
| Loading and preparation | Native file/buffer loading, parsing and runtime initialization separated from query execution; Rust rule preparation measured separately |
| Changing queries | Query construction and setup, dictionary changes, and any query-dependent regeneration; no assumed native amortization |
| Reusable artifacts | Retain only demonstrated reusable rules/plans; charge query-shape artifacts and verify their invalidation boundaries |
| Execution and observation | First answer, all answers, complete residual/alias validation outside timing, plus exact publication costs |
| Cancellation and disposal | Pending work, answer retention, query state, prepared artifacts and runtime teardown accounted for separately |
| Resource diagnostics | Counter-free ordinary timing qualified independently; work and requested heap allocation measured in separate configurations, with RSS kept distinct |
| Source-program compilation | Identify what is emitted, parsed or compiled; the runtime executable's build is not per-source native compilation |

This is a measurement design, not a registered timing matrix. Exact source sizes, useful-work variations, repetitions, practical thresholds and resource bounds must be registered after the runner's semantic and ownership gates. The present tiny correctness cases alone cannot supply representative architecture costs.

## Why native ownership is the next investigation

Continue T078 by testing a reusable native preparation boundary and its disposal invariants. Determine whether parsed rule definitions can survive changing query inputs while query state and observations are reclaimed. Challenge new atoms/predicates, output variables, cancellation and retained answers. If that boundary cannot safely be separated in this representation, record the concrete cause and measure the existing complete path without pretending the problem is solved.

This is more decision-relevant now than another engine timing matrix: ten configurations already agree, but an unqualified preparation boundary could dominate or invalidate the comparison. It is also a lower-cost prerequisite to the common whole-path study than implementing local concurrent native claims. General terms, integrated guard/search support, learning/reuse and local ownership remain required; this priority does not resolve them.

After the ownership gate, register the bounded complete-cost pilot with conventional Scan/Indexed, applicable specialization, contextual/conditional candidates and admitted lowering controls. Select candidate reductions only through explicit equivalence or evidence, not a preferred incumbent. Describe language restrictions and necessary responsibilities alongside efficiency.

## Validation and evidence

The final matrix contains 264 grouped processes and 1,500 configuration/source checks: 1,274 complete passes and 226 explicit unsupported cases. There are no final process failures or observation mismatches. The audit also verifies 220 initial process replays, the 19 adapter admission corrections and 264 exact observation replays following a lint correction. Native source/program/archive validation passes again without rerunning unchanged native experiments. Scoped strict Clippy and formatting pass.

The Rust build uses `--no-default-features`; its dependency feature tree is recorded. That flag alone does not certify every contextual diagnostic path as counter-free. No cost conclusion is drawn from this correctness runner.

[Registration](../registrations/S10-native-common-source.md), [runner](../../../research/chr-direct-conditional/examples/native_common_source.rs), [source groups](s10-native-common-source/groups.json), [raw process records](s10-native-common-source/runs.jsonl), [per-source outcomes](s10-native-common-source/results.json), [audit](s10-native-common-source/audit.json), [finite admission probe](s10-native-common-source/finite-admission.json), [input hashes](s10-native-common-source/validation.json), [feature tree](s10-native-common-source/features.txt), and [Clippy](s10-native-common-source/clippy.log) retain the evidence. The [native composition report](S03-native-choice-identity.md) records the native scope and limits.
