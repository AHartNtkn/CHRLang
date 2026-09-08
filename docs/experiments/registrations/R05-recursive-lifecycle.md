# Recursive native compilation and session lifecycle pilot

T046 asks whether generated body operations earn their added preparation relative to the checked direct equation loop and corrected Specialized. The source classes already have independent correspondence and common session gates. This comparison has greater immediate value than further executor tuning because it can reject or support native generation at a measured reuse boundary. R04 compilation remains the strongest alternative if this bounded artifact path cannot produce credible lifecycle evidence.

## Hypotheses and interpretations

- Native body statements may reduce repeated template/scope work enough to improve complete sessions, particularly at deeper controls and higher reuse. If end-to-end ranges overlap, that benefit is unresolved regardless of instruction counts.
- Compilation may dominate the measured session savings. Charge per-source native preparation separately from shared installation; do not make ordinary controls recompile for each family. Failure to recover preparation within measured reuse is a bounded negative result, not a claim about all future reuse.
- Term equality, output construction, transport or lifecycle overhead may dominate after direct lowering. The fresh-variable family and zero-depth controls expose those alternatives. Native speedup on one family does not imply general language superiority.

No workload weights or aggregate winner will be constructed. Interpret each family/depth/reuse cell separately. Five primary session ranges must be complete; overlapping observed ranges are unresolved. Three compiler repetitions give a median and observed range, not a distributional significance claim. Do not extrapolate a break-even reuse count beyond supported measurements without explicitly labeling assumptions and uncertainty.

## Sources, backends and requests

Use the authoritative `recursive_session_fixture::rules(kind)` definitions for `add` and `fresh`. Addition constructs a unary successor result over its payload. Fresh recursion accumulates pairs containing a distinct fresh existential per level. Both remain sealed certified two-rule sources; unknown payloads/results are admitted.

Backends: generated native; reusable checked direct-equation preparation; current Global/Indexed inferred Specialized with 20,000,000 ticks per request and explicit error at cutoff. Direct and Specialized executable source is identical across family emissions and accepts the family at startup. Each installed control binary serves both families, with source construction/preparation charged inside each session. Native is a separately generated artifact for each source family.

Depths: **0, 32, 128**. Requests per session: **1, 64, 512**. At request index i, alternate depth d and max(d−1,0), and cycle these payload/result pairs: unknown V10 / unknown V11; atom a / V11; V10 / V10; atom a / atom clash. Select outputs x=V10, y=V11 and unused=V99. The first request is a successful open result. This covers changing queries, aliases, finite failure and output-only unknowns without assuming a target application distribution.

Two families × three depths × three reuse levels × three backends = **54 cells**. Use one warmup and five primary sessions per cell, with each repetition block shuffled using seed **46046**: **324 planned sessions and 62,316 full responses**. Preparation is reused only within the stated session. Warmups are recorded and validated but excluded from primary summaries.

## Build and source preparation conditions

Build the counter-free generator once as shared infrastructure in a fresh target directory, recording its full build wall/RSS/command. Run three fresh emissions per family, retaining process wall and fixture construction, certification, generation, package-writing/disposal and total internal timings. Package writing emits all control/prerequisite wrappers as well as the native artifact; report that scope instead of calling it only native-code writing.

For native, measure three repetitions in each of two conditions per family: **12 artifact builds**. Clean condition uses a new empty target directory. Prerequisite-seeded condition uses another new target, first builds the matching empty-main dependency package, then builds the source-specific artifact. Record prerequisite builds separately. It must contain no previously compiled source-specific artifact. Do not infer either condition by subtraction from another run.

For installed Direct and Specialized, perform three clean installation builds each: **six control builds**. Their binaries are reusable across the two families; do not charge these six installation measurements as compulsory per-source compilation. Use repetition-zero clean native artifacts and repetition-zero installed controls for sessions. Build order is deterministic and seeded; preserve it in the manifest before compiler execution.

All artifact builds use the installed Rust toolchain, offline locked Cargo, opt-level 3, codegen-units 1 and incremental=false. Resolve each emitted manifest's lockfile offline before its build and record lock-resolution wall separately. Freeze source, generator, harness, registration, manifests, lockfiles and binaries with hashes. Verify relevant source hashes after the run. A seeded build may reuse dependency artifacts only; filesystem caches are not flushed or claimed cold. “Clean” describes Cargo target artifacts, not machine caches.

## Measurement and observation boundary

Use the same length-framed session protocol and `/usr/bin/time` wrapper for every backend. Prepare request bytes outside timing. Start parent wall timing before launching the wrapped process, pipeline the preencoded changed queries through that process, retain every complete raw response, close input and wait for exit. Stop timing before decoding, mathematical validation or writing result receipts. First-response latency ends when the parent receives the complete first frame and includes startup, wrapper and transport. This is a pipelined throughput session, not a per-request round-trip latency experiment; no subsequent round-trip latency is inferred.

Complete session wall includes control preparation, decoding, backend query setup/execution/export/kernel disposal, answer encoding, response buffering, child output/protocol disposal, shutdown and wrapper exit. Parent raw-response buffers remain available for validation; their later disposal is outside the timed session interval. The current backend execute API bundles internal phases; do not label that interval isolated execution or observation. Backend `preparation_ns` is a separate explanatory timer. Native preparation is represented by the source/artifact stages rather than invented runtime source preparation.

Peak RSS from `/usr/bin/time` is reported in KiB for its measured command (and maximum child usage for compiler commands); it is not requested allocation, a sum across concurrent processes, or parent-buffer memory. Parent response buffering is identical across backends and part of parent wall, but outside child RSS. Requested-heap attribution is not part of this first compilation pilot. No full process/native lifecycle superiority claim may ignore shared prerequisites or these boundaries.

Validate complete responses after process termination. An independent mathematical finite-tree unifier constructs s^n(payload) for addition or the fresh pair chain with distinct existential variables, unifies with the requested result, and projects all outputs jointly. Compare raw success/failure and exact alpha-equivalent observations, including output-only variables and empty residuals. Retain compressed raw response frames and request recipe so audit can replay every comparison. A label, count or output hash alone is insufficient semantic validation.

## Resource and failure policy

Pin measured commands to available CPU 0 and record affinity/hardware/toolchain. Session wall/CPU bounds are 30 seconds and address-space bound 1 GiB. Compiler/build wall/CPU bounds are 120 seconds and address-space bound 3 GiB. Generation/lock resolution also has explicit bounded subprocess execution. Whole pilot cap: 20 minutes from entry. Transport limits do not replace these execution limits.

On compiler failure, transport error, timeout, resource limit, source mutation or observation mismatch, preserve the outcome and stop further comparative attempts. Retain the prewritten manifest and mark remaining work unattempted; never silently retry or summarize incomplete five-repetition cells as complete. Root and independent review must audit the runner before comparative execution. No comparative run precedes this registration and source freeze.
