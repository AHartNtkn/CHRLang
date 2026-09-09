# Structural solving trades enumeration memory for other work

The finite-path cost controls now preserve complete answers and restore their requested-heap owners across changing queries and cancellation. Lazy solving uses less peak memory on the inspected equality case, but reused enumeration requests fewer bytes. Overlapping filter proofs expose a substantial remaining cost in the lazy solver.

These are allocation and ownership findings. No ordinary-allocator comparative timing matrix has run, and no architecture is selected.

## The controls are complete for this source family

The [registration](../registrations/S06-finite-cost-ownership-gate.md) compares five paths: unreduced lazy solving, structurally reduced lazy solving, fresh enumeration, enumeration retained across queries, and hand-derived elimination for the exact family. Enumeration memoizes grammar states, deduplicates candidate values and chooses a candidate root using a cardinality estimate that ignores identical transitions. Retained enumeration keeps selected root values across queries.

The exact-family control derives bit groups and allowed values directly. It checks requests against the prepared schema's two query forms; it is not a general compiler. All paths include grammar construction during preparation. The reduced path retains original alternatives for source counting as well as its reduced membership representation. The exact-family path needs only the checked schema and identifiers after preparation.

The six binary-list families expose unrestricted output, equal bit positions, selective membership, empty membership, identical filter alternatives and structurally different overlapping filter alternatives. Four queries alternate anchors or accepted leaf values over one prepared owner. Width zero is a useful boundary: every family accepts the empty list because there is no bit at which its filter can fail.

An independent binary-assignment oracle supplies the expected terms using each family's mathematical predicate. It does not call the candidate's intersection, enumeration, membership or count routines. All source derivations in this cost family are unique; filters can have multiple proofs. The broader source gate separately checks duplicate source counts. These measurements do not establish general consuming-CHR filter correspondence.

## Validation and the paired ownership correction

There are **360 cells**, crossing six families, widths 0/4/6, five paths, immediate versus retained answers and full completion versus cancellation after the first answer. Each cell runs twice in separate counter-free allocation-meter processes. Every process first validates all four complete query results outside measured intervals, then starts a fresh measured owner. Empty-result cancellation exhausts normally.

The initial 720-process gate and the corrected 720-process gate both pass: **1,440 processes, 720 exact allocation replays and 1,440 final owner restorations**. Adjacent phase live readings agree, phase order and cardinalities match the registration, and there are no recorded cutoffs. The fixed phase/result buffers are allocated outside the owner baseline; actual answer trees are measured.

The correction addresses an unused retaining owner in the exact-family control. Its prepared object held the full grammar despite no longer consulting it. The corrected object keeps no grammar. All **576 non-lowered runs** have identical allocation readings after normalizing live/peak values to each run's baseline. All **72 corrected exact-family cells** retain only a seven-byte mode string after preparation. Their cumulative requested traffic is unchanged: grammar disposal moves from final owner disposal into preparation.

Initial evidence, frozen source copies and the original binary identity are preserved. The corrected binary has a separate freeze. Process-owned argument strings differ because the executable paths differ; the paired audit excludes that outside-owner offset. Each two-process replay still requires identical absolute readings.

The structural package passes **24 tests** in both default and no-default-feature builds. The five cost paths pass **360 changed-query semantic comparisons** in each build. A dedicated test confirms that finite-solver diagnostics remain zero without the metrics feature while answers are unchanged. Strict scoped Clippy passes for the ordinary configuration and allocation-meter example. These cost paths do not invoke the older structural-operation counters.

## A favorable mechanism can have opposing total costs

The table covers four complete queries of six-bit lists, with answers released immediately. Requested traffic includes preparation, requests, setup, execution/observation and all disposal. Peak growth is the largest phase peak above the initial owner baseline. It is not RSS, and peaks are not summed.

| Family and path | Requested bytes | Peak requested growth |
|---|---:|---:|
| Equal bits — fresh enumeration | 471,690 | 91,567 |
| Equal bits — reused enumeration | 140,284 | 91,376 |
| Equal bits — lazy solving | 200,261 | 13,748 |
| Equal bits — reduced lazy solving | 206,882 | 14,564 |
| Equal bits — exact-family elimination | 18,716 | 2,007 |
| All values — fresh enumeration | 1,047,490 | 90,927 |
| All values — reused enumeration | 716,084 | 90,832 |
| All values — lazy solving | 3,660,263 | 66,450 |
| All values — reduced lazy solving | 3,666,884 | 67,266 |
| All values — exact-family elimination | 161,164 | 868 |

Equality makes the lazy solver avoid constructing the full candidate language. Reusing an enumerated domain instead saves repeated preparation, which lowers cumulative traffic in this small case but retains the candidate values. Both obligations matter; these numbers supply no timing ordering.

The unselective family still requires every output value. Its lazy state propagation and exact-observation history request more bytes than the enumeration controls. The hand-derived path shows how much work this particular source permits eliminating; its result is not evidence that a general compiler can do so across the language.

## Output retention and reusable preparation have different owners

In the six-bit all-values family, the reused-enumeration owner grows from **759 bytes after preparation to 49,183 bytes after query 1 and answer release**, and stays there through query 4. Those candidate values survive consumer release intentionally. Fresh enumeration returns to its 758-byte prepared owner after each query. Lazy solving returns to 753 bytes; reduced solving returns to 1,569 bytes.

Holding all answers until each query ends separates consumer memory from engine memory. After query disposal but before answer release, the lazy path has 39,729 bytes above baseline; after answer release it has 753. The difference, **38,976 bytes**, is the owned answer trees. Reused enumeration retains 88,159 bytes before release and 49,183 afterward: the same consumer output amount sits alongside retained candidates.

All prepared owners return to the initial baseline on final disposal, including after early cancellation. This establishes finite-lifetime ownership, not a bounded cache policy or sustained-memory guarantee. Immediate release does not reclaim the lazy solver's exact seen-value history while its query remains alive.

## Overlap is the consequential next attribution

The six-bit redundant-filter case requests **8,511,297 bytes** in unreduced lazy solving and **625,554 bytes** with structural reduction. Fresh and reused enumeration request 67,988 and 46,537 bytes. Reduction removes a real proof-duplication cost, but its full representation and execution costs still matter.

Structurally different overlap survives that reduction. The lazy and reduced paths request **61,274,233 and 61,298,434 bytes**, while fresh and reused enumeration request **1,058,460 and 727,054 bytes**. These grammars admit the same binary values through alternatives such as a fixed leaf or either leaf under the same list constructor. Their redundant proofs are not identical transitions, so the current structural reduction does not combine them.

This is evidence against treating the current reducer as sufficient for the overlap family. It is not a rejection of compact structural solving. The next bounded investigation should attribute repeated membership proofs versus state copying and test a source-valid reduction that can combine these overlapping descriptions. It must preserve correlations when several child positions differ; replacing correlated alternatives with independent unions can introduce values. Preserve source multiplicity separately.

Counting the finite-path gate, cost-control gate and isolated owner correction gives three bounded packages since structural solving was selected. The next attribution package triggers the four-package breadth review, including a fresh comparison with restoration/reunion and integrated dependency repair.

That attribution precedes broad timing confirmation because a cheap correction could change the comparison substantially. Then register ordinary-allocator timings with preparation, first/full observation, reuse and disposal. Checkpointed restoration/reunion remains the strongest distinct alternative at that boundary. Broader source theories, retained-history policies and coherent architecture comparison remain required.

## Reproduce and inspect

`python3 research/chr-structural/experiments/finite_ownership.py --audit` audits the final gate. `python3 research/chr-structural/experiments/audit_finite_ownership_pair.py` verifies the source/binary freezes and paired correction. Running the gate script without `--audit` launches the registered matrix and refuses to overwrite evidence.

[Final raw records](s06-finite-cost-ownership-gate/runs.jsonl), [phase summaries](s06-finite-cost-ownership-gate/summary.json), [audit](s06-finite-cost-ownership-gate/audit.json), [paired correction](s06-finite-cost-ownership-gate/paired-audit.json), [source/binary freeze](s06-finite-cost-ownership-gate/freeze.sha256) and [initial audit](s06-finite-cost-ownership-initial/audit.json) preserve the findings. Elapsed-time fields in meter records are not analyzed as speed evidence.
