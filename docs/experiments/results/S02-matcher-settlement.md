# Preserving consumer priority without settling every equality

**Consumer priority can be preserved while unrelated output equality remains unfinished.** The new controller saves work when a consumer fails early. Successful cases expose extra scheduling work, which must be repaired and measured before choosing this organization.

Two frozen confirmations each pass 120 new source cases, alongside the existing priority and early-failure controls. Three deliberately incomplete read analyses fail the independent answer checks. The research goal and T072 remain active.

## What the experiment changes

The [previous priority experiment](S02-partial-priority.md) established a real choice: consuming as soon as a guard succeeds can choose a different rule from waiting for equality to settle. This experiment asks whether preserving the existing fixed rule/tuple priority requires waiting for **all** equality.

The fixtures now separate information needed to choose a consumer from information needed only to finish its output. An earlier consumer can already be possible or impossible while a separate nested output equation remains pending. Paired sources make the earlier consumer depend on that same nested equation instead. These are source changes with independently evaluated answers, not assumed interface restrictions.

The candidate settles equations connected to values that any live rule can inspect. It infers those values from constructor patterns, repeated head variables and positive equality guards. It follows pending equations and constructor connections in both directions, recomputing after each deduction. Single opaque variables passed only to bodies do not force settlement.

The existing engine still executes source bodies, consumes exact occurrences, services background equality and refuses to publish an answer until consistency is settled. The experimental controller uses the complete prepared rule set, including contenders whose guards are currently unsuccessful. Newly posted requests change the live values it examines.

## The result, including the adverse cases

The table shows depth 64. Both choices of earlier guard and both token counts produce these work results. An engine advance is one call to the existing source executor; equality deductions inside a settlement loop are counted separately.

| Source outcome | Candidate deductions | Full-settlement deductions | Candidate advances | Full-settlement advances |
|---|---:|---:|---:|---:|
| Separate output equation; source fails early | 4 | 67 | 8 | 8 |
| Separate output equation; successful output | 68 | 68 | 72 | 9 |
| Separate output equation; late contradiction | 68 | 67 | 72 | 7 |
| Shared matcher/output equation; source fails | 67 | 67 | 8 | 8 |
| Shared matcher/output equation; success | 68 | 68 | 9 | 9 |
| Shared matcher/output equation; contradiction | 67 | 67 | 7 | 7 |

**Early failure retains a useful saving under fixed priority.** At depths 4, 16 and 64, the candidate performs 4 deductions versus 7, 19 and 67 with full settlement. The consumer fires with output equality pending, and failure disposes of that interpretation without finishing its output.

**Successful output still owes all its equality work.** The separate-component cases perform 8, 20 and 68 deductions in both schedules. The candidate takes 12, 24 and 72 advances versus 9 with full settlement, because background equality progresses through repeated source advances. Deduction counts alone would hide that overhead.

**Failed speculation can cost more.** A late contradiction in the separate component performs one extra deduction and more advances than full settlement. Both schedules publish no answer. When matcher and output dependencies are shared, the candidate waits and the measured deduction/advance counts match full settlement.

## What makes the result credible

Every completed candidate observation is checked against the independent scalar source evaluator under the original fixed-priority rule order. Successful output bindings, residual occurrences and failed interpretations participate in that comparison.

| Cases per confirmation | Question checked |
|---|---|
| 24 original priority cases | Does selective settlement preserve the consumer that full settlement selects? |
| 72 separate/shared cases | Can consumption proceed before output equality, and when does that save or add work? |
| 9 read cases | Do body posts, constructor patterns and repeated variables enter the readiness analysis? |
| 12 resource cases | Are shared and disjoint consuming resources handled with correct complete observations? |
| 3 cancellation/reuse cases | Can execution be abandoned with equality pending, then reuse the same preparation correctly? |

Removing guard reads, repeated-variable reads or constructor reads each produces a wrong source answer in its targeted test. All three mutations fail semantically, and the production source is restored byte-for-byte before confirmation. The unchanged 24 earlier priority rows and 12 useful-interleaving rows also repeat exactly. The full library has 12 passing tests; scoped Clippy passes.

The reasoning obligation is that remaining consistent equality cannot change a matcher-visible component. Two-way constructor incidence and pending-edge closure are necessary to account for information reaching a matcher indirectly. Source-body boundaries and live-occurrence lookup account for new requests. Failed interpretations still cannot publish. The tests challenge these obligations; they do not establish a theorem for arbitrary future guards or scheduling policies.

## Architecture decision and next experiment

**Retain selective settlement as a viable implementation of the fixed-priority option.** The witness establishes an opportunity that a complete equality barrier misses. It does not select fixed priority as a future language policy; the permissive successful-serialization option remains a distinct comparison.

**Repair and charge the controller's actual costs next.** The implementation builds read metadata, scans live occurrences, constructs pending-edge adjacency, traverses constructor connections, moves queued equations and invalidates matching candidates. Repeated source advances are now an observed cost, not merely a hypothetical concern. Register a paired quiescent-drain experiment: once all matcher-visible equality is settled and no source application can fire, finish unrelated equality without repeatedly rediscovering the same absence of work. Preserve source failure, new-post, cancellation and late-clash checks, then measure preparation, changing-query setup, execution, observation and disposal with separate ordinary and diagnostic builds.

This next step can change whether the demonstrated early-failure opportunity is worth its maintenance cost. Partner ordering is the strongest independent alternative for matching work; graph memo/lifecycle costs are also ready. A bounded repair and cost pilot has higher immediate value because it resolves a newly observed cost in a qualified source mechanism. Do not extend this into unrestricted relevance optimization: compare those alternatives again at the pilot result or obstruction. This is package two since the [full portfolio review](S08-traversal-portfolio-review.md); review all directions within four packages.

The current relevance closure is conservative: a live earlier contender can force settlement even when another consumer uses disjoint resources. Finer commutation is a separate executable question, not a reason to reject the entire integration direction. No elapsed-time, allocation or whole-architecture ranking follows from these work counts.

## Reproduce and inspect

- [Prospective registration](../registrations/S02-matcher-settlement.md)
- [Source and binary freeze](s02-matcher-settlement/freeze.json), [source archive](s02-matcher-settlement/sources.zip)
- [First confirmation](s02-matcher-settlement/confirmation-0.json), [second confirmation](s02-matcher-settlement/confirmation-1.json), [parsed rows](s02-matcher-settlement/audit.json)
- [Mutation receipts](s02-matcher-settlement/mutation-audit.json), [Clippy](s02-matcher-settlement/clippy-final.log)
- [Confirmation runner](../../../research/chr-relational/experiments/matcher_settlement.py), [independent receipt auditor](../../../research/chr-relational/experiments/audit_matcher_settlement.py), [read-mutation runner](../../../research/chr-relational/experiments/check_matcher_reads.py)

Run `python3 research/chr-relational/experiments/audit_matcher_settlement.py` to recheck the archived inputs, complete confirmations and reported formulas. Confirmations were serial, CPU-pinned, bounded at 60 seconds and 1 GiB, and used counters in the test controller. Their wall times are not comparative timing evidence. The retained parser-entry receipt passed the Rust tests but exposed overlapping row labels in the Python parser; the final parser distinguishes those labels and both final confirmations audit successfully.
