# S02 ordinary relational lifecycle pilot

## Decision and hypotheses

Determine whether direct constructor/source joins justify further relational representation work relative to integrated recursive matching and dedicated equality execution. H1: selective nested constructor access can offset relational maintenance. H2: broad successful updates and low-yield equality repair expose costs that selective access alone hides. H3: ordinary flat matching reveals fixed organizational overhead. Contrary results retain their scope; no architecture is selected from these sources.

## Frozen comparison

Four configurations: relational source executor; existing integrated engine; generic compiled executor with Global policy and Indexed access; the same with Scan access. All counters disabled. Generated rule code is absent from these dedicated controls: this pilot cannot establish superiority over generated execution. The existing independent source gate and mathematical answers admit every cell before measurement.

Five families: flat `open(K),ticket(K)` consumption; immediate selective `open(f(a,K)),ticket(K)`; immediate dense constructor matches; delayed selective structure; delayed dense structure. For selective sources only key0 carries tag a and others tag b. Delayed sources initially post open(Xi) plus bind(Xi,f(tag,Ki)); bind rules establish structure. Dense sources make all tags a. Every successful pair produces done(K); unsuccessful open and ticket occurrences remain. Keys are distinct ground constants. No choices or failed complete queries occur.

Each process prepares one fixed ruleset and runs 16 queries: sizes [4,16,64,16] repeated four times, over reused preparation. Four warmup queries (one size cycle) precede measured preparation. Input syntax and expected answers are constructed outside measurement; query cloning is outside setup. Prepared rules construction includes its own required rules ownership. Validate complete answers outside measured intervals against independently constructed mathematical outputs, also check the independent scalar oracle in a semantic-only gate.

Primary phases: preparation, query setup, execution plus observation, engine disposal, answer disposal, prepared disposal. The relational API releases completed state while producing its answer; that release is included in execution plus observation. A separate first-answer latency overlaps that interval and is never summed into lifecycle time. Native compilation, executable startup, fixture construction and oracle checks are excluded; do not claim complete cold architectural superiority. Requested allocation diagnostics use a separate allocator-meter build and are not RSS.

## Bounds and analysis fixed before comparisons

Twenty cells, five timing repetitions and two allocation repetitions per cell: 140 isolated processes. Randomized cell order within each repetition, seed 20260908. CPU affinity uses the lowest permitted CPU consistently; record hardware, toolchain and binary/source hashes before runs. Primary release build uses ordinary allocator, no engine/kernel counters. Allocation builds must replay exact non-time diagnostics across two repetitions.

Each process is bounded at 60 seconds, 1 GiB address space and 2,000,000 service advances per query. A cutoff or wrong answer is not a completed cost. Investigate it before using the comparison. The sizing/semantic gate may run before freeze but produces no comparative timing receipts. No confirmatory tuning on this pilot.

Report per-cell total lifecycle and component medians, paired within-repetition ratios, ranges and per-size query costs. A 20% paired median difference with all five ratios on the same side of one is a practically consequential pilot signal; other outcomes are unresolved at this precision. This threshold guides investigation, not universal selection. Inspect a consequential unfavorable result for repairable selection, enumeration or incidence costs before architectural rejection. A feasible repair needs separate prospective paired registration. Reassess S01 maintained joins and S06 direct solving after the pilot, rather than automatically continuing local tuning.
