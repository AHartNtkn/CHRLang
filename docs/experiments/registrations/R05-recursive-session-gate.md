# Shared recursive executable session gate

T046 needs matching process boundaries before compilation-lifecycle comparisons. This correctness gate extends the validated native artifact with one source-shared protocol for native, direct-equation and current Specialized executables. No comparative timings are registered here.

## Wire and outcome contract

Each request and response is one little-endian u32 payload length followed by those bytes. A complete malformed request produces one `Malformed` response and service continues with the next frame. Partial headers/payloads and oversized envelopes stop with a transport error; they cannot be treated as clean EOF, source failure or the next frame. Clean EOF ends the session.

A query payload contains a u32 constraint count, each constraint's UTF-8 name and argument terms, then a u32 output count with each name and u64 variable ID. Strings have u32 byte lengths; term/argument sequences have u32 lengths. A term is tag 0 plus u64 variable ID, or tag 1 plus constructor name and argument sequence. A successful answer contains the full ordered outputs followed by residual constraints, using the same shared variable IDs throughout. Exact payload consumption is required.

Response tags distinguish success (0), finite failure (1), unsupported query (2), malformed request (3), and execution/resource error (4). Error variants carry a UTF-8 message. Codec limits are transport constraints, not language restrictions: 1 MiB frame payload, term depth 512 and 100,000 aggregate term, constraint and output entries. Encoding and decoding must enforce equivalent limits. An answer exceeding encoding limits is an explicit error, never finite failure or a partial answer.

## External correspondence gate

Build three executable packages with the installed toolchain offline. Native depends only on syntax/persistent services; direct and Specialized depend on their current compiled package. All include the same authoritative protocol source. The source fixture is shared, and a native module is generated from its existing certificate. Preparation is reused within each control process.

Send one malformed complete payload, then twelve changing accepted queries (four depths and three result shapes), then an unknown-control query. Read every full framed response and require clean process completion. The **42 responses across three executables** must preserve source observations and prove the malformed payload did not terminate or desynchronize the session.

Native/direct report unknown control as unsupported. Ordinary Specialized preserves its source residual answer on that query. This distinction is intentional: the certificate is a sufficient optimization domain, not a change to ordinary source semantics. Comparative queries will be admitted by every selected backend. Expected accepted and ordinary residual observations come from independent scalar execution, with full joint-variable comparison.

Focused protocol tests additionally cover hand-encoded vectors, aliases across outputs/residuals, truncation, trailing bytes, invalid UTF-8/tags, aggregate limits and response encoding failure. Existing native-generation and mathematical correspondence suites remain required. Run package tests in both feature modes, formatting and strict Clippy. Preserve errors rather than introduce a backend fallback.

## Measurement dependency

The gate establishes actual session reuse, complete transport and observable error boundaries. It does not isolate compiler time or measure a backend ranking. The later pilot must record shared prerequisites, per-source preparation/compiler work and complete session lifecycle, with the same protocol/process organization. Any bundled execution/export/kernel-disposal interval must be named accurately; a single API timer cannot prove independent phase costs. Exact comparative repetitions, caps, workloads and interpretations remain to be registered after the measurement runner is reviewable.
