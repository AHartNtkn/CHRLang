# Local completion signals: a source-backed entry

The direct graph experiment need not assume that trustworthy completion requires
a global scan. Dijkstra and Scholten give a local signalling scheme for diffusing
computations on a finite directed network. Messages create obligations;
acknowledgements discharge them. Nonnegative edge deficits and a rooted engagement
structure connect outstanding work to the initiating environment. The environment
returning to its neutral state certifies termination; after the computation
terminates, signalling eventually returns it there. Nodes must receive service,
and the stated termination argument assumes finitely many computation messages.
The network may include merging and cyclic paths. These are claims about their
protocol, not named-choice execution. [Primary manuscript, EWD687a](https://www.cs.utexas.edu/~EWD/transcriptions/EWD06xx/EWD687a.html), especially P0–P4 and Theorems1–2.

## Consequence for this experiment (inference and proposed work)

An ordinary output-data path is not the only possible connection to an active
obligation. A local acknowledgement network is a credible independent candidate.
It must account for newly created work before acknowledging its creator. A terminal
failure outcome is distinct from successful completion, even when both cease work.

The source does not supply the missing named-choice lifting. One physical task
may serve several interpretations. Acknowledging its work globally cannot certify
an interpretation with an outstanding private descendant. Conversely a permanently
running descendant confined to one alternative must not prevent a completed
sibling's answer. The experiment must preserve those distinctions without assuming
one network copy per assignment.

The initial finite graph fragment can use explicit local completion/validity
connections. Subsequent dynamic work should test acknowledgement transfer when a
node spawns children, merges equivalent work, or loses only part of its valid
interpretations. A global compatibility implementation remains a control. The
relevant comparison includes bookkeeping, local propagation, and publication
costs; topology alone supplies no efficiency conclusion.

This entry identifies a feasible protocol direction. It does not prove the
prototype correct, establish fairness for graph collapse, or settle reclamation.

## First fragment invariant to test (analysis, not a proved implementation)

For any fixed interpretation of active source choices, each obligation denotes
one of pending, failed, or successfully discharged. An inactive obligation is
neutral in the conjunction of active obligations. Combining active obligations
fails if any has failed; otherwise it remains pending while any is pending; only
all-discharge permits success. Thus failure can invalidate an interpretation
without waiting for a divergent companion, while that companion still prevents
success in an interpretation where no failure has occurred.

A distributed graph may encode this function symbolically with choice nodes.
Combining two status choices with the same birth name must pair matching arms;
different names require independent combinations. This statement specifies
projection, not an obligation to materialize the Cartesian product. Choice
restriction can simplify status even when unrelated data remains shared.

Dynamic spawning adds an essential invariant: a parent's completion obligation
must transfer to its children before success can reach publication. An atomic
local rewrite replacing one obligation with a conjunction can satisfy this in a
small graph calculus. An asynchronous implementation needs an equivalent ordering
or acknowledgement argument. A scheduler simply marking the parent done and
later enqueueing children has a false-completion window.

Finite discharge is not the same as a source residual constraint. In full CHR,
a quiescent residual may be a legitimate part of an answer. The first carry/filter
fragment has explicit terminal obligations; extending it to CHR needs a
quiescence certificate including possible wake-ups and retained residuals. It
must not equate every remaining graph node with unfinished computation, nor every
unmatched occurrence with permanent completion.

These invariants give independent assertions for the semantic gate and a concrete
boundary for subsequent source integration. Their physical implementation and
cost remain open.
