# No-binding certificate: prospective admission and lifecycle pilot

Hypotheses: omitted persistent dependencies reduce allocation on blocked/read-heavy matching and constructor rewriting; checking and feature layout can add overhead on tiny or uncertified sources; declarations provide the same runtime shortcut as inference on accepted sources. No mandatory restriction or speed policy is assumed.

## Sources and controls

Three full CHR families in effect_source.rs: blocked `p(X)` requests awaiting `a`; constructor rewriting with blocked requests plus a finite countdown constructing nested `wrap` terms and consuming a token at finish; an actual writer `bind(X) <=> X=a` that wakes and consumes the requests. All queries expose the joint alias of two output names and a residual tag. Source rules are prepared once for changing queries. Work=4,payload=8,resource=true. Query depths increase n..n+reuse−1 and tags/input order alternate.

Controls: feature-off ordinary conditional; feature-on ordinary conditional; feature-on inferred, checked declaration and required declaration; feature-off conventional Scan and inferred specialization. All conditional controls use head-dispatch, serial-body-accounting and combined Boolean identities/result reuse. No-binding certification must add its benefit beyond those controls. All builds use precise equality invalidation and disabled engine/kernel counters for costs.

The two declaration controls reject the writer and therefore receive no executed-query cell there. Inference executes it through the ordinary dependency path. A separate admission-only group measures ordinary off/on and inferred/declared/required on the writer, including source construction, preparation/checking and disposal. Rejection is an admission result, never a fast completed query.

Six lifecycle cases per eligible family/control: (n0,reuse1,keep0); (n64,reuse1,keep0); (n64,reuse4,keep0); (n64,reuse4,keepall); (n64,reuse4,fail,keep0); (n64,reuse4,cancel,keepall). Cancellation stops query0 after100 engine service calls, after which later queries complete. It is a disposal/progress endpoint, not equal completed work across engines.

There are84 immutable-source cells plus30 writer cells =114 lifecycle cells, and five admission-only cells. Run two allocation repetitions(seed7930) and five ordinary timing repetitions(seed7931):238 allocation and595 timing processes,833 total. Randomize within each kind and run sequentially. No extra warmups beyond the disclosed complete analytical/scalar/engine replay before every lifecycle measurement. Timings describe warmed within-process lifecycle, not cold-cache performance.

## Accounting and interpretation

Measure source construction, preparation/checking, each input/setup/execution-observation/engine disposal, consumer release and prepared disposal. First observation is a contained subinterval. Fixed harness buffers, process startup, independent validation and native compilation are excluded. Requested heap bytes are not RSS. Validate full analytical and independent scalar answers; check complete or cancelled endpoints and final owner restoration. Compare consumer and engine owners independently.

Admission-only measurements allocate and dispose the actual accepted prepared owner or rejection result. No fabricated engine run follows rejection. Accepted does not mean certified: inferred writer admission succeeds without the no-binding shortcut.

Use1GiB address-space,60CPU-second,75wall-second and2,000,000-service-call/query limits. Preserve and investigate any cutoff before changing bounds. Freeze sources, manifests, binaries, toolchain and driver before comparisons. Allocation repetitions must replay exactly; compare feature-off/on ordinary owners and normalize only external root baselines when checking inferred/declared/required equality.

Report exact bytes/owner counts and timing medians/min/max. Five timing samples are exploratory; no significance or universal policy claim follows. Attribute consequential results to checking, setup, execution and disposal. If a material decision remains sensitive to measurement uncertainty, prospectively register confirmation.

This is the fourth package since the order-lifecycle breadth review. At completion compare further effect precision/cost refinement with direct source-derived solving and other ready distinct mechanisms. Separate semantic expressiveness from checker conservatism: writer outputs require bindings under the current observer, but rejected reflexive/unreachable equations do not prove actual binding effects. Broader effects, non-overlap, native compilation and coherent architecture selection remain required.
