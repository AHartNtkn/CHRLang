# Rust and native execution now deliver the same owned answer format

The qualified Rust and native paths now export the same binary answer grammar, with complete multiplicity and joint unknown identities preserved. The gate checks 958 native queries and 2,286 admitted Rust configuration/query observations. This establishes a common publication endpoint; it is not a comparative cost result.

## What the consumer owns

Each result consists of encoded answer bytes plus the predicate/atom dictionaries needed to interpret their indices. Outputs remain positional in the query's selected-output order. Residual occurrences remain distinct, including duplicates. Neither encoder sorts answers or performs alpha-canonicalization; those checks happen outside execution.

The dictionary starts with sorted source names and appends sorted query-only names. An earlier-alphabetic query name cannot renumber a source constant. Every Rust dictionary in the gate is checked against the independent source compiler's dictionary, in addition to checking decoded answers.

The Rust consumer object owns its dictionaries and bytes after typed answers are consumed. The common-source adapter retains these objects through disposal of prepared rules, source structures and shape artifacts, then publishes them. A fixed format test independently checks an unknown, an atom and duplicate residual occurrences; another checks dictionary ownership after source/query disposal.

Native publication writes directly into a separately allocated byte vector. The consumer receives that vector and its capacity. No pointer into the native term graph escapes. The ownership build poisons reclaimed query graph storage, retains all requested consumers, frees prepared runtime state, and only then checks and publishes retained bytes. Empty results have an explicit present marker, so zero-capacity storage is not confused with a missing result.

The native consumer's semantic dictionaries still belong to the host-side source/query descriptions. They remain available throughout these checks, but their construction, retention and disposal are not included in native C timings. Complete lifecycle measurement must charge that owner explicitly. Matching bytes alone does not make those costs disappear.

## Native serialization has an explicit allocation owner

The native encoder uses an ordinary growable buffer and transfers ownership directly. It does not use a libc memory stream or copy a printed constructor tree into a second consumer buffer. The diagnostic build accounts for allocated capacity, including unused space, rather than treating logical payload length as requested live memory.

The native reducer and matching program remain unchanged. The ownership observer's complete-root publication call changes to the encoder; the ordinary observer already has a publication callback. Service quotas, call counts, pending status and used graph words match the frozen observations. Timing still separates serialized publication from the combined reduction/traversal service interval.

This supplies a concrete owner for answer-buffer allocation. It does not establish complete heap accounting for frontend I/O, all libc internals or mapped runtime storage, and it does not measure RSS. Those remain explicit parts of the measurement qualification.

## Validation

| Check | Result |
|---|---|
| Prepared native schedules | 383 queries in each of ownership and ordinary builds |
| Substantive native queries | 96 queries in each build, with changing-query reuse |
| Sanitized native ownership | All 383 prepared queries replay exactly, including ownership records |
| Rust encoded observations | 2,286 admitted configuration/query checks pass; 587 admissions remain unsupported |
| Existing ordinary text output | All 264 prior grouped process observations replay exactly |
| Rust semantic tests | 11 pass in both default and no-default-feature builds |
| Strict scoped Clippy, formatting and native builds | Pass; native builds report no warnings |

Every native query checks decoded answer order and multiplicity against its frozen printed observations, including cancellation prefixes. The existing independent source expectations underpin those observations. Every admitted Rust result checks complete decoded multisets, exhaustion or ongoing status, and dictionary contents. The ownership sessions end at zero tracked live bytes after consumer disposal. Ordinary native phase totals and first-observation presence are checked separately.

The sanitizer also reproduces the ordinary ownership build's complete byte output and deterministic ownership records exactly. Historical source and binary snapshots preserve the preceding common-source adapter. The current adapter's ordinary output remains unchanged; binary output is an explicit experiment option.

## Next: qualify the complete Rust/native lifecycle runner

Use this owned endpoint in the timing runner, including per-answer first observation rather than only end-of-query publication. Charge Rust dictionary construction and retention, native host dictionaries, source construction/emission, parsing/preparation, query setup, representation conversion and every disposal. Keep primary ordinary allocation separate from work/allocation diagnostics and verify actual counter configuration.

This is preferable now to another representation expansion: the candidate paths and admitted source-derived controls agree on substantive sources and a common output contract, while total-cost evidence is still absent. Further native local ownership, richer terms, late-readiness solving, general observation and broader architectural alternatives remain required. Reassess breadth at the full cost-runner qualification boundary or a consequential obstruction.

The recorded native timestamps are qualification data. No performance ordering, memory-efficiency ranking or language restriction follows from this package.

## Evidence

[Registration](../registrations/S10-answer-wire.md), [native encoder](../../../research/chr-hvm/answer_wire/wire.h), [native build adaptation](../../../research/chr-hvm/answer_wire/build.py), [Rust encoder and owners](../../../research/chr-direct-conditional/examples/support/answer_wire.rs), [independent decoder](../../../research/chr-hvm/answer_wire/codec.py), [native prepared observations](s10-answer-wire/native-prepared.jsonl), [native substantive observations](s10-answer-wire/native-substantive.jsonl), [Rust observations](s10-answer-wire/rust.jsonl), [sanitized replays](s10-answer-wire/sanitized.jsonl), [ordinary-output replays](s10-answer-wire/ordinary-output-replays.jsonl), [audit](s10-answer-wire/audit.json), [input hashes](s10-answer-wire/validation.json), [default tests](s10-answer-wire/tests-default.log), [no-default-feature tests](s10-answer-wire/tests-off.log) and [Clippy](s10-answer-wire/clippy.log) retain the evidence.
