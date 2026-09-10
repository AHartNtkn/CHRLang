# Why designs were set aside—and what the experiments actually establish

The [combined native session](S10-host-session.md) validates 479 owned query endpoints across 26 sessions with artifact cleanup and unchanged native work signatures. Host artifact writing, process transport and batch publication now have measured boundaries; gross process elapsed remains distinct from nested phases. Allocation diagnostics and clock calibration remain before comparative registration.

The [native host frontend qualification](S10-host-frontend.md) reconstructs common source syntax, reproduces 26 frozen native programs and replays 479 queries exactly. Host emission/query encoding now have explicit diagnostic intervals. Combined host transport/lifetime, allocation scope and clock calibration remain before cost registration; no comparative ranking follows.

The [Rust lifecycle qualification](S10-rust-lifecycle.md) validates 2,286 admitted checks, 587 explicit exclusions and cancellation followed by reused preparation in all 13 modes. Full outputs match prior runners and independent expectations. Native host/frontend accounting, separate allocation diagnostics and clock calibration remain before comparative registration; no architecture ranking follows.

The [common owned-answer gate](S10-answer-wire.md) validates 958 native queries and 2,286 admitted Rust checks against a shared binary format. Consumer bytes survive producer disposal; dictionaries remain explicit owners. Checked finite solving remains available on early-readiness sources. T078 next integrates this endpoint into full lifecycle measurement; no comparative cost matrix has run.

The [native choice/identity composition gate](S03-native-choice-identity.md) validates 48 mixed sources, all 77 deterministic regressions and explicit failure/namespace boundaries. Finite siblings progress beside ongoing work. The common-source gate above now supplies the next comparison entry; local native ownership, broader language and lifetime remain required.

The [native identity-bearing source gate](S03-native-identity-source.md) passes 77 complete sources against independent expectations and the unchanged reference. Serial matching, replacement, equality and ordered histories now execute together. The composition gate above now tests interaction with choices and failure; local ownership and broader architecture comparisons remain required.

The [native identity operation gate](S03-native-identity-kernel.md) validates 340 cases separating equality, occurrence IDs and ordered history, including nonbinding probes. These are executable prerequisites, not native source correspondence. The complete source gate above now exercises those operations together; broader architectures remain unresolved.

The [controlled size/reuse extension](S02-multihead-scale.md) adds 52 qualified graph-scan gains, eight unresolved comparisons and no losses against Scan. It strengthens a bounded integrated candidate while leaving broader language and memory tradeoffs open. The next investigation qualifies native identities and equality; no architecture is selected.

The [general integrated matching lifecycle](S02-multihead-lifecycle.md) now supplies a positive timing result for local graph scanning against conventional Scan in the controlled width 64/four-query cases. Larger heap traffic does not imply slower execution. Retained joins have narrower benefits and dense-setup losses; broader sizes, language support, ownership alternatives and complete architecture comparison remain required.

The [finite-solver lifecycle pilot](S06-finite-lifecycle.md) completes 1,428 processes across 204 configurations. A [paired allocation correction](S06-empty-complement-attribution.md) completes another 252 processes. Selective four-query allocation falls to 0.795 MB, below both conditional orders; the unselective batch still requests 15.77 MB versus specialization’s 9.24 MB. These are bounded results, with compilation and broader solver questions still open. Complete costs for broader resources, learning and native compilation remain required; no solver family is selected or rejected.

**The experiments support several narrow rejections. They do not support treating every alternative architecture as resolved, or the research goal as complete.** Some implementations lost measured comparisons. Some proposed translations changed program behavior. Other directions received no direct experiment: I stopped investigating them on a judgment about the value of further work.

This report explains those distinctions design by design. It audits the decisions recorded through commit `a40b7de`, including directions described as rejected, unsuitable as a default, unselected, or deferred. Related variants share an entry where the same explanation applies. A design that won a bounded comparison is included when broader adoption was nevertheless set aside.

There is **no experimental finding that the language needs a second production baseline**. The additional executors supplied experimental controls: they helped distinguish an architectural effect from a weakness in an implementation. That role does not establish which implementation should become the language's architecture.

## Reading guide

Each numbered entry gives the result, the reason I stopped or limited adoption, and the conclusion the evidence can support. The status labels mean:

- **Measured limit:** a particular implementation lost, or its advantage depended on the workload.
- **Incorrect transformation:** a concrete example showed changed behavior. This rejects that transformation under the tested contract.
- **Unresolved comparison:** measurements did not establish an ordering, or stopped at a resource bound.
- **Not directly tested:** no experiment resolved the proposed design. The reason for stopping was a research-priority judgment.
- **Positive result, limited scope:** the design worked or won within a stated boundary; its wider applicability remains open.

“Lower observed ranges” means the recorded timing ranges did not overlap under that experiment's interpretation rule. It is not a confidence interval or a prediction for other machines. Early E-series work often measured operations and requested allocation bytes, with timing only diagnostic. Requested allocation bytes measure heap demand, not RSS. Different experiments used different source freezes and controls; their numbers cannot be assembled into a single ranking.

### Jump to a question

| Question | Entries |
|---|---|
| How should rules find and execute work? | [1–8: execution and access](#execution-and-access) |
| Should equality, terms and execution form one graph? | [9–16: representation and integration](#representation-and-integration) |
| How should alternatives share work and state? | [17–25: search and storage](#search-and-storage) |
| Can solving or compilation avoid execution? | [26–34: solving and compilation](#solving-and-compilation) |
| When should results or failures be reused? | [35–40: reuse](#reuse) |
| What must observations and scheduling preserve? | [41–47: observation and progress](#observation-and-progress) |
| Is parallel execution worthwhile? | [48–50: parallelism](#parallelism) |
| Which language restrictions were considered? | [51–54: language design](#language-design) |
| What remains unsupported in the overall decision? | [55–57: architecture and stopping](#architecture-and-stopping) |

## Execution and access

### 1. Scanning every possible partner — Measured limit

**Result.** Selective chains became much cheaper with activation and indexing. In the first R01 pilot, a size-64 flat chain took about 118 ms with global scanning and 6.46 ms with active indexed execution across four queries. However, global scanning was cheapest on the recursive-build case, and small workloads exposed indexing overhead.

**Decision and limit.** I set aside scanning as a general access policy. The evidence supports avoiding large irrelevant searches when usable keys exist. It does not support removing scanned execution or using an index on every path. [R01 pilot](R01-native-pilot.md)

### 2. Always enable indexes and activation — Measured limit

**Result.** On the active recursive-build path, no indexed bucket was read, yet maintaining indexes increased execution allocations from 3,216 to 11,863. Active low-yield binding repair visited the same 192 candidates with or without indexing but allocated more with it. Small chains also favored scanning.

**Decision and limit.** Universal indexing was not supported. This establishes a need to consider available information and maintenance cost together. A general adaptive access planner was not measured, so the experiment did not select its policy. [R01 pilot](R01-native-pilot.md)

**Additional scheduling result.** Activation could also postpone newly enabled rejection behind queued choices. In R03's size-eight chain, that produced 6,561 leaves, while restarting global rule selection produced only 44 splits. I used the global early-rejection control rather than blame that frontier on snapshot storage. Promoting rejection within an active queue was not comparatively tested. [Search diagnosis](R03-search-diagnosis.md)

### 3. Enumerate partners before using the active head's information — Measured limit; corrected

**Result.** The initial matcher could know the active occurrence's key but fail to use it while searching earlier heads. Making that information available first reduced size-64 chain candidate visits from 16,838 to 772 and lifecycle from 7.02 to 2.31 ms. Equal-key collisions received extra work without useful pruning.

**Decision and limit.** I rejected that unnecessary search in the experimental control. This was an implementation correction, not evidence against graph execution or in favor of a new production baseline. Nor does it prove that preliminary matching always pays. [Anchor comparison](R01-anchor-pilot.md)

### 4. Maintain full or selectively invalidated match prefixes — Measured limit and unresolved larger cases

**Result.** E09's selective maintenance avoided much repeated structural matching, but still repeatedly visited cached extensions. At a completed size-64 case, roughly 79% of counted actions were extension visits and retained-test checks. Thirteen of 24 sizing cells reached the action bound. Full invalidation often performed more work than recomputation.

**Decision and limit.** The tested prefix-maintenance organization did not resolve discovery efficiently. The cutoffs are unfinished measurements, not completed losses. Update-driven subscriptions, retained joins and other indexing organizations were not disproved. [Maintenance sizing](E09-maintained-sizing.md), [semantic gate](E09-maintained-gate.md)

### 5. Retained joins or subscriptions instead of repeated discovery — Not directly tested as a general design

**Result.** The later T057 screen found that current Active+Indexed execution serviced each additional keyed request with seven candidate visits and ten cursor steps, independent of the two registered table sizes. That left little size-dependent discovery for a retained join to remove in this particular family. It was a work-count screen, not a timing or heap comparison with retained joins.

**Why I stopped.** I judged implementing retained joins unnecessary for that keyed case, then required a different motivating workload before continuing. That supports stopping this particular experiment. It does not resolve weakly keyed, many-to-many or update-heavy joins, or establish that retained joins are generally too complex. [Current join screen](R01-current-join-screen.md), [other matching directions](A3-matching-directions.md)

### 6. Generated versus generic matching as the main optimization — Unresolved broad comparison

**Result.** Initial generated/generic differences were smaller than the large access-policy effects; representative size-64 differences fell below the registered 10% screen. Later checked specialization produced real gains by eliminating specific machinery, described in entry 30.

**Why I limited it.** I prioritized access and representation questions over more dispatch measurements. The early pilot does not reject code generation: it tested particular generated heads, and did not isolate compiling an independent user program. [R01 pilot](R01-native-pilot.md), [single-head specialization](R05-single-head-lifecycle.md)

### 7. Repeatedly traverse immutable ground structure — Measured work defect; corrected

**Result.** Index maintenance repeatedly traversed unchanged constructor suffixes. At depths 8, 16 and 32, each dependency/key traversal made 54, 170 and 594 visits. Cached structural closedness reduced these to 18, 34 and 66. A related occurs-check correction changed repeated closed-subtree traversal to linear visits in its regression.

**Decision and limit.** Recomputing those immutable properties was unnecessary. The index correction earned separated lifecycle improvements in four carry-heavy specialized cases; the occurs-check correction alone had no isolated lifecycle measurement. This is evidence for immutable metadata, with its storage cost, rather than a ranking of complete representations. [Structural maintenance](R01-structural-maintenance.md), [paired lifecycle](R01-closed-subtree-lifecycle.md), [occurs check](R01-closed-occurs.md)

### 8. Broader selective plans, dependency wake-ups and discrimination structures — Not directly resolved

**Result.** The access experiments identify both avoidable search and adverse maintenance. They do not compare every combination of generated discrimination, subscription-based wake-up, join ordering and selective index construction.

**Why I stopped.** I treated further work as refinements of the conventional executor and gave broader representation experiments priority. That explains sequencing. It does not establish that these choices could no longer materially affect the architectural comparison. [Matching directions](A3-matching-directions.md), [architecture assumptions](../architecture-assumptions.md)

## Representation and integration

### 9. Integrate equality, constructors, matching and consuming execution — Measured limit of the R02 implementation

The [source-derived nested-pattern gate](S02-local-pattern-gate.md) adds independently checked nonbinding matching and late-alias dependencies. Its broad control exposes avoidable conservative rechecking. The [breadth review](S02-local-rewrite-breadth-review.md) schedules call-level reuse next while retaining broader integrated execution and costs as required work.

The [local handle-rewrite gate](S02-local-ports-gate.md) subsequently tests direct handle repair and attached-request activation without a parent forest. It establishes bounded source correctness, not a general strategic-rewrite architecture or a cost ranking. Broader source rules and lifecycle remain required.

The subsequent [CHR-expressed constructor/consumption gate](S02-chr-constructors-gate.md) directly tests another mechanism: source-rule class merging and descriptor repair. It has independent bounded correctness evidence and an explicit resource-scheduling boundary, but no comparative costs. Strategic local rewriting and broader source correspondence remain unresolved.

**Result.** R02 demonstrated real interleaving: source rules could run while equality deductions and constructor repairs remained pending. After correcting redundant congruence work, the size-64 nested case still took 2.05 ms versus 1.71 ms for dedicated scanned execution. At size 256, the relevant stronger indexed control also beat nested integration; batch timing remained inconclusive. Integrated storage was higher in the reported comparisons.

**Why I stopped.** I judged the demonstrated integration benefit insufficient to repay this implementation's setup, maintenance and disposal. That supports a bounded negative cost result. It does **not** establish that separating equality is architecturally necessary, or resolve a different integrated representation that eliminates more boundaries. [Corrected integration comparison](R02-congruence-witness-pilot.md)

### 10. Connect every equivalent constructor to every peer — Measured work defect; corrected

**Result.** R02 initially queued redundant equalities among congruent constructors. One durable connection per repaired constructor reduced size-256 nested equality steps from 32,805 to 512 and lifecycle from 34.15 to 11.27 ms.

**Decision and limit.** I rejected the redundant connection policy. The initial large penalty must not be used as evidence against integration after its cause was repaired. The corrected comparison in entry 9 is the relevant result. [Congruence correction](R02-congruence-witness-pilot.md)

### 11. Maintain matcher structures for output-only occurrences — Measured limit; narrowed

**Result.** Keeping matcher memberships only for signatures appearing in rule heads lowered retained heap across several integrated cases. Fanout lifecycle improved from 1.406 to 1.248 ms; most timing comparisons remained inconclusive. Complete output occurrences still had to be retained for observation.

**Decision and limit.** I limited matcher ownership to occurrences that could participate. This reduced an implementation cost but did not close the remaining integrated-versus-dedicated gap or establish a universal representation preference. [Repair and maintenance pilot](R02-repair-maintenance-pilot.md)

### 12. Named choice families and eager supported partitions — Positive service result, limited scope

**Result.** Demand-driven named equality avoided distributing an opaque binding across 256 alternatives: one equation pair instead of 256. An eager, interned partition control sometimes performed fewer pairs than the named service, but paid much more to construct its projections: about 22.4 MB versus 0.135 MB of solver allocation in the correlated case.

**Decision and limit.** Eager projection was not supported merely because it reduced equation counts. The named service itself was not experimentally rejected; it showed useful sharing. It lacked dynamic births, consuming effects and complete CHR scheduling, so its favorable result could not select a whole architecture. Compressed partitions and symbolic equivalence before projection remain unresolved. [E05](E05.md)

### 13. Explicit net-encoded unification service — Measured limit of one encoding

**Result.** In E06's traced Python implementation, the net service had higher total time than every direct control on all 96 requests in both batches. Linear environment search, unary identifiers, explicit duplication/cleanup and retained node slots contributed concrete work. Even an identity equation became expensive when preserving unrelated stored data.

**Decision and limit.** I did not support this encoding as an efficient service. These results concern a generic graph rewriter and an explicit transactional data protocol. They do not compare optimized native graph execution with direct CHR compilation. Routed variable cells, shared immutable storage, binary identifiers, slot reuse and richer local rules were proposed but not established by this comparison. [Unification costs](E06-unification-costs.md)

**Lookup variants.** The earlier whole-table copy/erase protocol was directly improved by preserving the untouched tail and reconstructing only the visited prefix. A large first-entry request fell from 16,836 interactions to 75, with unchanged outputs. That rejects unnecessary whole-table copying in this protocol; it is positive evidence for a better net implementation, not a reason to carry the initial cost forward. [Preserving lookup ablation](E06-lookup-preserving.md)

### 14. Direct native superposition mappings without a correspondence protocol — Incorrect translations

**Current follow-up.** The [native structured observer](S03-native-structured.md) preserves tested correlation, raw multiplicity and joined failure with retained service. It still cannot recover disconnected source obligations. Bounded explicit labels do not qualify dynamic freshness, consuming-source correspondence or native lifecycle efficiency.

**Result.** Pinned HVM probes exposed label collisions correlating independent choices, copy labels selecting rather than copying, administrative superpositions differing from source births, and disconnected failure failing to affect an answer. Seven deliberately faulty encodings disagreed with their declared finite observations; thirteen encodings matched.

**Decision and limit.** Those mappings were invalid. The experiment identifies requirements for labels, ownership, failure and observation. It does not reject native named-choice execution with a correct protocol, and it provides no general CHR performance comparison. [Native correspondence probes](E06-native.md)

### 15. Wrap recursive backend work in a constructor or lambda to obtain a yield — Incorrect boundary assumption

**Current follow-up.** The [retained native service gate](S03-native-service.md) now returns finite leaf answers beside ongoing native reduction. It does not serialize whole source states. Reference-only cycles and traversal restart require an explicit quota contract. The [structured follow-up](S03-native-structured.md) now qualifies bounded correlation and completion; consuming source correspondence and complete costs remain unanswered.

**Result.** On the pinned backend, neither wrapper returned around the adverse recursive call within the bound. Source inspection explained why normalization still entered the work. Explicit finite data continuations did return the expected toy states.

**Decision and limit.** The wrapper was not a valid suspension boundary for that runtime. A generated data-controller implementation remained feasible but was not completed and costed as a general CHR engine. This does not establish that every graph backend needs whole-state serialization. [Continuation boundary](E06-hvm-boundary.md)

### 16. Direct choice graphs, derivation nets, pull-tabbing and richer local graph calculi — Not directly tested as complete alternatives

**Identity follow-up.** The [61 source-obligation configurations](S03-native-identity-obligations.md) distinguish occurrence history from current values and final equality from legal effect order. These are reference checks, not a native implementation. The breadth review selects integrated matching timing next while keeping native identity and local ownership required.

**Current follow-up.** The [native ground compiler](S03-native-consuming.md) now carries actual consuming source rules through native choices and retained observation, with independent complete residual checks. Its admitted count quotient does not qualify identity-bearing values, propagation, general equality or local claims/commits. Complete native costs remain unmeasured.

**Result.** E04 tested occurrence-local dispatch and expansion caching; E06 tested services and native mapping hazards; E18 established finite monotone relational feasibility. None supplies a matched whole-path result for a direct distributed choice graph, general derivation-net compiler, memoized pull-tabbing engine, or richer additive/multiport execution architecture. The direct-choice entry did not become a completed semantic gate.

**Why I stopped.** I prioritized other experiments and later described these as lacking a selected source need. That is a prioritization judgment. Neither a slow service encoding nor the existence of a conventional control is experimental grounds for rejecting these architectures. Their integration, correctness and total costs remain unanswered. [Direct-choice entry](../../goals/chr-experiments/notes/T021-distributed-choice-entry.md), [E18 gate](E18-relational-gate.md), [framing review](../research-framing-review.md), [earlier direction audit](../../goals/chr-sharing/notes/T018-final-direction-audit.md)

## Search and storage

**Other named representation alternatives.** Flat relational e-matching, contextual/colored equality overlays, union-find expressed through CHR, specialized incidence rewrites and strategic port graphs were identified as distinct possibilities. E18's explicit equality-closure gate established finite denotation, not their relative efficiency. R02 tests one integrated organization, not every one of these alternatives. There is no direct rejection result for each named design. [Representation assessment](E18-source-assessment.md)

**Current follow-up.** The [direct-argument pull-tab source gate](S03-local-pulltab-source.md) implements an actual local rewrite and passes bounded effect/progress checks after a resource-scheduling repair. The [work attribution](S03-pulltab-work.md) isolated a cache-validity advantage; the [matched dependency control and lift repair](S03-match-dependencies.md) remove the expansion difference in all 72 registered configurations. The [corrected lifecycle pilot](S03-pulltab-lifecycle.md) now shows zero practical gains, four losses and 76 unresolved configurations for local lifting under its criterion; the [fresh-derivation source gate](S03-fresh-derivation-source.md) now demonstrates multi-rule reuse with fresh identities and exposes a committed-scheduling boundary. Its costs remain unmeasured. Costs, broader demand and fresh derivation reuse remain unresolved; these bounded results supply no whole-architecture ranking.

### 17. Copied versus persistent snapshots — Positive storage result, limited scope

**Result.** E01 persistence saved snapshot allocations while performing exactly the same source work. In the largest carry case both modes performed 65,791 applications, and peak retained heap was effectively the same because full outputs dominated. Other cases had meaningful allocation and peak savings.

**Decision and limit.** Persistence was not rejected. The experiment rejected the inference that sharing snapshots also shares execution. Cross-engine timings could not isolate storage, and this comparison did not settle trailing, replay or reclamation. [E01](E01.md)

### 18. Eagerly group compatible ready work — Measured adverse case

**Result.** E03 reduced 65,791 physical expansions to 264 in a favorable common-work case. In a discriminating case it saved only eleven expansions, increased matching candidates from 8,975 to 140,563, and raised requested allocation from about 13.6 MB to 133.2 MB.

**Decision and limit.** Unconditional aggressive grouping was unsupported. Its recognition work could overwhelm reuse. That is evidence against this eager preview policy, not against all conditional execution or common-work sharing. A separate output-copying defect was corrected before interpreting the allocation results. [E03](E03.md)

### 19. Retain occurrence-expansion events for later contexts — Measured tradeoff

**Result.** E04 showed that caching could share an expansion even when a context arrived later. With sparse reuse, however, 1,028 retained entries yielded only eleven avoided expansions, while peak allocation rose from about 0.29 MB to 4.70 MB.

**Decision and limit.** I did not support universal event retention. This cache has a real reuse mechanism and a demonstrated retention cost. Eligibility, reclamation, contextual compilation and more general derivation sharing were not resolved by the sparse-reuse result. [E04](E04.md)

### 20. Delay splitting to perform common work first — Measured tradeoff

**Result.** E07's bounded lifting reduced a favorable case from 65,791 applications to 511. When both alternatives would immediately fail, a large quota instead performed 256 unnecessary common reductions. Unlimited preference spent its entire bound on a looping producer without servicing the finite refutation.

**Decision and limit.** An unconditional preference for delay was unsupported. Finite quotas preserved progress in the tested witnesses. Adaptive quotas, nested child boxes, early failure and promotion were not compared as complete policies. [E07](E07.md)

### 21. Assume current variable disjointness proves independence — Incorrect certificate

**Result.** A branch-local binding could later enable consumption of a common ground occurrence. Checking only variable writes would therefore authorize invalid lifting. Factoring also needed to account for future calls and linking, not just the present variable graph.

**Decision and limit.** I rejected that insufficient certificate. It does not establish that all useful independence analysis must use the prototypes' conservative whole-predicate grouping. Finer ownership and future-effect analyses remain possible. [Delayed splitting](E07.md), [factoring](E08.md)

### 22. Permanently factor independent regions — Positive result, limited scope

**Result.** Eight independent components reduced source applications from 524,543 to 4,104 and requested allocation from about 1.95 GB to 12.4 MB. Small or connected cases offered little benefit or extra overhead. Returning the full Cartesian product still required output work.

**Decision and limit.** Factoring was retained as useful for certified independence, but not made universal. The positive result does not depend on workers. Finer same-predicate decomposition, persistent fact interfaces and long-stream cache policies remain unmeasured alternatives. [E08](E08.md)

### 23. General direct Conditional execution — Mixed results, not a general rejection

The [finite-solver lifecycle pilot](S06-finite-lifecycle.md) completes 1,428 processes across 204 configurations. A [paired allocation correction](S06-empty-complement-attribution.md) completes another 252 processes. Selective four-query allocation falls to 0.795 MB, below both conditional orders; the unselective batch still requests 15.77 MB versus specialization’s 9.24 MB. These are bounded results, with compilation and broader solver questions still open. The stronger source-derived control changes the selective allocation comparison; it does not reject general Conditional execution. The [earlier qualified controls](S10-arrival-lifecycle.md) retain explicit alternatives, and explicit execution remains stronger on the low-sharing stream.

**Result.** R03 showed favorable opaque shared work and adverse ordinary computation. A real maintenance defect initially exaggerated some losses; filtering consumed work materially changed the result. Later mixed-work tests favored Conditional for heavy common work before discrimination, while specialized explicit execution won eleven of fourteen placement/reuse pairs. Pure-carrier lowering then changed that comparison again. Finally, substantive equality favored Conditional in all six common pre-discrimination clash cells, while explicit controls won the other eighteen cells.

**Decision and limit.** These results reject “fewer source applications means lower total cost.” They do not establish a universal explicit winner. Conditional has demonstrated advantages and costs whose relevance depends on what work is shared, when it fails, and what a compiler can eliminate. Treating those exceptions as necessarily unimportant would require workload assumptions the research does not supply. [Maintenance correction](R03-conditional-maintenance.md), [mixed work](R05-current-mixed.md), [carrier costs](R05-carrier-prefix-cost.md), [substantive equality](R05-equation-cost.md)

**Support and ownership variants.** R03 chose a Boolean decision DAG and a serial source-effect owner. Explicit lists of branch tickets were excluded from that experiment because constructing them would already enumerate the alternatives whose enumeration it sought to avoid. A term-only conditional graph lacked consuming-resource and completion state; a fully distributed commit protocol was not implemented. These are experiment-boundary reasons, not measured proof that every alternative support or ownership design is inferior. [Protocol choices](R03-conditional-protocol.md)

### 24. Copy the whole arena at each fork, or universally use copy-on-write — Measured tradeoff

**Result.** Attribution found arena copying accounted for 36.6% of split-requested bytes in the large mostly-failing case. Copy-on-write then improved eight of 32 paired comparisons, ordinary cloning won one, and 23 overlapped. Read-heavy reuse benefited; all sixteen insertion comparisons overlapped. Later carrier and substantive-equation comparisons also failed to separate the ownership choices.

**Decision and limit.** Copying was not intrinsic to explicit search, and copy-on-write was not universally better. The tested optimization shares immutable arena storage while retaining separate mutable bindings. It does not settle broader state ownership or compare all persistent, trailed and replay-based designs. [Owner attribution](R03-fork-owner.md), [arena comparison](R03-arena-ownership.md)

### 25. Trailing, replay, snapshot intervals, temporary separation and reunion — Not directly resolved

**Result.** No matched architecture comparison established the relative costs of these alternatives. R04 uses trailing inside a finite solver, but that is not a comparison of general CHR search storage. Permanent factoring does not test regions that later reconnect; branch cloning does not test replay.

**Why I stopped.** I selected measured copying owners and immutable arena sharing as a bounded intervention, and did not continue into these broader designs. That is a sequencing decision, not negative experimental evidence. Their recomputation, rollback, retained-state and reconnection tradeoffs remain open. [Storage receipt](E01.md), [factoring receipt](E08.md), [state-preservation design](R03-state-preservation-design.md), [temporary decomposition analysis](../../goals/chr-sharing/notes/T017-temporary-decomposition.md)

## Solving and compilation

### 26. Memoize closed structural operations — Measured tradeoff

**Result.** On repeated immutable input, E10 reduced 524,224 node computations to thirteen. On structurally similar but unshared input it avoided no computations and added about 41 MB of reduction allocation. Immediate failure also supplied no reuse benefit.

**Decision and limit.** Always memoizing was unsupported. Identity and repeated work matter; shape alone does not guarantee profitable reuse. The experiment supports a closed service, not a general solver for changing logical variables. [E10](E10.md)

### 27. Intersect constrained term spaces before enumeration — Measured tradeoff

**Result.** Intersection eliminated an empty space cheaply and helped selective restrictions. For overlapping restrictions, its nondeterministic grammar generated 3,228 candidates versus 1,355 for the filter-first control. Choosing the smallest filter also incurred the cost of constructing domains.

**Decision and limit.** No universal intersection or filter policy followed. Lazy generation, less redundant automata, arbitrary finite path constraints and dynamic equality-constrained automata were not resolved. The bounded skeleton experiment was not a general ECTA compiler. [E10](E10.md)

### 28. Encode bounded interpreter traces in a solver — Measured loss for that encoding

**Current follow-up.** The [source-derived finite lifecycle](S06-finite-lifecycle.md) now measures an admitted atom-domain phase with resumed caller execution. Selective gains and cold/unselective costs are both recorded. This does not resolve broader theories, compatible-query learning or native compilation.

**Result.** E11's term-expression symbolic encoding cost more than its direct arena control on every completed workload. Relational decomposition timed out at 60 seconds in all three repetitions while the direct control completed in about 0.028 seconds. Construction, solving and observation all mattered.

**Decision and limit.** This was a defensible negative result for the tested trace encoding and bounds. It does not reject compiling the relation directly, solver learning, compatible-prefix reuse, or other representations that avoid encoding interpreter transitions. [Matched comparison](E11-matched.md), [projection follow-up](E11-projection-followup.md)

**Earlier symbolic representations.** Dense heap/handle encoding also encountered construction bottlenecks. Concrete projection preserved the checked bounded outcomes and reduced a diagnosed cost; equality microstep pruning enabled the registered type prefix, while addition remained unresolved. This led to the term-expression comparison above. Compatible-prefix reuse, larger bounds and more direct relation encodings did not receive a decisive comparative result. [Projection and encoding follow-up](E11-projection-followup.md)

### 29. Use support-CNF or conflict-CNF for the tested finite relation — Measured loss to the native finite solver

**Current follow-up.** The [source-derived finite lifecycle](S06-finite-lifecycle.md) now measures an admitted atom-domain phase with resumed caller execution. Selective gains and cold/unselective costs are both recorded. This does not resolve broader theories, compatible-query learning or native compilation.

**Result.** R04's native trailed finite solver had the lowest lifecycle in all twenty cold/reuse cells, with separated ranges against both Boolean encodings and the explicit CHR controls. Independent exhaustive assignments and source correspondence checked the relation being solved.

**Decision and limit.** Neither CNF encoding was preferred for this bounded relation. This positively supports avoiding CHR execution where the relation has a suitable certificate. It does not reject SAT-based compilation for different relations or larger instances, and it says nothing general about learned constraints or arbitrary synthesis. [R04 lifecycle](R04-lifecycle-pilot.md), [source correspondence](R04-source-correspondence-gate.md)

### 30. Bounded entry unfolding versus checked single-head specialization — Early loss, later positive result

**Result.** E13's first unfolding compiler reduced dispatch but did not repay compilation allocation within three searches on any terminating workload. It constructed substantial specialized syntax. Later R05 specialization eliminated generic cursor and history work for eligible single-consuming heads: medians improved in fifteen of sixteen comparisons, nine with separated ranges.

**Decision and limit.** The early result limited that unfolding implementation, not specialization. The later result supports eliminating specific runtime responsibilities under checked source properties. Neither establishes unrestricted partial evaluation or a universal code-generation policy. [E13 costs](E13-costs.md), [R05 single-head lifecycle](R05-single-head-lifecycle.md)

### 31. Native compilation of the tested recursive fragments — Measured amortization limit

**Result.** Checked Direct execution captured most of the gain over Specialized. Native generation improved three Direct session cells with separated ranges, but did not recover its additional compilation cost relative to Direct within any measured session. The study included 324 sessions and validated 62,316 responses.

**Why I stopped.** I judged another same-boundary reuse sweep less valuable without a deployment reason for much greater reuse. The experiment establishes a measured break-even limit, not that native compilation can never pay. Other deployment lifetimes, generated representations and programs remain unresolved. [Recursive lifecycle](R05-recursive-lifecycle.md)

### 32. Eagerly evaluate a region because dispatch is unique — Incorrect transformation

**Result.** Concrete contextual examples changed answers when a unique predicate was moved earlier. An external rule could observe a variable before it was bound, or observe an occurrence before it was consumed. A looping higher-priority computation also differed from eagerly executing a later failure.

**Decision and limit.** Unique dispatch alone does not authorize reordering. This rejects that certificate, not contextual compilation. A stronger commutation, termination or divergence-sensitive argument could permit more transformations. [Region boundaries](R05-region-boundaries.md)

### 33. Require a completely ground carrier before contracting it — Unnecessarily narrow premise; improved

**Result.** A checked pure countdown carrier eliminated intermediate occurrence and selection work and beat Conditional on the fourteen ground comparisons. Unknown tails exposed the initial boundary. A later certificate contracted only the known prefix and resumed at the actual tail: long-unknown timing overlapped, short-unknown favored Conditional, and Conditional retained a lower peak.

**Decision and limit.** Complete groundness was unnecessary for the proven prefix steps. The broader claim that carrier lowering eliminates every sharing advantage was not supported. The transformation covers a particular source-derived pure carrier; arbitrary effectful or multiple-carrier contraction remains unresolved. [Carrier comparison](R05-carrier-cost.md), [prefix gate](R05-carrier-prefix-gate.md), [prefix costs](R05-carrier-prefix-cost.md)

### 34. General relational compilation, contextual lowering and richer structural theories — Not directly resolved

**Current follow-up.** The [source-derived finite lifecycle](S06-finite-lifecycle.md) now measures an admitted atom-domain phase with resumed caller execution. Selective gains and cold/unselective costs are both recorded. This does not resolve broader theories, compatible-query learning or native compilation.

**Result.** There are positive finite-solving and recursive-lowering examples, and concrete counterexamples to insufficient certificates. There is no complete comparative implementation covering general contextual relation compilation, arbitrary equality-constrained term spaces, existential projection, or the broader normal/neutral/name/disequality theories considered in the analysis.

**Why I stopped.** I described further proofs and implementations as expensive without a selected source need. That cost judgment was not an experiment showing these directions could not change the architecture. The supplied examples were investigative cases, not a basis for declaring other computational regimes irrelevant. [Structural scope](E10.md), [framing review](../research-framing-review.md), [earlier direction audit](../../goals/chr-sharing/notes/T018-final-direction-audit.md)

## Reuse

### 35. Memoize owned equation requests — Measured representation and key costs

**Result.** E12's memoized owned representation achieved 255 hits in 256 repeated calls and reduced equation-pair processing dramatically. Yet the shared-arena direct control was still cheaper in the representative repeated case: roughly 15–18 ms versus 21–26 ms for memoization. Building keys and replaying results remained work. Unique and identity requests exposed adverse overhead.

**Decision and limit.** High hit rate did not justify this memo interface. It does not reject caching using stable identities in a shared representation; that is a materially different design. [Equation tables](E12-equations.md)

### 36. Table complete continuations using exact fixed identities — Measured tradeoff

**Result.** The full-state table avoided repeated execution on duplicate choices, but reduced no transitions on the measured application cases. In a size-16 retention example it retained about 2.47 MB versus 87 KB without the table.

**Decision and limit.** Universal full-state tabling was unsupported. Alpha-renamed state keys, relevance-based projections and call-level tables were not implemented by that comparison, so their potential is unresolved. [Continuation tables](E12-continuations.md)

### 37. Learn and check failures — Positive bounded result, adverse checking overhead

**Result.** E12 demonstrated selective proof checking that avoided repeated failing work at a valid boundary. Cheap clashes exposed checking overhead. Failure reuse needed a reason the proof still applied; matching a superficial request shape was insufficient.

**Decision and limit.** This remained a credible optimization rather than a general winner. The experiment does not establish that all failures are worth caching or that proof checking is necessarily too expensive. [Native failure checking](E12-failure-native.md), [projection](E12-failure-projection.md)

### 38. Cache common equality failures in the current explicit representation — Not directly tested

**Result.** T059 found a real Conditional advantage for common clashes before discrimination. The current explicit path was not compared with a new shared-representation clash cache. Earlier failure-checking work establishes feasibility and overhead in another setting, not the outcome of this comparison.

**Why I stopped.** I argued that a cache would mainly narrow Conditional's favorable exception and strengthen the explicit recommendation. That reasoning assumed the recommendation whose adequacy was being assessed. It is not experimental evidence that the cache is unnecessary or that this remaining distinction cannot affect architecture. [Substantive equality](R05-equation-cost.md), [recorded stopping rationale](R07-sufficiency-current.md)

### 39. Group identical whole states, including structural comparison — Measured tradeoff

**Result.** E09 grouping reduced duplicate-choice time from about 7.49 ms to 0.90 ms while preserving raw multiplicity. On thirteen exhausted application workloads no grouped variant reduced source-job counts. Reverse structural grouping made SK duplication much slower; even identity grouping had a separated adverse round-scheduler case.

**Decision and limit.** Universal grouping was unsupported. Identical-state recognition is distinct from sharing one operation across different states; the losses do not resolve that broader mechanism. Comparator direction and identity checks also matter. [Scheduling and grouping costs](E09-costs.md)

### 40. General reconvergence, alpha-equivalent tables and selective recomputation — Not directly resolved

The [complete-caller ownership gate](S05-caller-ownership-gate.md) executes fresh replay and caller continuations directly and verifies prepared-cache/consumer lifetime separation. This enables a full lifecycle pilot; it does not establish a reuse or architecture winner.

The [checked initial-phase entry](S05-call-entry-gate.md) accepts shared caller variables under an explicit priority/ownership contract and validates full resumption. Counterexamples bound simpler admission arguments. General continuation extraction, effects and lifecycle costs remain required.

The subsequent [call-level transport gate](S05-call-transport-gate.md) directly tests reuse across different caller states and fresh-result transport. It exposes a call-entry scheduling counterexample; general relevance, effectful replay, retention and complete costs remain unresolved.

**Result.** Exact tables, occurrence caches and failure proofs cover separate bounded mechanisms. There is no complete comparison that chooses among generalized caller projections, state renaming, recomputation, cache eviction and semantic dependency tracking across the whole evaluator.

**Why I stopped.** I cited key construction, retained state and validity responsibilities as reasons to demand a more specific motivating case. Those are real engineering obligations, but their existence is not a measured unfavorable cost balance for every unimplemented design. [Continuation limits](E12-continuations.md), [observation and state-reuse analysis](../../goals/chr-sharing/notes/T016-observation-and-state-reuse.md)

## Observation and progress

### 41. Eager tree answers versus graph observation — Measured tradeoff

**Result.** E14 graph observation improved repeated unary-answer timing, lost on distinct DAG8 answers, and left most ranges overlapping. All first-answer comparisons in the relevant contrast overlapped. Allocation traffic fell for repeated streams and rose for all eight distinct streams.

**Decision and limit.** Neither observation policy was universally preferred. Avoiding duplicate export can help, but graph resolution, snapshots and exact comparison cost something. Mapping-sensitive comparator reuse and broader lifetime policies remain untested. [Repeated observation costs](E14-graph-costs.md)

**Comparator variants.** Avoiding eager answer clones produced five separated cold-time improvements. Changing only the eager comparator produced overlapping timing ranges in all sixteen comparisons despite allocation differences. Exact rollback comparison therefore cannot be credited with an unmeasured universal speedup. Local-degree fingerprints also cannot decide equality alone: an eight-cycle and two four-cycles require global correlation checks. Stronger exact indexing and symmetry refinement remain open. [Cost controls](E14-graph-costs.md), [exact-observer probe](E14.md)

### 42. Observe only a partial answer or collapse meaningful distinctions — Incorrect shortcut under the tested contract

**Result.** Independent gates require joint variable aliases, full residual multisets and the distinction between raw alternatives and unique answers. Region witnesses show that two identical successful arms have different raw multiplicity from one arm. Constructor matching on an unknown also cannot be replaced by a binding equation without changing residuals and aliases.

**Decision and limit.** Successful-value equality alone was an insufficient correctness check. This does not require every architecture to build identical physical answer trees or branch objects. A compressed representation can satisfy the same observations, and a different public observation contract would be a language decision requiring explicit consideration. [Graph observation gate](E14-graph-gate.md), [region witnesses](R05-region-boundaries.md), [reference registry](E00.md)

### 43. Round barriers versus asynchronous service — Measured latency tradeoff

**Result.** E09's large-equation witness returned the first answer at about 0.883 ms asynchronously versus 1.227 ms with rounds, with separated ranges; total timing overlapped. Other workloads showed queue and grouping costs. A round can delay an available answer behind other finite service.

**Decision and limit.** A single universal scheduler was not established. Asynchronous service has a demonstrated latency benefit in that case, but fairness, total throughput, memory and cancellation are separate properties. [E09 costs](E09-costs.md), [policy work](E09-policy-work.md)

### 44. Prioritize all pending publication before more source work — Measured progress tradeoff

**Result.** R06's strict global observation priority preserved finite exact answers but delayed a distinct finite sibling until an exponentially large duplicate-answer region drained. The gate did not establish a memory improvement.

**Decision and limit.** I stopped that particular scheduler intervention because the demonstrated delay opposed its intended benefit. It does not reject bounded publication quotas, regional backpressure or other policies that balance output and source progress. [Publication gate](R06-publication-flow-gate.md)

### 45. Traverse all history during Conditional publication — Measured defect and remaining retention limit

**Result.** Restricting traversal to the observation's region materially improved delivery. A finite 128-answer stream fell from about 565 ms to 56 ms, but the explicit control took about 3.16 ms. Roughly 3.2 MB of historical support remained. An ongoing alias case still reached only 117 of 128 requested answers at its service bound.

**Decision and limit.** Irrelevant history traversal was unnecessary, and the remaining implementation was poor for these low-sharing streams. The cutoff is not a complete 128-answer timing. This does not establish that Conditional execution intrinsically needs unbounded history or that broader reclamation cannot work. [Restricted publication](R06-restricted-publication.md), [stream lifetime](R06-streaming-lifetime.md)

### 46. Broad reclamation, compressed histories and alternative output lifetimes — Not directly resolved

The [order lifecycle pilot](S08-order-lifecycle.md) reduces the alias-stream peak from2.83MB to0.20MB by changing support order, with a contrary arrival-order regime. All owners restore. A576-byte cross-engine output difference is exactly explained by spare residual-vector capacity. This is representation and lifecycle evidence, not an intra-query reclamation proof; broader roots and retention remain open.

**Result.** E01, E04, E14 and R06 expose distinct retained owners: answers, event bodies, source arenas and choice/support history. Narrow traversal and export changes do not compare general garbage collection, history compression, bounded consumer retention or regeneration policies.

**Why I stopped.** I argued the explicit low-sharing stream already served the measured endpoint. That establishes a usable control, not that another architecture's lifetime design cannot materially change its overall suitability. [Lifetime evidence](R06-streaming-lifetime.md), [coverage dispositions](../coverage.md)

### 47. Treat failure, exhaustion and unfinished work as interchangeable — Incorrect semantic shortcut

**Result.** Delayed-splitting and region witnesses distinguish a reachable finite failure from spending the bound on continuing computation. Native mapping probes distinguish connected failure from an inert failed object. T056/T059 additionally checked source-failure lineages in their chosen gate; moving a clash before the choices changes that lineage account.

**Decision and limit.** The tested transformations must preserve their stated failure and progress contract. These checks do not prove that a production architecture must allocate one physical engine per failed leaf, or expose every diagnostic lineage publicly. Choosing a representation and choosing the language's observable failure behavior are separate questions. [Delayed splitting](E07.md), [region boundaries](R05-region-boundaries.md), [substantive-work gate](R05-substantive-work-entry.md), [equation comparison](R05-equation-cost.md)

## Parallelism

### 48. Offload owned equation requests to cold workers — Measured loss

**Result.** Across seventeen queries, shared-arena scalar execution had lower cold-time, first-answer and metered-peak ranges than either worker mode. Two workers did improve some large independent cases relative to one worker, but not enough to beat the shared control.

**Decision and limit.** That owned-request, cold-worker interface was unsupported as an optimization for the tested workloads. Transfer representation and worker startup are part of the result. It is not a rejection of parallelism with shared representation or coarser work. [E16 costs](E16-costs.md)

### 49. Parallel certified regions — Unresolved benefit over inline execution

**Result.** The repeated regional study showed two workers beating one on balanced independent work. Against Inline, the favorable two-worker ranges narrowly overlapped; other registered regional cases favored Inline. Following the structural-maintenance correction, the current worker ordering still remained unresolved.

**Decision and limit.** There was no supported general worker speedup. There was also no completed evidence that balanced coarse regions cannot benefit. Factoring's serial savings are independent of this worker result. [Regional lifecycle](R08-regional-lifecycle.md), [current paired comparison](R01-closed-subtree-lifecycle.md)

### 50. Warm pools, larger regions, shared worker representations and wider scaling — Not directly tested

**Result.** Cold workers, admission bounds and service quantum were investigated. A comparative warm-pool deployment, larger worker counts and general connected-work partitioning were not.

**Why I stopped.** I required a concrete deployment or granularity need before more runs. That can explain research priority, but the existing experiments do not settle these designs' total efficiency or justify treating all parallel architecture questions as resolved. [Worker limits](E16-costs.md), [regional limits](E16-regions-pilot.md), [stopping assessment](R07-sufficiency-current.md)

## Language design

### 51. Make single-head, nonoverlap, groundness or termination mandatory — No adoption result

**Result.** Certificates established useful fragments. E04's whole-program certificate excluded multihead/history cases, overlap and literal lambda programs. R05 then showed that useful specialization can accept recursion, unknown arguments and guards without requiring every program to be ground or terminating.

**Decision and limit.** I did not adopt mandatory restrictions. Their coverage costs are demonstrated, but no experiment determines whether the resulting language would be preferable for the user's purposes. Optional/inferred facilities are feasible; their coexistence is not automatically the simplest final architecture. [Initial property inventory](E02-initial-inventory.md), [E04](E04.md), [single-head specialization](R05-single-head-lifecycle.md)

### 52. Treat nonbinding purity as sufficient for partial-equality interleaving — Incorrect generalization beyond the current guard fragment

**Result.** The analysis gives a read-only `var` guard that can fire between partial deductions but cannot fire before or after the complete equation. Positive equality entailment has a stability property that such a guard lacks. Current implemented guards use equality entailment; the adverse guard is hypothetical language extension analysis.

**Decision and limit.** Purity alone is not the needed premise. This is an analytical counterexample, not a benchmark against integrated equality. Stable guards, regional certification and different completion protocols remain possible language/runtime choices. [Interleaving analysis](E18-source-interleaving.md), [serialization analysis](R02-partial-equality-serialization.md)

### 53. Replace consuming multiset CHR with a purely logical/set interpretation — Not an experimentally selected language

**Result.** Finite logical regions can avoid source execution, while source gates separately check consumption, propagation and multiplicity. Logical consequences do not automatically stand for the same identified CHR resources. No experiment compared adopting an entirely different global semantics on the user's intended programs.

**Decision and limit.** I treated logical solving as a certified region rather than choosing a new language meaning. That preserves the tested semantics but does not establish that a different language design is undesirable. Its expressiveness and semantic tradeoffs need an explicit comparison, not an implementation-only verdict. [R04 correspondence](R04-source-correspondence-gate.md), [E18 assessment](E18-source-assessment.md), [earlier language opportunities](../../goals/chr-sharing/notes/T016-language-opportunities.md)

**Implicit branching on rule competition.** The source analysis also rejected directly turning every competing CHR match into a multiway branch: that introduces alternatives beyond explicit source OR. This is a semantic mismatch with the investigated language, not a performance result against event graphs or multiway systems with a different contract. [Source assessment](E18-source-assessment.md)

### 54. Ownership, linearity, modes, finite domains and stronger certificates — Not comprehensively compared

The [effect lifecycle comparison](S07-effect-lifecycle.md) now measures checking, execution and disposal: eight completed-query traffic reductions,seven unchanged cases,and identical allocation for inferred/checked/required accepted sources. A writer exposes the expressive loss of mandatory no-binding semantics. Broader precision, effects and language adoption remain unresolved.

The [binding-effect gate](S07-binding-effects-gate.md) now implements a distinct beneficiary: certified immutable bindings omit persistent equality wake-up indexing. Groundness and non-overlap are unnecessary for this benefit; consumption interference remains observable. Checking/lifecycle payback and broader property designs remain unresolved.

**Result.** Local immutable closedness, source-derived carrier properties and finite-region certificates remove particular responsibilities. No complete experiment ranks mandatory affine ownership, read/write handle distinctions, inferred modes, explicit relational cases or general finite-service requirements as language-wide designs.

**Why I stopped.** I retained these as possible ways to obtain cheaper execution while avoiding an unauthorized language restriction. That correctly leaves adoption open, but it does not finish investigating their benefits, necessary complexity or coverage costs. [Architecture assumptions](../architecture-assumptions.md), [language opportunities](../../goals/chr-sharing/notes/T016-language-opportunities.md), [backend boundaries](../../goals/chr-sharing/notes/T018-backend-boundaries.md)

## Architecture and stopping

### 55. A combined architecture with independently certified execution paths — Proposed, not validated as a complete architecture

**Result.** Separate experiments support indexed/activated execution in some regimes, finite solving in an eligible relation, direct recursive lowering, factoring, and bounded sharing. They do not demonstrate a single integrated system with measured selection, interoperability, compilation, lifetime and maintenance costs across those paths.

**Why I recommended it.** I combined the strongest bounded results into a proposed portfolio. That is an architectural inference. The experiments do not by themselves establish that this portfolio has the best total efficiency or the least necessary complexity. Automatic routing or hybrid execution was not validated merely by validating its possible components. [Architecture checkpoint](R07-architecture-checkpoint.md), [framing review](../research-framing-review.md)

### 56. Stop broader application and regime investigation — Research-priority judgment

**Result.** The repository includes ordinary keyed computation, binding, alternatives, finite solving, recursion and streams. E17 supplied application/oracle preparation, not a completed broad application comparison. There are no justified workload weights, and supplied application names do not define the language's intended distribution.

**Why I stopped.** I argued that more application labels would add less information than a new computational contrast. That is a sound caution about benchmark design. It does not establish that every consequential contrast was covered, especially where a broader design had not received a direct experiment at all. [Coverage map](../coverage.md), [application entry](../../goals/chr-experiments/notes/T017-application-entry.md)

### 57. Declare the research goal complete — Unsupported by the full set of dispositions

**Recorded reasoning.** The final assessment assigned every question a disposition and judged further work unlikely to change the recommendation enough to justify its cost. Several reasons depended on treating explicit execution as already recommended, or on requiring a new source/deployment need before investigating an alternative.

**What the evidence establishes.** There is substantial useful evidence about specific mechanisms and implementations. There is not an evidence-backed resolution of every consequential architectural direction listed above. Naming a direction “deferred” records that investigation stopped; it does not supply the missing result or justify closure. The overall completion claim exceeded what these experiments establish. [Recorded sufficiency assessment](R07-sufficiency-current.md), [closure audit](R07-closure-audit.md)

## Coverage check

This index makes the scope auditable without repeating every result table. Registrations, source freezes and raw runs are linked from the cited receipts. The report adds no new measurements.

| Evidence family | Designs covered here |
|---|---|
| E00 semantic registry | 42, 47; correctness controls throughout |
| E01 storage | 17, 24–25, 46 |
| E02 source properties | 51–54 |
| E03 conditional grouping | 18, 23 |
| E04 local dispatch and event reuse | 1, 8, 16, 19, 51 |
| E05 named families and partitions | 12 |
| E06 nets and native correspondence | 13–16, 47 |
| E07 delayed splitting | 20–21, 47 |
| E08 independent factors | 21–22, 25, 49 |
| E09 matching, grouping and scheduling | 4–5, 39, 43 |
| E10 structural operations and spaces | 26–27, 34 |
| E11 bounded symbolic traces | 28 |
| E12 equation, continuation and failure reuse | 35–38, 40 |
| E13 bounded unfolding | 30–31 |
| E14 exact observation | 41–42, 46 |
| E15 whole-state grouping and net-service composition | 13, 39, and the note below |
| E16 workers and regions | 48–50 |
| E17 application preparation | 56 |
| E18 integration and source analysis | 9, 16, 52–53 |
| R01 access, immutable structure and current joins | 1–8, 49 |
| R02 consuming integration and repairs | 9–11, 52 |
| R03 conditional execution and state ownership | 17–25 |
| R04 finite solving | 29, 53 |
| R05 specialization, carriers and substantive equations | 23, 30–34, 38, 47 |
| R06 streaming, history and publication | 44–47 |
| R07 recommendation and stopping | 55–57 |
| R08 regional workers | 49–50 |
| Earlier conceptual direction audits | 16, 25, 34, 40, 51–54 |

**E15's additional controlled variants.** The corrected complete-session comparison found direct equality cheaper than both ScanNet and CountedNet in all 45 matched workload/policy pairs. Maintaining status counts reduced administrative actions without establishing a runtime benefit. This supports a negative result for those composed service organizations. It does not test direct graph compilation, and whole-state grouping remains distinct from operation sharing across different states. [E15 complete costs](E15-costs.md), [status accounting](E15-status.md)

The unresolved entries are not claims that those designs would win. They identify where there is no experimental result supporting rejection, and where the stated reason for stopping was a judgment that the report must expose rather than disguise.
