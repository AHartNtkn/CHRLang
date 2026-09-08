# R01 native cost pilot

Run a bounded exploratory comparison of generated execution, activation and argument indexing. The result will select the next architectural investigation, not a production winner. The runner and build freeze must exist before `pilot.py` starts.

## Competing hypotheses and next decisions

- Generated selectors/bodies reduce generic template traversal at matched policy/access. If indexed request cost still improves consistently, keep generation in subsequent controls; if only execution improves, examine setup/observation/disposal dilution. Native compilation is not included, so neither outcome settles cold compilation break-even.
- Active occurrence scheduling avoids global rediscovery. If indexing largely explains the benefit, prioritize access and representation over more activation tuning. Global and active policies may make different legal application orders; independent complete answers govern this comparison.
- Indexing reduces selective flat-chain partner work, but equal-key buckets and low-yield binding repair may make maintenance uneconomical. A contrary regime is evidence for conditional access planning, not a reason to exclude that workload.
- Preparation reuse and actual reclamation may alter execution-only conclusions. Charge each query's setup, first export and disposal, and release preparation after all four queries. Any retained query allocations require investigation before memory interpretation.

R02 consuming class/constructor integration is the strongest ready alternative. It could alter representation and equality boundaries but requires a new semantic implementation. This already implemented R01 control can now distinguish major costs with bounded runs, and can change which dedicated control or representation uncertainty matters for R02. This justifies the pilot first, without requiring R02 to inherit its interfaces or await an R01 win. R04 direct finite solving remains eligible under its independent entry.

## Exact registry

Full product: five families `build`, `chain`, `delayed`, `collision`, `repair`; base sizes 8 and 64; `generic|generated`; `global|active`; `scan|indexed`. There are 80 cells. Every process prepares once and starts four queries with sizes n, n+1, n, n+1. Source programs are respectively fixture IDs 0, 1, 2, 13, 11.

Build constructs one recursive list. Chain uses fixed-width atomic keys with reach propagation and consuming jobs. Delayed also grounds edge keys through source bind rules. Collision has n equal-key q occurrences and a late extra occurrence, retaining multiplicity and propagation history. Repair grounds n nested f(X) keys to f(b), never satisfying p(f(a)); bind applications are real work although match yield is zero. Expected full joint outputs and residual multisets come from fixture formulas, checked outside timing through the independent observer. Existing semantic tests also check legal commits and terminal enabledness independently of the reference interpreter.

Five isolated primary timing processes, two counted processes and one allocation process per cell: 640 total. All jobs are shuffled together with Python `random.Random(20260908)`, preserving the order in `order.json`. There is no discarded warm-up; first query and subsequent queries are reported separately. The primary unit for distributions is the process, not four independent replicates. No adaptive expansion is authorized by this registration; any follow-up needs a new prospective registry.

Release Cargo builds, installed rustc, repository profile defaults; no custom RUSTFLAGS. Package-isolated target directories prevent feature unification with other consumers:

```
cargo build --release -p chr-compiled --no-default-features --features experiment --target-dir /tmp/chr-r01-time
cargo build --release -p chr-compiled --features experiment --target-dir /tmp/chr-r01-work
cargo build --release -p chr-compiled --no-default-features --features alloc-meter --target-dir /tmp/chr-r01-memory
```

Copy each `release/chr-compiled-cost` to `/tmp/chr-r01-bins/{time,work,memory}`. Resolved features, binary/source/generated hashes and environment are recorded in `results/R01-native-pilot/`. Primary build has both counters off and ordinary System allocator; work has both counters on and ordinary allocator; memory has both counters off and allocation meter. No metered timing is primary evidence. A mode/build disagreement is an error. Both debug and release allocation self-checks must pass; a `clock-check` measures 10,000 empty Instant intervals without any fixed subtraction. Timed source execution is bounded to 1,000,000 engine steps per query.

Run `python3 research/chr-compiled/experiments/pilot.py`. Each child is pinned to the minimum allowed CPU (recorded before runs), with 2 GiB address-space limit, 20 CPU-second limit, 30-second wall timeout, and no core dump. Whole batch is bounded to 15 minutes. Child execution is serial. Preserve every error, timeout and cutoff; a cutoff is not a completed-answer time or source failure. Stop on feature/answer/retention validation errors. Save raw stdout/stderr for unsuccessful processes. Do not run builds or other experiments concurrently with measurements.

## Accounting and interpretation

Schema 2 reports preparation; query setup, execution, first observation, engine disposal and answer disposal; preparation disposal; and harness fixture/validation time. `request_ns` spans setup through engine disposal, including timing probes and (only in work runs) counter serialization. Query lifecycle cost is request plus answer disposal. Four-query lifecycle is preparation plus all four query lifecycles plus preparation disposal. These start at owned ASTs, exclude fixture/oracle work and native compilation, and are not total architectural cold cost. `harness_ns` reports fixture/oracle work separately; process wall time additionally includes launch, JSON serialization and harness bookkeeping.

Preparation includes generated-source identity validation for the generated path. It is a necessary operation in this measured API and must be visible in the result. Bundled build compilation is not credibly isolated per user ruleset. Generated code is shared process code, not allocated query heap.

Report per-cell medians and ranges; compare execution and complete measured lifecycle at matched policy/access. Flag differences below 10% or overlapping five-process ranges as unresolved by timing alone; these are exploratory screens, not statistical significance claims. Work-count reductions explain mechanisms without combining unlike counts. Require phases above 100 times the median empty clock interval for phase-level direction claims; do not subtract overhead. Retain small phases as accounting, with their uncertainty.

Allocation readings are requested heap bytes, not RSS. Phase absolute peaks include the baseline (source ASTs, prepared state, preallocated report storage); phase increases are relative to their recorded starts. Query AST storage is consumed during setup. Exported answers remain alive through engine disposal. After each answer's disposal, equal live bytes across all four queries are a direct retention check. Prepared disposal must release its owned heap. No architecture-wide memory conclusion follows from these bounded single-thread lifetimes.

No workload weights, cross-family winner, language restriction, parallel scaling, OR/search storage or compiler amortization conclusion is supported. Additional sizes, held-out cases, rule-count variation, structured-key scaling and further R02/R04 work remain decisions to select from findings.
