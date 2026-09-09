# T070 native, prepared and retained lifecycle cost pilot

This registration precedes comparative runs. The decision is whether source-native
execution earns its generation, compilation and ownership costs against prepared
execution and applicable dedicated/retained controls. ThinLTO may improve runtime
but increase compilation; source selection and query reuse may reverse the choice.
No workload weights, universal winner or automatic routing policy are assumed.

## Fixed sources and controls

Use the artifact runner's chain, private payload, dispatch64 and subscription source.
Add existing low-stable, low-reopen and low-churn queries through their unchanged
source builder, with 64 requests per query. These three share the subscription
ruleset and native artifact; they require no additional ruleset compilation. Their
size stays fixed while each query changes its seed marker. Other families alternate
size and input order as in the preceding gates. Independent scalar execution checks
all complete answers outside measured runtime intervals.

Exact cells (family, size, query count): chain32 at 1/16; payload64 at 1/16/256;
dispatch64 at positions0/32 and 1/16 queries; subscription16 at 1/16; each of the
three low-yield profiles at size8 and 1/8 queries. Dispatch position is not store
size. Modes: generic, planned, specialized, native, native-generic-repair,
native-planned, native-specialized. Subscription and low-yield profiles additionally
run retained-indexed, retained-eager and retained-subscribed. This is 143 cells per
compiler configuration, 286 primary timing cells.

Use ordinary optimized linking and ThinLTO with a fixed bitcode-bearing runtime,
no default features and the experiment feature. Build a separate alloc-meter
runtime for diagnostics. Primary timing always has ordinary allocation and no
engine/kernel/observer counters. The reference implementation remains independent.

## Preparation, repetitions and bounds

Compile generic, chain, payload, subscription and dispatch64 artifacts under both
compiler configurations. Measure emission (source generation, write and buffer
release), compilation wall/child CPU, and artifact size. Use five fresh output
paths per artifact/configuration; no shared-runtime rebuild is charged per ruleset.
All use edition2024, opt-level3 and one codegen unit. The first ordinary runtime
build and diagnostic runtime build are preparation receipts, not repeated cold
bootstrap estimates. Source and binary hashes and rustc version freeze the inputs.

Before timings, check all three low-yield profiles in ten modes and both compiler
configurations with two changed queries. Any source disagreement blocks affected
comparisons and must be repaired before confirmation.

Runtime runs: one warmup block, then seven measured blocks, each containing every
primary timing cell. Shuffle within each block with seed7100+block. Pin runtime
children to the smallest CPU in the current allowed affinity set and record it;
compiler processes retain the host's available affinity. Do not run other builds
concurrently with primary timings. Record process wall and child CPU separately.
This is 2,288 timing processes including warmups. Diagnostics run each of the143
cells twice under ordinary linking, requiring exact phase allocation replay and
query/prepared live-byte restoration. Metered timings are never ranked.

Each compile/build/run has a60-second timeout. Compilation has a4GiB address-space
ceiling, runtime1GiB; completed engine and oracle queries have a2million-step bound.
A failure/cutoff is an unfinished cell, never a completed cost. Preserve its receipt
and investigate before making the affected claim. No post-result cell substitution.

## Endpoints and interpretation

Primary runtime endpoint: source construction + preparation + completed query
setup/execution/full observation/engine and answer disposal + prepared disposal.
Report each phase, requested traffic and absolute requested-live peaks separately.
Inputs, independent oracle, validation, stdout and process startup/exit are excluded
from this sum; process wall/CPU include harness costs and are separate endpoints.
No claim that a phase sum equals process wall or complete operating-system cost.
Artifact filesystem disposal is timed after users exit, with the prior gate's
syscall-only interpretation. No durable-storage synchronization is assumed.

For each registered paired contrast, compute the seven within-block runtime ratios,
report their range and median. A practical gain requires median<=0.90 and every
ratio<1; a loss requires median>=1.10 and every ratio>1. Otherwise report unresolved
under this pilot's criterion. These are descriptive repeated-run rules, not
confidence intervals or a population-wide statistical guarantee.

Registered contrasts: planned/generic; native/generic; native/planned;
native/native-generic-repair; native/native-planned; specialized/planned;
native-specialized/native; each retained mode/planned and each retained mode/native;
eager/indexed and subscribed/indexed among retained controls; ThinLTO/off for each
mode. Favorable/adverse source placements must remain visible, without averaging
across profiles. Paired allocations are diagnostic exact counts, not timing proxies.

Compilation sensitivity: report all five observations, use generation + writing +
buffer disposal + artifact compilation as one-time native cost. Generic compilation
is reusable across rulesets and must not be charged to every generic query/ruleset.
For measured query counts, report compilation-inclusive costs with that convention.
Any crossover beyond observed reuse is explicitly a model based on measured steady
query costs, never an observed result. No extrapolation establishes indefinite
cache/lifetime behavior. If plausible uncertainty could change the decision,
investigate it instead of declaring a winner from a component result.

## Follow-through

A runtime gain must survive preparation/compilation and stronger controls. A loss
with consequential avoidable preparation, code or ownership cost requires bounded
attribution. Overlap requires further precision only if it can alter the decision.
After this first complete contrast, compare the value of that follow-up with direct
pull-tabbing/derivation reuse. Broad access policy tuning does not automatically
remain first. The goal and broader S01 obligations remain open.
