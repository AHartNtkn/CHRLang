# Compiler analyses and process-network compilation

Compiler certificates should describe the operation they justify. The newly inspected sources support resource typing and a process-network route to nets, but neither automatically makes a CHR variable store local or its search fair.

## Resource semirings

Ghica and Smith, *Bounded Linear Types in a Resource Semiring* (ESOP 2014), §2.3 Theorem 1, makes inference conditional on a model of generated resource constraints. Its timing case study in §3 uses a recursion-free higher-order language with local store; §3.2 discusses the additional solver work for its chosen semiring. [Institutional author manuscript](https://pure-oai.bham.ac.uk/ws/files/23697858/esop14.pdf)

**Project assessment.** This supports compositional annotations for selected finite services or bounded regions, not a termination claim for recursive synthesis. A certificate for how often a service is used is different from a certificate that its input is independent of a choice. A certificate for finite work per service can support scheduler assumptions without requiring the entire search to terminate.

For a concrete compiler, the resource algebra must charge condition manipulation, aliases, wake-up and allocation. Treating one symbolic operation as unit cost could hide exponential condition growth. A global resource bound would constrain unbounded programs; optional certificates can instead identify safe chunks or eligible regions. An actual inference algorithm should be derived after the service cost vocabulary is fixed, rather than choosing a type system first and counting only costs it expresses conveniently.

## Process networks as an intermediate representation

Mackie, *Compiling Process Networks to Interaction Nets* (TERMGRAPH 2016), §§2–4, maps deterministic processes, FIFO channels and messages to nets. Each channel has one writer and one reader; process states select which input is read. Section 4 sketches the general compilation and supplies concrete examples; Theorem 3 states channel-move correspondence. The paper explicitly assumes fair net evaluation and also presents value attributes and multiple-principal-port shorthand with stated restrictions. [Paper](https://arxiv.org/pdf/1609.03640)

**Project assessment.** A fixed network of a scheduler, binding service, matcher and observation service is a more concrete route than a bare appeal to universality. Requests can carry opaque source handles and contextual support. One writer per request stream can be a dispatcher; logical aliases remain values referring to the same binding-service entry. This routes references without physically duplicating unknown variables.

The route does not make source relations directional by itself. A service can interpret arbitrary relational equations. It does make the service communication protocol directional: request and reply streams have defined endpoints and read order. A direct dataflow compilation that assigns every source variable one producer is a stronger restriction and must not be conflated with a network implementing a general unifier.

Blocking reads introduce a specific hazard. A merge process that alternates between two producer streams can wait forever for an empty stream while another stream has ready work. Replacing that read by “take whichever is ready” changes the Kahn-process assumptions. A safe finite-service protocol can instead use one dispatcher-owned work queue or require each admitted request to return a finite reply, including a suspended result that releases its worker. These options have different parallelism and routing costs.

Dynamic source occurrences can be data records in a fixed network. Dynamic creation of a native net process for each occurrence is another design, whose topology and channel ownership need separate proof. The source paper's fixed-network correspondence does not automatically supply that dynamic compiler.

## Specialization and graph invariants

Béchet's already inspected partial-evaluation paper distinguishes behavioral laws from preservation of termination. Further targeted title searches did not retrieve the promised total-correctness conditions. Therefore those unspecified conditions are not used as an optimization theorem.

A conservative alternative can be justified directly: replace a finite sequence of administrative transitions on a private service state by one macro only when it preserves the external interface and effects, terminates on the declared inputs, and does not hide an unbounded source computation between yields. This allows finite dispatch specialization. It does not justify arbitrary folding that changes productivity or evaluation of active failing constraints.

Banach's static graph-class invariants and Mackie's 1997 distributed-placement analysis remain only partially accessible in the inspected material. Their claims concern graph safety and communication placement, respectively. Neither currently proves a missing source-language eligibility condition in this dossier. Further retrieval is useful when an exact chosen encoding uses those graph classes or requires a placement certificate; historical title coverage alone is not a reason to invent such a dependency.

## Queries and source limits

Targeted searches on 2026-09-06 included `occur check free unification linear disjoint terms nicely moded NSTO Drabent`, `"Partial Evaluation of Interaction Nets" "termination"`, `"Partial Evaluation of Interaction Nets" "conditions"`, `"Mackie" "Static Analysis" "Interaction Nets" 1997 pdf`, `"Ghica" "Smith" "Bounded Linear Types" 2014 pdf`, and `"Compiling Process Networks to Interaction Nets"`. Primary texts and inspected sections are identified above and in T016-net-locality-and-equality.md. Aggregator claims and forum explanations are not evidence for the transfer.

The process-network source adds a candidate compilation route, not another proof that the existing conditional experiment is ready. It should now receive a concrete service-protocol construction and a progress review. The remaining source gaps cannot be called implementation dependencies; their relevance must be decided against that construction.
