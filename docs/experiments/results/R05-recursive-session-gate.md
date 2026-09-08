# Recursive executable sessions: common transport validated

Native, direct-equation and current Specialized now serve complete changing queries through the same binary protocol. **All 42 external session responses validate.** T046 remains active for the measurement runner and prospective compilation-lifecycle registration. No comparative timing matrix has run.

[Registration](../registrations/R05-recursive-session-gate.md) defines framing, full observation transport, admission distinctions and the external gate. The [protocol implementation](../../../research/chr-compiled/src/recursive/session.rs) depends only on syntax and the standard library. Generated native, direct and Specialized executables include that same source. The native package has no chr-compiled dependency, verified in its [lockfile](r05-recursive-session-gate/artifacts/native/Cargo.lock).

Each executable receives a malformed complete frame, twelve accepted queries varying depth and result structure, then an unknown-control query. All responses are checked against independent scalar source observations with joint variable identities intact. The malformed request produces its own response and does not prevent subsequent service. Each process reuses its prepared source and exits cleanly after EOF.

Unknown control is explicitly unsupported in the certified native/direct paths. Ordinary Specialized instead returns the source residual observation. This preserves the difference between an optimization admission domain and the language's ordinary behavior. Future comparisons use admitted queries; there is no interpreted fallback in the native path.

## Validation and receipts

The initial API gate failed before implementation. The first external build exposed an incorrect root harness assumption about the observation API; using its actual optional-answer result fixed the harness. [API RED](r05-recursive-session-gate/initial-api-red.log), [first compiler outcome](r05-recursive-session-gate/first-artifact-compile.log), [passing artifact gate](r05-recursive-session-gate/artifact.log). This was correctness compilation, with no timing samples selected or discarded.

Root validation passes **60 package all-target tests in each parent feature configuration**, including five protocol tests and the external session gate: [default](r05-recursive-session-gate/default.log), [counter-free](r05-recursive-session-gate/counter-free.log). Standalone executable dependencies disable defaults in both runs. Strict Clippy passes in [default](r05-recursive-session-gate/clippy-default.log) and [counter-free](r05-recursive-session-gate/clippy-counter-free.log) modes; [formatting](r05-recursive-session-gate/fmt.log) passes. [Artifact audit](r05-recursive-session-gate/artifact-audit.json) retains source hashes and the three generated package manifests, lockfiles and main sources.

Focused tests verify an independently hand-encoded query, UTF-8 names, alias sharing across outputs and duplicate residual constraints, all response kinds, exact payload consumption, malformed-frame recovery, truncation and oversized frame rejection, aggregate entry/depth bounds, output encoding errors and per-response flushing. Independent review found no framing, limit-symmetry or outcome blocker.

## Responsibility and measurement boundary

Protocol limits cover frame bytes, term depth and aggregate term/constraint/output entries. They do not bound backend execution or intermediate term growth. External time and memory limits remain necessary for the pilot. Unencodable answers produce an explicit resource error, never a finite failure or partial success.

The shared protocol introduces decoding, full answer encoding and I/O responsibilities common to the three paths. Source preparation is retained once per process for the controls; the native executable contains generated source operations. An execute call currently bundles backend query construction, execution, kernel export and kernel disposal. Timing that call alone must not be labeled isolated execution or observation. Output/protocol disposal and process startup/shutdown are additional lifecycle costs.

The next work is a reviewable runner and exact prospective pilot registration: shared prerequisite builds versus per-source compilation, compiler/cache conditions, matched sessions, first-answer latency and changed-query reuse, complete validation outside measured intervals, and explicit external resource caps. The process boundary is now executable, so the comparison need not assume an in-process control is equivalent to a generated subprocess. T046 and the broader architecture goal remain active.
