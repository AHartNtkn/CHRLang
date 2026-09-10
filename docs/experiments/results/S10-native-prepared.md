# Native rules can be reused while query storage is reclaimed

All 22 common rulesets now execute changing native queries without recompiling or reparsing their rules. The final gate passes 383 query runs, retains answers through prepared-state disposal, and reproduces three successive sessions in one process. This supplies the missing native reuse boundary for a lifecycle comparison; it establishes no performance advantage.

## What preparation and queries now own

Preparation emits the existing rule selector with an empty query, then parses its definitions once. Source predicate and atom IDs form stable dictionary prefixes. Query-only names receive additional IDs in per-query dictionaries, so they cannot renumber source constants.

Each query builds native occurrence, argument, output-variable and initial-state data directly, then applies the prepared selector. Matching and body functions are unchanged. Query construction does not emit or parse another matching program, and preparation does not execute the source or precompute answers.

| Owner | Contents | Disposal evidence |
|---|---|---|
| Prepared rules | Parsed definitions, source dictionaries, native symbol names and parser paths | Heap cells and definition entries remain unchanged across queries; names and paths are explicitly released at session end |
| Query | Native data and reduction allocations, pending observer frames and reduction stack | Observer frames and stack are released; the used query region is poisoned before restoring the allocation frontier |
| Consumer | Serialized complete-answer bytes | Bytes remain valid through later queries and after prepared definitions, tables and heap have been released |

This ownership contract exports copied answer bytes. It does not permit a consumer to retain an internal native term pointer across query reset. Retained native graph handles, simultaneous queries and concurrent sessions would need different ownership evidence; they remain open design alternatives.

## Why the frontier reset is justified in this gate

Before resetting query storage, the harness compares every prepared heap cell and definition entry with its preparation snapshot. It also checks symbol-table length and name hashes, verifies the reduction stack is released, and frees all observer frames. The snapshot is used only for comparison; it is never copied back to conceal a mutation.

The harness then poisons the entire used dynamic region and restores the allocation frontier. Later queries must still produce the independently expected answers. Retained answer buffers are checked and published after prepared-state disposal, when the native heap and symbol tables are unavailable. These checks directly test the required responsibility boundary rather than assuming that moving an allocation pointer proves safety.

This is evidence for the generated programs and query schedules tested here. It is not a proof that arbitrary native programs cannot mutate prepared storage or retain hidden roots.

## The session cleanup defect was real and bounded

The existing process-oriented teardown releases its table arrays and mapped heap but leaves interned name strings and parser path records allocated. The recorded failing check leaves exactly 1,314 requested bytes; summing those names and paths gives the same number.

The experimental session owner now releases those objects and resets the associated global lengths and parser state before another preparation. The frozen reducer source is unchanged. The first, last and first ruleset are then prepared and disposed in the same process. All 50 queries in that sequence reproduce their standalone outputs and ownership records exactly, with zero tracked live bytes at each session boundary.

This corrects the ownership needed for reusable sessions. It does not show that process-based execution previously accumulated those objects after process exit, or that the native architecture requires such retention.

## What the query challenges establish

The 383-query schedule contains 295 complete/progress checks and 88 cancellations. It runs every common case twice, retains first-round answers, adds a new-symbol query to every ruleset, and reruns the first case after cancellation. All previous 382 query observations replay exactly after the final symbol challenge is added.

The 23 new-symbol cases also agree with the unchanged reference. Twenty-two add an unused predicate, new atom and selected unknown. The remaining case puts a new atom into an actively matched choice input; both attempted source bindings conflict, yielding complete failure. That catches a dictionary collision that an unused-symbol observation alone might miss.

Prepared storage ranges from 1,769 to 4,512 native heap words in this gate. Peak used query storage is 2,305,839 words across the bounded schedules. These are arena word counts, not RSS, live reachable graph size, or allocator traffic. The large peak includes service of continuing work before cancellation.

## Validation and measurement limits

The independent audit reconstructs rules-only preparation, runtime query encodings, dictionary prefixes, complete answer multisets, exhaustion/pending status, cancellation outputs, retained-byte counts and session disposal. It verifies all input hashes, 382 earlier query replays and the three-session comparison. A separate undefined-behavior-sanitized build reproduces all 383 outputs and ownership records without diagnostics. The ordinary harness builds with `-Wall` and no warnings.

Requested-live tracking covers explicit runtime and harness allocation calls. Libc stream internals and virtual mappings are outside that counter. Every query restores its tracked baseline after accounting for retained consumer bytes; every session reaches zero tracked live bytes after consumer disposal. This is not a complete resident-memory measurement.

The pinned runtime reserves large virtual heap/stack regions and allocates fixed-capacity definition/name-table arrays. The arrays alone request 256 MiB, although the useful prepared graph in these examples is much smaller. Requested capacity, used arena words and physical residency must remain separate in the cost study. These implementation choices are not architectural lower bounds.

Allocator wrappers, static snapshots, retained-byte checks and poisoning are diagnostic overhead. The primary timing runner must disable diagnostic work without changing query ownership or native service semantics. The gate's quota-based visits are part of interruptible execution, not interchangeable with Rust engine steps. No comparative timing or allocation-efficiency conclusion follows from this package.

## Next: implement and qualify complete-cost measurement

Continue T078 with an ordinary-allocator timing build and separate allocation/work diagnostics using this preparation/query boundary. Preserve the ownership-gate build as a control. Qualify complete answers outside measured intervals for every intended size and query-reuse setting before registering comparative runs.

Measure source construction and emission, native parsing/loading, preparation, query construction/setup, execution, first/full observation, cancellation and disposal separately. The native output path currently serializes answers; any comparison must account for equivalent externally owned observations or state the representation difference explicitly. Prepared graphs, shape-specific lowering artifacts and retained consumers must be charged to their actual lifetimes.

The common-source gate already provides conventional Scan/Indexed, specialization, contextual/conditional paths and applicable prefix/finite-solving competitors. Extend the cost sources to vary useful work and sharing independently of size; the tiny identity obligations alone cannot justify architectural cost conclusions. Investigate consequential overheads before rejecting a design, and do not assume native runtime compilation is the same as source-program compilation.

This is now a credible next comparison relative to further native feature work: both sides have a demonstrated query-reuse boundary. General terms, local claims, integrated language support, learning/reuse and broader retained-output ownership remain required. The research goal remains active.

## Evidence

[Registration and ownership extensions](../registrations/S10-native-prepared.md), [session harness](../../../research/chr-hvm/prepared/harness.c), [query gate](../../../research/chr-hvm/prepared/gate.py), [audit](s10-native-prepared/audit.json), [query plans](s10-native-prepared/plans.json), [run records](s10-native-prepared/runs.jsonl), [same-process sessions](s10-native-prepared/multi-session.json), [new-symbol reference checks](s10-native-prepared/symbol-references.json), [teardown residual](s10-native-prepared/red.json), [sanitized replays](s10-native-prepared/ubsan.json), [input hashes](s10-native-prepared/validation.json), and [build](s10-native-prepared/build.json) retain the evidence.
