# Scheduled templates do not repay their costs against the simpler controls

**Scheduled templates qualify as faster than zero-follow templates in 69 of 96 cases, but in none against dependency execution, Direct, sealed compiled execution or inferred carriers.** They qualify as slower than Direct in 91 cases and carriers in 90. Keep the scheduling mechanism as correctness evidence; investigate coarser recognition next rather than continue tuning this representation.

## Validated lifecycle comparison

The [registration](../registrations/S06-scheduled-timing.md) precedes all comparative runs. All 1,152 metered entries match the qualified ownership campaign before warmup or timing. The ordinary campaign completes 11,520 measured processes, 57,600 sessions and 6,336 registered contrasts, plus 1,152 excluded warmup processes. Complete scalar answers pass outside measured intervals.

Each process averages five fresh sessions. Each session prepares rules, runs two changing-depth queries, disposes producers, retains answers through prepared-rule disposal, and disposes consumer ownership. Half the source cases cancel the first query after one source tick. Primary builds use the ordinary allocator with all engine, kernel, work, validity and allocation counters disabled. Query-time derivation is charged to execution. Source construction, validation and process startup are outside the endpoint; compilation is not isolated.

A qualified gain requires a median paired ratio at most 0.90 and all ten blocks below one; a qualified loss requires median at least 1.10 and all blocks above one. Other comparisons remain unresolved. No sample was excluded and no source frequencies were assumed.

## Results

Candidate is scheduled templates. Ratios are candidate/reference total session time.

| Reference | Qualified faster | Qualified slower | Unresolved | Median ratio range |
|---|---:|---:|---:|---:|
| Zero-follow templates, same build | 69 | 0 | 27 | 0.415–0.915 |
| Dependency execution, same build | 0 | 63 | 33 | 1.059–2.723 |
| Ordinary contracted templates | 0 | 89 | 7 | 1.557–5.940 |
| Direct | 0 | 91 | 5 | 1.648–8.578 |
| Sealed compiled | 0 | 85 | 11 | 1.368–4.694 |
| Inferred carriers | 0 | 90 | 6 | 1.465–5.818 |

Scheduled-template gains over zero-follow occur in 30 of 32 deterministic-chain cases, 21 of 32 repeated-choice cases and 18 of 32 distinct-choice cases. The benefit is therefore real within that comparison, but does not supply a competitive candidate against the stronger controls in this campaign. The ownership result also remains adverse against those controls in every case.

Ordinary contracted templates are a qualified cost control on these sources, not a scheduling-preserving substitute for arbitrary resource programs. The scarce-token counterexample remains relevant to language-design choices.

## Consequential uncertainty was retained

All five unresolved Direct comparisons were inspected. For the failed pure depth-32 chain with forward insertion and no cancellation, the registered median scheduled/Direct ratio is 3.205, but block six reverses it to 0.371. Its five Direct sessions take approximately 244, 3,220, 198, 185 and 198 microseconds; scheduled sessions take 362, 279, 321, 266 and 272. That block remains in the calculation. No machine-level cause is inferred and no alternative endpoint is substituted.

The [diagnosis](s06-scheduled-timing/diagnosis.json) records all five contrary blocks. It also asks whether disposal alone could reverse the comparison: set every scheduled disposal phase to zero while leaving Direct unchanged. All 96 resulting median ratios remain above one, ranging from 1.402 to 8.119. This analytical bound rules out a disposal-only remedy for these medians; it does not bound savings from changing execution or representation.

## Decision

The [portfolio review](S06-scheduled-timing-review.md) selects T075: qualify a coarser recognition mechanism while preserving source service, fresh-result identity and invalidating near misses. T073 remains unfinished for broader source inference, recursive/contextual eligibility, compiler costs and alternative representations. Its scheduled-body mechanism preserves tested service points and reduces runtime matching, but that saving does not justify the measured total cost here.

Reproduce the audit with `python3 research/chr-reuse/experiments/scheduled_timing.py --audit` and the post-campaign diagnosis with `python3 research/chr-reuse/experiments/scheduled_timing_diagnosis.py`. [All contrasts](s06-scheduled-timing/analysis.json), raw sessions, frozen sources and binary hashes are retained. The research goal remains active.
