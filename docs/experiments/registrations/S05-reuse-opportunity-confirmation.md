# Confirm reuse opportunity against source-derived contraction

The [sizing pilot](../results/S05-reuse-opportunity-sizing.md) finds a potential high-reuse crossover and verifies that existing source-derived contraction also operates. This prospective confirmation tests both mechanisms with complete query costs. It does not select a universal architecture or settle broader call reuse.

## Hypotheses and configurations

1. Compact keys can reduce total cost relative to equivalent owned keys without changing reused transitions.
2. High shared/renamed alternative counts can repay recognition; one alternative and distinct futures expose overhead.
3. Early resource consumption can make live-history projection useful where renaming alone retains distinct histories.
4. Source-derived contraction can change the apparent reuse advantage over ordinary specialization. Its eligibility and runtime machinery may also affect the control build and must be isolated.

Use the opportunity source's five families, depth 0/32, alternatives 1/16, payload depth 0/32 and resources absent/present: 80 configurations. Each has two changed queries, depth n/n+1, first forward then reversed input order. Width four remains sizing evidence; this confirmation retains both endpoint regimes. Two-query reuse is a bounded contract, not a sustained-lifetime claim.

Eleven build/mode controls: feature-off `direct`, `alpha`, `live`, `compact-alpha`, `compact-live`, `sealed`, `dependencies`, `templates`, `lowered`; feature-on `sealed`, `contracted`. Feature-on means `carrier-contraction`; otherwise source, compiler and build options are identical. Ordinary builds use release and no default features. Separate meter builds add `alloc-meter`. No diagnostic counters in either. Work diagnostics use a separate counter-enabled build and cannot supply timing evidence.

## Gates, freeze and resource bounds

Before timing, preserve the 240-configuration independent complete raw-answer gate, all 180 owned/compact work pairs and 60 contraction work pairs. Run ordinary and meter self-checks, verify all counter guards, and freeze source, registration, exact command matrix, compiler, binaries and analysis. This registration is written before confirmation; no confirmation matrix has run at registration time.

Run two meter repetitions for all 880 cells: 1,760 processes, requiring exact allocation replay and query/prepared requested-live restoration. Then run seven ordinary blocks, each shuffling the 880 cells using seed `7851 + block` for zero-based blocks: 6,160 processes. Pair contrasts within each block. Cancel each of the eleven controls in ordinary and meter builds at tick one on early-history/depth16/payload32/width16/two queries/resources/forward: 22 processes. Total 7,942 processes, excluding build/source tests, separate work diagnostics and meter self-checks.

Pin to the first available CPU, use 60-second wall and CPU bounds per process, a 1 GiB address-space limit and two-million-step source/scalar bounds. Preserve every failure and cutoff. A correctness or ownership failure stops dependent measurements for repair; a resource cutoff is an incomplete cost, not a loss. Freeze changes and explicitly re-register affected comparisons after a repair. No sample exclusions or selective reruns.

## Accounting and interpretation

Primary time sums preparation, both query setups, complete execution/observation, engine and answer disposal, and prepared disposal. Also report source/input-inclusive cost and every phase. First observation is reported alongside setup, including eager answer construction. Complete raw validation is outside measured intervals. Requested traffic and peak requested growth come only from meter runs; neither is RSS. Native compilation remains excluded and prevents complete architectural lifecycle superiority claims.

Nine prospective paired contrasts per configuration, numerator first: compact-alpha/alpha; compact-live/live; compact-live/direct; compact-live/feature-off-sealed; compact-live/feature-on-contracted; feature-on-contracted/feature-on-sealed; feature-on-sealed/feature-off-sealed; compact-live/templates; lowered/compact-live. Other measurements remain descriptive.

For each contrast compute seven log primary-time ratios and their geometric mean. Bootstrap the seven paired logs 10,000 times, seed `7852 + 9*configuration_index + contrast_index`, where configurations use the family/depth/width/payload/resource order stated above and contrasts use the listed order. Exponentiate sorted bootstrap means; positions 249 and 9749 define the interval. Classify a practical gain only when its upper bound is below 0.90, a loss only when its lower bound exceeds 1.10, otherwise unresolved. These are pointwise exploratory-family intervals, not simultaneous guarantees or a workload-weighted architecture ranking.

## Follow-through and breadth

A confirmed reuse gain requires checking the contraction and exact-source comparisons before proposing broader architectural benefit. A loss still leaves broader keys, call-level transport and other useful work unresolved. A feature-control shift requires attribution before treating cross-build contrasts as mechanism effects. Overlap is retained without claiming equivalence; further repetitions need a decision-value rationale. Allocation or timing anomalies are investigated when they could change the conclusion.

At the package boundary compare sustained S08 publication/lifetime with remaining call-level reuse and distinct integrated S02 execution. Record which decision each could change and implementation readiness before choosing more local work. This confirmation is selected because a now-verified, inexpensive stronger control could reverse the immediate crossover interpretation. It does not discharge any broader stage or complete T075 or the goal.
