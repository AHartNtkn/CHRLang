# Complete ordered caller cost pilot

Compare three complete paths: projected accepted-assignment ordering, explicit
Cartesian enumeration with filtering and the same ordering, and existing Direct
source execution. Reuse prepared source/host rules across queries. The first two
use the same Direct consuming caller per admitted assignment. Do not compare
against a deliberately slow reference interpreter in timed intervals.

Hypotheses: contraction avoids failed search but may lose to enumeration because
of its frontier and visited set; preparation may require reuse; cancellation may
benefit from lazy ordering; dense admissibility can favor simpler enumeration.

Matrix: three modes,2/4 coordinates, sparse unequal-name star/dense unconstrained
hidden choices, padding0/3, query counts1/8, immediate/retained answers,
complete/first-answer cancellation. Fix duplicate domain [a,b,c,a], visible first,
token priority2 and plain output; earlier source tests cover the other semantic
variants. This is64 scenarios,192 cells. Validate all full answers against source
reference output outside measurements before any comparative runs.

Enumerating control builds all satisfying index tuples and sorts by summed branch
padding then source indices. Its setup and disposal are charged. Direct mode runs
the original source with only actual caller outputs; tagged reference output
checks raw order outside timing. Projection and enumeration use the same prepared two-argument launch
rule to bind a changing delivered value at its source binding point.

Charge source construction, preparation, query setup, execution/observation,
consumer retention, query disposal, prepared/source disposal and consumer disposal.
Report first-answer time separately. Compiler costs remain outside this pilot.
Primary builds disable engine/kernel counters and use the ordinary allocator;
allocation diagnostics are a separate build. Requested allocation is not RSS.

Two allocation repetitions and five randomized ordinary blocks (seed607107):
384 allocation plus960 ordinary processes. Bound each process to30 CPU seconds,
45 wall seconds and1GiB. Save numerical phase results and compare repeated memory
accounting. Require full release and identical retained answers. Gain/loss requires
at least10% median paired difference and all five blocks in the same direction;
otherwise uncertain. Both medians must exceed the recorded clock floor.
Investigate consequential costs before selecting the next package; no universal
architecture winner follows from this fixture.

## Challenge registered after the initial matrix

Projection qualifies faster than enumeration only in three dense cancellation
scenarios: [n4,dense,padding0,q8,keep1,cancel1] and padding3,q8 with keep0/1.
Dense choices require no solving. Add lazy enumeration using the same ordering
frontier over unconstrained Cartesian choices, deriving multiplicities directly
from domain counts without projection. Filter sparse choices after admission.
Challenge those three scenarios plus their full-output counterparts: six scenarios,
four modes,24 cells,48 allocation and120 ordinary runs, seed607108. Keep all
original modes fresh in this comparison. These runs test whether the apparent
gain belongs to lazy ordering rather than elimination.

The full source-input validation rerun shifts one qualified dense cancellation
case from padding3/keep0 to padding0/keep0. Extend the challenge to all four
padding/retention combinations and their full-output counterparts: eight scenarios,
32 cells,64 allocation and160 ordinary runs. This covers every observed gain,
including both sides of the shift, instead of selecting only stable-looking cases.

Final consumer validation uses a dynamically growing answer buffer in every mode;
no oracle answer count sizes a measured buffer. Re-run both registered matrices
with this shared consumer, keeping the entire dense challenge.
