# Contextual ownership now executes complete source rules

**The contextual executor preserves complete answers across consuming alternatives, fresh variables, propagation and failure.** All 61 finite source configurations agree with the independent scalar evaluator and the corrected relational executor. A separate finite sibling publishes within 500 advances while another source branch continues running. This makes a lifecycle comparison possible; it establishes no performance advantage.

## What was implemented

The executor uses the [contextual store](S02-contextual-store-gate.md): alternatives share append-only constructor storage and fork context-local equality and live-occurrence maps. It adds pending body effects, fresh variable instantiation, positive equality guards, propagation history, exact residual publication and round-robin service of alternatives.

Source scheduling follows the relational control: service one equality deduction, then one pending effect or source application; choose rules in source order; finish pending body effects before another application; publish only after both equality and source work are exhausted. Candidate lists are invalidated after equality work and relevant arrivals. This intentionally controls scheduling while testing a different store and direct constructor matching. It is not an independent implementation of the reference semantics and is not strategic port rewriting.

The contextual matcher scans live occurrences and reads context-local descriptors. It does not construct the relational control's indexed constructor/source tables. Repeated variables use positive structural equality; matching does not bind unknowns. Occurrence claims check kept and removed identities atomically. Forks carry propagation history and unfinished effects as well as the store.

## Correctness coverage

The existing 23 finite source cases run with original and changed queries over reused preparation, giving 46 configurations. Six additional existing cases check candidate priority, new arrivals, late guard enablement, stale consuming tuples, fork-local bindings and repeated-variable matches enabled by equality. The test harness now runs both executors for every one of these cases before independent scalar comparison.

Nine focused finite configurations exercise a common fresh prefix followed by consuming siblings, duplicate alternatives, an extra equal-valued token, later failure, fresh residual-variable aliasing, kept permission occurrences, and propagation history with context-local guard enablement. Both executors agree with scalar answers, including raw multiplicity and complete joint output/residual aliases.

The separate progress witness forks into endless source applications and a finite answer. The finite answer has the expected binding and done residual within the registered bound. A further 32 advances remain live after publication. This proves service for that witness, not constant-time matching or bounded latency for every source: a single match search and equality repair can still do size-dependent work.

All 20 relational-package tests pass. Strict Clippy passes for the changed crate; existing integrated build-script dead-code warnings are recorded. Reference evaluators are unchanged. See the [registration](../registrations/S02-contextual-source-gate.md), [existing source checks](s02-contextual-source/existing-source.log), [focused checks](s02-contextual-source/focused-source.log), [full tests](s02-contextual-source/full-tests.log), and [Clippy](s02-contextual-source/clippy.log).

## Architecture obligations and costs

Sharing the constructor arena avoids copying those nodes at each fork. It does not eliminate state: equality maps, live-occurrence maps, cached candidates, history, output handles and pending effects still have owners. A context-map write may copy its snapshot. Direct matching performs scans and recursive descriptor inspection. The arena retains nodes from failed alternatives until its final owner is dropped. Body preparation can allocate nodes for both alternatives before the fork.

The implementation accepts the syntax used by the relational control, including kept/removed heads, positive equality guards, explicit choice and propagation. It rejects rules with no heads. Agreement on this finite gate is not a proof for arbitrary source programs or every permitted nonconfluent schedule. The chosen source-order policy is part of the experimental contract.

This path shares pre-fork consequences through snapshots. It does not share newly derived consequences between already-divergent contexts. Distributed claims, contextual support compression, union-find expressed through CHR and strategic port rewrites remain separate S02/S03 questions.

## Next comparison

Measure complete lifecycle costs against the corrected relational executor and a competent compiled control. Sources must vary common constructor storage, number of alternatives, context-local equality changes, resource consumption and match selectivity independently. Include no-choice and low-sharing cases. Shared prefixes alone cannot define the comparison.

First build a bounded runner with changed queries over reused preparation and cancellation. Register exact configurations after sizing, then use counter-free ordinary timing and separate allocation diagnostics. Charge setup, execution/observation and disposal, report retained arena memory, and investigate consequential losses before attributing them to the architecture. Review breadth after that package: the interleaving control, contextual store and source integration are three packages since T072 selection. T072 and the wider goal remain active.

[Implementation](../../../research/chr-relational/src/contextual_execute.rs), [store](../../../research/chr-relational/src/contextual.rs), [focused tests](../../../research/chr-relational/tests/contextual_source.rs), and [shared finite source tests](../../../research/chr-relational/tests/source.rs).
