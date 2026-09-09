# Integrated local execution shows lower sampled times and higher memory costs

The corrected lifecycle sizing gives the local executor a credible favorable result on the tested source fragment. Specialized compiled scanning often requests fewer bytes and reaches a substantially lower heap peak. These are conditional time–memory findings, not a choice of complete architecture.

Two prospectively registered matrices completed **3,248 processes** in total. The first exposed a consequential allocation side effect in answer validation; the paired rerun corrects it. All complete answers agree with independent scalar semantics. Each matrix has 840 counter-free ordinary-allocator timing processes, 448 allocation processes and 336 work processes. Every paired diagnostic and allocation record agrees, and every measured allocation session restores its baseline.

## Validation outside a timer can still disturb the next query

The first runner cloned both complete answers before exact comparison between queries. Those clones were outside every phase interval, but deep recursive outputs created substantial allocator traffic before the following setup. That made later setup look expensive despite unchanged setup allocation counts.

The [paired registration](../registrations/S02-local-validation-attribution.md) changes validation to compare the actual and independent expected answers by reference. Raw cardinality is checked separately; the exact comparator still checks outputs, aliases and residual multiplicity. The scalar semantics and measured source computations are unchanged.

At chain depth 128 with four queries, local Endpoint setup medians by query position changed from **143 / 1,181 / 1,139 / 1,143 µs** to **132 / 292 / 288 / 290 µs**. Specialized Scan changed from **162 / 1,261 / 1,237 / 1,238 µs** to **149 / 399 / 405 / 407 µs**. Both organizations were affected. This supports a harness-allocation explanation; it does not attribute the remaining position effect exclusively to the engine.

All **224 normalized allocation cells** and **168 work cells** match between the two matrices. Normalization subtracts the session baseline because the different executable paths change process argument storage. Requested traffic and relative live/peak heap are unchanged. The [paired audit](s02-local-validation-attribution/paired-audit.json) preserves this check.

The borrowed-validation matrix is the primary sizing evidence below. The original [registration](../registrations/S02-local-lifecycle-sizing.md), [receipts](s02-local-lifecycle-sizing/runs.jsonl), source archive and frozen binaries remain an attribution control. Necessary comparator scratch and cache effects can still affect subsequent queries; this experiment does not claim a measurement harness has no influence.

## Complete costs, including preparation and disposal

Seven controls execute the same source: three local dependency organizations, compiled generic Scan/Indexed, and compiled specialized Scan/Indexed. Each ruleset is prepared once and reused for one or four changing queries. Preparation, setup, execution, complete observation, query disposal, answer release and preparation disposal are measured separately. Phase sums supply the lifecycle total. First/full observation coincide for these deterministic single-answer sources.

The sources include recursive constructor bodies with fresh residual links, independent ground requests, requests enabled by a shared binding, and low-yield unknown aliases that leave other requests suspended. Sizes are 4, 32 and 128. The full [summary](s02-local-validation-attribution/summary.json) contains all 168 control cells; the table shows size 128 with four queries, in microseconds. Values are medians of five isolated timing processes, **not confirmed speed classifications**.

| Source | Local Endpoint | Local Filtered | Local Indexed | Specialized Scan |
|---|---:|---:|---:|---:|
| Recursive bodies | 7,640 | 7,736 | 7,774 | 8,275 |
| Independent ground requests | 429 | 447 | 466 | 760 |
| Shared binding enables requests | 493 | 484 | 593 | 847 |
| Low-yield alias repair | 588 | 545 | 569 | 925 |

Local medians are lower than Specialized Scan in all 24 source/lifetime cells for each dependency mode. Some timing ranges overlap or contain substantial outliers. This motivates further architectural investigation; it does not establish a universal ordering or justify turning these cells into workload weights. Generic Scan and Indexed controls are retained in the raw evidence, including their contrary allocation results.

Memory gives a different comparison. The following values use **requested heap traffic in decimal MB** and **peak requested-live growth in decimal kB**, including prepared state and retained output through query disposal. They exclude source/oracle/harness storage already present at the session baseline and are not RSS.

| Source, size 128 / four queries | Endpoint traffic | Specialized Scan traffic | Endpoint peak | Specialized Scan peak |
|---|---:|---:|---:|---:|
| Recursive bodies | 6.709 MB | 6.100 MB | 1,359 kB | 1,326 kB |
| Independent ground requests | 1.104 MB | 0.970 MB | 137 kB | 49 kB |
| Shared binding enables requests | 1.412 MB | 1.199 MB | 161 kB | 64 kB |
| Low-yield alias repair | 1.552 MB | 1.438 MB | 206 kB | 170 kB |

The low-yield Filtered mode requests about 1.437 MB, while generic Scan requests 1.386 MB. The compiled variant with the smallest traffic is therefore not always the specialized one. The local graph retains nodes, handles and request slots through query disposal; the current evidence establishes that ownership boundary but does not yet attribute every peak difference to necessary versus avoidable representation.

## Fewer notifications do not settle the dependency organization

For 128 low-yield requests and one query, all controls perform one consuming source application. Endpoint performs **257 inspections and 128 notifications**; Filtered performs **129 inspections and 128 notifications**. Local Indexed also performs **129 inspections**, avoids notifications, and pays **128 pair operations, 128 membership edits and 256 incident edits** during registration.

The lifecycle matrix charges those costs rather than ranking modes by notifications. Local Indexed has a higher sampled total and peak than Filtered in the large low-yield cell, while their ranges overlap. This source has only one alias update: it is an overhead challenge for the index, not its strongest reuse opportunity. The prior repeated-alias work evidence cannot substitute for a complete source-driven lifecycle comparison.

Recursive outputs expose another limit on an execution-only claim. Observation and answer release are substantial because complete residual links repeatedly contain nested structure. Specialized Scan's execution phase can be cheaper even where the local total median is lower. All outputs remain fully materialized and independently checked; no output work is silently excluded.

## What changes in the architecture decision

**Direct local integration remains a credible candidate on its accepted fragment.** It avoids generic tuple discovery and some repeated repair work while showing lower sampled complete times. The current compiled control is genuinely source-specialized, so this comparison is stronger than one against a generic interpreter alone.

**The memory and capability obligations remain material.** The local plan still accepts one particular two-head source shape, no guards or OR, and a bounded body language. General heads, propagation history, multiple-rule scheduling and sustained reclamation are untested architectural work. Compiled preparation retains broader capabilities, and this experiment does not prove which preparation or representation responsibilities a complete local architecture can avoid.

**The next bounded question is repeated source-driven alias updates.** Before choosing Endpoint, Filtered or Indexed, give registration-heavy indexing a favorable lifecycle opportunity: vary the number of alias-producing source applications independently from suspended requests, using the existing rule/body capability. Compare the same complete controls and retained owners. This can change whether maintaining pair incidence is worth its complexity; one low-yield update cannot answer it.

The strongest ready alternatives are repeated dynamic reunion and broader source-derived elimination, both of which could change the execution organization more substantially. The selected follow-up has low implementation cost and directly tests the missing favorable condition of a measured mechanism. It must end with a bounded dependency conclusion or a consequential defect diagnosis, followed by selection against those alternatives; it does not authorize indefinite local refinement. Peak ownership attribution and broader integration remain required. This sizing and its validation attribution are two packages since the last four-package breadth review.

The [independent audit](s02-local-validation-attribution/audit.json) verifies order, commands, repetitions, phase arithmetic, restoration, source/binary freezes and work signatures. Default relational tests, all-feature test targets, source smoke gates, scoped strict Clippy and formatting pass. The all-features package command is rejected by the existing counter-free-only lifecycle example; its safeguard is preserved and the rejected command is recorded in the validation logs. No compilation cost, complete architecture superiority, language adoption or goal completion follows.
