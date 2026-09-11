# Query-window reclamation

Question: can periodic reclamation retain useful call reuse while reducing the
state left by completed or cancelled queries? Reuse the existing table, caller,
source fixtures and lifecycle runner. No new executor or language restriction.

Clear the table between queries, retaining prepared rules. Require no live query
handles so existing work cannot refer to reclaimed trace indices. Compare no
clearing with windows 1, 4 and 16. Check exact delivery/exhaustion against Direct
at every service step, independent complete answers, cancellation/restarts, fresh
output aliases and answers retained after clearing and producer disposal.

The ownership pilot uses four existing families, depth8,32 queries, policies
repeat, four-query cycle and sixteen distinct depths then revisit. Exhaust or
cancel after the first answer. Dispose outputs immediately or retain all outputs.
Compare the four trace policies, Direct and data plans plus inference:288 cells.
Run each allocation cell twice in a single-process-at-a-time release build;
require equal requested bytes, peak/live ownership and consumer bytes across
repetitions. Keep preparation, query service, maintenance and final disposal
accounting explicit. Every actual answer is checked outside measured phases.
Per process: 1GiB address space,60 CPU seconds,90 wall seconds; existing100000
service-step limit per query. Record compact numerical results, not source copies
or hashes. Stop and repair any wrong answer or accounting failure.

Counters in a separate semantic/work test quantify regeneration versus replay.
Do not infer time gains from allocation or work counts. Select ordinary lifecycle
timing only after judging whether a policy changes the useful time–memory tradeoff.
The following portfolio review must consider sparse projection and broader caller
observers alongside retention; a reclamation pilot does not resolve general reuse.

## Ordinary timing, registered after ownership results

Ownership qualifies all288 cells. Window1 lowers peak in27/48 scenarios, window4
in16/48, window16 in none; all increase requested allocation. Measure the same
288 cells in five randomized blocks, seed607100, with an ordinary allocator and
engine/kernel counters disabled. Build once before running; no concurrent builds.
Use complete phase sums including maintenance; validate answers outside phases.
Compare each of four trace policies with Direct and planned-plus-inferred, and
each bounded policy with unbounded traces:528 comparisons. Gain requires median
paired ratio<=0.9 and every repetition<1; loss requires median>=1.1 and every
repetition>1. Other results remain uncertain. Both medians must exceed100 times
the largest of three existing clock-check p99 readings. Same process bounds.
Record samples and results compactly. Inspect peak regressions by phase in the
largest relative-regression case for each window, including retained consumers.
