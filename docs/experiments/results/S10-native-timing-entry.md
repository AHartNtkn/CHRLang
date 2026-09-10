# Native lifecycle timing preserves the qualified observations

An ordinary-allocator native runner now reproduces all 383 prepared-query observations and cancellation results. A sanitized build reproduces them again, and 50 additional queries pass across three successive sessions. This qualifies native measurement boundaries for the next cost study; it provides no architecture ranking.

## What changed

The native runtime uses ordinary libc allocation, with no requested-byte headers, prepared-heap snapshots or query-region poisoning. The separate ownership gate still checks those responsibilities. Query construction and session cleanup use exactly its qualified functions.

Two checked edits adapt the frozen native source. Diagnostic iteration updates are compiled out; the visits needed to enforce interruptible service remain. Complete-root publication calls a serialization callback. Matching, reduction, choice handling and observer traversal otherwise remain byte-identical to the frozen source.

The callback flushes each complete serialized root into the memory stream before recording first observation. The final export then copies the stream into independently retained consumer bytes. This adds an explicit publication responsibility that a future Rust comparison must match or account for separately.

## What the intervals mean

| Interval | Work included |
|---|---|
| Runtime initialization | Global runtime arrays and heap setup |
| Source loading | Reading the emitted rules-only program |
| Preparation | Parsing definitions, locating the selector and interning query constructor names |
| Source disposal | Releasing the loaded source buffer |
| Consumer setup | Reading the query count and allocating retained-result slots |
| Query setup | Decoding the numeric query protocol and constructing native query data |
| Observer setup | Memory stream, reduction stack and initial observer frame |
| Service | Reduction, structural traversal and complete-root serialization |
| Serialization, nested in service | Printing a complete root and flushing its bytes to the stream owner |
| Pending-work disposal | Releasing observer frames remaining at cancellation |
| Export | Closing the stream, copying consumer bytes and releasing the stream buffer |
| Query disposal | Releasing the reduction stack and resetting query storage |
| Prepared disposal | Releasing definitions, names, parser paths, arrays and mapped runtime storage |
| Consumer disposal | Releasing immediate or retained answer bytes and result slots |

**Service includes structural observation traversal.** Reduction and traversal interleave in the existing native observer. The runner does not pretend they are independent execution and observation phases. It records service as a whole and partitions off serialized publication; it does not insert timestamps around every reduction step.

First-observation latency starts at service entry and ends when the first complete serialized root is available in the memory stream. It is null when no root is published. In each 383-query replay, 317 queries publish bytes and 66 do not. Complete failure and early cancellation therefore cannot acquire a spurious first-answer timestamp.

The lifecycle total sums disjoint named intervals. Serialization is already inside service and is never added twice. Transport to the external validator and result-log writes are outside the total. Native source emission and the construction/encoding of source-level queries are also outside this runner; these must be measured before claiming source-to-consumer total cost. Numeric protocol decoding is explicitly charged within query setup.

## Evidence and its limits

Both builds preserve complete output bytes, raw multiplicity, cancellation prefixes, service-call counts, pending status, unsupported counts and used native heap words for every registered query. The ordinary three-session sequence reproduces the first, last and first rulesets' standalone observations. Retained outputs are published after prepared disposal.

The independent audit checks 816 query phase partitions, all input hashes, exact observation replays, the two native-source edits and the unchanged query/cleanup functions. The earlier ownership audit also passes unchanged. UBSan reports no runtime diagnostics. Compilation with `-Wall -Wextra` reports five unused-parameter warnings already present in the frozen reducer; no new harness warning appears.

The recorded nanoseconds are qualification data, not repeated comparative measurements. Timestamp cost, stream flushing and allocation behavior need to be considered when selecting a practical timing scale. An ordinary allocation path does not itself measure requested traffic, resident memory or reclamation. The diagnostic ownership meter excludes libc stream internals and mapped regions; it cannot silently become a complete allocation-efficiency measure.

## Next decision and competing investigation

Continue T078 by qualifying substantive mixed sources and the Rust measurement paths. Vary useful work, choice count, collision/repair pressure and query reuse independently where the source semantics allow it. Include applicable source-derived elimination controls. Then register the exact comparative matrix after checking complete observations, counter configuration, equivalent consumer ownership, clock overhead and allocation-accounting scope.

This remains more decision-relevant now than extending native local claims: both execution paths have reusable entry points, and a complete-cost contrast can expose which native responsibilities warrant further investment. A plausible contrary result is that preparation, serialization or allocation overwhelms a reduction advantage. Another is that competent source analysis avoids work on both paths. Either would change the next architecture experiment.

Local claims, general constructors, sustained graph handles, learning, restoration and the other mapped directions remain required. The next package must include a breadth review before committing to a cost matrix, comparing that matrix with the strongest ready mechanism investigation. No architecture is selected by completing this runner.

## Reproduce and inspect

Run `python3 research/chr-hvm/lifecycle/build.py`, then `gate.py` and `audit.py` from that directory's repository path. The build requires the frozen native source from the qualified prepared gate. Existing raw records are the current evidence; rerunning the gate produces fresh timing records rather than a confirmation matrix.

[Registration](../registrations/S10-native-timing-entry.md), [runner](../../../research/chr-hvm/lifecycle/harness.c), [build adapter](../../../research/chr-hvm/lifecycle/build.py), [semantic and phase checker](../../../research/chr-hvm/lifecycle/check.py), [audit](s10-native-timing-entry/audit.json), [native-source edits](s10-native-timing-entry/native-source.diff), [ordinary replay](s10-native-timing-entry/ordinary.jsonl), [sanitized replay](s10-native-timing-entry/ubsan.jsonl), [successive sessions](s10-native-timing-entry/sessions.json), [build receipts](s10-native-timing-entry/build.json), [upstream warning check](s10-native-timing-entry/upstream-warnings.json), [input hashes](s10-native-timing-entry/validation.json) and the [ownership evidence](S10-native-prepared.md) retain the inputs and results.
