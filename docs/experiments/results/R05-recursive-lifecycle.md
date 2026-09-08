# Recursive lifecycle: checked lowering earns more than native compilation

Checked Direct execution captures most of the complete-session gain over Specialized in the tested recursive families. Native generation improves three Direct session cells with separated ranges, but its additional compilation cost is not recovered relative to Direct within any measured session. T046 is complete; T047 selects an updated architecture checkpoint before another compiler or executor refinement.

## Validated experiment

The [prospective registration](../registrations/R05-recursive-lifecycle.md) and runner were frozen in `182ff29` before comparative execution. All **67 preparation/build stages**, **324 sessions**, and **62,316 responses** validate: 36,396 complete successes and 25,920 finite failures. All 54 cells have five complete primary repetitions, plus validated warmups. There are no missing attempts, compiler failures, transport failures, source mismatches, cutoffs or timeouts. The longest session took 0.772 seconds, below its 30-second cap.

[Live audit](r05-recursive-lifecycle/audit.json), [source/artifact check](r05-recursive-lifecycle/source-check.json), [environment, hashes and exact prewritten orders](r05-recursive-lifecycle/metadata.json), [compiler receipts](r05-recursive-lifecycle/builds.json), [session receipts](r05-recursive-lifecycle/sessions.jsonl), and adjacent compressed raw response streams preserve reproducible evidence. The auditor decodes every full response and compares joint aliases, fresh existentials, output-only variables and finite failure with an independent mathematical finite-tree oracle. Root and independent review checked the runner and resulting interpretation. [Readiness tests](r05-recursive-runner/README.md) preserve both feature-mode semantic gates and failure-path checks.

Both installed controls accept either source family at runtime and prepare it once per session. They do not recompile per source. Native artifacts are generated separately for addition and fresh-pair recursion. All query execution disables diagnostics and uses ordinary allocation. Compiler/build options, offline lock resolution and CPU 0 affinity are recorded. A clean target does not mean cold filesystem or machine caches.

## Complete session results

Milliseconds are median [minimum–maximum] across five primary sessions. These are pipelined sessions, not per-request latency measurements. Each nontrivial reuse cell cycles query depth, payload and result, including failures.

| Family | Depth | Requests | Native ms | Direct ms | Specialized ms |
|---|---:|---:|---:|---:|---:|
| add | 0 | 1 | 2.650 [1.873–2.979] | 2.807 [2.007–3.224] | 2.936 [1.896–3.210] |
| add | 0 | 64 | 2.891 [2.169–3.147] | 2.477 [2.115–2.932] | 3.020 [2.496–3.204] |
| add | 0 | 512 | 3.142 [2.791–3.455] | 3.718 [2.961–4.071] | 3.718 [3.387–4.529] |
| add | 32 | 1 | 2.431 [1.995–2.924] | 2.432 [2.210–4.037] | 2.347 [2.191–2.723] |
| add | 32 | 64 | 4.601 [3.694–5.302] | 4.747 [3.718–5.366] | 7.352 [6.361–7.634] |
| add | 32 | 512 | 15.373 [15.044–16.088] | 17.097 [16.374–29.702] | 39.448 [38.906–41.247] |
| add | 128 | 1 | 2.686 [2.129–3.439] | 2.965 [2.098–8.961] | 2.726 [2.629–3.067] |
| add | 128 | 64 | 9.316 [8.633–9.877] | 9.479 [9.176–9.707] | 42.638 [38.780–48.106] |
| add | 128 | 512 | 55.283 [53.715–56.817] | 60.972 [59.868–63.145] | 306.922 [302.214–312.039] |
| fresh | 0 | 1 | 2.550 [2.258–3.044] | 2.049 [1.980–2.420] | 2.271 [2.134–3.574] |
| fresh | 0 | 64 | 2.447 [1.975–3.026] | 2.540 [2.061–3.048] | 2.751 [2.174–3.110] |
| fresh | 0 | 512 | 3.571 [2.783–3.931] | 3.058 [2.753–3.662] | 3.834 [3.660–4.247] |
| fresh | 32 | 1 | 2.660 [2.042–3.103] | 2.270 [2.088–2.957] | 2.550 [2.420–3.075] |
| fresh | 32 | 64 | 4.470 [3.401–4.873] | 4.681 [4.287–5.075] | 10.387 [10.144–10.684] |
| fresh | 32 | 512 | 14.990 [14.342–15.680] | 15.993 [15.791–17.069] | 69.504 [67.619–70.398] |
| fresh | 128 | 1 | 2.693 [2.464–2.833] | 3.186 [2.503–3.440] | 4.048 [3.331–4.459] |
| fresh | 128 | 64 | 8.379 [7.433–15.263] | 8.997 [8.370–13.629] | 97.148 [93.318–101.241] |
| fresh | 128 | 512 | 48.080 [45.700–55.329] | 52.283 [49.910–52.958] | 763.711 [758.516–771.112] |

Direct is faster than Specialized with separated ranges in all eight nonzero-depth cells with 64 or 512 requests. At depth 128 and 512 requests, addition falls from 306.922 ms to 60.972 ms; fresh-pair recursion falls from 763.711 ms to 52.283 ms. The smaller/cold query cells mostly overlap.

Native improves Direct with separated ranges only on add32/512, add128/512 and fresh32/512. Fresh128/512 overlaps despite its lower native median. The other fifteen Native/Direct comparisons are unresolved under the registered rule, with no separated native regression. No native first-response advantage over Direct is resolved. Native additionally has a separated single-request advantage over Specialized for fresh128, but that does not establish an advantage over Direct or recover compilation.

## Compiler and preparation costs

| Build condition | Median seconds [range] |
|---|---:|
| Native add, clean target | 2.196 [2.175–2.206] |
| Native fresh, clean target | 2.205 [2.204–2.212] |
| Native add, prerequisite-seeded target | 0.497 [0.492–0.501] |
| Native fresh, prerequisite-seeded target | 0.508 [0.505–0.510] |
| Direct installed control, clean target | 7.966 [7.864–7.994] |
| Specialized installed control, clean target | 8.310 [8.245–8.334] |

These are directly measured conditions, not differences between unrelated builds. Dependency seeding itself takes about 1.76 seconds per repetition. It includes the explicit empty-main prerequisite package and is reported separately. The one shared generator installation takes 11.081 seconds in the current package organization. That generator builds through the compiled research crate; this is a current implementation cost, not an inherent lower bound for compiler construction.

Source construction/certification/generation are small: certificate checks take 3.7–4.7 microseconds and Rust emission 7.5–13.5 microseconds in these sources. Generator process wall is about 2.6–3.1 ms; internal total is 0.244–0.305 ms and includes writing all six source/control/prerequisite wrappers and disposal. All 36 lock-resolution process measurements are recorded separately; their combined experimental work is 1.146 seconds, not a per-source cost. [All compiler/preparation samples](r05-recursive-lifecycle/build-summary.json).

Even the lowest seeded native compiler time alone exceeds the highest observed Direct session time in this matrix. Thus a fresh native artifact does not recover its compilation increment over an installed Direct control within the measured reuse range. Fresh128/512 saves enough execution time relative to Specialized to make compiler amortization plausible with shared prerequisites, but Direct already obtains most of that saving without per-source compilation. It is not a reason to choose Native over the stronger control. No untested break-even reuse count is asserted.

Clean native artifact compilation must not be compared with installed-control execution while omitting generator installation, prerequisite state, generation and lock resolution. Conversely, control installation is not charged afresh for every source or session. No native cold-installation victory follows from its smaller artifact build.

## Memory, ownership and remaining work

Measured child peak RSS ranges are 2,048–2,304 KiB for Native, 2,048–2,432 KiB for Direct, and 2,304–2,560 KiB for Specialized. Both checked paths have separated lower RSS ranges than Specialized in each matched cell; Native versus Direct generally overlaps. These process peaks include code and runtime, are not requested heap measurements, and exclude parent response buffers. Native executables are about 538 kB, Direct 613 kB and Specialized 850 kB in this build condition; this does not measure shared installation footprint.

Session wall starts before the common time wrapper/process launch and ends after exit. It includes source preparation in controls, transport, backend import/execution/export/kernel disposal, child output disposal and shutdown. Parent buffers remain for validation and are disposed outside that interval. Internal backend phases remain bundled; these results cannot attribute all savings to a particular kernel operation. The codec's resource limits do not bound intermediate backend term growth.

The certificate is still sealed, single-entry, finite-control recursion with possibly nonground payloads/results. There is no general contextual inlining, unknown-control synthesis or language restriction claim. Native body statements eliminate template handling; Direct has already eliminated source occurrence selection and history. This distinction between checked lowering and native compilation is the architectural finding.

T047 will update the R07 comparison and Q1–Q12 dispositions. It must distinguish certificate benefits from code generation, retain the unresolved current worker benefit, and identify which conditional/explicit conclusions remain mechanism evidence rather than a current comparison after serial changes. Reconcile those consequences before a narrower R04 compilation or kernel refinement. The checkpoint must yield bounded recommendations and a justified next investigation or explicit closure gap; it does not complete the goal by itself.
