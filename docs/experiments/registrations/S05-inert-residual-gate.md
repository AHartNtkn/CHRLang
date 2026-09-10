# Source-derived inert residual separation

T075 package three after the adaptive breadth review. Previous source gates establish missed reuse between distinct inert caller tags and reject atomic effect contraction. Implement per-derivation output ownership while preserving resumable active execution; test semantics before any cost claim.

## Mechanism and competing designs

A ground constraint whose name/arity appears in no kept or removed head of the fixed source program cannot affect future matching, consuming execution or bindings. Its actual value and multiplicity still belong to its derivation's output. Detach such live constraints into owned residual data after each logical source transition. Keep active constraints, variable-bearing observations, pending goals, bindings and history in execution. A term that becomes ground may become eligible later. No source-language restriction is introduced.

The candidate tables alpha/live-history active states and caches one-step edges with newly detached output data. Each FIFO job retains its own previous residual data. Split copies that ownership; failure discards it; answer delivery reattaches it. Preserve one logical service opportunity per source step. The extraction API derives its readable signatures from its own fixed machine rules and checks cursor ownership; callers cannot assert inertness themselves. Initial implementation may scan/export candidates; charge that work in future costs rather than assume economical inference.

Controls: existing Direct and AlphaLive whole-state execution; separated execution without memoization; separated memo execution. H1: distinct ground inert callers can reuse transitions with identical per-step outputs/exhaustion. H2: readable signatures, variable-sharing observations, arity distinctions and late bindings cannot be treated as inert without their dependencies. H3: dynamically emitted inert data, duplicate residuals, failure and cancellation retain correct owners.

## Tests and bounds

First write a behavioral test requiring different caller tags to obtain reuse and preserve the actual tags; observe it fail before implementation. Then compare all four paths on 3 work depths (0,1,4), 2 variable namespaces, 2 equal/distinct tags, and 6 cases: inert ground output, readable caller, variable-bearing caller with late binding, same name at a different readable arity, dynamic duplicate output, and early failed branch. This is 72 source configurations. Verify every FIFO answer and exhaustion against Direct, and complete raw multisets against the unchanged independent scalar evaluator. Record actual hits/executed work with engine metrics enabled; repeat semantics metrics-off.

Include explicit consuming competition and propagation history controls, a wrong-owner extraction rejection, retained answers after preparation/search disposal, and partial search disposal followed by reuse at budgets 0/1/5. Preserve source snapshots for modified persistent interfaces and hashes for current sources and evidence. Test suites have 60 seconds wall/CPU and 1 GiB address space per executable, 200,000 logical steps per query. Compile outside those execution bounds. No timing, heap or broad architecture selection.

Any output/order/history defect requires repair with the failing witness retained. A positive source gate only qualifies this separation; wider variable dependencies and general call-local effects remain required. Next compare full ownership/allocation qualification against adaptive timing attribution, native ownership, conditional lifetime and incremental projection. The full breadth review is due after one further independently counted package, even if attribution or repair is needed.
