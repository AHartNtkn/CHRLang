# A finite net encoding for resumable services

A deterministic finite-control service over encoded data can be compiled to ordinary active-pair rules while keeping source logical variables as data handles. This gives a concrete, conservative route for finite-tree equality and scheduling. It leaves useful locality and performance as separate questions.

This construction supplies more detail than the universal-machine route in T016-net-locality-and-equality.md. It uses the process/channel organization investigated in T016-compiler-source-assessment.md, but states its own restricted compiler discipline instead of assuming that a sketch of arbitrary process compilation proves the CHR services.

## Service data and control language

Encode source terms with a finite backend signature: Ref(id), constructor-tag plus a child list, and list/tuple/Boolean constructors. Source logical variables are Ref values whose IDs are ground data. Bindings, worklists, visited sets, contextual conditions and occurrence records are finite data structures. An unbound source variable is a missing binding entry, not an unconnected backend wire.

A service program has finitely many control locations. Each location inspects exactly one data constructor, retains finitely many other values, and emits a fixed finite graph containing data constructors and a successor control location. Inspection of two operands uses two locations. Comparing arbitrary IDs, scanning a binding table or traversing a term uses a loop of locations, not an unbounded host operation hidden in one rewrite.

No instruction relies on infinitely many numeric agent labels. A counter can use unary or binary constructors. Constructor tags from the finite source program can have finitely many cases; an open atom namespace uses encoded name data and explicit comparison. All copied values are encoded data, including logical-variable handles. Source identity is interpreted by the service, not by identity of their physical copies.

## Translation to active pairs

For each control location q create a controller agent C_q. Its principal port connects to the next inspected data constructor. Its auxiliary ports hold the finite environment, continuation, and required channel endpoints. For each data constructor F that q can inspect, emit one rule for the pair `(C_q,F)`.

The compiler distinguishes duplicable encoded data from linear channel endpoints and control continuations. An instruction that copies or discards a channel endpoint or active continuation is rejected unless a separately specified protocol implements that action. Only immutable encoded data may receive the fan/eraser treatment below.

The right-hand side is the finite instruction graph. It routes F's fields and the saved environment to the successor controller or output data. An external wire used once is routed once; a value used several times passes through an explicit finite fan tree; an unused data value connects to an eraser. There is at most one rule per controller/constructor pair because the instruction's tag cases are disjoint. Missing cases must be deliberately represented as a service error or impossible typed input; they cannot silently stand for CHR failure.

Nested case analysis introduces fresh intermediate controller types. These remember the first result in an auxiliary field while their principal port inspects the next operand. Since the service program and data signature are finite, this transformation introduces finitely many agent types and finite right-hand sides. No rule has to match an arbitrary whole binding table or rewrite distant cells at once.

Duplication here is copying immutable encoded data. Two copies of Ref(i) still refer to the same source variable when interpreted through a binding table. A fan must never turn them into Ref(i_left) and Ref(i_right). Similarly, erasing an encoded temporary is not permission to discard a source obligation still listed in the machine state.

## A resumable equality service

The service continuation explicitly records these stages:

1. Pop an equation from the finite equation worklist, or finish with the private binding table when it is empty.
2. Scan binding entries to dereference the left operand, preserving a traversal continuation; do the same for the right.
3. Compare representatives or constructor tags. Equal representatives continue; a tag clash returns failure; equal constructors push paired children.
4. For an unbound representative X and a term t, traverse t under the same contextual table, maintaining a visited set. Encountering X returns failure. Otherwise append the binding and continue the worklist.
5. Return successful bindings and wake-up information only after the worklist completes. Earlier intermediate tables are private.

Each scan comparison and visited-set operation is itself finite-control microcode. An arbitrary amount of work is therefore represented as many local interactions. At a service boundary, return More(continuation), Success(effect), or Failure(support). More is administrative scheduling information, not a source constraint or a source choice.

The correctness of this algorithm is the finite-tree equation-worklist argument used elsewhere in the dossier. This compiler does not change that algorithm; it makes its data access explicit. An optimized indexed table or contextual simultaneous unifier is another service implementation requiring the same external contract.

## Scheduling without an availability race

Use a coordinator that owns a FIFO of resumable service jobs and chooses one admitted job per turn. It sends a bounded microcode request on one request channel; the service returns a finite response on the reply channel. Suspension returns a response and releases service capacity. It never waits for a future source binding while keeping the request/reply cycle occupied.

This fixed two-process organization avoids the unsafe “alternate reads from possibly empty worker streams” merge. It is deliberately serialized at the service-request level, and therefore supplies a correctness control. Multiple workers can be added by finite rounds in which every admitted request has a finite response, or by a separately justified dispatcher protocol. The language does not gain a source-order guarantee from either implementation.

The coordinator can store the asynchronous symbolic entries from T016-asynchronous-symbolic-scheduler.md. One step of a shared job operates on an encoded support family. The service compiler does not force a ticket per leaf, but neither does it optimize the condition algorithm for free. Constructor dispatch and opaque propagation can be shared through the event graph; exact bookkeeping remains charged work.

## Correspondence and progress

Interpret each controller plus its environment as one service-machine configuration. A controller/data interaction performs its matching tag-case instruction, reconnects the same logical values, and produces the prescribed successor. Fan and eraser steps on encoded immutable data preserve that interpretation up to graph sharing and completed data copying. Thus induction over completed microcode instructions gives the same service trace.

Combining that simulation with the equality algorithm and conditional source-step proof gives source safety for this restricted compilation discipline. The proof relies on finite, correctly translated instruction graphs and preserves source failure/OR through explicit protocol messages. It does not equate an arbitrary backend normal form with a CHR answer.

For a finite service computation on finite data, all data copying and traversal demanded along that computation is finite. A fair active-pair scheduler eventually performs its required interactions. A More boundary cannot contain a recursive source evaluator that might diverge. These premises supply eventual service replies; the coordinator's FIFO discipline then supplies finite-answer progress. A backend that normalizes an entire potentially infinite interpreter result before yielding would not implement this protocol.

## What the encoding establishes

This is a finite compiler recipe and a service protocol, not a minimal hand-optimized active-pair table. It establishes a route that needs neither cyclic source terms, implicit narrowing, nor a ban on aliased source variables. Its conservatism makes the costs visible: encoded IDs, table scans, explicit fans, request/reply traffic, and service serialization.

The decisive direct-net optimization question is how much of this administrative work can be specialized away while retaining the protocol and projection invariants. Region-local dispatch, freshness certificates, ownership, and fixed-topology channels each remove particular work. They do not automatically remove all of it. Measurements should compare this explicit service baseline with optimized direct regions and the graph/multiset engines, rather than using universal expressibility as evidence of net speed.

The remaining validation is to instantiate and check the compiler recipe on concrete microcode and then measure service and sharing costs. Further analysis remains useful for selecting optimizations and reviewing the invariants; a new theorem about whether finite logical-variable services are possible is no longer the missing prerequisite.
