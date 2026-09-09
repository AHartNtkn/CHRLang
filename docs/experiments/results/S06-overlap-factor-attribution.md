# Factoring removes a large overlap cost without losing correlation

Factoring constructor alternatives substantially reduces the tested overlap allocation cost while preserving values and original source counts. It adds preparation work and does not make the lazy solver cheaper than enumeration across the inspected cases. This is an allocation attribution, not a timing result or architecture selection.

## The transformation has an exact boundary

Two constructor alternatives can be combined when only one child position differs. For example, `f(A,C) ∪ f(B,C)` denotes the same values as `f(A ∪ B,C)`. The implementation constructs and recursively reduces that child union while retaining identical children in their original positions. It operates on finite acyclic membership descriptions; original source transitions remain available for multiplicity counting.

Several differing children cannot be unioned independently. `pair(a,a) ∪ pair(b,b)` has two values; independent choices for both positions would introduce `pair(a,b)` and `pair(b,a)`. The new counterexample test requires those extra values to remain absent. Another test requires the overlapping value `pair(a,a)` to retain two source derivations after membership factoring removes its duplicate proof.

Each factoring operation reduces the number of alternatives at its current node. Recursive unions use lower-rank child languages, so they do not introduce a reference back to the parent. This supplies a finite-description termination argument. The reducer is still not complete language minimization; its preparation and retained intermediate states remain implementation costs.

## Correctness and controls pass

The [prospective registration](../registrations/S06-overlap-factor-attribution.md) fixes this transformation, adverse correlation test and repeated allocation matrix. The initial overlap test failed because the structural reducer still produced one duplicate value; it now passes.

The new exhaustive test checks **8,192 overlapping-product requests** over every subset of nine binary transitions, both alternative orders, two root/filter choices and four path-constraint choices. Each runs unreduced and reduced and agrees with independent enumeration on both values and counts. Together with the existing 7,840 requests, this gives **32,064 solver comparisons**. The 160 actual CHR source checks remain passing.

All **26 package tests** pass with default and counter-free features, including the five cost controls' 360 changed-query comparisons in each build. Strict scoped Clippy passes. The independent oracles, source counting and execution algorithms are unchanged; the membership reducer is the only candidate behavior changed by this attribution.

The new allocation gate completes **720 processes**, with **360 exact allocation replays and full owner restoration**. All **576 non-reduced runs** have identical phase allocation readings to the prior corrected gate after baseline normalization. The paired audit checks both source freezes, including baseline source from commit `12f1a409e`. It treats elapsed-time fields in meter records as diagnostics, not speed evidence.

## Full allocation accounting exposes both benefit and overhead

These examples run four changing queries over six-bit descriptions, release answers immediately and enumerate completely. Requested bytes include preparation, query inputs, setup, execution, observation and all disposal. Peak growth is measured above the process's initial owner baseline, not RSS.

| Reduced solver family | Before: requested bytes | After: requested bytes | Before → after peak growth |
|---|---:|---:|---:|
| Structurally different overlap | 61,298,434 | 3,719,454 | 118,155 → 71,550 |
| Identical redundant alternatives | 625,554 | 640,994 | 21,144 → 21,144 |
| Selective filter | 702,556 | 717,996 | 26,425 → 26,425 |
| Equal bit positions | 206,882 | 211,666 | 14,564 → 14,564 |
| All values | 3,666,884 | 3,671,668 | 67,266 → 67,266 |
| Empty membership | 396,770 | 412,210 | 16,383 → 16,383 |

The overlap improvement survives complete accounting. Preparation itself increases from **30,028 to 60,064 requested bytes**; later execution saves much more. Factoring changes how many membership derivations reach the same value. The search-state copying policy is unchanged, so this isolates the effect of the reduced description through that policy; it does not measure copying's separate contribution or prove that copying is optimal.

The other inspected families have no corresponding execution saving. Their increases occur in preparation. These are costs of this reducer implementation, not lower bounds for structural solving. More precise timing or preparation optimization must be justified by the architectural decision it could change.

The strong controls still matter. On this overlap case, fresh enumeration requests **1,058,460 bytes**, reused enumeration **727,054**, and exact-family elimination **166,166**. Factoring closes much of the avoidable gap but does not erase it. Conversely, the earlier equality case shows lower peak memory for lazy solving than enumeration. No single allocation endpoint settles the choice.

## Next decision

This fourth bounded package triggers the [structural breadth review](S06-structural-breadth-review.md). The review selects a bounded ordinary-timing pilot next because validated controls and lifecycle endpoints are ready and allocation alone cannot establish total efficiency. That additional package must include preparation overhead and adverse sources; it must not become another unrestricted reducer-refinement cycle.

Restoration/reunion and integrated dependency repair remain required distinct investigations. Broader consuming-source correspondence, recursive descriptions, retained exact-observation history, richer structural theories and complete architecture comparison also remain open. The current finite fragment supports further comparison, not architectural closure.

## Evidence and reproduction

Run `cargo test -p chr-structural` and repeat with `--no-default-features`. Run `python3 research/chr-structural/experiments/audit_factor_pair.py` to check the paired evidence and freezes. The registered runner is `factor_ownership.py`; it refuses to overwrite existing records.

[Raw runs](s06-overlap-factor-attribution/runs.jsonl), [phase summaries](s06-overlap-factor-attribution/summary.json), [gate audit](s06-overlap-factor-attribution/audit.json), [paired audit](s06-overlap-factor-attribution/paired-audit.json), [source freeze](s06-overlap-factor-attribution/freeze.sha256), [driver/registration freeze](s06-overlap-factor-attribution/driver.sha256), [package tests](s06-overlap-factor-attribution/package.log) and [counter-free tests](s06-overlap-factor-attribution/package-off.log) preserve the evidence.
