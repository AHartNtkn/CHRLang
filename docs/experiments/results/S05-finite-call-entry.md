# Existing call reuse can cover a bundled finite phase

The existing call table can reuse the finite-learning workload when its three private goals enter through one explicit bundle. All 864 complete-query comparisons pass in each of the default and counter-free configurations. This establishes a bounded semantic control; it does not establish that reuse is faster than learning.

## What the experiment changes

**The admission gap is real but does not require another call table.** The original query contains two finite-domain producers and a pair check. `Table::expand_query` requires exactly one initial private occurrence and rejects that query. Nine fixed bundle rules cover the tested domain pairs. Each posts the original three private goals, in order, and passes their shared variables through one call. The existing table handles renaming and result transport.

**Independently completing the three calls is incorrect.** A source accepting all four pairs over `{a,b}` has four answers. Isolated expansion of its initially unbound pair check produces none: nonbinding matching cannot select a ground success rule before the producers bind the variables, so the failure rule applies. A product of these independently computed results would lose all four answers. This rejects that decomposition, not multi-goal reuse in general.

**The bundle preserves the complete caller in the tested family.** The comparison includes independent and aliased inputs, duplicate choice weights, two variable-number ranges, and changing residual caller markers with an unrelated shared variable. It checks all output bindings and residual occurrences against the independent scalar interpreter. A separate enumeration of the nine ground pairs checks answer counts, including zero-answer sources. The ordinary caller consumes `out` and `token` and produces `result` after the private phase.

## Why this boundary works, and where it stops

The bundle is the only initial private occurrence and its rules form the first part of the private priority prefix. Its one deterministic expansion posts exactly the original producer/producer/entry goals. Thereafter the private source runs with the same goal order and shared variables. The outside caller cannot intervene in this finite phase. A source step is added; this is a complete-answer correspondence, not equality of step counts or cancellation latency.

Both domain tags are explicit call arguments. Alias topology also participates in the existing normalized key. Within each prepared source, 18 distinct domain/alias keys are followed by their renamed counterparts. Diagnostic builds record 18 computations and 18 hits with memoization; the uncached control records 36 computations and zero hits. Hits include successful and failed results. This demonstrates actual reuse rather than merely accepting the transformed query.

A zero service bound reports unfinished work. A subsequent adequately bounded call still produces all 36 weighted answers in the tested all-accepting case. Existing call regressions additionally check fresh returned aliases, independent fresh results, caller effects, priority counterexamples and cancellation followed by reuse. Those regressions remain separate evidence; the finite bundle itself creates no fresh result variables.

This is an explicit transformation of the registered family, not an implemented general source-admission checker. It does not qualify arbitrary multi-call contraction, competing external heads, private two-head capacity consumption, unrestricted fresh effects, or sustained cache lifetime. The original call table and reference interpreter are unchanged.

## Evidence and limits

The [prospective registration](../registrations/S05-finite-call-entry.md) specifies the 864 comparisons: two cache modes, three accepted-pair masks, two choice weights, two prefix depths, nine domain pairs, two alias cases and two variable-number ranges. One prepared caller serves 36 changing queries. The [executable gate](../../../research/chr-reuse/tests/finite_call_entry.rs) independently constructs this bounded family, including a deterministic prefix of depth zero or four; it does not import the learning implementation or use its results as expected answers.

Recorded checks in [the receipt directory](s05-finite-call-entry/):

- Three new tests pass with default features and with `--no-default-features`.
- All 16 existing call-reuse tests pass in both configurations.
- Clippy passes for the new test target in both configurations.
- No comparative timing or allocation matrix ran. Test elapsed times are not architectural cost evidence.

The [receipt manifest](s05-finite-call-entry/manifest.json) records source hashes, commands and the parent commit after the runs. The registration preceded execution; these hashes identify the tested files and are not a claim of a separately recorded pre-run binary freeze.

## The next decision

**A fair learning comparison still needs to separate reuse from its executor.** This call table runs persistent continuations; failure learning runs the direct finite solver. Comparing their totals is useful as a whole-path comparison, but a loss by the call table would not show that learning beats result reuse using the same direct finite plan. The next T075 gate should establish a renaming-aware completed-result control at that finite-phase boundary, reusing the existing key/transport arguments where applicable. It must cover changed domains, multiplicity, caller projection, incomplete queries and retained ownership before costs are registered. It should not duplicate the existing persistent call-table mechanism.

At this gate, broader capacity phases remain the strongest ready alternative: they could remove execution in more effectful sources, but require a new source/resource correspondence. The missing same-plan reuse control can change interpretation of the measured learning gains and now has a concrete whole-phase correspondence. Select that bounded control investigation before a larger cache campaign. Broader resource phases, compilation and lifetime return at its semantic/ownership boundary or an obstruction.

This is one experimental package since the sequence revision. The broader continuation/relevance questions and the architecture goal remain active.
