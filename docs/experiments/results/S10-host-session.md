# Combined native session accounts for host transport and cleanup

The native candidate now runs from common source text through externally delivered owned answer bytes in one measured host session. All 479 query endpoints preserve the qualified answers and native work signatures. This qualifies the combined runner; no timing comparison between architectures has been run.

## Evidence

All 22 common prepared sessions and four substantive sessions pass. They contain 383 and 96 queries respectively, including changed symbols, complete failure, continuing work and cancellation. The runner reuses one emitted ruleset and native prepared state across each session's changing queries. Every native consumer is retained for the batch endpoint.

Independent decoding checks dictionary contents, joint unknowns, residual multiplicity and completion status against the previously qualified native observations; complete cases also match the independent source expectations. Native service counts, pending/unsupported flags and dynamic graph-word counts match exactly. Every caller-owned temporary root is empty after the host exits, verifying cleanup of emitted artifacts.

The initial executable gate fails on the unimplemented session. The implemented gate and archive audit pass all 26 sessions and 479 endpoints. The reference interpreter and native backend are unchanged.

## Which time means what

| Measurement | Boundary and interpretation |
|---|---|
| Named host phases | Input read/decode/release; source emission; temporary artifact creation/write; query encoding; native process invocation; consumer assembly; prepared/artifact cleanup; external publication; consumer release |
| Native process invocation | Includes spawning the child, pipes, native execution and waiting for completion; native internal phases are nested within it |
| Host-session elapsed | From input loading through publication and consumer release, including orchestration gaps between named phases |
| Parent-observed process elapsed | Surrounds the entire Python host process, including startup/imports, final diagnostic reporting and exit |

The audit checks named phases sum correctly, fit inside host-session elapsed, and fit inside parent-observed elapsed. Native internal lifecycle totals fit inside the invocation phase. They must not be added again to the host total.

The actual host/runtime boundary uses a temporary emitted program, a numeric input pipe and captured native answer bytes. The host constructs dictionary-bearing answer envelopes and writes/flushed bytes to its caller. Those copies, artifact operations and transport are now inside measured intervals. The lifecycle therefore describes this concrete implementation, not an intrinsic cost of native graph execution or a prediction about an in-process embedding.

## Observation and ownership contract

This is a batch external endpoint. The native child retains answers until its prepared state is disposed; the host keeps dictionaries alive through child termination, assembles owned envelopes, disposes of frontend state and the temporary artifact, then publishes and releases its consumers. The parent receives the same WIRE format used by the Rust lifecycle runner.

Native internal first-answer timestamps still mean bytes available within the native runner. They are not external first-answer latency. Streaming publication and backpressure require a different explicit service protocol and their own comparison. This runner does not claim those properties.

## Consequence and remaining work

A native comparison no longer needs to omit host artifact writing, process transport or consumer publication from elapsed costs. Both internal phase diagnostics and gross process elapsed are available, so later experiments can distinguish execution from the chosen host/runtime arrangement. One correctness observation per session supplies no stable cost ordering.

Separate allocation diagnostics and clock-overhead calibration remain required before comparative registration. Account for Python and native owners separately, including interpreter state, requested heap, native virtual mappings and RSS. The fixed runtime binary is reused; independent user-program compilation, loading and artifact lifetime still need their own experiment. Do not infer compilation superiority from emission of native-language source.

T078 remains active. At full measurement qualification, compare one bounded mixed-source pilot with broader direct source analysis under the [current sequence](../next-cycle.md). Broader graph ownership, constructors, sustained lifetime and other mapped investigations remain required.

## Reproduction

The [registration](../registrations/S10-host-session.md) fixes the endpoint, corpus, budgets and resource limits. [The scripts](../../../research/chr-hvm/host_session/) contain the runnable host, execution gate and archive audit. Run `python3 research/chr-hvm/host_session/gate.py` and then `python3 research/chr-hvm/host_session/audit.py` from the repository root.

The [evidence directory](s10-host-session/) records commands, complete outputs, phase records, process elapsed time, cleanup checks and source/input/binary hashes. Outer hosts are bounded at 30 seconds wall and 20 CPU seconds; native children retain 15 seconds wall and 10 CPU seconds. Both permit the qualified 96-GiB virtual mapping allowance. Recorded query service bounds remain unchanged, and pending work remains explicit.
