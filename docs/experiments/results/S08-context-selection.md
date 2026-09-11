# Context selection: density does not qualify a general policy

**Neither tested selector qualifies for adoption.** The dense-context speedup is real, but cardinality does not tell us whether checking will succeed or stop at the first conflict. We implemented and tested a repair, including new sizes; it preserves the dense gain and still has consequential adverse cases.

This narrows a concrete design choice. It does not reject context optimization, adaptive execution or graph architectures. The production default remains individual lookup; these selectors are isolated experiments.

## The experiment and the repair

The first selector uses seeking when required assignments exceed one quarter of the context, otherwise lookup. Its 360 cases include tiny contexts, both sides of that boundary, prefix/suffix/separated keys, missing keys and early/late conflicts. It produces 131 qualified improvements and 63 qualified regressions against lookup; 166 comparisons are unresolved under the registered criterion.

Separate comparisons identify seeking initialization as an important cause. A late-key probe first starts at the context beginning and then seeks again. For the 16-key context with five late keys and an immediate conflict, lookup performs two key comparisons, seeking four, and the selector four. The selector's median time is 3.22× lookup; seeking alone is 3.01×. Dispatch is not the sole cause.

The repair starts traversal at the first required key and uses lookup for contexts of at most eight entries. We registered that change before confirmation. All 360 original cases remain controls; 292 cases at sizes 32, 65, 127 and 257 are new challenges.

| Confirmation group | Qualified faster | Qualified slower | Unresolved |
|---|---:|---:|---:|
| Original 360 cases | 65 | 69 | 226 |
| New 292 cases | 62 | 57 | 173 |

These counts describe fixture coverage, not workload weights. They must not be averaged into an architecture ranking.

## Why the repair is still insufficient

At the new size of 257, matching all assignments costs **0.298× lookup time** (nine paired ratios: 0.289–0.312). An immediate conflict at that same density costs **2.379×** (2.271–2.408). Range initialization pays for an iterator that the failed check never needs to advance.

The repair reduces the 16-key late-conflict case from four key comparisons to three, versus lookup's two; its confirmation time is still 2.465× lookup. At size 256, dense success needs 259 comparisons versus lookup's 3,135. The operation saved by traversal is real, and so is the setup cost in short-lived checks.

Tiny matching late-key cases no longer show the initial roughly 3× penalty, but dispatch still has measurable costs in some tiny cases. This prevents claiming that selecting the lookup branch makes the candidate identical in cost to an unconditional lookup.

The analytical conclusion is specific: this cardinality rule cannot distinguish these successful and early-failing cases. A prefix probe, observation-driven selection or different representation could distinguish more of the relevant work; none has been disproved. Each would need to charge its probing or state costs and survive new cases.

## Architectural consequence and next investigation

The preceding [complete lifecycle comparison](S08-context-inclusion.md) establishes a dependency benefit from seeking, but its representative repaired graph lifecycle still costs 33.1 ms versus 0.56 ms direct. Neither selector passed the operation gate, so neither was wired into production or credited with a lifecycle gain. Repeating the complete matrix for a failed general policy would not answer its adverse operation cases.

Perform the cross-family review now, before spending a fourth package on predicate refinement. Compare a prefix-probe intervention against memo-storage attribution and partner planning, along with the other required directions. Partner planning could avoid whole discovery paths; memo investigation needs fresh attribution after context savings. A further selector has low implementation cost but a narrower potential benefit and now needs failure-sensitive logic. These are priority judgments, not resolutions of those directions.

T074 remains active for that review; this is package three since the preceding full review. No language restriction, fixture exclusion or workload weighting is introduced. Complete architecture comparison and the 57-question investigation remain unfinished.

## Evidence

The initial and repaired truth gates each check all 531,441 independent assignment pairs. Native evidence contains 9,720 initial and 17,604 confirmation rows, with ordinary allocation, no engine counters, fixed CPU, nine interleaved pairs and all fixture answers checked outside timing. A 10% median change counts only when every pair is on the same side of one. Large-ratio ranges and all unresolved results remain in the receipts.

Default library and integration regression tests pass; scoped Clippy passes. Changes are confined to test-only candidates and experiment infrastructure. Sources used for each native campaign are frozen separately; the later diagnostic source and its hash accompany the work-attribution receipt.

Registrations: [selection](../registrations/S08-context-selection.md), [repair](../registrations/S08-context-selection-repair.md). Analyses: [initial](s08-context-selection/analysis.json), [confirmation](s08-context-selection-repair/analysis.json). Reproduce their audits with `research/chr-reuse/experiments/audit_context_selection.py` and `audit_context_selection_repair.py`.
