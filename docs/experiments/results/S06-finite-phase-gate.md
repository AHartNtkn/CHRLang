# A source-derived solver avoids the closed choice product

The solver reduces the 64-choice selective source to 322 symbolic steps and 64 partitions, producing one successful phase result. It derives the choices and restrictions from the rules and query. It does not enumerate the source's 2^64 combinations or emit the benchmark's expected answer.

Independent checks pass on the small cases, including resumed consuming rules and duplicate answers. This qualifies a bounded direct-solving mechanism for lifecycle measurement. It does not establish a speed advantage or a complete architecture.

## What is implemented

The [experimental solver](../../../research/chr-compiled/experiments/finite_phase.rs) extracts weighted finite atom domains from unary choice rules. For example, two `X=a` alternatives and one `X=b` alternative produce weights two and one. Multiple requests sharing `X` intersect their domains and multiply the corresponding counts.

The solver then follows the private rules' priority. When a head pattern requires a domain value, it partitions the current domain restrictions into the matching region and its complement. Each region executes the rule that ordinary source matching would select there. A failing region contributes no successful answers. This avoids constructing the full choice product before the checks can reject it.

Unconstrained choices still require enumeration at the end of the phase. Duplicate derivations remain integer weights on solution rows. The caller must repeat the resumed alternatives according to those weights; a weighted row is not a claim that all raw answers have been observed cheaply.

Prepared rules are reused across changed queries. The owned query machine advances one symbolic state at a time. It keeps successful rows private until every branch finishes or fails. Completion transfers those rows to the caller; cancellation drops the machine; an error releases its pending states and unpublished rows. A service call has variable work, including source scanning and term traversal. This is an actual yield boundary, not a constant-latency guarantee.

## Admission is explicit and limited

The caller selects an initial rule-priority prefix; the implementation checks that boundary. Automatic region discovery is not implemented. The accepted prefix requires:

- Initial finite atom-choice producers with unique, unconstrained unary heads, preceding the other private rules.
- Unguarded, linear single consumed heads for the remaining rules. Their bodies may use equality, conjunction and private calls, with no fresh body variables.
- No private body that creates another choice producer or posts outside work.
- No outside kept or consumed head that touches a private predicate at the same arity.
- Completion or failure of every private branch within the limits. An unknown tail, suspended case, continuing branch or exceeded bound returns an error for the whole invocation.

These are compiler eligibility conditions, not adopted language restrictions. Overlapping private heads are permitted: source priority is preserved. Shared caller variables are also permitted, because their bindings are transported after the complete private phase. Multihead token consumption remains in the ordinary caller. This does not yet implement direct resource derivations inside the solver.

The admission check enforces the previous [linked-consumer and early-binding obligations](S06-direct-solving-obligations.md). It rejects an outside consumer of private work. A multihead coin/token producer cannot enter the private prefix, and preceding ordinary work cannot be skipped merely to expose later choices. A caller that starts another private computation later resumes under the complete original rules.

## Evidence that the mechanism works

| Check | Result |
|---|---|
| Existing arrival source | All 224 configurations agree on complete outputs, aliases, residual multiplicity and raw answers. They vary depth 0–6, arrival order, work 0/2, token use, final failure and query tags. |
| Other finite relations | All 64 configurations agree. Three atom values, source-derived pair cases, different predicate names, duplicate domain values, query aliases, case order and eight acceptance masks vary independently. A lower-priority general case must not add another derivation. |
| Weighted aliased requests | One through five requests with two successful duplicate choices produce one phase row with weights 2 through 32. Expanding those weights matches ordinary raw answers. |
| Equality and transport | Domain intersection, alias merging, constructor outputs, query/rule variable-number overlap, occurs-check failure, shared residuals and unused output variables agree. |
| Rule-priority complements | A selective earlier case and a general later case preserve all remaining domain combinations, including aliased inputs. |
| Resumed caller | Ordinary consuming completion preserves its token effects. A later caller rule can post another private choice and produce the independently expected answer product. |
| Large selective witness | Depth 64 with work 2 uses 322 symbolic steps and 64 partitions; one result resumes to the analytically expected complete answer. The 2^64 scalar product is not executed. |
| Admission and limits | Linked heads, unsupported guards/heads, fresh variables, dynamic choices and invalid producer order reject. Unknown recursion, a successful sibling beside continuing work, and quota/overflow cases return errors without partial answers. |
| Service and cancellation | Cancel after 0, 1, 16 or 100 advances while work remains. A fresh query still completes correctly. Completion and errors leave no machine-owned pending states or unpublished solution rows. Allocation restoration has not yet been measured. |

Every small completed comparison uses independent owned-syntax scalar semantics and compiled Global Scan, both for original execution and resumed solutions. The arrival matrix additionally checks analytical complete answers. The large witness establishes avoided enumeration and checks its final answer; it is not an exhaustive scalar comparison at depth 64.

The default and counter-free builds each pass all 11 focused tests and all 151 package tests. Strict scoped Clippy and formatting pass. The initial missing-implementation test, a missing source-name admission check and the initial missing service API each have recorded failing tests followed by passing implementations. An early relation fixture used duplicate rule names; the compiled control exposed that invalid source, and the fixture now assigns distinct names.

## Why the transformation preserves this finite contract

This is an operational argument supported by the tests, not a machine-checked proof.

**Choice accumulation preserves successful valuations and their counts.** The admitted producer rules are unconditional unary consumers at the front of the source program. Their bodies only select atoms or fail. They finish before deterministic private work. Independent occurrences multiply counts; aliased occurrences intersect values and multiply counts for equal values. Incompatible assignments fail in both executions.

**Each symbolic state represents disjoint ordinary valuations.** Matching a constructor against a finite-domain variable adds a value condition. Splitting the complement of a conjunction keeps disjoint regions, rather than merging different source histories. Source-priority scanning selects the first matching rule separately in those regions. Head variables remain nonbinding matches; actual equality is performed by rule bodies with a finite-tree occurs check.

**Caller work does not interleave with an admitted private phase.** Private calls stay inside the prefix, and outside rules cannot take their occurrences. If a surviving private state has no applicable rule but still has private work, admission fails; the solver does not assume that outside linking will complete it. Only after all private work is consumed are interface equations and the remaining caller query returned.

**All source multiplicity survives successful extraction.** Partitioning preserves distinct valuation regions; weights preserve repeated choice derivations. The test transport resumes the original caller for each weighted derivation and compares the resulting raw multiset. This contract does not select an answer ordering or extend the admission proof to ongoing programs.

## Limits and necessary complexity

Default query limits are 100,000 symbolic steps, 4,096 cumulative generated partitions, 4,096 returned weighted rows and 2,000,000 charged term visits. Terms have depth at most 128. Preparation permits at most 4,096 source rules and has a separate 2,000,000-visit budget. Counts use checked `u128` arithmetic. These are experimental admission bounds, not CHR semantics.

A step counts one serviced symbolic state; a partition counts an explicitly generated region or a remaining-domain enumeration branch. Neither is an equal-cost unit across engines. Term limits charge traversal, not RSS or heap allocation. Intermediate count overflow can reject a computation even if a later rule would fail it; that is conservative admission, not proof that the source lacks an answer.

The solver adds its own bindings, weighted domains, source-priority partitioning and owned work queue alongside the ordinary caller engine. The boundary materializes terms and replays output equations. Those responsibilities may outweigh avoided search on tiny, unselective or short-lived queries. The implementation does not eliminate the ordinary engine, nor demonstrate that combining the two is simpler than another complete organization.

## Next: measure the complete path, including contrary cases

A bounded lifecycle pilot is selected next. This is a prioritization judgment: the mechanism now eliminates the dominant explicit alternative product in a consequential source comparison. Its additional state, preparation and transport costs are unmeasured and could reverse that advantage. Measuring them has greater immediate decision value than tuning the solver or refining effects again.

Compare against the qualified Scan, specialization, applicable prepared-prefix and conditional controls. Include selective and unselective finite sources, tiny cold queries, repeated changed queries, duplicate derivations, retained answers, cancellation and disposal. Measure source preparation, query setup, private solving, caller resumption, first/full raw observation, consumer retention and final disposal separately. Expand weighted rows honestly before claiming full observation. Use an allocation/ownership gate and prospective sizing before comparative ordinary-allocator timings.

Native graph/connected feasibility remains the strongest distinct alternative. Reassess its value at the next package boundary and at the four-package breadth review. Broader resource-aware solving, constructed domains, fresh identities, recursive choice generation, suspended interfaces, compatible-query learning and whole architectures remain required. This finite phase does not resolve them.

## Reproduce and inspect

[Registration](../registrations/S06-finite-phase-gate.md), [test source](../../../research/chr-direct-conditional/tests/finite_phase_gate.rs), [default tests](s06-finite-phase-gate/default.log), [counter-free tests](s06-finite-phase-gate/counter-free.log), [default package](s06-finite-phase-gate/package-default.log), [counter-free package](s06-finite-phase-gate/package-counter-free.log), [Clippy](s06-finite-phase-gate/clippy.log), [formatting](s06-finite-phase-gate/format.log), [source hashes](s06-finite-phase-gate/freeze.sha256) and [environment](s06-finite-phase-gate/environment.txt) preserve the gate.

Run `cargo test -p chr-direct-conditional --test finite_phase_gate -- --nocapture`, then repeat with `--no-default-features`. Use `--lib --tests` for the package checks. The recorded commands have 180-second process limits. No engine or reference implementation is changed.

T073 and the research goal remain active. This package supplies correctness and mechanism evidence, not comparative timing or architectural lifecycle superiority.
