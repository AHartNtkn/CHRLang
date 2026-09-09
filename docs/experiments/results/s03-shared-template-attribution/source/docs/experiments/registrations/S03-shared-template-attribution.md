# Paired attribution of fresh-template copying

The [sizing result](../results/S03-derivation-sizing.md) exposes a specific construction hypothesis: owned terms repeatedly copy countdown tails and duplicating accumulators. This package tests shared immutable compiler terms and sharing of ground values during instantiation. It precedes T072's broader integration and T073's reusable lowering because this cost can distort the immediate derivation comparison. Its boundary is one paired attribution, then renewed selection.

## Change and correctness

The candidate substitutes immutable reference-counted term nodes instead of copying bound trees. Groundness is cached per node. One instantiation memoizes ground nodes; fresh locals resolve through that application's environment. Branch-local expressions are not memoized across environments. The plan evaluator still creates actual calls, choices, posts and consumption obligations. Both source and template plans use the same plan traversal.

The growing-accumulator regression must derive all 12 steps and use fewer than 100 runtime nodes before observation; the current implementation stops at 10 steps due to copying. Complete output correctness remains independently tested. Separate witnesses must preserve ordinary execution after the 16,384 construction budget and live continuation after 64 followed calls. The existing 64-entry cache/key limits, fresh-identity, source-effect, scheduling and finite-service gates remain applicable.

## Hypotheses and exact comparison

H1: sharing improves substantive single/distinct construction and duplicating accumulators. H2: its reference counts and instantiation lookup can hurt shallow work. H3: repeated calls retain their reuse benefit. These hypotheses concern the representation repair, not templates versus every architecture.

Use the frozen ordinary/meter binaries from `/tmp/chr-derivation-lifecycle-b5d235e3/` as before. Build the candidate with the identical release commands into `/tmp/chr-shared-template-83deb0d3/`. Freeze both binary hashes, changed source, source runner, registration and toolchain before running. Native compilation is outside these measured totals.

Template configurations cross four families (`single`, `repeat`, `distinct`, `grow`), depth 0 or 32 (grow 0 or 12), one/eight changing queries, resource absence/presence and both starting orders: **64 configurations**. Add ordinary-demand controls for the four substantive eight-query, resource-present, forward configurations, verifying the shared evaluator does not create an unrelated material shift. This makes 68 paired configurations.

Run **seven ordinary timing repetitions**, randomizing configuration order and within-pair binary order with seed 7121: 952 processes. Use a fresh process per member and pin the same first available CPU. Run every configuration twice under each allocation binary: 272 processes, requiring exact allocation replay per configuration/build. Cancellation checks run templates/choice/depth8/two queries/resources/forward with zero/one first-query ticks, each build and allocator: eight processes. Total: **1,232**, plus both meter self-checks.

Each process has 60 seconds wall/CPU, 1 GiB address space and the existing two-million-step candidate/scalar limit. Preserve cutoffs and diagnose them before assigning a cost conclusion. Complete answers are checked outside the measured phases. Timing uses counter-free ordinary allocation; requested-allocation diagnostics are separate and do not measure RSS.

## Analysis fixed before running

Primary cost is preparation plus query setup, execution with complete observation, engine/answer disposal and prepared disposal. Report first-answer and individual phases, source/input construction separately. Charge all template construction inside execution. Prepared source is reused; the template cache remains query-owned.

For each configuration calculate seven paired after/before primary ratios. Report their geometric mean and pointwise 95% percentile bootstrap interval from 10,000 resamples of the seven pairs, seed 7122 plus configuration index. A practical repair gain requires the upper interval bound below 0.90; a loss requires the lower bound above 1.10; otherwise the result is unresolved under this rule. Report counts descriptively, without population-wide or simultaneous-confidence claims. Preserve contrary configurations; do not choose only favorable rows.

Attribute changes with phase and exact requested-allocation records. A gain does not establish a template architecture winner; remaining key, instantiation, retention and complete-output costs still matter. A loss prompts inspection of lookup/ownership overhead, but another refinement requires a consequential decision and comparison against T072/T073. After this package, register any broader template/control confirmation prospectively; earlier exploratory cross-engine timings remain exploratory.
