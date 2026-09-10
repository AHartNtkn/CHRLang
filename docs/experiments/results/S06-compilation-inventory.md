# Compilation already has direct evidence; the next missing comparison is learning cost

Independent native compilation has been implemented and measured in this repository. Recursive lowering and generated access already have useful, bounded results. Resume the qualified learning cost investigation while keeping broader compilation and resource derivations required.

## Existing evidence changes the selection

| Question | Evidence already available | What it establishes |
|---|---|---|
| Can generated user programs execute independently of the CHR engine? | [R05 recursive lifecycle](R05-recursive-lifecycle.md), standalone generated modules depending on syntax and the persistent equality kernel | Yes, for certified ground-controlled equation recursion. The generated program and prepared Direct executor consume the same authoritative source plan. |
| Does compilation repay its cost there? | 67 preparation/build stages, 324 sessions, 54 timing cells; two sources, depths 0/32/128 and 1/64/512 changing queries | Native has three separated session gains over Direct. Even the lowest seeded compilation observation exceeds the highest Direct session observation; compilation is not recovered within this measured range. |
| Can compilation repay its cost at greater reuse elsewhere? | [S01 native amortization](S01-native-amortization.md), generated versus prepared access | Yes: compilation-inclusive ratios are 0.897 for 16,384 chain queries and 0.872 for 8,192 payload queries. Both still lose at 1,024 queries. These are observed cases, not a universal crossover. |
| Can a stronger organization outweigh generation gains? | [S01 native cost pilot](S01-native-cost-pilot.md), dedicated subscription controls | Yes in the recorded subscription sources. A faster generated generic engine does not settle whether the generic machinery should exist. |
| Does the current HVM pilot answer those questions? | [S10 mixed-source pilot](S10-mixed-pilot.md) | It compares a particular runtime evaluating emitted programs. Its measured loss neither repeats nor overturns the independent compiler results. |

The source-plan distinction matters. Prepared Direct recursion already avoids CHR occurrence selection and history. Its generated form additionally replaces body-template interpretation with native statements and local registers, while retaining the same equality kernel and answer obligations. A comparison only against an ordinary CHR engine would conflate these two benefits.

## What was revalidated now

The R05 archived auditor again verifies all 324 sessions, all 54 complete primary cells, build conditions, raw response hashes and independently decoded outcomes, with no failed or missing stages. This is an archive audit; it does not assert that all historical dependencies or binaries equal the current worktree. The plan implementation, native emitter, session protocol, package emitter and auditor themselves match their archived hashes exactly. See [audit](s06-compilation-inventory/r05-audit.json) and [source comparison](s06-compilation-inventory/source-comparison.json).

Current metrics-off tests independently compile and execute seven generated modules, checking 182 outcomes, and three executable session organizations, checking 42 responses. Both tests pass. These are current semantic checks, not new timings. They exercise changed queries, alias/failure cases and admission boundaries. [Test log](s06-compilation-inventory/current-tests.log).

The S01 analysis was rerun against its archived process records, including complete-query validation and repeated allocation agreement. It reproduces all five reuse contrasts and checks 696,320 recorded query outcomes. Its scope remains the archived sources/builds; this is not a current-speed assertion. [Audit log](s06-compilation-inventory/s01-audit.log).

Reproduce the R05 inventory with `python research/chr-compiled/experiments/compilation_inventory.py`. Current tests use `cargo test --offline -p chr-compiled --no-default-features --test recursive_native_artifact --test recursive_session_artifact -- --nocapture`. The S01 descriptive reanalysis uses `python research/chr-compiled/experiments/artifact_amortization_analysis.py`.

## What remains genuinely unanswered

Broader source lowering still needs direct trials: contextual effects, consuming-resource derivations, richer recursion, partially known control and mixed compiled/interpreted work. The two-rule recursive certificate requires a finite ground control spine and one call; those restrictions are consequential language boundaries. Generated access retains more general runtime machinery and cannot stand in for eliminating it.

Compilation policy remains unresolved across source changes, code growth, artifact caching and invalidation, broader lifetimes and complete architectures. Existing observed recovery forbids treating compilation as categorically uneconomic. Existing short-use losses and stronger dedicated controls forbid selecting it solely from reduced dispatch. Further trials must name the additional mechanism or regime they could resolve and include the strongest applicable control.

The mixed HVM path's compilation costs remain outside its pilot. The broader repository's compilation evidence is available and must participate in architecture selection. These scopes are now explicit in the decision map.

## Selection at this entry boundary

Resume T073's compatible-query learning cost investigation. Its semantic, ownership and counter-free measurement gates are already qualified; the missing substantive common-prefix source can be built from the existing independent depth witness. The next package extends that witness into complete lifecycle measurements, including a reusable caller, changed queries, recognition, retention and disposal. Register the exact cost matrix only after complete-answer and measurement qualification.

The strongest distinct alternative is direct consuming-resource derivation. It could eliminate machinery retained by both learning and existing compilers, but still needs a concrete admitted source contract and an independent complete-answer implementation. That has broader potential but greater immediate implementation cost. Select the bounded learning package first because it can now answer whether a demonstrated work saving actually pays; review direct resource derivations at its first substantive source/accounting gate or any obstruction, before expanding the timing campaign.

Broader native compilation stays required under investigation 2, with review at the same boundary. Do not reconstruct the existing recursive compiler entry or repeat its matrix on the premise that compilation has no direct measurements. A new compiler package must identify a responsibility, language boundary or reuse regime not already answered by R05/S01. None of these bounded results completes the architecture goal.
