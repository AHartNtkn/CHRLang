# Finite structural cost controls and ownership gate

Before comparing timings, validate complete query results and requested-heap ownership across five candidate paths. This gate measures requested allocations only; allocation-meter elapsed time supplies no speed evidence.

## Controls and source families

Compare unreduced lazy solving, structurally reduced lazy solving, fresh dynamic enumeration, enumeration reused across queries, and exact-family elimination. Fresh enumeration memoizes each grammar state while constructing a candidate language; it estimates cardinality after ignoring identical transitions to choose a small source/filter domain and deduplicates membership values. Reused enumeration retains each selected root's values across queries. Neither is the independent correctness oracle.

The exact-family control derives independent bit groups and permitted values directly for these registered sources. It checks requests against the prepared family's two query forms. It is a hand-derived control, not a general source compiler. All controls include grammar construction in preparation. The reduced solver additionally prepares its reduced membership representation and retains original source alternatives for counting.

Six fixed-length binary-list families: all values; equal bit positions; selective membership fixing all but the last bit; empty positive-width membership; redundant identical leaf alternatives; overlapping but structurally different filter transitions. Queries alternate their anchor position or accepted leaf value. Source derivations are unique, while filter proofs may repeat. At width zero, every family accepts the empty list: the empty-membership restriction has no bit on which to fail. An independent binary-assignment oracle checks complete terms against the mathematical family predicate.

## Exact gate matrix

Use widths 0, 4 and 6, all six families and all five modes. Each prepared owner handles four alternating queries. Cross immediate answer release versus retaining all answers until that query's end, and complete enumeration versus cancellation after its first answer. Empty-result cancellation runs exhaust normally. There are 360 cells, each run in two fresh allocation-meter processes: 720 processes total.

Each process first validates all four complete query results outside measured intervals, disposing those owners before measurement. Measured queries check their observed cardinality and multiplicity. Preallocate the result buffer and phase-record buffer outside the measured owner baseline to separate harness retention. Record preparation, query-request construction, setup, execution/observation (including immediate release in that consumer mode), query disposal, retained-answer release and prepared disposal. Require exact replay of phase allocation readings and full restoration to the owner baseline. Report requested traffic, live bytes and peaks, never RSS. A caller-owned retained buffer has fixed capacity before measurement; actual answer trees are measured.

Use counter-free release code with the shared requested-allocation meter. Run its self-check before the gate. Each process is limited to 60 seconds and 1 GiB address space. Each query has a 2,000,000-service-unit bound; bounded partial work is never counted as completed enumeration. Freeze source/binary/toolchain before running. Any error or replay mismatch stops comparisons and requires diagnosis.

## Interpretation

This gate establishes the applicability of the controls and who retains memory across changing queries and cancellation. It does not establish a timing crossover, sustained bounded-memory behavior, compilation-inclusive superiority or general consuming-source correspondence. Counter-free code and allocator validation are prerequisites to a separate prospectively registered ordinary-timing pilot. If the controls expose a consequential avoidable cost, investigate it before making an architectural cost claim.

## Prospective correction after the first ownership gate

The first 720-process gate completed with exact replays and disposal restoration. Its records and frozen source copies are preserved under `results/s06-finite-cost-ownership-initial/`. Inspection shows the hand-derived control retaining grammar data despite requiring only its checked schema, root identifiers and query policy after preparation.

Correct that owner to retain no grammar. Repeat the same 720-process matrix with a new binary freeze, all other code paths and endpoints unchanged. Expect its prepared live growth to fall and final disposal work to move into preparation. Require identical non-lowered allocation records and unchanged complete source answers; compare exact phase readings to attribute the correction. This is an allocation/ownership correction, not a timing experiment. Preserve the initial binary at its recorded path.

For the paired ownership check, compare requested traffic/calls directly and live/peak readings relative to each run's initial owner baseline. The frozen executable paths have different lengths and their argument strings live outside that baseline; this process-owned offset is not engine retention. Within each two-run block, require exact absolute readings as before.
