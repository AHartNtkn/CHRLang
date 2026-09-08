# Arena ownership comparison design (T052)

The [owner investigation](R03-fork-owner.md) justifies one narrow intervention:
share immutable arena data across search branches and detach only on actual
insertion. It does not justify changing occurrence ownership, scheduling or
bindings. The strongest alternative remains pure-carrier contraction; this
comparison is selected because existing measurements identify both copied bytes
and continuations that need no arena mutation.

## Ownership contract

A private ArenaData owns nodes, closedness, the intern table, predicates and the
predicate lookup. The candidate stores one Rc<ArenaData>; the control owns the
same data directly. Immutable lookup precedes Rc::make_mut. Inserting a term or
predicate passes through one controlled mutation boundary. Stable node indices
and closedness move together; variable bindings remain independently persistent.
Predicate access is read-only, with consuming ownership transfer during
preparation to avoid an incidental dictionary copy.

The experimental build feature selects the ownership contrast; it is not an
adopted language policy. Fork diagnostics name the actual owner: the control's
five containers or the candidate's shared handle. Candidate detachment occurs
inside the existing service interval and must not be called a fork allocation.
Diagnostic fields and checkpoints remain outside primary builds.

## Source gates and adverse control

The state-preservation read case keeps its full independent answer oracle.
A second source case must insert constructors from a rule fired after a fork,
so query setup cannot pre-intern the inserted value. Exercise distinct siblings,
mutation before a later failed equation, and a subsequent fork inheriting the
new value. Verify complete exported terms and aliases, not only allocator or
reference-count state. Predicate insertion must preserve sibling dictionaries.
The no-choice variant tests insertion when the arena has a unique owner.

The lifecycle matrix must include both read-preserving and immediate-insertion
cases, both mostly-fail and all-success outcomes, cold and reused preparation,
and size-zero/no-choice controls. All generated observations are validated outside
timing intervals. Insertion, lookup indirection, ownership counts, detachment,
full observation and disposal are part of the comparison.

## Intended bounded comparison

A prospective registration will freeze exact source APIs and commands after the
semantic gate. The intended screen uses n0/512 and a1/64, both outcomes, q1/4,
and both mutation placements:32 cells per ownership configuration. For each
configuration, one warmup, five primary repetitions, one allocation run and one
work run would total512 fresh processes. These extremes discriminate fixed
ownership overhead from avoiding substantial state copies without a new tuning
sweep. A middle-size follow-up requires a consequential unresolved crossover.

Primary builds disable engine, kernel and observer counters and use the ordinary
allocator. Diagnostic builds are separate. Compare complete per-query lifecycle
including amortized preparation/disposal at the registered reuse; retain first
observation separately. Do not derive speedups from diagnostic owner fractions,
compare against a previous source freeze, infer workload weights or claim complete
architectural lifecycle superiority without credible compilation accounting.


## Validated entry

The candidate and source gate pass default, counter-free, COW and COW+fork
configurations. Independent review found no semantic or preparation-comparability
blocker. Source replay validates18 changed-query insertion cases, failure after
construction and re-fork after insertion. Kernel tests check actual sharing on
hits, node/predicate detachment, stable IDs and unique-owner insertion. This Rc
candidate is a serial ownership experiment; it supplies no cross-thread result.

Root workspace tests, runner complete/cutoff checks, actual work/allocation
insertion smokes, adverse analyzer checks, strict Clippy and formatting pass.
Receipts are in [r03-arena-ownership-gate](r03-arena-ownership-gate/).
The [prospective registration](../registrations/R03-arena-ownership.md) fixes the
512-process comparison before execution.
