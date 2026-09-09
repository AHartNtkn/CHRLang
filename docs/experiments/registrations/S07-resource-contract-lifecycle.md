# S07 checked-contract lifecycle pilot

Status: registered before comparative runs. This pilot charges the concrete resource-access and ground-query contract established by the S07 gates. No language policy is selected.

## Questions and controls

H1: a declaration adds source checking, retained declaration storage and input validation to inferred counting; measure their full effect rather than assuming that explicit properties remove work. H2: preparation and shallow-input regimes expose overhead that deeper execution may hide. H3: unknown submitted depths distinguish admission policies rather than execution efficiency; rejecting a query is not a faster way of producing its answer.

Compare `inferred` (no declaration, source-derived counting enabled) and `declared` (checked fuel-access region and ground start argument, same counting enabled). Both use the same existing Scan prepared executor through the owned source/query boundary. No new baseline engine is introduced. The declared optional and required paths are identical when the same declaration is present; do not duplicate their timings. Required admission without a declaration is already an executable rejection test.

The counting program is inferred once from the same source that is prepared. Groundness of the original submission is checked before transformation. Failed counting eligibility retains ordinary execution; failed declaration admission returns an error. These outcomes must remain distinct.

## Matrix and process bounds

- Families: common and independent branch-specific traversal.
- Choices: 0 and 3; common depth: 0 and 16; changed queries per prepared source: 1 and 4.
- Schedules: eligible; missing-permit on odd query indices; unknown depth on odd query indices. Cancellation uses index zero for eligible and index one otherwise.
- Two modes × two families × two choice counts × two depths × two reuse counts × three schedules = 96 cells.
- Two separate allocation processes per cell (192 total), shuffled with seed 7900. Every memory reading must replay exactly. Complete this gate before timing.
- Five ordinary-allocator timing processes per cell (480 total), shuffled with seed 7901. Total 672 processes. Timings are exploratory medians and ranges, not formal practical-win claims.
- Each process: 1 GiB address-space limit, 60 CPU seconds, 75 wall seconds; 500,000 public service calls per finite query. Cutoffs are failures requiring diagnosis, not losses or empty answers. Do not restart a confirmed live process.

## Lifecycle and correctness

Charge source cloning, declaration construction/checking, counting inference and prepared-rule creation together in preparation. Retain that prepared object across changed queries. Charge original input construction; submitted-query cloning, original admission, lowering and search setup; first complete answer or exhaustion; remaining complete delivery; search/error disposal; answer disposal; original input disposal. Then charge a one-service cancellation probe (or admission rejection) and all prepared/certificate/declaration disposal.

Preparation owns all retained certificate and declaration storage. Every query and cancellation must restore the preparation baseline. Final disposal must restore the initial live-allocation baseline; every adjacent measured phase must agree on live ownership. Allocation instrumentation reports requested heap bytes, not RSS.

Validate complete observations against independently constructed expectations and the independent scalar outside measured intervals. Unknown-depth declared submissions must return errors, and inferred submissions must preserve their actual suspended answers. Track rejection counts including cancellation explicitly. No error can be treated as exhaustion. For unknown schedules, report admission and execution outcomes separately; do not rank their whole-batch timing or allocation as equivalent work.

Engine/kernel counters are off in both builds. Allocation and timing use separate frozen binaries; timing uses the ordinary allocator. Record source/binary hashes, toolchain, shuffled orders, all raw phases and an independent audit. Source AST construction before preparation, process startup, native compilation, expected/scalar validation and preallocated bookkeeping are excluded. Execution and observation remain coupled in delivery. Do not claim complete architectural lifecycle superiority or sustained-lifetime coverage.

## Interpretation and next selection

For equivalent-work cells (eligible and missing-permit), report whole-lifecycle requested traffic and peak, phase attribution and exploratory time ranges. Do not invent weights or average across unlike regimes. Resolve consequential ownership failures or semantic differences before interpreting costs. A correct measured overhead is evidence about this checked interface, not proof against declarations in a compiler that exploits stronger properties.

After this bounded pilot, reconsider selective conditional discovery against further declaration work. The current declaration provides predictability and exclusion without removing the remaining executor. A proposed next language refinement must identify an actual responsibility it could remove and explain why it is more valuable than the ready discovery comparison. Broader S07 contracts, sustained lifetime and architecture closure remain required.
