# E11 direct term-expression evidence

The direct term-expression candidate completes forward addition at the registered
trace-derived bounds. This is a representation feasibility result; it does not
establish a competitive execution architecture.

Code: `e5a91d5`. Backend: pinned Z3 5.1.0.0, Python 3.11. The reference interpreter
remains independent. All returned models pass independent per-transition replay,
including full residuals, variable aliases, occurrence identities and history.

The [64-case inventory](E11-terms-registry-v1.jsonl) has 23 complete matches and
41 explicit bounded-incomplete results. Answer counts, raw model counts, complete
answer agreement and both cutoff predicates equal the dense heap inventory in
every case. Small bounds do not establish completeness for those 41 cases.

The [application runs](E11-terms-app-v1.jsonl) use the same registered bounds and
60-second process limit as the dense heap follow-up:

| Case | Result | Construction | Observation | Boundary checks |
| --- | --- | ---: | ---: | ---: |
| Forward addition | One expected answer, no cutoff | 15.527 s | 0.333 s | 0.125 s |
| Relational decomposition | Construction timeout | 60 s limit | — | — |
| Type synthesis prefix | Expected `k`; other paths resource-bounded | 1.033 s | 0.160 s | 0.040 s |

Forward addition makes 1,020 reachability-pruning queries, of which 848 prove
unreachability; queries consume 14.006 seconds of construction. Formula building
and solver-query overhead therefore remain material costs. The dense candidate
times out on forward addition at these bounds. This is exploratory evidence about
these implementations, not a controlled speed ratio or a general lower bound.

The [two successful application replays](E11-terms-app-replay.jsonl) agree in every
non-time field. The full Python suite has 41 passing tests, including independent
unifier comparisons for both representations and branch-conditioned projection.

Reproduction: export cases with `cargo run --quiet -p chr-symbolic-fixtures
--example export_cases`; run `research/chr-symbolic/run_conformance.py` with that
manifest, `--representation terms`, and either `--all` for the small-bound
inventory or `--bounds docs/experiments/results/E11-derived-bounds.jsonl` plus
the three application `--case` arguments. Use the pinned Python environment.

E11 remains open. Next obligations include a credible equal-bound direct control,
decomposition cost attribution, increasing bounds and prefix reuse, and larger
relational/synthesis workloads. The timeout does not justify closure. Private node
IDs retain matching allocation and pair-service limits across representations;
changing allocation policy would require a separately declared comparison.

## Independent bounded direct control

`direct.py` now supplies a solver-independent scalar arena control. Immutable term
nodes are shared within a branch; branching copies the node-reference list,
substitution map, queue, live store and history. It retains full answers for exact
deduplication, but no execution traces or completed branch states. This is a
specific copying-metadata baseline, not the best possible scalar implementation.

The [small-bound gate](E11-direct-small.jsonl) matches every recorded symbolic
answer count, raw multiplicity, agreement flag and cutoff predicate across all 64
cases. Every finite-case output is independently compared with registered full
answers. The [trace-derived gate](E11-direct-derived.jsonl) completes 58 cases;
six retain explicit cutoffs. These include the infinite sibling, type-prefix
remainder, and four workloads whose U=8 equation budget is insufficient. Complete
finite cases also match registered raw multiplicity. Five direct-control tests
cover all 121 arena operand pairs against the independent tree unifier, identity
sensitive service limits, failure, resource boundaries and occurrence history.
The control now permits registered comparative runs; broader E11 remains open.
