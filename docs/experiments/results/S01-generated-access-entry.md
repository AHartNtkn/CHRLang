# Next: generate multihead access, not only pattern tests

T070 investigates whether source-derived native access and update code can replace generic multihead discovery machinery economically. Compare it with competent indexed rediscovery and retained matching; no production architecture is selected in advance.

## Existing code and the missing contrast

Inspection of `research/chr-compiled/src/generate.rs` shows that existing generated selectors emit constructor/variable tests and body operations, but still call `core.candidate`, copy generic frames and advance a generic Cursor. `build.rs` bundles fixture programs into generated code. These are useful controls, not evidence that independent user-program access compilation has been measured.

The existing single-head specialization and carrier contraction cannot establish this multihead result. The [worker comparison](S09-factored-lowering.md) demonstrates why a control must include an available optimization that eliminates work; merely comparing dispatch styles can leave the important responsibility unchanged.

## First coherent candidate and gate

Start with source-derived plans for two- and three-data-head rules plus live requests, using the subscription/low-yield sources as known controls. Generate access selection and continuation state from information available at each anchor. Determine which generic frame, cursor, head interpretation or wake-up work is actually absent. Do not make the candidate retain those structures solely to fit the existing generator API.

Preserve nonbinding matching, distinct occurrence identity, kept/consumed partners, propagation history, source-order competition, guards, late binding and complete observations. Gate repeated equal-valued occurrences, aliases enabling a match, consumed/inactivated partners, fresh locals, changed queries and off-output failure against independent source execution. Add faults that invalidate candidate selection or source effects. A generated program must be derived from its rules, not from expected queries or answers.

Compare plain indexed discovery, the current generated-selector organization, source-derived access code, and retained/subscription policies where they implement the same source contract. Check that available anchor/key information reaches each competent control. If a source schedule differs, establish permitted equivalence or expose the contract difference before ranking cost.

## Cost and language questions

Favor many rules with selective keys/constructors and sparse changes. Challenge with tiny cold programs, weak keys/equal-key buckets, broad alias repair, dense updates and low reuse. Vary rule count and join fanout separately. Include changed queries over prepared rules and at least one independent generated artifact so rule generation, compilation, code size and artifact disposal are charged credibly.

Record what source properties buy each simplification, which properties are inferred or checked, and which near misses cannot use the plan. Do not adopt mandatory modes, groundness, ownership or head-count restrictions through an experimental eligibility test. Runtime savings, compile/reuse crossover, retained memory, necessary invariants and source coverage all participate in the decision.

Prospectively register exact sources, sizes, repetition counts, bounds and interpretation after the source gate and exploratory sizing. The outcome may favor rediscovery, generated access, retention or conditional source eligibility; no invented workload weights choose among them.

## Selection against the strongest alternatives

Direct pull-tabbing/derivation reuse is the strongest distinct ready alternative: it could replace explicit search organization. Parallelizing the now-contracted service is another concrete follow-up. All remain required, with corrected restoration and the other coverage obligations intact.

Generated multihead access goes first because it targets a known remaining generic selection boundary, has ready source/oracle and retention controls, and extends beyond the single-head properties responsible for the just-completed worker result. The estimated work is a source/plan compiler and independently compiled artifact path with effect-correctness validation. Direct graph work requires a different correlation/effect/lifetime protocol; worker refinement would revisit a now-bounded source family. Reassess ordering if the compiler gate exposes an actual obstacle or a more consequential ready comparison.

## First implementation: source-ordered streaming continuations

The first candidate emits one continuation type per rule. It owns one binding frame,
fixed-size occurrence/range arrays and a source-head program counter. Generated
assignments clear variables no longer justified by the retained source-head prefix;
anchor-bound variables survive rollback. Matching never binds source unknowns.
Access expressions are compiled from source patterns and use the smallest available
ground-key bucket. Ordered range traversal replaces candidate-vector snapshots.
The store cannot change while one selector is suspended: source effects commit only
after a complete application is returned. This invariant justifies streaming and
must be revisited before introducing concurrent store mutation.

This removes frame snapshots, candidate bucket materialization and template-key
interpretation. It retains occurrence/index storage, binding repair, propagation
history, source scheduling and the existing term primitives. Reordering heads could
change consuming competition; this candidate preserves their order. Retained joins
are a different comparison, not part of this continuation implementation.

Before costs, compare complete answers and source effects against independent scalar
execution, generic selection and the existing generated selectors. Exercise all
existing finite access sources across scan/indexed and global/active policies;
compare exact occurrence traces within a policy. Check changed queries over one
prepared source and independent OR/failure witnesses. Work diagnostics must establish
zero frame-copy and candidate-vector traffic on the new path, while controls perform
that work. These are mechanism observations, not performance results. Add directed
rollback/anchor/consumption faults and subscription sources before declaring the
full T070 source gate complete. Independent compilation and selective update code
remain subsequent obligations; this first candidate retains generic update repair.
