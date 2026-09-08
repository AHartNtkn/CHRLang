# Generated continuations eliminate copied frames and candidate vectors

The source-generated continuation passes the first correctness and work gate. It removes three specific discovery costs while preserving the tested source effects. There is no timing result yet, and T070 remains active.

## What changed

The [generator](../../../research/chr-compiled/src/generate.rs) now emits a continuation type for each rule, with fixed-size occurrence/range arrays and one binding frame. Source-derived assignments reset bindings on backtracking while retaining variables justified by earlier heads or the active anchor. Key expressions are emitted directly from source patterns. The [range primitives](../../../research/chr-compiled/src/native_access.rs) traverse an ordered predicate or index bucket without copying its occurrences into a vector.

Head order, occurrence identity, propagation history and committing source effects remain with the existing executor. Source effects run only after selection returns an application, so the selected store cannot change between range steps. Concurrent mutation would require a different argument. The candidate retains ordinary index repair, equality, argument/constructor copies, one boxed continuation and indirect continuation calls. A source compiler can remove generic matching state without yet eliminating those other responsibilities.

## Evidence and limits

| Check | Observation | What it establishes |
|---|---|---|
| Twelve finite source programs, four sizes, two policies and two access modes | 192 configurations agree on complete answers and exact occurrence traces with generic and existing generated controls; Global also agrees with independent scalar execution | Finite source correspondence under each tested policy; policies are not assumed equivalent |
| Work on that corpus | Native: zero copied binding slots, candidate-vector entries and interpreted key-template visits. Existing generated control: 4,554 copied slots, 2,322 candidate-vector entries and 907 template visits | The intended responsibilities disappear from this path; this does not measure elapsed time or heap savings |
| Explicit branch gate | The native path joins the existing ten-source, two-policy, two-access comparison, adding forty configurations with independent branch replay | Source effects, aliases, failure, multiplicity and the existing finite-sibling witness remain observable |
| Live-demand source gate | The common full-answer checker now compares native and generic occurrence traces for scan/indexed execution alongside the retained/subscription controls | Consumption, propagation, demand retirement, updates and binding-driven eligibility on these sources |
| Independent generated artifact | Two additional guarded/unguarded multihead sources pass 112 changed-query/policy/access checks, including adverse partial matches, late aliases and fresh residual variables | Code is derived without queries and works outside the fixture bundle; Global answers also agree with independent scalar execution |
| Interleaved query lifetimes | Prepared rules may be dropped and another query abandoned while surviving queries finish with their complete answers | Query-local continuation ownership; not a retained-memory or cancellation-latency measurement |

The artifact gate compiles a separate executable against the counter-free runtime. It uses a reusable build cache and makes no claim to isolate compilation cost. Comparisons still need independently charged generation, compilation, artifact size and disposal.

The [validation manifest](s01-native-continuation-gate/validation.json) records the 104-test package run, affected counter-free release suite, final lifetime checks, strict package Clippy and workspace all-target compilation. The [source freeze](s01-native-continuation-gate/freeze.json) identifies the implementation and oracle inputs. The previous S09 factored-comparison freeze now resolves its sources at `dc7ac5d`; its existing analyzer reproduces the recorded classifications. These checks add no comparative timings.

## Faults the gate detects

The [raw records](s01-native-continuation-gate/) include exact mutations, commands, source digests and failure output. Each modified source was restored after its run.

- **Omit rollback:** the chain witness misses a later consuming application.
- **Allow one occurrence to fill two heads:** the propagation witness produces invalid self-tuples.
- **Skip range candidates:** an enabled base application is missed.

Running the mechanism check against the existing generator also fails, with its nonzero frame, pool and template work. The semantic faults fail on source effects, independently of those work assertions. These fault runs used the default debug build; the unmodified gate is additionally checked in counter-free release execution.

## Architectural consequence and next comparison

The generic frame stack and candidate snapshots are implementation choices, not demonstrated necessities of multihead CHR execution. The new candidate must now earn its extra generated code and continuation machinery in a complete cost comparison. Streaming adds repeated tree lookups where a materialized vector supports cheap sequential access; small buckets and dense scans are important contrary cases.

Next establish the independent artifact measurement boundary, source-derived selective update plans and a competent access control using all available endpoint information. Then register counter-free lifecycle timing, separate allocation/work diagnostics, changed-query reuse and compilation crossover. Compare indexed discovery and retained/subscription organizations on both costly rediscovery and cheap/dense maintenance cases. No comparative timing matrix for this candidate has run.

Direct pull-tabbing and derivation reuse remain the strongest distinct ready alternative. Completing this gate does not resolve S01 or change the remaining sequence obligations.
