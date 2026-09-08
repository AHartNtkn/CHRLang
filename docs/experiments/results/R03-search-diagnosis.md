# R03 source-search scheduling diagnosis

Delayed failure service explains the large branch-work difference in the R04 cold chain and contradiction cases. This is a scheduling effect amplified by copying and FIFO frontier retention. It is not an intrinsic indexing penalty or evidence that conditional sharing is necessary to make these cases economical.

The [registration](../registrations/R03-search-diagnosis.md) specified 72 cells and exact repetition. All 144 processes completed within bounds, with identical diagnostic output on each repeat. Every full raw answer set and residual multiset passed an independent Cartesian check; branch conservation (`splits + 1 = failures + answers`) passed. [Raw results](R03-search-diagnosis/raw.jsonl), [source/binary hashes](R03-search-diagnosis/freeze.json), [frozen sources](R03-search-diagnosis/source.tar.gz) and [readable results](R03-search-diagnosis/run.log) preserve the evidence. These are counted diagnostics, not timing runs.

## Diagnostic results

Generated/global/scan and generated/active/scan shown below. Indexed and generic variants have identical listed counts in every corresponding cell. Peak is the number of live frontier branches, not bytes. Failure-first is the cumulative number of splits when the first failed branch is delivered.

| Family | n | Policy | Splits | Failed | Answers | Peak frontier | Failure-first |
|---|---:|---|---:|---:|---:|---:|---:|
| chain | 4 | global | 20 | 18 | 3 | 9 | Some(8) |
| chain | 4 | active | 80 | 78 | 3 | 81 | Some(80) |
| chain | 6 | global | 32 | 30 | 3 | 9 | Some(8) |
| chain | 6 | active | 728 | 726 | 3 | 720 | Some(584) |
| chain | 8 | global | 44 | 42 | 3 | 9 | Some(8) |
| chain | 8 | active | 6560 | 6558 | 3 | 6360 | Some(3616) |
| contradiction | 4 | global | 32 | 33 | 0 | 18 | Some(8) |
| contradiction | 4 | active | 80 | 81 | 0 | 80 | Some(72) |
| contradiction | 6 | global | 32 | 33 | 0 | 18 | Some(8) |
| contradiction | 6 | active | 728 | 729 | 0 | 696 | Some(424) |
| contradiction | 8 | global | 32 | 33 | 0 | 18 | Some(8) |
| contradiction | 8 | active | 6560 | 6561 | 0 | 6200 | Some(2600) |
| weak | 4 | global | 80 | 0 | 81 | 81 | None |
| weak | 4 | active | 80 | 0 | 81 | 64 | None |
| weak | 6 | global | 728 | 0 | 729 | 716 | None |
| weak | 6 | active | 728 | 0 | 729 | 512 | None |
| weak | 8 | global | 6560 | 0 | 6561 | 6304 | None |
| weak | 8 | active | 6560 | 0 | 6561 | 4160 | None |

## Causal source explanation

Cold chain n=8 initially queues 42 forbidden occurrences and then eight choosers. Forbidden guards are not enabled while variables are unknown. A chooser binding refreshes affected dependencies, but those activations enter behind the remaining choosers; coalescing does not promote them. All eight source choices can therefore occur before rejection, yielding 6,561 assignment leaves: three successful and 6,558 failed. The observed 6,560 splits agree exactly with that explanation. Global selection restarts given/forbid/choose rule order after an application and checks newly ground adjacent pairs before choosing farther; it produces only 44 splits and 42 failed leaves.

The active/global peak-frontier difference is 6,360 versus 9 on chain n=8. Contradiction n=8 likewise has 6,560 versus 32 splits and peaks of 6,200 versus 18. Each split clones complete source state, including occurrences, queues, dependencies and indexes. These counts establish excess branch creation and simultaneous retained states; the prior R04 RSS measures the physical consequence for its registered paths. No byte size is inferred from these logical counts.

Weak n=8 requires 6,561 answers and 6,560 splits under both policies. It provides the opposing control: failure service cannot prune unconstrained answers. The distinct frontier widths there concern service ordering rather than avoidable answers. Scanned/indexed access has no candidate-work distinction on these single-head cases; generated/generic dispatch also preserves the diagnosed branching.

The same compiled source gates continue to protect branch-local failure, occurrence effects, raw multiplicity and finite sibling progress. None of this authorizes a universal failure-first schedule for arbitrary nonconfluent CHR. The finite fragment's schedule-independent complete answer relation is what permits the present comparison.

## Observation uncertainty

All chain n=8 completions contain eight outputs and 42 residual forbidden occurrences. Both policies deliver three observations; maximum terminal arena size is three term nodes. Source inspection confirms export traverses output/store terms and bindings, without traversing frontier, indexes or activation queues. These facts rule out larger logical outputs as an explanation for the large active observation interval in R04. They do not establish an allocator/cache explanation. Physical allocator history and finer export profiling remain plausible follow-ups, with no claim of causal attribution here. Complete lifecycle totals already include the interval, so its phase attribution does not invalidate the measured whole-path comparison.

## Architectural disposition

The global source path already supplies a competent early-rejection control; another active-queue tuning exercise has lower decision value than testing a distinct organization. R03 comparisons must use that control on these regimes. The current evidence does not justify replacing copied search solely because the active policy creates a large frontier: scheduling removes most of that frontier on constrained cases. On weak cases, output cardinality remains unavoidable under full enumeration.

Select T034 to specify and independently check a coherent direct conditional activation/resource/publication protocol, including consumption, propagation identity, correlated births and off-output failure. Its discriminating opportunity is avoiding repeated ordinary source work across choices while charging compatibility and complete observation. Cheap/immediate discrimination and global early rejection are necessary opposing controls. Begin with the semantic and responsibility boundary before implementation or prospective cost registration; no priority transfers to an unfinished prototype. This is more valuable now than further finite-solver repetitions or active-queue tuning because it tests a surviving architectural organization not represented by R01/R02/R04 complete paths. Compilation costs, static eligibility, lifetime and parallel ownership remain unresolved cross-cutting questions.

## Validation and reproducibility

Compiled all-target tests and Clippy pass; formatting passes. The diagnostic checks source outputs and full residuals independently on every run, preserves exact repeats and sums owned monotone segments once. A lint annotation was moved to its function after execution; the archived measured source and hashes preserve the actual input, and executable behavior is unchanged. The reference interpreter is independent. No comparative timing or allocator ranking is inferred from this diagnostic.
