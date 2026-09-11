# Stable CHR request selection preserves the tested source priority

The CHR protocol now preserves request priority across representation repair and skips older requests that are still suspended. A consuming result can enable an older request for the next token. This qualifies a common source fragment for cost comparison; it does not establish an architecture advantage.

Two registered repeats each pass **784 compiled comparisons**, with independent ordinary-source, encoded-source and local-handle agreement. All 26 constructor tests and scoped strict Clippy pass. No timing or allocation comparison ran.

## What makes the protocol work

The [previous counterexample](S02-request-priority.md) separated source request identity from the occurrence created when repair reposts it. The new [source rules](../../../research/chr-relational/tests/support/chr_selection.rs) carry an immutable ticket in each request. Repair changes only its input/output representatives. Explicit `before` facts record original request precedence.

An epoch marks a selection round. Propagation discovers requests whose inputs have the required constructor while a token exists. Duplicate candidates are absorbed; a candidate with an earlier eligible competitor is discarded. The remaining candidate consumes its request and one token, posts the result equation and renews the epoch. Kernel equality and repair rules precede discovery and commit rules in the ordered rule agenda.

Selection compares eligible requests, so an older unknown input does not block a younger ready request. If consuming that younger request binds an older input, equality repair completes before the next round selects again. These are properties exercised by the complete source results, not conclusions from the presence of ticket fields.

Epoch renewal is necessary for this implementation. Without it, propagation history remembers candidates generated in the earlier round, even after those candidates were discarded during selection. The adverse mutation leaves a token and an eligible surviving request without consuming them. Independent source evaluation detects the different complete answer. A stable ticket alone is therefore insufficient.

## Evidence and boundaries

The [registration](../registrations/S02-stable-selection.md) crosses all six permutations of three requests, two descriptor orders, repair/no repair of the oldest input, four readiness masks and zero through three tokens. These 384 cases each run under Scan and Indexed. Eight additional cases use a consuming output to enable an older suspended request, under two request orders and four token counts. Every case has ordinary scalar, encoded scalar and counter-free local-handle checks.

The tests compare complete decoded outputs and residual multisets, including aliases and unconsumed tokens. Internal unused node identities do not become source outputs. Decoding requires one epoch and rejects leftover eligibility records. Both registered runs reproduce all 768 matrix and 16 chain comparisons. Runtime bounds are 60 seconds wall/CPU, 1 GiB address space and 200,000 service steps; no cutoff occurs.

This gate uses preassigned source requests and one consuming rule whose body posts a represented equality. Fresh request insertion, arbitrary multioperation bodies, guards, choices and cancellation remain unqualified for this protocol. The [body-completion counterexample](S02-integrated-bodies.md) still applies to any extension: making requests ready during a body does not authorize another source firing before that body completes.

No reference-interpreter code or host matcher behavior changes. The independent scalar evaluator also executes the encoded rules, so discrepancies cannot be concealed merely by agreeing with the compiled implementation. All 26 containing tests retain the earlier consistency, branch, alias, dependency and body checks, with their original scopes.

## Costs and responsibilities now exposed

The experiment supplies a concrete organization to measure. Tickets survive repair; precedence facts maintain source order; eligibility propagation discovers candidates; pruning selects among them; epochs renew discovery; host propagation history prevents duplicate firings. Kernel rules still own represented equality, descriptor repair and finite-tree consistency. The ordered host agenda remains part of the protocol's execution assumptions.

For N preassigned requests, this encoding constructs N(N−1)/2 precedence facts. That is an analytical property of this implementation, not a lower bound for CHR selection. Each renewed round can discover surviving eligible requests again. Descriptor repair and the existing eager ancestry closure add separate obligations. None of these operations has yet been charged in a full lifecycle comparison of this protocol.

The local-handle competitor preserves the request occurrence while repairing registered handles. The conventional source executor uses its existing request ordering and dedicated equality representation. The CHR protocol cannot be credited with eliminating those services until its own discovery, history, translation and ownership costs are included. Different implementations of ticket ordering or cycle detection remain viable if attribution shows these costs can change the decision.

## Decision after the four-package review

The four packages since the nonground-post breadth review are ownership/work, candidate-copy attribution, request-order attribution and stable CHR selection. The [portfolio review](S02-stable-selection-review.md) selects **one bounded ownership/work comparison** on this qualified common fragment before adding source capabilities. It can answer whether the new protocol's selection and representation costs are consequential against existing local and ordinary controls.

Intermediate joins are the strongest distinct alternative, followed by compact solving's missing operations. Demand repair and runtime remain required. At the next cost result or obstruction, reconsider those alternatives rather than automatically extending this protocol.

The [audit](s02-stable-selection/audit.json), [source/binary freeze](s02-stable-selection/freeze.json) and [raw receipts](s02-stable-selection/) preserve the evidence. The result supports bounded source correspondence, not speed, memory superiority or research completion.
