# Independent artifact and lifecycle-accounting gate

This prospective gate checks the T070 compilation boundary and runtime accounting.
It is exploratory infrastructure validation, not a comparative timing pilot.

Build the shared counter-free runtime and emitter once into a dedicated Cargo
release target. Compile the generic wrapper once. Separately emit and compile four
source artifacts: chain, private payload, nonconsuming subscription, and sixteen
competing multihead rules (`dispatch16`). Generation receives rules, never queries.
The generic wrapper constructs these source rules at runtime and requires no
per-ruleset Rust compilation.

Use `rustc --edition 2024 -C opt-level=3 -C codegen-units=1` for each wrapper/artifact,
with an explicit frozen runtime rlib and dependency directory. This first gate uses
no LTO. Before comparative confirmation, inspect cross-crate primitive calls and
establish a credible optimization configuration, charging any link-time work.
The shared runtime includes experiment/oracle support; bootstrap timing is not an
isolated production-runtime compilation claim.

For each of the four sources, run sizes 0 and 5, with three changing queries per
process. Generic modes: generic, prepared plan, prepared plan plus inferred
specialization. Native modes: generated repair, generic repair, prepared repair,
and inferred specialization with generated repair. All use Global/Indexed, ordinary
allocation and disabled engine/kernel/observation counters. This is 56 processes
and 168 complete query checks. Require independent scalar full-answer agreement,
truthful exhaustion and exact integer phase-accounting sums. Run one native/source
mismatch and require rejection before any query result. Empty/partial phase output,
cutoffs and compiler failures fail the gate.

Record source generation and source-file writing separately; hash emitted source,
shared rlibs, toolchain and executables. Check shared rlib hashes after artifact
compilation to establish that the fixed runtime was not rebuilt. Runtime accounting
separates source construction, preparation, query setup, execution, observation,
engine disposal, answer disposal and prepared-state disposal. Source AST disposal
is included in preparation because preparation consumes it; an independent oracle
source is constructed outside those intervals. Emitted source-buffer disposal is
recorded separately from generation and file writing.
Observation is synchronous full publication, so there is no distinct earlier
partial-answer event in this gate. Independent expected answers and input builders
run outside measured runtime intervals; their allocation/cache interference must
be considered before confirmation. Summed lifecycle excludes validation, input
construction and process startup and must not be reported as process wall time.

Use a 60-second timeout for each build/compile/run and 2,000,000 engine/scalar steps
per query. Rust compilation has a 4 GiB address-space ceiling; runs have 1 GiB.
Compiler ceilings are larger than the ordinary pilot starting bound because LLVM
and linker memory are part of this boundary test. Only one repetition is required:
no variance, speedup, crossover or architectural ranking will be inferred. Any
consequential cutoff or linkage/correctness defect is investigated before a pilot.

After this gate, size the full cost study with prepared and source-eligible
specialized controls, retained controls where source-equivalent, changing reuse,
rule count and adverse work placement. Register its repetitions, practical
thresholds and resource bounds before comparative confirmation.
