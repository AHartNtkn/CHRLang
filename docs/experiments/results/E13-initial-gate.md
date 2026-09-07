# E13 first specialization gate

A bounded query-entry specializer now implements the registered single-active-call
certificate. Its initial inventory accepts 20 of 64 E00 cases and records explicit
reasons for the other 44. All accepted cases match the recorded observations, raw
success counts and exhaustion/prefix status at expansion budget eight. This is a
coverage/correctness result, not a performance result or a general compiler.

The [inventory](E13-initial-inventory.tsv) includes all arithmetic directions and
subtraction. SK evaluation, type inference/synthesis and lambda examples are not
covered by the first certificate: they involve non-tail calls or structural
multi-rule predicates. Those applications remain important research obligations.

Seven tests cover arithmetic at budgets 0,1,2,4,8; retaining the zero arm for
add(X,Y,s(B)); branch-local bindings and fresh variables; raw duplicate derivations;
passive residual aliases and generated-name collisions; finite-tree occurs failure;
unused active failure/divergence; and interception/non-tail eligibility boundaries.
Compiled programs are executed through the independent reference and compared with
registered or directly checked observations. Compilation keeps binding equations,
preserves every OR and leaves original calls when its global expansion budget ends.

An executable counterexample distinguishes two operational claims. With p <=> p
and q <=> fail, the fixed experimental selector can repeatedly expand p while q
remains in the store. Inlining q's failure into the caller's pending body terminates
that branch. This concerns fixed-selector termination, not refutation of closed
logical finite-answer equivalence or legality under all ordinary CHR schedules.
The first compiler avoids this boundary through its certificate. Broader work can
preserve source events, prove stronger commutation, or explicitly investigate
another permitted schedule; none is closed by the initial restriction.

Next work is a registered scalar cost comparison including compilation/code growth,
followed by broader applicability and variant-policy experiments. Generated variants
are currently limited to one query entry. No language restriction or production
architecture has been adopted. Workspace validation and the inventory are reproducible
with `cargo test -p chr-specialize` and its `inventory` example.
