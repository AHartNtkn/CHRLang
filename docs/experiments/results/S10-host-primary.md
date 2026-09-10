# The combined host now uses the qualified primary native path

The source-to-owned-wire host passes all 479 query endpoints with native serialization diagnostics disabled. Explicit binary selection and a runtime check prevent diagnostic clocks from entering this primary path. Memory accounting remains unfinished; a source audit identifies the owners it must cover.

## Primary composition evidence

The host takes an explicit native executable argument. The gate checks its bytes against the previously qualified publication-clock primary binary before replay. Host diagnostics record the selected path, and every native query must report null serialization and derived-compute intervals. Supplying the diagnostic-clock binary instead is rejected before host publication, with the temporary artifact cleaned up.

All 26 sessions pass: 22 prepared common-source sessions and four substantive sessions, containing 479 queries. Complete answers match the independent source expectations; every endpoint also preserves the frozen dictionaries, answers, completion flags, service counts and dynamic-word counts. Cancellation and continuing-work cases retain their recorded budgets and pending status. Every temporary root is empty after exit.

Named host intervals remain inside the host elapsed interval, which remains inside parent-observed elapsed. Native internal lifecycle time remains nested inside native process time. None is added twice. These are qualification checks, not comparative timing results. The native executable, source semantics and independent reference are unchanged.

The primary runner no longer needs a repeated answer-serialization clock. It still measures first internal native observation and host phases. External publication remains batch publication after child completion, consumer assembly and artifact disposal; this does not qualify streaming latency or backpressure.

## What the source says about memory

The [source accounting audit](s10-host-primary/ownership-scope.json) records exact source hashes and excerpts for the qualified 64-bit native binary. The quantities below are source-derived requests or reservation capacities, not measured physical memory.

| Owner or mechanism | Source-derived behavior | Required measurement |
|---|---|---|
| Runtime rule book and name-pointer table | Two calloc requests of 128 MiB each | Requested heap traffic/live bytes; physical residency separately |
| Main term heap | A 32-GiB anonymous, no-reserve mapping | Mapping lifecycle and size, touched/used graph words, OS RSS separately |
| Evaluator stack | A separate 32-GiB mapping when initialized; malloc fallback if mapping fails | Which path actually executes, reservation lifetime and physical residency |
| Query reset | Releases evaluator stack and resets the allocation cursor to the prepared seal; retains the main mapping | Per-query graph use versus retained runtime capacity |
| Pending observations and owned wire | Separate allocated queue nodes and growable answer buffers | Canceled work disposal and retained-consumer live bytes |
| Final native disposal | Releases preparation/runtime owners before retained wire consumers | Boundary readings proving both stages of release |
| Python host | Source objects, emitted text, encoded pipes, child captures, dictionaries and owned envelopes overlap | Separate Python-tracked allocation and process RSS; exclude neither by assuming native measurements cover them |

A lower dynamic-word count does not prove a smaller reservation or lower RSS. Conversely, a large no-reserve mapping does not prove that its full address range is physically resident. Requested allocation, mapped address space, graph use and physical residency answer different questions.

## Next experiment and decision value

T078 remains active for separate native/host allocation qualification. Instrument native direct heap requests and mappings in a diagnostic build, with self-checks for allocation, reallocation, strdup, release and mapping failure handling. Replay complete, canceled and changed-query sessions, checking preparation disposal before consumer disposal. Keep the primary binary as the unchanged timing control.

Use Python allocation tracing only as a separately scoped diagnostic; it does not cover all interpreter/libc memory and cannot substitute for native measurements. Collect process-residency evidence separately and name exactly which process and boundary it represents. If tracing perturbs time, those times stay diagnostic. The primary timing comparison must use the qualified ordinary paths.

After this ownership gate, compare the bounded mixed-source cost pilot with T073's learning cost pilot. T078 currently has greater breadth: it can test interactions among matching, equality, choices and complete observation in coherent paths. That priority does not settle either architecture. Compilation, streaming publication, local ownership, constructors and sustained lifetime retain their separate investigations.

## Evidence and reproduction

- [Registration](../registrations/S10-host-primary.md)
- [26 raw sessions](s10-host-primary/runs.jsonl), [source/binary hashes](s10-host-primary/validation.json), [gate log](s10-host-primary/gate.log), [archive audit](s10-host-primary/audit.log)
- [Diagnostic rejection](s10-host-primary/rejection.json), [rejection check](s10-host-primary/reject_diagnostic.py)
- [Allocation/mapping source scope](s10-host-primary/ownership-scope.json), [binary format](s10-host-primary/binary-format.log)

From the repository root, run `python3 research/chr-hvm/host_session/gate.py --output docs/experiments/results/s10-host-primary --native-binary target/s10-publication-clock/native-primary`, then `python3 research/chr-hvm/host_session/audit.py --input docs/experiments/results/s10-host-primary`. The explicit output path keeps separate qualifications distinct. The goal remains active.
