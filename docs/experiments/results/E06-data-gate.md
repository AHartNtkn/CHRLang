# E06a: finite data service conformance gate

`research/chr-nets/net.py` now generates 161 rules over 30 agent types and executes
literal port-graph replacement. There is no host callback implementing equality
or lookup inside an interaction. The eight-test suite passes, including 18,818
ordered data comparisons: 97 values squared, under FIFO and newest-first order.
Run `PYTHONDONTWRITEBYTECODE=1 python -m unittest discover -s research/chr-nets -v`.

The suite validates the complete graph and active queue after each interaction.
It also checks copying/erasure, composed copy then comparison, table preservation
with duplicate keys, invalid rule interfaces, an internal boundary wire, and
resuming unary comparison at quanta 1, 3 and 16. Comparing two depth-64 unary
values takes 130 interactions at every quantum. Direct control generators cover
all 45 registered lookup requests; a small instrumented three-mode run checks the
measurement interface. These are conformance checks, not comparative cost results.

## Correspondence argument for this gate

A rule consumes two connected principal-port agents. Its auxiliary boundary is
reconnected through a statically validated linear RHS. Internal boundary wires
are followed when gluing; no endpoint is copied implicitly. The generated system
has one rule per unordered pair and no symmetric self-pairs. Runtime invariants
check every live port has exactly one reciprocal connection and every reducible
pair occurs exactly once in the queue.

Eq consumes one root and saves its children in MatchTag, which consumes the other
root. Unequal constructors produce false and explicit erasers for unused fields.
Equal nullary constructors produce true; unary constructors recurse; binary
constructors compare both children and combine Booleans. Structural induction on
finite input data proves the returned Boolean equals structural equality. An
unused pending comparison still completes before its result meets an eraser.
There are no cyclic input data or recursive controllers on unchanged arguments.

Dup distributes over each data constructor and duplicates every child through
another explicit Dup. Erase distributes similarly. Induction preserves the
encoded value in both copies, including Ref's numeric handle. This is immutable
data copying, not creation of a fresh source logical variable.

Look consumes successive Cons entries, Entry exposes each key/value pair, and
Decide either returns Some(value) or continues to the strict table tail. Eq tests
the encoded keys. The caller's Dup preserves a complete table copy, so the result
is first-match lookup plus the unchanged table. Finite list length and finite
data comparisons/copies establish termination on well-formed requests.

Every step handles a bounded rule interface and finite RHS; advance can return
between any two interactions. Result decoding rejects pending work and any stuck
controller. This does not prove unbounded source-search fairness. Slot metadata
currently accumulates while live graph agents are reclaimed; measurements expose
both quantities. Python and the generic graph interpreter are not a native backend.

Transactional unification, dereferencing, occurs traversal, source regions,
additional graph protocols, encoded backend execution and native choice
correspondence remain open and feasible. No language contract is adopted.
