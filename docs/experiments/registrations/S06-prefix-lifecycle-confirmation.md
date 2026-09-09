# Confirm complete prefix-transformation costs

Sizing completed all 192 ordinary, 64 allocation and eight cancellation/reuse processes without cutoff or answer failure. Use its unchanged frozen binaries and sources for confirmation; this measures the current compiler API, including per-query lowering/target preparation and entry overhead.

Retain all 192 configurations: four original/sealed/lowered/lowered-sealed modes, chain lengths1/8/32, one/four calls, deterministic/choice terminals, consumption absent/present and one/eight queries. Run one warmup plus seven measured blocks, each independently shuffled by seed7302+block. Run all configurations twice with the allocation binary (384 processes). Sizing's cancellation and meter self-check remain valid for the unchanged binaries.

Primary endpoint and all phases, correctness gates, source/binary freeze checks and first-allowed-CPU/60-second/1-GiB/two-million-tick bounds remain those of sizing. Compilation here means the charged source transformation and emitted CHR preparation; the harness Rust build is not isolated native program compilation. Require exact repeated allocation readings and baseline restoration. Preserve independent scalar validation outside timings.

For each source configuration report same-block ratios for sealed/original, lowered/original, lowered/sealed and lowered-sealed/sealed. Seven pairs with median <=0.90 and all <1 establish a practical gain; median >=1.10 and all >1 establish a practical loss; otherwise unresolved. These descriptive criteria are not confidence intervals and configurations have no workload weights.

Distinguish reduced execution from total benefit; no execution-only superiority claim may omit transformation or preparation. Report cold/single-call losses alongside repeated/long-chain gains. Diagnose consequential failures or implementation costs before extending conclusions to broader lowering. After confirmation, compare the next investigation with the strongest ready distinct alternative; finite-prefix results do not resolve effectful recursion, source-query specialization or the overall goal.
