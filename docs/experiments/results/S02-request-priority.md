# Representation repair can change which source request wins

Moving request selection ahead of descriptor selection fixes the CHR encoding's descriptor-order discrepancy when no repair occurs. It does not preserve request priority after equality repair removes and reposts an older request. The new occurrence can lose to a newer source request even though the represented source inputs are unchanged.

**The next CHR integration experiment needs stable source-request identity and eligible-request selection.** This is a concrete missing responsibility for a comparison under ordinary request priority. It does not reject CHR-expressed equality or adopt a language scheduling policy.

## Why this experiment comes next

The [local lifecycle study](S02-local-lifecycle-sizing.md) and [multihead comparison](S02-multihead-lifecycle.md) already give integrated local execution complete-source cost evidence, including adverse memory results. The [CHR constructor gate](S02-chr-constructors-gate.md) has independently checked equations and consuming chains, but its competing-consumer example produces a different answer from the ordinary source control. Repeating local timing would not resolve that CHR-specific obstacle.

The [registered experiment](../registrations/S02-request-priority.md) tests a small source-protocol change before introducing broader selection machinery. The unchanged rule keeps a constructor descriptor and consumes a request and token. The alternative consumes request, token and descriptor in that order, then reposts the descriptor and emits the result equation. This makes the request the first matching head without modifying the host executor.

The alternative preserves the descriptor's logical content, but changes its occurrence identity and can repeat propagation work. Passing this correctness question would not establish that the alternative is economical or generally interchangeable with a kept descriptor.

## Exact outcomes, including the remaining request

Two requests seek the children of `f(a)` and `f(b)`. One token lets only one request consume. Ordinary source priority gives it to the earlier request; both the bound output and the residual losing request identify the outcome.

We cross both request orders, both descriptor orders, no repair or repair of either input through an equivalent fresh node, and zero/one/two tokens. There are 36 source configurations and two encoding variants. Scan and Indexed give 144 compiled comparisons per repeat. The independent scalar evaluator executes both ordinary source and encoded rules; complete decoded answers preserve output aliases and residual multiplicity. The existing local-handle path agrees with ordinary source in all 36 cases.

| Contrast, per repeat | Kept descriptor first | Request first, descriptor reposted |
|---|---:|---:|
| Complete compiled comparisons | 72 | 72 |
| Disagreements with ordinary source | 12 | 8 |
| Disagreements without input repair | 4 | 0 |
| Disagreements after input repair | 8 | 8 |

All disagreements occur with one token. Zero tokens leave both requests; two tokens service both. Both compiled access modes agree with independent execution of their encoding, and the two registered repeats reproduce every result exactly. These are precise scheduling counterexamples, not implementation crashes or unexplained semantic failures.

In the decisive case, the older request initially precedes the newer one. An initial equality moves its input to an equivalent representative. The source-level repair rule consumes the old request occurrence and posts its corrected copy. The independent trace confirms that repair runs before the consuming application. Request-first tuple discovery now encounters the newer request first, and that request wins. Descriptor order no longer explains the discrepancy; source identity and representation occurrence identity have separated.

Changing the request's input representative does not change the source-level priority under the matched control contract. The local-handle implementation preserves the source request while repairing its handle, which is why it passes this contrast. This is an observed distinction between organizations, not evidence that only local handles could preserve priority.

## What must be qualified before costs

A CHR source protocol must preserve the original request identity across repair and select the earliest eligible source request under the chosen comparison contract. A numeric ticket by itself does not make tuple discovery choose the least ticket. Selection must also avoid blocking behind an older suspended request when a younger request is eligible. Fresh posts, source-body completion, failures and cancellation can invalidate pending eligibility or claims.

The next bounded gate should exercise those obligations with two and three competing requests, delayed constructors and merge-induced readiness. Keep complete ordinary and scalar-encoded answers, the existing local path, and the present request-first variant as the causal control. Measure selection and repair work once correctness qualifies; charge that machinery in the eventual common-source lifecycle comparison. An alternative committed-choice language contract needs its own explicitly specified comparison rather than treating different winners as equal work.

This is package three since the nonground-post breadth review, following ownership and copy attribution. One selection-protocol gate has a direct path to making the CHR organization eligible for the required comparison. At that result or obstruction, the four-package review must compare further integration work with intermediate joins, compact solving and the remaining demand cost/capability work. Neither the bounded source gate nor scheduling priority resolves those alternatives.

## Validation

Both targeted repeats pass; all 24 containing constructor tests pass, including finite-tree consistency, hidden cycles, branches, aliases, integrated bodies and ownership. Strict scoped release Clippy passes. Runtime processes have 60-second wall/CPU and 1-GiB address-space limits. No cutoff occurs and no timing or allocation conclusion follows.

The [audit](s02-request-priority/audit.json) verifies the full cartesian coverage and exact repeated outcomes from raw receipts. The [source and binary freeze](s02-request-priority/freeze.json) preserves the executed test and dependencies in an archive. [Build, runtime and lint receipts](s02-request-priority/) retain exact commands and outcomes. The reference interpreter is unchanged.
